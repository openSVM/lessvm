use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;
use log::{info, LevelFilter};
use std::path::PathBuf;

mod config;
mod project;
mod build;
mod deploy;

use crate::project::create_new_project;
use crate::build::build_project;
use crate::deploy::{deploy_project, update_project, check_status};

#[derive(Parser)]
#[command(name = "lessvm")]
#[command(author = "LessVM Team")]
#[command(version = "0.1.0")]
#[command(about = "CLI tool for managing LessVM applications", long_about = None)]
struct Cli {
    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new LessVM project
    New {
        /// Name of the project
        name: String,
        /// Template to use (default: basic)
        #[arg(short, long, default_value = "basic")]
        template: String,
    },
    /// Build the LessVM application
    Build {
        /// Path to project directory
        #[arg(short, long)]
        path: Option<String>,
    },
    /// Deploy application to Solana
    Deploy {
        /// Path to project directory
        #[arg(short, long)]
        path: Option<String>,
        /// Solana cluster to deploy to (default: devnet)
        #[arg(short, long, default_value = "devnet")]
        cluster: String,
    },
    /// Check deployment status
    Status {
        /// Path to project directory
        #[arg(short, long)]
        path: Option<String>,
    },
    /// Update deployed application
    Update {
        /// Path to project directory
        #[arg(short, long)]
        path: Option<String>,
        /// Hot reload if possible
        #[arg(short, long)]
        hot_reload: bool,
    },
    /// View application logs
    Logs {
        /// Path to project directory
        #[arg(short, long)]
        path: Option<String>,
        /// Follow log output
        #[arg(short, long)]
        follow: bool,
    },
}

fn setup_logging(verbose: bool) {
    let mut builder = env_logger::Builder::new();
    
    builder.filter_level(if verbose {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    });

    builder.format_timestamp(None);
    builder.format_module_path(false);
    
    builder.init();
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    setup_logging(cli.verbose);

    match cli.command {
        Commands::New { name, template } => {
            create_new_project(&name, &template)?;
        }
        Commands::Build { path } => {
            let path = path.map(PathBuf::from)
                .unwrap_or_else(|| std::env::current_dir().unwrap());
            let artifact = build_project(&path).await?;
            info!("Build artifact created at: {}", artifact.program_binary.display());
        }
        Commands::Deploy { path, cluster } => {
            let path = path.map(PathBuf::from)
                .unwrap_or_else(|| std::env::current_dir().unwrap());
            
            info!("Deploying to {} cluster", cluster);
            
            // First build the project
            println!("{} Building project before deployment", "→".blue());
            let artifact = build_project(&path).await?;
            
            // Then deploy
            deploy_project(&path, &artifact)?;
        }
        Commands::Status { path } => {
            let path = path.map(PathBuf::from)
                .unwrap_or_else(|| std::env::current_dir().unwrap());
            check_status(&path)?;
        }
        Commands::Update { path, hot_reload } => {
            let path = path.map(PathBuf::from)
                .unwrap_or_else(|| std::env::current_dir().unwrap());
            
            // First build the project
            println!("{} Building project before update", "→".blue());
            let artifact = build_project(&path).await?;
            
            // Then update
            update_project(&path, &artifact, hot_reload)?;
        }
        Commands::Logs { path: _, follow } => {
            // Path will be used when log viewing is implemented
            if follow {
                println!("{} Following program logs (Ctrl+C to stop)", "→".blue());
                // TODO: Implement log following
                println!("Log following not yet implemented");
            } else {
                println!("{} Fetching recent logs", "→".blue());
                // TODO: Implement log fetching
                println!("Log fetching not yet implemented");
            }
        }
    }

    Ok(())
}
