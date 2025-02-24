use anyhow::{Result, Context};
use std::{path::{Path, PathBuf}, thread::sleep, time::Duration, process::Command};
use solana_sdk::{
    signature::{read_keypair_file, Keypair},
    signer::Signer,
    pubkey::Pubkey,
    transaction::Transaction,
    message::Message,
};
use solana_client::rpc_client::RpcClient;
use log::info;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use tempfile::NamedTempFile;

use lessvm_solana::solana::instruction::create_initialize_instruction;

use crate::config::Config;
use crate::build::BuildArtifact;

const DEVNET_AIRDROP_AMOUNT: u64 = 1_000_000_000; // 1 SOL per request
const MINIMUM_BALANCE: u64 = 15900_000_000; // 1.9 SOL minimum
const AIRDROP_RETRIES: u32 = 5; // Increased retries to handle rate limits
const CONFIRMATION_RETRIES: u32 = 15; // Increased confirmation retries
const RETRY_DELAY: Duration = Duration::from_secs(2);

/// Manages deployment of LessVM programs to Solana
pub struct Deployer {
    config: Config,
    project_path: PathBuf,
    /// Solana keypair for program deployment and updates
    keypair: Keypair,
    cluster_url: String,
}

impl Deployer {
    pub fn new(config: Config, project_path: PathBuf) -> Result<Self> {
        let keypair_path = if let Some(ref path) = config.solana.keypair_path {
            if Path::new(&path).is_relative() {
                // If path is relative, make it relative to project directory
                project_path.join(path).to_string_lossy().to_string()
            } else {
                path.to_string()
            }
        } else {
            String::from("~/.config/solana/id.json")
        };
        
        let expanded_path = shellexpand::tilde(&keypair_path);
        let keypair = read_keypair_file(expanded_path.as_ref())
            .map_err(|e| anyhow::anyhow!("Failed to read keypair at {}: {}", expanded_path, e))?;

        let cluster_url = match config.solana.cluster.as_str() {
            "mainnet-beta" => "https://api.mainnet-beta.solana.com",
            "testnet" => "https://api.testnet.solana.com",
            "devnet" => "https://api.devnet.solana.com",
            _ => "http://localhost:8899", // Assume local validator
        };

        Ok(Deployer {
            config,
            project_path,
            keypair,
            cluster_url: cluster_url.to_string(),
        })
    }

