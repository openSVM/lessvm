use anyhow::{Result, Context};
use std::path::{Path, PathBuf};
use std::fs;
use log::info;
use colored::*;

use crate::config::{Config, init_config};

const BASIC_TEMPLATE: &str = r#"
// main.less
fn main() {
    print("Hello from LessVM!");
}
"#;

const TEMPLATE_FILES: &[(&str, &str)] = &[
    ("src/main.less", BASIC_TEMPLATE),
    ("tests/main_test.less", "// Add your tests here"),
    ("README.md", "# LessVM Project\n\nCreated with lessvm-cli"),
];

/// Represents a LessVM project with its configuration and file structure
pub struct Project {
    pub path: PathBuf,
    /// Project configuration for build, deployment, and runtime settings
    /// Currently used during project creation and validation
    /// Will be used more extensively in future implementations
    #[allow(dead_code)]
    config: Config,
}

impl Project {
    pub fn create(name: &str, template: &str) -> Result<Self> {
        let project_path = PathBuf::from(name);
        if project_path.exists() {
            return Err(anyhow::anyhow!("Project directory already exists"));
        }

        // Create project directory structure
        create_project_structure(&project_path)?;

        // Initialize project with template
        apply_template(&project_path, template)?;

        // Create and save config
        let config = init_config(name, template, &project_path)?;

        Ok(Project {
            path: project_path,
            config,
        })
    }

    pub fn validate(&self) -> Result<()> {
        // Check for required files
        let required_files = [
            "src/main.less",
            "lessvm.toml",
        ];

        for file in required_files {
            let file_path = self.path.join(file);
            if !file_path.exists() {
                return Err(anyhow::anyhow!("Missing required file: {}", file));
            }
        }

        // Validate config file
        Config::load(&self.path)?;

        Ok(())
    }
}

fn create_project_structure(project_path: &Path) -> Result<()> {
    info!("Creating project directory structure...");
    
    // Create main directories
    let directories = ["src", "tests", "build"];
    for dir in directories {
        fs::create_dir_all(project_path.join(dir))
            .with_context(|| format!("Failed to create directory: {}", dir))?;
    }

    Ok(())
}

fn apply_template(project_path: &Path, template: &str) -> Result<()> {
    info!("Applying template: {}", template);

    match template {
        "basic" => {
            for (file_path, content) in TEMPLATE_FILES {
                let full_path = project_path.join(file_path);
                if let Some(parent) = full_path.parent() {
                    fs::create_dir_all(parent)?;
                }
                fs::write(&full_path, content.trim())
                    .with_context(|| format!("Failed to create file: {}", file_path))?;
            }
        }
        _ => return Err(anyhow::anyhow!("Unknown template: {}", template)),
    }

    Ok(())
}

pub fn create_new_project(name: &str, template: &str) -> Result<()> {
    println!("{} Creating new LessVM project: {}", "→".blue(), name.green());
    
    let _project = Project::create(name, template)?;
    
    println!("\n{} Project created successfully!", "✓".green());
    println!("\nNext steps:");
    println!("  cd {}", name);
    println!("  lessvm build");
    println!("  lessvm deploy");

    Ok(())
}