mod asm;
mod cli;
mod render;
mod run;
mod dbg;
mod lessvm;

use std::{fs, path::PathBuf, process::ExitCode};

use anyhow::{Context, Result};
use clap::Parser;
use cli::{Cli, CliCommand};
use lessvm::VM;
use run::{Runner, spawn_run_thread};

fn main() -> ExitCode {
    // Initialize logger
    tui_logger::init_logger(log::LevelFilter::Info).unwrap();
    tui_logger::set_default_level(log::LevelFilter::Debug);

    // Parse command line arguments
    let args = Cli::parse();

    // Match command and execute
    match execute_command(args.command) {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("Error: {}", err);
            ExitCode::FAILURE
        }
    }
}

fn execute_command(command: CliCommand) -> Result<()> {
    match command {
        CliCommand::Check { path, log, kind: _ } => {
            if let Some(log_level) = log {
                tui_logger::set_level_for_target("ideless", log_level.to_level_filter());
            }
            
            let bytecode = fs::read(&path)
                .context(format!("Failed to read file: {}", path.display()))?;
            
            println!("Checking LessVM bytecode file: {}", path.display());
            let disasm = asm::Disassembler::new(bytecode);
            
            for line in disasm.disassembled {
                if !line.issues.is_empty() {
                    for issue in &line.issues {
                        println!("Issue at offset {:#04X}: {}", line.offset, issue);
                    }
                }
            }
            
            println!("Check complete");
            Ok(())
        },
        
        CliCommand::Dasm { path, log, kind: _ } => {
            if let Some(log_level) = log {
                tui_logger::set_level_for_target("ideless", log_level.to_level_filter());
            }
            
            let bytecode = fs::read(&path)
                .context(format!("Failed to read file: {}", path.display()))?;
            
            println!("Disassembling LessVM bytecode file: {}", path.display());
            let mut disasm = asm::Disassembler::new(bytecode);
            disasm.run();
            
            println!("{}", disasm);
            Ok(())
        },
        
        CliCommand::Run { path, debug, cpf, hz, log, kind: _, rpc_url: _, network: _ } => {
            if let Some(log_level) = log {
                tui_logger::set_level_for_target("ideless", log_level.to_level_filter());
            }
            
            let bytecode = fs::read(&path)
                .context(format!("Failed to read file: {}", path.display()))?;
            
            // Create VM and load bytecode
            let mut vm = VM::new();
            vm.load_bytecode(&bytecode)?;
            
            // Set cycles per frame if specified
            let cycles_per_frame = cpf.unwrap_or_else(|| {
                if let Some(hertz) = hz {
                    hertz / 60
                } else {
                    100 // Default 100 cycles per frame
                }
            });
            
            // Create debugger if needed
            let debugger = if debug {
                Some(dbg::Debugger::new(&vm, cycles_per_frame))
            } else {
                None
            };
            
            // Create VM runner
            let runner = Runner::new_lessvm(vm, debugger);
            
            // Create render controller
            let (render_controller, render_thread) = render::spawn_render_thread(
                runner.lessvm(),
                log.is_some()
            );
            
            // Create run thread
            let run_thread = spawn_run_thread(
                runner,
                render_controller,
                debug,
                log.is_some(),
            );
            
            // Join threads
            run_thread.join().expect("Failed to join run thread")?;
            
            Ok(())
        },
        
        CliCommand::RunSol { address, debug, cpf, hz, log, rpc_url, network } => {
            println!("Running Solana program from address: {}", address);
            println!("This feature is not yet implemented");
            Ok(())
        },
        
        CliCommand::New { name, template, output } => {
            let target_dir = output.unwrap_or_else(|| PathBuf::from(".").join(&name));
            
            println!("Creating new LessVM project: {} (template: {})", name, template);
            println!("Project directory: {}", target_dir.display());
            
            // Create project directory
            fs::create_dir_all(&target_dir)
                .context(format!("Failed to create directory: {}", target_dir.display()))?;
            
            // Create project files based on template
            // For now, just create a basic structure
            fs::create_dir_all(target_dir.join("src"))
                .context("Failed to create src directory")?;
            
            // Create a simple "Hello World" program bytecode
            let hello_world = vec![
                0x01, 0x48, // PUSH1 'H'
                0x01, 0x65, // PUSH1 'e'
                0x01, 0x6C, // PUSH1 'l'
                0x01, 0x6C, // PUSH1 'l'
                0x01, 0x6F, // PUSH1 'o'
                0xFF,       // HALT
            ];
            
            fs::write(target_dir.join("src/main.bin"), hello_world)
                .context("Failed to create main.bin file")?;
            
            // Create a README.md file
            let readme_content = format!(
                "# {}\n\nA LessVM project created with ideless.\n\n## Building\n\n```\nideless build\n```\n\n## Running\n\n```\nideless run\n```\n",
                name
            );
            
            fs::write(target_dir.join("README.md"), readme_content)
                .context("Failed to create README.md file")?;
            
            println!("Project created successfully!");
            Ok(())
        },
        
        CliCommand::Analyze { path, detailed, log } => {
            if let Some(log_level) = log {
                tui_logger::set_level_for_target("ideless", log_level.to_level_filter());
            }
            
            let bytecode = fs::read(&path)
                .context(format!("Failed to read file: {}", path.display()))?;
            
            println!("Analyzing compute units for: {}", path.display());
            let mut disasm = asm::Disassembler::new(bytecode);
            disasm.run();
            
            let total_cu = disasm.analyze_compute_units();
            println!("Total compute units: {}", total_cu);
            
            if detailed {
                println!("\nDetailed breakdown:");
                println!("------------------");
                println!("Offset | Instruction | CU");
                println!("----------------------------");
                
                for (offset, instruction, cu) in disasm.get_detailed_cu_analysis() {
                    println!("{:#06X} | {:?} | {}", offset, instruction, cu);
                }
            }
            
            Ok(())
        },
        
        CliCommand::Assist { path, model: _, log } => {
            if let Some(log_level) = log {
                tui_logger::set_level_for_target("ideless", log_level.to_level_filter());
            }
            
            println!("Starting AI-assisted development session");
            println!("AI assistance is not yet implemented");
            Ok(())
        },
    }
}