    fn ensure_funded(&self, rpc_client: &RpcClient) -> Result<()> {
        if !["devnet", "local"].contains(&self.config.solana.cluster.as_str()) {
            return Ok(());
        }

        let mut current_balance = rpc_client.get_balance(&self.keypair.pubkey())?;
        
        if current_balance >= MINIMUM_BALANCE {
            info!("Current balance: {} SOL (sufficient for deployment)", 
                current_balance as f64 / 1_000_000_000.0);
            return Ok(());
        }

        info!("Current balance: {} SOL", current_balance as f64 / 1_000_000_000.0);
        info!("Requesting airdrops for deployment...");

        for attempt in 1..=AIRDROP_RETRIES {
            if current_balance >= MINIMUM_BALANCE {
                break;
            }

            info!("Airdrop attempt {} of {}", attempt, AIRDROP_RETRIES);
            
            match rpc_client.request_airdrop(&self.keypair.pubkey(), DEVNET_AIRDROP_AMOUNT) {
                Ok(signature) => {
                    // Wait for airdrop confirmation
                    for _ in 0..CONFIRMATION_RETRIES {
                        sleep(RETRY_DELAY);
                        if rpc_client.confirm_transaction(&signature).unwrap_or(false) {
                            if let Ok(new_balance) = rpc_client.get_balance(&self.keypair.pubkey()) {
                                if new_balance > current_balance {
                                    info!("Received {} SOL", (new_balance - current_balance) as f64 / 1_000_000_000.0);
                                    current_balance = new_balance;
                                    break;
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    info!("Airdrop request failed: {}", e);
                    sleep(RETRY_DELAY);
                }
            }
        }

        if current_balance < MINIMUM_BALANCE {
            return Err(anyhow::anyhow!(
                "Failed to get sufficient funds. Current balance: {} SOL",
                current_balance as f64 / 1_000_000_000.0
            ));
        }

        info!("Account funded successfully");
        Ok(())
    }

    fn check_validator(&self) -> Result<()> {
        if self.config.solana.cluster == "local" {
            info!("Checking local validator...");
            let output = Command::new("solana")
                .args(["cluster-version"])
                .output()
                .context("Failed to check validator status")?;

            if !output.status.success() {
                return Err(anyhow::anyhow!("Local validator not running. Please start it first."));
            }
        }
        Ok(())
    }

    pub fn deploy(&self, artifact: &BuildArtifact) -> Result<Pubkey> {
        info!("Starting deployment process...");
        
        let pb = ProgressBar::new(4);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
                .unwrap()
                .progress_chars("=>-"),
        );

        // Step 1: Check validator (for local deployment)
        pb.set_message("Checking validator");
        self.check_validator()?;
        pb.inc(1);

        // Step 2: Prepare program data
        pb.set_message("Preparing program data");
        let program_data = std::fs::read(&artifact.program_binary)
            .context("Failed to read program binary")?;
        pb.inc(1);

        // Step 3: Deploy program
        pb.set_message("Deploying program");
        let program_id = if self.config.solana.cluster == "local" {
            self.deploy_program_local(&program_data)?
        } else {
            self.deploy_program(&program_data)?
        };
        pb.inc(1);

        // Step 4: Update config with program ID
        pb.set_message("Updating configuration");
        self.update_config(program_id)?;
        pb.inc(1);

        pb.finish_with_message("Deployment completed successfully");

        Ok(program_id)
    }

    fn deploy_program_local(&self, program_data: &[u8]) -> Result<Pubkey> {
        // Create temporary file for program binary
        let temp_file = NamedTempFile::new()?;
        std::fs::write(&temp_file, program_data)?;

        // Get program keypair path
        let program_keypair_path = self.config.solana.program_keypair_path
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Program keypair path not found in config"))?;

        let program_keypair_path = if Path::new(program_keypair_path).is_relative() {
            self.project_path.join(program_keypair_path)
        } else {
            PathBuf::from(program_keypair_path)
        };

        let program_keypair = read_keypair_file(&program_keypair_path)
            .map_err(|e| anyhow::anyhow!("Failed to read program keypair: {}", e))?;
        let program_id = program_keypair.pubkey();

        let keypair_path = if let Some(path) = &self.config.solana.keypair_path {
            if Path::new(path).is_relative() {
                // If path is relative, make it relative to project directory
                self.project_path.join(path).to_string_lossy().to_string()
            } else {
                path.clone()
            }
        } else {
            "~/.config/solana/id.json".to_string()
        };
        let expanded_path = shellexpand::tilde(&keypair_path);

        // Deploy using solana CLI
        let status = Command::new("solana")
            .args([
                "program",
                "deploy",
                "--keypair",
                expanded_path.as_ref(),
                "--program-id",
                &program_keypair_path.to_string_lossy(),
                &temp_file.path().to_string_lossy(),
            ])
            .status()
            .context("Failed to deploy program")?;

        if !status.success() {
            return Err(anyhow::anyhow!("Program deployment failed"));
        }

        // Initialize the program
        let rpc_client = RpcClient::new(self.cluster_url.clone());
        let init_ix = create_initialize_instruction(&program_id, &program_id)?;
        let message = Message::new(&[init_ix], Some(&self.keypair.pubkey()));
        let mut transaction = Transaction::new_unsigned(message);
        let blockhash = rpc_client.get_latest_blockhash()?;
        transaction.sign(&[&self.keypair, &program_keypair], blockhash);
        rpc_client.send_and_confirm_transaction_with_spinner(&transaction)?;

        info!("Program deployed successfully");
        info!("Program ID: {}", program_id);

        Ok(program_id)
    }

    fn deploy_program(&self, program_data: &[u8]) -> Result<Pubkey> {
        let rpc_client = RpcClient::new(self.cluster_url.clone());
        
        // Ensure account is funded for devnet
        self.ensure_funded(&rpc_client)?;
        
        // Create temporary file for program binary
        let temp_file = NamedTempFile::new()?;
        std::fs::write(&temp_file, program_data)?;

        // Get program ID from config or generate new one
        let program_id = if let Some(id) = &self.config.solana.program_id {
            Pubkey::try_from(id.as_str())
                .map_err(|_| anyhow::anyhow!("Invalid program ID in config"))?
        } else {
            Keypair::new().pubkey()
        };

        let keypair_path = if let Some(path) = &self.config.solana.keypair_path {
            if Path::new(path).is_relative() {
                // If path is relative, make it relative to project directory
                self.project_path.join(path).to_string_lossy().to_string()
            } else {
                path.clone()
            }
        } else {
            "~/.config/solana/id.json".to_string()
        };
        let expanded_path = shellexpand::tilde(&keypair_path);

        // Deploy using solana CLI
        let status = Command::new("solana")
            .args([
                "program",
                "deploy",
                "--keypair",
                expanded_path.as_ref(),
                "--program-id",
                &program_id.to_string(),
                "--url",
                &self.cluster_url,
                &temp_file.path().to_string_lossy(),
            ])
            .status()
            .context("Failed to deploy program")?;

        if !status.success() {
            return Err(anyhow::anyhow!("Program deployment failed"));
        }

        // Initialize the program
        let init_ix = create_initialize_instruction(&program_id, &program_id)?;
        let message = Message::new(&[init_ix], Some(&self.keypair.pubkey()));
        let mut transaction = Transaction::new_unsigned(message);
        let blockhash = rpc_client.get_latest_blockhash()?;
        transaction.sign(&[&self.keypair], blockhash);
        rpc_client.send_and_confirm_transaction_with_spinner(&transaction)?;

        info!("Program deployed successfully");
        info!("Program ID: {}", program_id);

        Ok(program_id)
    }

    fn update_config(&self, program_id: Pubkey) -> Result<()> {
        let mut config = self.config.clone();
        config.solana.program_id = Some(program_id.to_string());
        config.save(&self.project_path)?;
        Ok(())
    }

    pub fn hot_reload(&self, artifact: &BuildArtifact) -> Result<Pubkey> {
        let program_id_str = self.config.solana.program_id
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("No program ID found in config"))?;
        
        let program_id = Pubkey::try_from(program_id_str.as_str())
            .map_err(|_| anyhow::anyhow!("Invalid program ID in config"))?;

        info!("Hot reloading program: {}", program_id);

        if self.config.solana.cluster == "local" {
            self.hot_reload_local(artifact, program_id)?;
        } else {
            self.hot_reload_remote(artifact, program_id)?;
        }

        Ok(program_id)
    }

    fn hot_reload_local(&self, artifact: &BuildArtifact, program_id: Pubkey) -> Result<()> {
        // Check validator
        self.check_validator()?;

        // Create temporary file for program binary
        let temp_file = NamedTempFile::new()?;
        std::fs::write(&temp_file, std::fs::read(&artifact.program_binary)?)?;

        let keypair_path = if let Some(path) = &self.config.solana.keypair_path {
            if Path::new(path).is_relative() {
                // If path is relative, make it relative to project directory
                self.project_path.join(path).to_string_lossy().to_string()
            } else {
                path.clone()
            }
        } else {
            "~/.config/solana/id.json".to_string()
        };
        let expanded_path = shellexpand::tilde(&keypair_path);

        // Create buffer account
        let buffer_keypair = NamedTempFile::new()?;
        Command::new("solana-keygen")
            .args([
                "new",
                "--no-bip39-passphrase",
                "--force",
                "-o",
                &buffer_keypair.path().to_string_lossy(),
            ])
            .status()
            .context("Failed to create buffer keypair")?;

        // Get buffer address
        let buffer_address = String::from_utf8(
            Command::new("solana-keygen")
                .args([
                    "pubkey",
                    &buffer_keypair.path().to_string_lossy(),
                ])
                .output()
                .context("Failed to get buffer address")?
                .stdout,
        )?;

        // Write program to buffer
        let status = Command::new("solana")
            .args([
                "program",
                "write-buffer",
                "--buffer",
                &buffer_keypair.path().to_string_lossy(),
                "--keypair",
                expanded_path.as_ref(),
                &temp_file.path().to_string_lossy(),
            ])
            .status()
            .context("Failed to write program to buffer")?;

        if !status.success() {
            return Err(anyhow::anyhow!("Failed to write program to buffer"));
        }

        // Set buffer authority
        let status = Command::new("solana")
            .args([
                "program",
                "set-buffer-authority",
                &buffer_address.trim(),
                "--new-buffer-authority",
                &self.keypair.pubkey().to_string(),
                "--keypair",
                expanded_path.as_ref(),
            ])
            .status()
            .context("Failed to set buffer authority")?;

        if !status.success() {
            return Err(anyhow::anyhow!("Failed to set buffer authority"));
        }

        // Upgrade program from buffer
        let status = Command::new("solana")
            .args([
                "program",
                "upgrade",
                &buffer_address.trim(),
                &program_id.to_string(),
                "--keypair",
                expanded_path.as_ref(),
                "--program-authority",
                expanded_path.as_ref(),
            ])
            .status()
            .context("Failed to upgrade program")?;

        if !status.success() {
            return Err(anyhow::anyhow!("Failed to upgrade program"));
        }

        info!("Program upgraded successfully");
        Ok(())
    }

    fn hot_reload_remote(&self, artifact: &BuildArtifact, program_id: Pubkey) -> Result<()> {
        let rpc_client = RpcClient::new(self.cluster_url.clone());
        
        // Ensure account is funded for devnet
        self.ensure_funded(&rpc_client)?;
        
        // Create temporary file for program binary
        let temp_file = NamedTempFile::new()?;
        std::fs::write(&temp_file, std::fs::read(&artifact.program_binary)?)?;

        // Create buffer account
        let buffer_keypair = NamedTempFile::new()?;
        Command::new("solana-keygen")
            .args([
                "new",
                "--no-bip39-passphrase",
                "--force",
                "-o",
                &buffer_keypair.path().to_string_lossy(),
            ])
            .status()
            .context("Failed to create buffer keypair")?;

        // Get buffer address
        let buffer_address = String::from_utf8(
            Command::new("solana-keygen")
                .args([
                    "pubkey",
                    &buffer_keypair.path().to_string_lossy(),
                ])
                .output()
                .context("Failed to get buffer address")?
                .stdout,
        )?;

        let keypair_path = self.config.solana.keypair_path.clone()
            .unwrap_or_else(|| "~/.config/solana/id.json".to_string());
        let expanded_path = shellexpand::tilde(&keypair_path);

        // Write program to buffer
        let status = Command::new("solana")
            .args([
                "program",
                "write-buffer",
                "--buffer",
                &buffer_keypair.path().to_string_lossy(),
                "--keypair",
                expanded_path.as_ref(),
                "--url",
                &self.cluster_url,
                &temp_file.path().to_string_lossy(),
            ])
            .status()
            .context("Failed to write program to buffer")?;

        if !status.success() {
            return Err(anyhow::anyhow!("Failed to write program to buffer"));
        }

        // Set buffer authority
        let status = Command::new("solana")
            .args([
                "program",
                "set-buffer-authority",
                &buffer_address.trim(),
                "--new-buffer-authority",
                &self.keypair.pubkey().to_string(),
                "--keypair",
                expanded_path.as_ref(),
                "--url",
                &self.cluster_url,
            ])
            .status()
            .context("Failed to set buffer authority")?;

        if !status.success() {
            return Err(anyhow::anyhow!("Failed to set buffer authority"));
        }

        // Upgrade program from buffer
        let status = Command::new("solana")
            .args([
                "program",
                "upgrade",
                &buffer_address.trim(),
                &program_id.to_string(),
                "--keypair",
                expanded_path.as_ref(),
                "--program-authority",
                expanded_path.as_ref(),
                "--url",
                &self.cluster_url,
            ])
            .status()
            .context("Failed to upgrade program")?;

        if !status.success() {
            return Err(anyhow::anyhow!("Failed to upgrade program"));
        }

        info!("Program upgraded successfully");
        Ok(())
    }
}

pub fn deploy_project(path: &Path, artifact: &BuildArtifact) -> Result<()> {
    let config = Config::load(path)?;
    let deployer = Deployer::new(config, path.to_path_buf())?;
    
    println!("{} Deploying to Solana {}", "→".blue(), deployer.cluster_url.yellow());
    let program_id = deployer.deploy(artifact)?;
    
    println!("\n{} Deployment completed successfully!", "✓".green());
    println!("  Program ID: {}", program_id);
    println!("  Cluster: {}", deployer.cluster_url);

    Ok(())
}

pub fn update_project(path: &Path, artifact: &BuildArtifact, hot_reload: bool) -> Result<()> {
    let config = Config::load(path)?;
    let deployer = Deployer::new(config, path.to_path_buf())?;
    
    if hot_reload {
        println!("{} Hot reloading program", "→".blue());
        let program_id = deployer.hot_reload(artifact)?;
        println!("\n{} Hot reload completed successfully!", "✓".green());
        println!("  Program ID: {}", program_id);
    } else {
        println!("{} Updating program", "→".blue());
        let program_id = deployer.deploy(artifact)?;
        println!("\n{} Update completed successfully!", "✓".green());
        println!("  New Program ID: {}", program_id);
    }

    Ok(())
}

pub fn check_status(path: &Path) -> Result<()> {
    let config = Config::load(path)?;
    
    if let Some(program_id) = config.solana.program_id {
        println!("{} Program Status:", "→".blue());
        println!("  Program ID: {}", program_id);
        println!("  Cluster: {}", config.solana.cluster);
        
        let rpc_client = RpcClient::new(
            match config.solana.cluster.as_str() {
                "mainnet-beta" => "https://api.mainnet-beta.solana.com",
                "testnet" => "https://api.testnet.solana.com",
                "devnet" => "https://api.devnet.solana.com",
                _ => "http://localhost:8899",
            }
        );

        let program_id = Pubkey::try_from(program_id.as_str())
            .map_err(|_| anyhow::anyhow!("Invalid program ID in config"))?;

        match rpc_client.get_account(&program_id) {
            Ok(_) => println!("  Status: {}", "Active".green()),
            Err(_) => println!("  Status: {}", "Inactive".red()),
        }
    } else {
        println!("{} No program deployed yet", "!".yellow());
    }

    Ok(())
}
