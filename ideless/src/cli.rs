use std::path::PathBuf;
use clap::{Parser, Subcommand, ValueEnum};
use log::LevelFilter;

#[derive(Debug, Copy, Clone, PartialEq, Eq, ValueEnum)]
pub enum LogLevel {
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl LogLevel {
    pub fn to_level_filter(&self) -> LevelFilter {
        match self {
            LogLevel::Off => LevelFilter::Off,
            LogLevel::Error => LevelFilter::Error,
            LogLevel::Warn => LevelFilter::Warn,
            LogLevel::Info => LevelFilter::Info,
            LogLevel::Debug => LevelFilter::Debug,
            LogLevel::Trace => LevelFilter::Trace,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, ValueEnum)]
pub enum EmulatorKind {
    LessVM,
    Solana,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, disable_help_flag = true)]
pub struct Cli {
    /// Command to run
    #[command(subcommand)]
    pub command: CliCommand,
}

#[derive(Subcommand, Debug)]
pub enum CliCommand {
    /// Check a LessVM program file
    #[command(alias = "validate")]
    Check {
        /// Path to LessVM program file
        #[arg(value_name = "FILE")]
        path: PathBuf,

        /// Set logging level
        #[arg(short, long, value_enum)]
        log: Option<LogLevel>,
        
    },
    
    /// Disassemble a LessVM program to human-readable form
    #[command(alias = "disasm")]
    Dasm {
        /// Path to LessVM program file
        #[arg(value_name = "FILE")]
        path: PathBuf,
        
        /// Set logging level
        #[arg(short, long, value_enum)]
        log: Option<LogLevel>,
        
    },
    
    /// Run a local LessVM program
    Run {
        /// Path to LessVM program file
        #[arg(value_name = "FILE")]
        path: PathBuf,
        
        /// Start in debug mode
        #[arg(short, long)]
        debug: bool,
        
        /// Cycles per frame
        #[arg(short, long)]
        cpf: Option<u32>,
        
        /// CPU frequency in Hz
        #[arg(short, long)]
        hz: Option<u32>,
        
        /// Set logging level
        #[arg(short, long, value_enum)]
        log: Option<LogLevel>,
                
        /// RPC URL (for Solana mode)
        #[arg(short = 'u', long, default_value = "https://api.devnet.solana.com")]
        rpc_url: String,
        
        /// Network (for Solana mode): mainnet-beta, testnet, devnet
        #[arg(short = 'n', long, default_value = "devnet")]
        network: String,
    },
    
    /// Run a Solana program
    RunSol {
        /// Program address
        #[arg(value_name = "ADDRESS")]
        address: String,
        
        /// Start in debug mode
        #[arg(short, long)]
        debug: bool,
        
        /// Cycles per frame
        #[arg(short, long)]
        cpf: Option<u32>,
        
        /// CPU frequency in Hz
        #[arg(short, long)]
        hz: Option<u32>,
        
        /// Set logging level
        #[arg(short, long, value_enum)]
        log: Option<LogLevel>,
        
        /// RPC URL (default: https://api.opensvm.network)
        #[arg(short = 'u', long, default_value = "https://api.opensvm.network")]
        rpc_url: String,
        
        /// Network: mainnet-beta, testnet, devnet
        #[arg(short = 'n', long, default_value = "mainnet-beta")]
        network: String,
    },
    
    /// Create a new LessVM project
    New {
        /// Project name
        #[arg(value_name = "NAME")]
        name: String,
        
        /// Project template (default: basic)
        #[arg(short, long, default_value = "basic")]
        template: String,
        
        /// Output directory
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    
    /// Analyze compute units usage
    Analyze {
        /// Path to LessVM program file
        #[arg(value_name = "FILE")]
        path: PathBuf,
        
        /// Show detailed breakdown
        #[arg(short, long)]
        detailed: bool,
        
        /// Set logging level
        #[arg(short, long, value_enum)]
        log: Option<LogLevel>,
    },
    
    /// Start AI-assisted development session
    Assist {
        /// Path to project directory
        #[arg(value_name = "DIR")]
        path: PathBuf,

        /// AI model to use (default: gpt-4o)
        #[arg(short, long, default_value = "gpt-4o")]
        model: String,
        
        /// Set logging level
        #[arg(short, long, value_enum)]
        log: Option<LogLevel>,
    },
    
    /// Start the TUI IDE for lessVM development
    #[command(alias = "ide")]
    Tui {
        /// Path to project directory or file
        #[arg(value_name = "PATH")]
        path: Option<PathBuf>,
        
        /// Set logging level
        #[arg(short, long, value_enum)]
        log: Option<LogLevel>,
    },
}
