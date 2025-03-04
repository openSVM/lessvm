use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use anyhow::{Result, anyhow};
use crossterm::event::KeyCode;
use log::{debug, info, warn};

use crate::lessvm::{VM, VMState};
use crate::asm::Disassembler;

/// Debugger state
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DebuggerState {
    /// Paused at specific PC
    Paused(usize),
    
    /// Running until condition
    Running,
    
    /// Stepping a single instruction
    Stepping,
}

/// Breakpoint type
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Breakpoint {
    /// Program counter breakpoint
    PC(usize),
    
    /// Memory address breakpoint
    Memory(usize),
    
    /// Register value breakpoint
    Register(usize, u32),
}

/// LessVM Debugger
pub struct Debugger {
    /// Debugger state
    state: DebuggerState,
    
    /// Breakpoints
    breakpoints: HashSet<Breakpoint>,
    
    /// Disassembler for code analysis
    disasm: Option<Disassembler>,
    
    /// Instruction history
    history: Vec<(usize, u8)>, // (PC, opcode)
    
    /// Memory watch addresses
    memory_watches: HashSet<usize>,
    
    /// Cycles per frame
    cycles_per_frame: u32,
    
    /// Command history
    command_history: Vec<String>,
}

impl Debugger {
    /// Create a new debugger
    pub fn new(vm: &VM, cycles_per_frame: u32) -> Self {
        let mut disasm = Disassembler::new(vm.memory.clone());
        disasm.run();
        
        Debugger {
            state: DebuggerState::Paused(vm.pc),
            breakpoints: HashSet::new(),
            disasm: Some(disasm),
            history: Vec::new(),
            memory_watches: HashSet::new(),
            cycles_per_frame,
            command_history: Vec::new(),
        }
    }
    
    /// Get debugger state
    pub fn state(&self) -> &DebuggerState {
        &self.state
    }
    
    /// Get mutable reference to debugger state
    pub fn state_mut(&mut self) -> &mut DebuggerState {
        &mut self.state
    }
    
    /// Set debugger state
    pub fn set_state(&mut self, state: DebuggerState) {
        self.state = state;
    }
    
    /// Get cycles per frame
    pub fn cycles_per_frame(&self) -> u32 {
        self.cycles_per_frame
    }
    
    /// Set cycles per frame
    pub fn set_cycles_per_frame(&mut self, cpf: u32) {
        self.cycles_per_frame = cpf;
    }
    
    /// Add a breakpoint
    pub fn add_breakpoint(&mut self, bp: Breakpoint) {
        self.breakpoints.insert(bp);
    }
    
    /// Remove a breakpoint
    pub fn remove_breakpoint(&mut self, bp: &Breakpoint) -> bool {
        self.breakpoints.remove(bp)
    }
    
    /// Clear all breakpoints
    pub fn clear_breakpoints(&mut self) {
        self.breakpoints.clear();
    }
    
    /// Check if a breakpoint is hit
    pub fn is_breakpoint_hit(&self, vm: &VM) -> bool {
        for bp in &self.breakpoints {
            match bp {
                Breakpoint::PC(pc) => {
                    if vm.pc == *pc {
                        return true;
                    }
                },
                Breakpoint::Memory(addr) => {
                    // Memory breakpoints not implemented yet
                },
                Breakpoint::Register(reg, value) => {
                    if *reg < vm.registers.len() && vm.registers[*reg] == *value {
                        return true;
                    }
                },
            }
        }
        
        false
    }
    
    /// Add a memory watch
    pub fn add_memory_watch(&mut self, addr: usize) {
        self.memory_watches.insert(addr);
    }
    
    /// Remove a memory watch
    pub fn remove_memory_watch(&mut self, addr: usize) -> bool {
        self.memory_watches.remove(&addr)
    }
    
    /// Clear all memory watches
    pub fn clear_memory_watches(&mut self) {
        self.memory_watches.clear();
    }
    
