use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "debug")]
pub struct DebugCli {
    #[command(subcommand)]
    pub command: DebugCliCommand,
}

#[derive(Copy, Clone, Debug, clap::ArgEnum)]
pub enum Register {
    #[clap(aliases = &["v0"])]
    R0,
    #[clap(aliases = &["v1"])]
    R1,
    #[clap(aliases = &["v2"])]
    R2,
    #[clap(aliases = &["v3"])]
    R3,
    #[clap(aliases = &["v4"])]
    R4,
    #[clap(aliases = &["v5"])]
    R5,
    #[clap(aliases = &["v6"])]
    R6,
    #[clap(aliases = &["v7"])]
    R7,
    #[clap(aliases = &["v8"])]
    R8,
    #[clap(aliases = &["v9"])]
    R9,
    #[clap(aliases = &["va"])]
    RA,
    #[clap(aliases = &["vb"])]
    RB,
    #[clap(aliases = &["vc"])]
    RC,
    #[clap(aliases = &["vd"])]
    RD,
    #[clap(aliases = &["ve"])]
    RE,
    #[clap(aliases = &["vf"])]
    RF,
}

impl Register {
    pub fn to_index(&self) -> u8 {
        match self {
            Register::R0 => 0,
            Register::R1 => 1,
            Register::R2 => 2,
            Register::R3 => 3,
            Register::R4 => 4,
            Register::R5 => 5,
            Register::R6 => 6,
            Register::R7 => 7,
            Register::R8 => 8,
            Register::R9 => 9,
            Register::RA => 10,
            Register::RB => 11,
            Register::RC => 12,
            Register::RD => 13,
            Register::RE => 14,
            Register::RF => 15,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Pointer {
    Pc,
    I,
}

#[derive(Copy, Clone, Debug)]
pub enum SemanticLocation {
    Start,
    End,
}

#[derive(Clone, Debug)]
pub enum GotoOption {
    SemanticLocation(SemanticLocation),
    Pointer(Pointer),
    Address(u32),
}

#[derive(Clone, Debug)]
pub enum WatchOption {
    Pointer(Pointer),
    Register(Register),
    Address(u32),
}

#[derive(Copy, Clone, Debug)]
pub enum ShowHideOption {
    Display,
    Memory {
        verbose: bool,
    },
}

#[derive(Copy, Clone, Debug)]
pub enum WatchBreakOption {
    Watch,
    Break,
}

#[derive(Clone, Debug)]
pub enum KeyCommand {
    Down {
        key: u8,
    },
    Up {
        key: u8,
    },
    Press {
        key: u8,
    },
    Switch,
}

#[derive(Clone, Debug)]
pub enum ClearCommand {
    Watch {
        watchpoint: WatchOption,
    },
    Break {
        breakpoint: u32,
    },
    All {
        what: WatchBreakOption,
    },
    Keyboard,
}

#[derive(Clone, Debug)]
pub enum DumpOption {
    Memory {
        path: PathBuf,
    },
}

#[derive(Subcommand, Clone, Debug)]
pub enum DebugCliCommand {
    /// Continue execution
    #[clap(aliases = &["c", "cont"])]
    Continue,

    /// Restart the program
    #[clap(aliases = &["rst"])]
    Reset,

    /// Single-step the program
    #[clap(aliases = &["s"])]
    Step {
        /// Amount of steps to take
        #[arg(default_value = "1")]
        amount: usize,
    },

    /// Set execution frequency in Hz
    Hertz {
        /// Hertz frequency
        #[arg(value_name = "HERTZ")]
        hertz: u32,
    },

    /// Show output history
    #[clap(aliases = &["o", "out"])]
    Output,

    /// Open memory inspector
    #[clap(aliases = &["m", "mem"])]
    Memory,

    /// Scroll to address in memory view
    #[clap(aliases = &["g"])]
    Goto {
        /// Goto target location 
        #[arg(value_name = "LOCATION")]
        location: GotoOption,
    },

    /// Set a breakpoint
    #[clap(aliases = &["b"])]
    Break {
        /// Address to break at
        #[arg(value_name = "ADDRESS")]
        address: u32,
    },

    /// Set a watchpoint
    #[clap(aliases = &["w"])]
    Watch {
        /// What to watch
        #[arg(value_name = "TARGET")]
        watchpoint: WatchOption,
    },

    /// View execution history
    #[clap(aliases = &["hist"])]
    History,

    /// Redo step(s)
    Redo {
        /// Amount of steps to redo
        #[arg(default_value = "1")]
        amount: usize,
    },

    /// Undo step(s)
    Undo {
        /// Amount of steps to undo
        #[arg(default_value = "1")]
        amount: usize,
    },

    /// Dump to file
    Dump {
        /// What to dump
        #[command(subcommand)]
        what: DumpOption,
    },

    /// Show widget
    Show {
        /// What to show
        #[arg(value_name = "WIDGET")]
        view: ShowHideOption,
    },

    /// Hide widget
    Hide {
        /// What to hide
        #[arg(value_name = "WIDGET")]
        view: ShowHideOption,
    },

    /// Information about debug state
    #[clap(aliases = &["i"])]
    Info {
        /// What to get info about
        #[arg(value_name = "TARGET")]
        what: WatchBreakOption,
    },

    /// Clear watchpoint/breakpoint
    #[clap(aliases = &["clr"])]
    Clear {
        /// What to clear
        #[command(subcommand)]
        command: ClearCommand,
    },
}

#[derive(Clone, Debug)]
struct ParseGotoOption;

impl std::str::FromStr for GotoOption {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "start" => Ok(GotoOption::SemanticLocation(SemanticLocation::Start)),
            "end" => Ok(GotoOption::SemanticLocation(SemanticLocation::End)),
            "pc" => Ok(GotoOption::Pointer(Pointer::Pc)),
            "i" => Ok(GotoOption::Pointer(Pointer::I)),
            _ => {
                if let Some(address_str) = s.strip_prefix("0x") {
                    match u32::from_str_radix(address_str, 16) {
                        Ok(addr) => Ok(GotoOption::Address(addr)),
                        Err(_) => Err(format!("Invalid hexadecimal address: {}", s)),
                    }
                } else {
                    match s.parse::<u32>() {
                        Ok(addr) => Ok(GotoOption::Address(addr)),
                        Err(_) => Err(format!("Invalid address: {}", s)),
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
struct ParseWatchOption;

impl std::str::FromStr for WatchOption {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pc" => Ok(WatchOption::Pointer(Pointer::Pc)),
            "i" => Ok(WatchOption::Pointer(Pointer::I)),
            _ => {
                // Check for register syntax
                if s.len() >= 2 && s.chars().next().unwrap() == 'r' {
                    let register_idx = &s[1..];
                    match u8::from_str_radix(register_idx, 16) {
                        Ok(idx) if idx < 16 => {
                            let register = match idx {
                                0 => Register::R0,
                                1 => Register::R1,
                                2 => Register::R2,
                                3 => Register::R3,
                                4 => Register::R4,
                                5 => Register::R5,
                                6 => Register::R6,
                                7 => Register::R7,
                                8 => Register::R8,
                                9 => Register::R9,
                                10 => Register::RA,
                                11 => Register::RB,
                                12 => Register::RC,
                                13 => Register::RD,
                                14 => Register::RE,
                                15 => Register::RF,
                                _ => unreachable!(),
                            };
                            Ok(WatchOption::Register(register))
                        }
                        _ => Err(format!("Invalid register: {}", s)),
                    }
                } else if let Some(address_str) = s.strip_prefix("0x") {
                    match u32::from_str_radix(address_str, 16) {
                        Ok(addr) => Ok(WatchOption::Address(addr)),
                        Err(_) => Err(format!("Invalid hexadecimal address: {}", s)),
                    }
                } else {
                    match s.parse::<u32>() {
                        Ok(addr) => Ok(WatchOption::Address(addr)),
                        Err(_) => Err(format!("Invalid address or register: {}", s)),
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
struct ParseShowHideOption;

impl std::str::FromStr for ShowHideOption {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "display" => Ok(ShowHideOption::Display),
            "memory" | "mem" => Ok(ShowHideOption::Memory { verbose: false }),
            "memory_verbose" | "mem_verbose" | "memverbose" => Ok(ShowHideOption::Memory { verbose: true }),
            _ => Err(format!("Invalid view option: {}", s)),
        }
    }
}

#[derive(Clone, Debug)]
struct ParseWatchBreakOption;

impl std::str::FromStr for WatchBreakOption {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "watch" | "watchpoints" | "watches" => Ok(WatchBreakOption::Watch),
            "break" | "breakpoints" | "breaks" => Ok(WatchBreakOption::Break),
            _ => Err(format!("Invalid option: {}", s)),
        }
    }
}

#[derive(Subcommand, Clone, Debug)]
pub enum ClearWatchpoint {
    /// Clear watchpoint on a register
    #[clap(aliases = &["r", "reg"])]
    Register {
        /// Which register
        #[arg(value_name = "REG")]
        register: Register,
    },

    /// Clear watchpoint on a pointer
    #[clap(aliases = &["p", "ptr"])]
    Pointer {
        /// Which pointer
        #[arg(value_name = "PTR")]
        pointer: Pointer,
    },

    /// Clear watchpoint on an address
    #[clap(aliases = &["a", "addr"])]
    Address {
        /// Which address
        #[arg(value_name = "ADDR")]
        address: u32,
    },
}

#[derive(Subcommand, Clone, Debug)]
pub enum ClearAll {
    /// Clear all watchpoints
    #[clap(aliases = &["w", "watches"])]
    Watchpoints,

    /// Clear all breakpoints
    #[clap(aliases = &["b", "breaks"])]
    Breakpoints,
}

#[derive(Subcommand, Clone, Debug)]
pub enum DumpMemory {
    /// Dump memory to file
    #[clap(aliases = &["m", "mem"])]
    Memory {
        /// Output path
        #[arg(value_name = "PATH")]
        path: PathBuf,
    },
}
