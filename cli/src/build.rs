use anyhow::{Result, Context};
use std::path::{Path, PathBuf};
use log::info;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use chrono::{DateTime, Utc};

use crate::config::Config;

pub struct BuildArtifact {
    pub program_binary: PathBuf,
    pub metadata: BuildMetadata,
}

/// Metadata about the build process and resulting artifact
#[derive(Debug)]
pub struct BuildMetadata {
    /// Build timestamp for tracking deployment history
    #[allow(dead_code)]
    pub timestamp: DateTime<Utc>,
    pub optimization_level: String,
    pub target: String,
}

pub struct Builder {
    config: Config,
    project_path: PathBuf,
    build_dir: PathBuf,
}

impl Builder {
    pub fn new(config: Config, project_path: PathBuf) -> Self {
        let build_dir = project_path.join("build");
        Builder {
            config,
            project_path,
            build_dir,
        }
    }

    pub async fn build(&self) -> Result<BuildArtifact> {
        info!("Starting build process...");
        
        let pb = ProgressBar::new(4);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
                .unwrap()
                .progress_chars("=>-"),
        );

        // Step 1: Prepare build directory
        pb.set_message("Preparing build environment");
        self.prepare_build_dir()?;
        pb.inc(1);

        // Step 2: Compile source files
        pb.set_message("Compiling source files");
        let compiled_files = self.compile_sources()?;
        pb.inc(1);

        // Step 3: Run optimization passes
        pb.set_message("Running optimization passes");
        let optimized_binary = self.optimize(compiled_files)?;
        pb.inc(1);

        // Step 4: Generate build artifacts
        pb.set_message("Generating build artifacts");
        let artifact = self.generate_artifact(optimized_binary)?;
        pb.inc(1);

        pb.finish_with_message("Build completed successfully");

        Ok(artifact)
    }

    fn prepare_build_dir(&self) -> Result<()> {
        if self.build_dir.exists() {
            std::fs::remove_dir_all(&self.build_dir)
                .context("Failed to clean build directory")?;
        }
        std::fs::create_dir_all(&self.build_dir)
            .context("Failed to create build directory")?;
        Ok(())
    }

    fn compile_sources(&self) -> Result<Vec<PathBuf>> {
        let mut compiled_files = Vec::new();
        let src_dir = self.project_path.join("src");
        
        for entry in std::fs::read_dir(src_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("less") {
                let output_path = self.build_dir.join(
                    path.file_name().unwrap().to_str().unwrap()
                        .replace(".less", ".o")
                );
                
                // Read the source file
                let source = std::fs::read_to_string(&path)
                    .context("Failed to read source file")?;
                
                // Parse and compile the source code
                let program = lessvm_solana::compile(&source)
                    .context("Failed to compile source file")?;
                
                // Write the compiled bytecode
                std::fs::write(&output_path, program)
                    .context("Failed to write compiled file")?;
                
                compiled_files.push(output_path);
            }
        }

        Ok(compiled_files)
    }

    fn optimize(&self, input_files: Vec<PathBuf>) -> Result<PathBuf> {
        let output_path = self.build_dir.join("program.bin");
        
        // Link the compiled files into a single program
        let mut program = Vec::new();
        for file in input_files {
            let bytecode = std::fs::read(&file)
                .context("Failed to read compiled file")?;
            program.extend(bytecode);
        }

        // Apply optimization passes based on level
        let optimized = match self.config.build.optimization_level.as_str() {
            "release" => lessvm_solana::optimize(&program, true)?,
            _ => lessvm_solana::optimize(&program, false)?,
        };
        
        // Write the final program
        std::fs::write(&output_path, optimized)
            .context("Failed to write optimized binary")?;

        Ok(output_path)
    }

    fn generate_artifact(&self, binary_path: PathBuf) -> Result<BuildArtifact> {
        let metadata = BuildMetadata {
            timestamp: Utc::now(),
            optimization_level: self.config.build.optimization_level.clone(),
            target: self.config.build.target.clone(),
        };

        Ok(BuildArtifact {
            program_binary: binary_path,
            metadata,
        })
    }
}

pub async fn build_project(path: &Path) -> Result<BuildArtifact> {
    let config = Config::load(path)?;
    let builder = Builder::new(config, path.to_path_buf());
    
    println!("{} Building LessVM project", "→".blue());
    let artifact = builder.build().await?;
    
    println!("\n{} Build completed successfully!", "✓".green());
    println!("  Target: {}", artifact.metadata.target);
    println!("  Optimization: {}", artifact.metadata.optimization_level);
    println!("  Output: {}", artifact.program_binary.display());

    Ok(artifact)
}