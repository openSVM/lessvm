use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub project: ProjectConfig,
    pub solana: SolanaConfig,
    pub build: BuildConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectConfig {
    pub name: String,
    pub version: String,
    pub template: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SolanaConfig {
    pub cluster: String,
    pub program_id: Option<String>,
    pub keypair_path: Option<String>,
    pub program_keypair_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BuildConfig {
    pub target: String,
    pub optimization_level: String,
}

impl Config {
    pub fn new(name: &str, template: &str) -> Self {
        Config {
            project: ProjectConfig {
                name: name.to_string(),
                version: "0.1.0".to_string(),
                template: template.to_string(),
            },
            solana: SolanaConfig {
                cluster: "devnet".to_string(),
                program_id: None,
                keypair_path: None,
                program_keypair_path: None,
            },
            build: BuildConfig {
                target: "solana".to_string(),
                optimization_level: "release".to_string(),
            },
        }
    }

    pub fn load(project_path: &Path) -> Result<Self> {
        let config_path = project_path.join("lessvm.toml");
        let config_str = std::fs::read_to_string(&config_path)
            .context("Failed to read config file")?;
        toml::from_str(&config_str).context("Failed to parse config file")
    }

    pub fn save(&self, project_path: &Path) -> Result<()> {
        let config_path = project_path.join("lessvm.toml");
        let config_str = toml::to_string_pretty(self)
            .context("Failed to serialize config")?;
        std::fs::write(config_path, config_str)
            .context("Failed to write config file")?;
        Ok(())
    }

    pub fn get_project_path() -> Result<PathBuf> {
        let current_dir = std::env::current_dir()
            .context("Failed to get current directory")?;
        
        let mut dir = current_dir.as_path();
        loop {
            if dir.join("lessvm.toml").exists() {
                return Ok(dir.to_path_buf());
            }
            
            if let Some(parent) = dir.parent() {
                dir = parent;
            } else {
                break;
            }
        }
        
        Err(anyhow::anyhow!("Not in a LessVM project directory"))
    }
}

pub fn init_config(name: &str, template: &str, path: &Path) -> Result<Config> {
    let config = Config::new(name, template);
    config.save(path)?;
    Ok(config)
}

pub fn load_or_init_config(path: Option<&str>) -> Result<(Config, PathBuf)> {
    let project_path = if let Some(path) = path {
        PathBuf::from(path)
    } else {
        Config::get_project_path()?
    };

    let config = if project_path.join("lessvm.toml").exists() {
        Config::load(&project_path)?
    } else {
        return Err(anyhow::anyhow!("No LessVM project found at {:?}", project_path));
    };

    Ok((config, project_path))
}