    /// Get disassembly around PC
    pub fn get_disassembly_at_pc(&self, vm: &VM, count: usize) -> Vec<(usize, String)> {
        if let Some(disasm) = &self.disasm {
            let mut result = Vec::new();
            let pc = vm.pc;
            
            // Find the instruction at PC
            let mut instructions = Vec::new();
            for (i, line) in disasm.disassembled.iter().enumerate() {
                if line.offset <= pc && 
                   (i + 1 >= disasm.disassembled.len() || 
                    disasm.disassembled[i + 1].offset > pc) {
                    // Found the instruction at PC
                    let start_idx = i.saturating_sub(count / 2);
                    let end_idx = (i + count / 2 + 1).min(disasm.disassembled.len());
                    
                    for j in start_idx..end_idx {
                        let line = &disasm.disassembled[j];
                        let instr = line.instruction.clone().unwrap_or_else(|| "???".to_string());
                        instructions.push((line.offset, instr));
                    }
                    
                    break;
                }
            }
            
            // If we couldn't find the instruction at PC, just show some instructions around it
            if instructions.is_empty() {
                // Find closest instruction before PC
                let mut closest_idx = 0;
                let mut closest_offset = 0;
                
                for (i, line) in disasm.disassembled.iter().enumerate() {
                    if line.offset <= pc && line.offset > closest_offset {
                        closest_idx = i;
                        closest_offset = line.offset;
                    }
                }
                
                let start_idx = closest_idx.saturating_sub(count / 2);
                let end_idx = (closest_idx + count / 2 + 1).min(disasm.disassembled.len());
                
                for j in start_idx..end_idx {
                    let line = &disasm.disassembled[j];
                    let instr = line.instruction.clone().unwrap_or_else(|| "???".to_string());
                    instructions.push((line.offset, instr));
                }
            }
            
            result = instructions;
            return result;
        }
        
        // If no disassembler is available, return empty list
        Vec::new()
    }
    
    /// Run a single instruction
    pub fn step(&mut self, vm: &mut VM) -> Result<()> {
        // Save current PC to history
        self.history.push((vm.pc, vm.memory[vm.pc]));
        if self.history.len() > 100 {
            self.history.remove(0);
        }
        
        // Step the VM
        match vm.step() {
            Ok(_) => {
                self.state = DebuggerState::Paused(vm.pc);
                Ok(())
            }
            Err(e) => {
                self.state = DebuggerState::Paused(vm.pc);
                Err(anyhow!("VM error: {}", e))
            }
        }
    }
    
    /// Run until breakpoint or end
    pub fn run_until_breakpoint(&mut self, vm: &mut VM) -> Result<()> {
        self.state = DebuggerState::Running;
        
        loop {
            // Check if a breakpoint is hit
            if self.is_breakpoint_hit(vm) {
                self.state = DebuggerState::Paused(vm.pc);
                return Ok(());
            }
            
            // Step the VM
            match vm.step() {
                Ok(continue_execution) => {
                    // Save current PC to history
                    self.history.push((vm.pc, vm.memory[vm.pc]));
                    if self.history.len() > 100 {
                        self.history.remove(0);
                    }
                    
                    // Check if we should stop execution
                    if !continue_execution {
                        self.state = DebuggerState::Paused(vm.pc);
                        return Ok(());
                    }
                },
                Err(e) => {
                    self.state = DebuggerState::Paused(vm.pc);
                    return Err(anyhow!("VM error: {}", e));
                }
            }
        }
    }
    
    /// Run a frame (multiple instructions)
    pub fn run_frame(&mut self, vm: &mut VM) -> Result<()> {
        match self.state {
            DebuggerState::Paused(_) => {
                // If paused, do nothing
                Ok(())
            },
            DebuggerState::Stepping => {
                // If stepping, execute a single instruction
                self.step(vm)?;
                Ok(())
            },
            DebuggerState::Running => {
                // If running, execute multiple instructions
                let mut cycles = 0;
                
                while cycles < self.cycles_per_frame {
                    // Check if a breakpoint is hit
                    if self.is_breakpoint_hit(vm) {
                        self.state = DebuggerState::Paused(vm.pc);
                        break;
                    }
                    
                    // Step the VM
                    match vm.step() {
                        Ok(continue_execution) => {
                            // Save current PC to history
                            self.history.push((vm.pc, vm.memory[vm.pc]));
                            if self.history.len() > 100 {
                                self.history.remove(0);
                            }
                            
                            cycles += 1;
                            
                            // Check if we should stop execution
                            if !continue_execution {
                                self.state = DebuggerState::Paused(vm.pc);
                                break;
                            }
                        },
                        Err(e) => {
                            self.state = DebuggerState::Paused(vm.pc);
                            return Err(anyhow::anyhow!("VM error: {}", e));
                        }
                    }
                }
                
                Ok(())
            }
        }
    }
    
    /// Handle keyboard input
    pub fn handle_key(&mut self, vm: &mut VM, key: KeyCode) -> Result<()> {
        match key {
            KeyCode::Char('n') => {
                // Step next instruction
                if let DebuggerState::Paused(_) = self.state {
                    if let Err(e) = self.step(vm) {
                        warn!("Step error: {}", e);
                    }
                }
            },
            KeyCode::Char('c') => {
                // Continue execution
                if let DebuggerState::Paused(_) = self.state {
                    self.state = DebuggerState::Running;
                }
            },
            KeyCode::Char('s') => {
                // Stop execution
                self.state = DebuggerState::Paused(vm.pc);
            },
            KeyCode::Char('b') => {
                // Add PC breakpoint at current location
                self.add_breakpoint(Breakpoint::PC(vm.pc));
                info!("Added breakpoint at {:#06X}", vm.pc);
            },
            KeyCode::Char('r') => {
                // Reset VM
                vm.reset();
                self.state = DebuggerState::Paused(0);
                self.history.clear();
                info!("VM reset");
            },
            KeyCode::Char('q') => {
                // Quit debugger (handled by runner)
            },
            _ => {}
        }
        
        Ok(())
    }
    
    /// Parse and execute a debug command
    pub fn execute_command(&mut self, vm: &mut VM, command: &str) -> Result<String> {
        // Save command to history
        self.command_history.push(command.to_string());
        if self.command_history.len() > 100 {
            self.command_history.remove(0);
        }

        // Split command into words
        let words: Vec<&str> = command.split_whitespace().collect();
        if words.is_empty() {
            return Ok("".to_string());
        }

        // Parse command
        match words[0] {
            "help" | "h" => {
                Ok("Available commands:\n\
                    help, h - Show help\n\
                    step, s - Step one instruction\n\
                    continue, c - Continue execution\n\
                    break, b <addr> - Set breakpoint at address\n\
                    delete, d <addr> - Delete breakpoint at address\n\
                    list, l - List breakpoints\n\
                    info, i - Show VM info\n\
                    reset, r - Reset VM\n\
                    quit, q - Quit debugger".to_string())
            },
            "step" | "s" => {
                if let Err(e) = self.step(vm) {
                    Ok(format!("Step error: {}", e))
                } else {
                    Ok(format!("Stepped to {:#06X}", vm.pc))
                }
            },
            "continue" | "c" => {
                self.state = DebuggerState::Running;
                Ok("Continuing execution".to_string())
            },
            "break" | "b" => {
                if words.len() < 2 {
                    Ok("Usage: break <addr>".to_string())
                } else {
                    if let Ok(addr) = usize::from_str_radix(words[1].trim_start_matches("0x"), 16) {
                        self.add_breakpoint(Breakpoint::PC(addr));
                        Ok(format!("Added breakpoint at {:#06X}", addr))
                    } else {
                        Ok(format!("Invalid address: {}", words[1]))
                    }
                }
            },
            "delete" | "d" => {
                if words.len() < 2 {
                    Ok("Usage: delete <addr>".to_string())
                } else {
                    if let Ok(addr) = usize::from_str_radix(words[1].trim_start_matches("0x"), 16) {
                        if self.remove_breakpoint(&Breakpoint::PC(addr)) {
                            Ok(format!("Removed breakpoint at {:#06X}", addr))
                        } else {
                            Ok(format!("No breakpoint at {:#06X}", addr))
                        }
                    } else {
                        Ok(format!("Invalid address: {}", words[1]))
                    }
                }
            },
            "list" | "l" => {
                if self.breakpoints.is_empty() {
                    Ok("No breakpoints set".to_string())
                } else {
                    let mut result = String::from("Breakpoints:\n");
                    for bp in &self.breakpoints {
                        match bp {
                            Breakpoint::PC(addr) => {
                                result.push_str(&format!("  PC: {:#06X}\n", addr));
                            },
                            Breakpoint::Memory(addr) => {
                                result.push_str(&format!("  Memory: {:#06X}\n", addr));
                            },
                            Breakpoint::Register(reg, value) => {
                                result.push_str(&format!("  Register: R{} == {:#010X}\n", reg, value));
                            },
                        }
                    }
                    Ok(result)
                }
            },
            "info" | "i" => {
                let mut result = String::new();
                
                result.push_str(&format!("PC: {:#06X}\n", vm.pc));
                result.push_str(&format!("State: {:?}\n", vm.state));
                result.push_str("Registers:\n");
                
                for (i, reg) in vm.registers.iter().enumerate() {
                    result.push_str(&format!("  R{}: {:#010X}\n", i, reg));
                }
                
                result.push_str(&format!("Stack: {:?}\n", vm.stack));
                
                Ok(result)
            },
            "reset" | "r" => {
                vm.reset();
                self.state = DebuggerState::Paused(0);
                self.history.clear();
                Ok("VM reset".to_string())
            },
            "quit" | "q" => {
                // Quit command is handled by the runner
                Ok("Quitting debugger".to_string())
            },
            _ => {
                Ok(format!("Unknown command: {}", words[0]))
            }
        }
    }
}
