use std::fmt;
use thiserror::Error;
use anyhow::{Result, Context};

/// LessVM Error Types
#[derive(Error, Debug)]
pub enum VMError {
    #[error("Stack overflow")]
    StackOverflow,
    
    #[error("Stack underflow")]
    StackUnderflow,
    
    #[error("Out of memory at address {0}")]
    OutOfMemory(usize),
    
    #[error("Invalid opcode: {0:#04X}")]
    InvalidOpcode(u8),
    
    #[error("Invalid jump destination: {0:#06X}")]
    InvalidJumpDestination(usize),
    
    #[error("Execution timeout after {0} cycles")]
    ExecutionTimeout(u32),
    
    #[error("Gas limit exceeded")]
    OutOfGas,
    
    #[error("System error: {0}")]
    SystemError(String),
}

/// VM State
#[derive(Debug, Clone)]
pub enum VMState {
    Ready,
    Running,
    Halted,
    Error(String),
}

impl fmt::Display for VMState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VMState::Ready => write!(f, "Ready"),
            VMState::Running => write!(f, "Running"),
            VMState::Halted => write!(f, "Halted"),
            VMState::Error(msg) => write!(f, "Error: {}", msg),
        }
    }
}

/// LessVM - Virtual Machine for the Solana blockchain
pub struct VM {
    /// Program counter
    pub pc: usize,
    
    /// SIMD vector registers (4 vector registers with 4 elements each)
    pub vector_registers: [[u32; 4]; 4],

    /// Floating-point registers (8 64-bit floating point values)
    pub fp_registers: [f64; 8],
    
    /// Matrix registers (2 4x4 matrices for advanced linear algebra)
    pub matrix_registers: [[[f32; 4]; 4]; 2],

    /// Complex number registers (4 complex numbers with real/imaginary parts)
    pub complex_registers: [[f64; 2]; 4],

    /// String buffer for string operations
    pub string_buffer: String,

    /// Memory
    pub memory: Vec<u8>,
    
    /// Registers (16 general purpose registers)
    pub registers: [u32; 16],
    
    /// Stack
    pub stack: Vec<u32>,
    
    /// VM state
    pub state: VMState,
    
    /// Cycle counter for execution tracking
    pub cycle_counter: u32,
    
    /// Gas usage
    gas_used: u64,
    
    /// Gas limit (0 means unlimited)
    gas_limit: u64,
}

impl Default for VM {
    fn default() -> Self {
        Self::new()
    }
}

impl VM {
    /// Create a new VM instance
    pub fn new() -> Self {
        VM {
            pc: 0,
            memory: vec![0; 65536], // 64KB memory space
            registers: [0; 16],
            vector_registers: [[0; 4]; 4],
            fp_registers: [0.0; 8],
            matrix_registers: [[[0.0; 4]; 4]; 2],
            complex_registers: [[0.0; 2]; 4],
            string_buffer: String::with_capacity(1024),
            stack: Vec::with_capacity(1024),
            state: VMState::Ready,
            cycle_counter: 0,
            gas_used: 0,
            gas_limit: 0, // Unlimited gas by default
        }
    }

    /// Reset the VM to its initial state
    pub fn reset(&mut self) {
        self.pc = 0;
        self.registers = [0; 16];
        self.vector_registers = [[0; 4]; 4];
        self.fp_registers = [0.0; 8];
        self.matrix_registers = [[[0.0; 4]; 4]; 2];
        self.complex_registers = [[0.0; 2]; 4];
        self.string_buffer.clear();
        self.stack.clear();
        self.state = VMState::Ready;
        self.cycle_counter = 0;
        self.gas_used = 0;
    }
    
    /// Load bytecode into memory
    pub fn load_bytecode(&mut self, bytecode: &[u8]) -> Result<()> {
        if bytecode.len() > self.memory.len() {
            return Err(anyhow::anyhow!("Bytecode too large: {} bytes (max {})", 
                       bytecode.len(), self.memory.len()));
        }
        
        // Reset VM state
        self.reset();
        
        // Copy bytecode to memory
        self.memory[..bytecode.len()].copy_from_slice(bytecode);
        
        Ok(())
    }
    
    /// Set gas limit
    pub fn set_gas_limit(&mut self, limit: u64) {
        self.gas_limit = limit;
    }
    
    /// Get current gas usage
    pub fn gas_used(&self) -> u64 {
        self.gas_used
    }
    
    /// Get current gas limit
    pub fn gas_limit(&self) -> u64 {
        self.gas_limit
    }
    
    /// Run the VM for a specified number of cycles
    pub fn run(&mut self, max_cycles: u32) -> Result<()> {
        self.state = VMState::Running;
        
        for _ in 0..max_cycles {
            match self.step() {
                Ok(true) => {
                    self.cycle_counter += 1;
                    
                    // Check if we've hit the gas limit
                    if self.gas_limit > 0 && self.gas_used >= self.gas_limit {
                        self.state = VMState::Error(VMError::OutOfGas.to_string());
                        return Err(anyhow::anyhow!("{}", VMError::OutOfGas));
                    }
                }
                Ok(false) => {
                    // Program halted normally
                    return Ok(());
                }
                Err(e) => {
                    self.state = VMState::Error(e.to_string());
                    return Err(e);
                }
            }
        }
        
        Ok(())
    }
    
    /// Execute a single instruction
    pub fn step(&mut self) -> Result<bool> {
        if let VMState::Halted = self.state {
            return Ok(false);
        }
        
        if let VMState::Error(_) = self.state {
            return Err(anyhow::anyhow!("VM is in error state"));
        }
        
        // Check if we're at the end of memory
        if self.pc >= self.memory.len() {
            self.state = VMState::Error(format!("{}", VMError::OutOfMemory(self.pc)));
            return Err(anyhow::anyhow!("{}", VMError::OutOfMemory(self.pc)));
        }

        // Fetch the opcode
        let opcode = self.memory[self.pc];
        
        // Execute the instruction
        match self.execute_instruction(opcode) {
            Ok(continue_execution) => Ok(continue_execution),
            Err(e) => {
                self.state = VMState::Error(e.to_string());
                Err(e)
            }
        }
    }
    
    /// Execute a single instruction
    fn execute_instruction(&mut self, opcode: u8) -> Result<bool> {
        match opcode {
            // PUSH1 - Push 1-byte value onto stack
            0x01 => {
                self.pc += 1;
                if self.pc >= self.memory.len() {
                    return Err(anyhow::anyhow!("{}", VMError::OutOfMemory(self.pc)));
                }
                
                let value = self.memory[self.pc] as u32;
                self.stack.push(value);
                self.pc += 1;
                self.gas_used += 3;
            }
            
            // PUSH2 - Push 2-byte value onto stack
            0x02 => {
                self.pc += 1;
                if self.pc + 1 >= self.memory.len() {
                    return Err(anyhow::anyhow!("{}", VMError::OutOfMemory(self.pc)));
                }
                
                let value = ((self.memory[self.pc] as u32) << 8) | (self.memory[self.pc + 1] as u32);
                self.stack.push(value);
                self.pc += 2;
                self.gas_used += 3;
            }
            
            // PUSH4 - Push 4-byte value onto stack
            0x03 => {
                self.pc += 1;
                if self.pc + 3 >= self.memory.len() {
                    return Err(anyhow::anyhow!("{}", VMError::OutOfMemory(self.pc)));
                }
                
                let value = ((self.memory[self.pc] as u32) << 24) |
                            ((self.memory[self.pc + 1] as u32) << 16) |
                            ((self.memory[self.pc + 2] as u32) << 8) |
                            (self.memory[self.pc + 3] as u32);
                self.stack.push(value);
                self.pc += 4;
                self.gas_used += 3;
            }
            
            // POP - Remove top item from stack
            0x04 => {
                if self.stack.is_empty() {
                    return Err(anyhow::anyhow!("{}", VMError::StackUnderflow));
                }
                
                self.stack.pop();
                self.pc += 1;
                self.gas_used += 5;
            }
            
            // DUP - Duplicate nth stack item
            0x05 => {
                self.pc += 1;
                if self.pc >= self.memory.len() {
                    return Err(anyhow::anyhow!("{}", VMError::OutOfMemory(self.pc)));
                }
                
                let n = self.memory[self.pc] as usize;
                if n >= self.stack.len() {
                    return Err(anyhow::anyhow!("{}", VMError::StackUnderflow));
                }
                
                let value = self.stack[self.stack.len() - 1 - n];
                self.stack.push(value);
                self.pc += 1;
                self.gas_used += 5;
            }
            
            // SWAP - Swap nth stack item with top
            0x06 => {
                self.pc += 1;
                if self.pc >= self.memory.len() {
                    return Err(anyhow::anyhow!("{}", VMError::OutOfMemory(self.pc)));
                }
                
                let n = self.memory[self.pc] as usize;
                if n >= self.stack.len() {
                    return Err(anyhow::anyhow!("{}", VMError::StackUnderflow));
                }
                
                let top_idx = self.stack.len() - 1;
                let swap_idx = self.stack.len() - 1 - n;
                self.stack.swap(top_idx, swap_idx);
                self.pc += 1;
                self.gas_used += 5;
            }
            
            // ADD - Addition
            0x10 => {
                if self.stack.len() < 2 {
                    return Err(anyhow::anyhow!("{}", VMError::StackUnderflow));
                }
                
                let b = self.stack.pop().unwrap();
                let a = self.stack.pop().unwrap();
                self.stack.push(a.wrapping_add(b));
                self.pc += 1;
                self.gas_used += 10;
            }
            
            // SUB - Subtraction
            0x11 => {
                if self.stack.len() < 2 {
                    return Err(anyhow::anyhow!("{}", VMError::StackUnderflow));
                }
                
                let b = self.stack.pop().unwrap();
                let a = self.stack.pop().unwrap();
                self.stack.push(a.wrapping_sub(b));
                self.pc += 1;
                self.gas_used += 10;
            }
            
            // MUL - Multiplication
            0x12 => {
                if self.stack.len() < 2 {
                    return Err(anyhow::anyhow!("{}", VMError::StackUnderflow));
                }
                
                let b = self.stack.pop().unwrap();
                let a = self.stack.pop().unwrap();
                self.stack.push(a.wrapping_mul(b));
                self.pc += 1;
                self.gas_used += 20;
            }
            
            // DIV - Division
            0x13 => {
                if self.stack.len() < 2 {
                    return Err(anyhow::anyhow!("{}", VMError::StackUnderflow));
                }
                
                let b = self.stack.pop().unwrap();
                let a = self.stack.pop().unwrap();
                
                if b == 0 {
                    self.stack.push(0);
                } else {
                    self.stack.push(a.wrapping_div(b));
                }
                
                self.pc += 1;
                self.gas_used += 20;
            }
            
            // AND - Bitwise AND
            0x20 => {
                if self.stack.len() < 2 {
                    return Err(anyhow::anyhow!("{}", VMError::StackUnderflow));
                }
                
                let b = self.stack.pop().unwrap();
                let a = self.stack.pop().unwrap();
                self.stack.push(a & b);
                self.pc += 1;
                self.gas_used += 10;
            }
            
            // OR - Bitwise OR
            0x21 => {
                if self.stack.len() < 2 {
                    return Err(anyhow::anyhow!("{}", VMError::StackUnderflow));
                }
                
                let b = self.stack.pop().unwrap();
                let a = self.stack.pop().unwrap();
                self.stack.push(a | b);
                self.pc += 1;
                self.gas_used += 10;
            }
            
            // XOR - Bitwise XOR
            0x22 => {
                if self.stack.len() < 2 {
                    return Err(anyhow::anyhow!("{}", VMError::StackUnderflow));
                }
                
                let b = self.stack.pop().unwrap();
                let a = self.stack.pop().unwrap();
                self.stack.push(a ^ b);
                self.pc += 1;
                self.gas_used += 10;
            }
            
            // NOT - Bitwise NOT
            0x23 => {
                if self.stack.is_empty() {
                    return Err(anyhow::anyhow!("{}", VMError::StackUnderflow));
                }
                
                let a = self.stack.pop().unwrap();
                self.stack.push(!a);
                self.pc += 1;
                self.gas_used += 10;
            }
            
            // JUMP - Unconditional jump
            0x30 => {
                if self.stack.is_empty() {
                    return Err(anyhow::anyhow!("{}", VMError::StackUnderflow));
                }
                
                let dest = self.stack.pop().unwrap() as usize;
                if dest >= self.memory.len() {
                    return Err(anyhow::anyhow!("{}", VMError::InvalidJumpDestination(dest)));
                }
                
                self.pc = dest;
                self.gas_used += 15;
            }
            
            // JUMPIF - Conditional jump
            0x31 => {
                self.pc += 1;
                if self.pc >= self.memory.len() {
                    return Err(anyhow::anyhow!("{}", VMError::OutOfMemory(self.pc)));
                }
                
                let dest = self.memory[self.pc] as usize;
                self.pc += 1;
                
                if self.stack.is_empty() {
                    return Err(anyhow::anyhow!("{}", VMError::StackUnderflow));
                }
                
                let condition = self.stack.pop().unwrap();
                if condition != 0 {
                    if dest >= self.memory.len() {
                        return Err(anyhow::anyhow!("{}", VMError::InvalidJumpDestination(dest)));
                    }
                    self.pc = dest;
                }
                
                self.gas_used += 15;
            }
            
            // CALL - Function call
            0x32 => {
                if self.stack.is_empty() {
                    return Err(anyhow::anyhow!("{}", VMError::StackUnderflow));
                }
                
                let dest = self.stack.pop().unwrap() as usize;
                if dest >= self.memory.len() {
                    return Err(anyhow::anyhow!("{}", VMError::InvalidJumpDestination(dest)));
                }
                
                self.stack.push(self.pc as u32 + 1);
                self.pc = dest;
                self.gas_used += 25;
            }
            
            // RETURN - Return from function
            0x33 => {
                if self.stack.is_empty() {
                    return Err(anyhow::anyhow!("{}", VMError::StackUnderflow));
                }
                
                let ret_addr = self.stack.pop().unwrap() as usize;
                if ret_addr >= self.memory.len() {
                    return Err(anyhow::anyhow!("{}", VMError::InvalidJumpDestination(ret_addr)));
                }
                
                self.pc = ret_addr;
                self.gas_used += 25;
            }
            
            // LOAD - Load from memory
            0x40 => {
                self.pc += 1;
                if self.pc >= self.memory.len() {
                    return Err(anyhow::anyhow!("{}", VMError::OutOfMemory(self.pc)));
                }
                
                let addr = self.memory[self.pc] as usize;
                if addr >= self.memory.len() {
                    return Err(anyhow::anyhow!("{}", VMError::OutOfMemory(addr)));
                }
                
                self.stack.push(self.memory[addr] as u32);
                self.pc += 1;
                self.gas_used += 30;
            }
            
            // STORE - Store to memory
            0x41 => {
                self.pc += 1;
                if self.pc >= self.memory.len() {
                    return Err(anyhow::anyhow!("{}", VMError::OutOfMemory(self.pc)));
                }
                
                let addr = self.memory[self.pc] as usize;
                if addr >= self.memory.len() {
                    return Err(anyhow::anyhow!("{}", VMError::OutOfMemory(addr)));
                }
                
                if self.stack.is_empty() {
                    return Err(anyhow::anyhow!("{}", VMError::StackUnderflow));
                }
                
                let value = self.stack.pop().unwrap() as u8;
                self.memory[addr] = value;
                self.pc += 1;
                self.gas_used += 30;
            }
            
            // SOLTRANSFER - Transfer SOL
            0x50 => {
                if self.stack.len() < 2 {
                    return Err(anyhow::anyhow!("{}", VMError::StackUnderflow));
                }
                
                let amount = self.stack.pop().unwrap();
                let to_addr = self.stack.pop().unwrap();
                
                // In a real implementation, this would interact with the Solana system
                // For now, we'll just log the transfer
                log::info!("SOL transfer: {} lamports to address {:#010X}", amount, to_addr);
                
                self.pc += 1;
                self.gas_used += 100;
            }
            
            // TOKENTRANSFER - Transfer tokens
            0x51 => {
                if self.stack.len() < 3 {
                    return Err(anyhow::anyhow!("{}", VMError::StackUnderflow));
                }
                
                let amount = self.stack.pop().unwrap();
                let to_addr = self.stack.pop().unwrap();
                let token_addr = self.stack.pop().unwrap();
                
                // In a real implementation, this would interact with the Solana token system
                // For now, we'll just log the transfer
                log::info!("Token transfer: {} tokens from address {:#010X} to address {:#010X}", 
                           amount, token_addr, to_addr);
                
                self.pc += 1;
                self.gas_used += 100;
            }
            
            // SYSCALL - System call
            0x52 => {
                if self.stack.is_empty() {
                    return Err(anyhow::anyhow!("{}", VMError::StackUnderflow));
                }
                
                let syscall_id = self.stack.pop().unwrap();
                
                // Process system call based on ID
                match syscall_id {
                    0 => {
                        // Print top of stack
                        if self.stack.is_empty() {
                            return Err(anyhow::anyhow!("{}", VMError::StackUnderflow));
                        }
                        let value = self.stack.pop().unwrap();
                        log::info!("SYSCALL print: {}", value);
                    }
                    1 => {
                        // Get timestamp
                        let timestamp = std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_secs() as u32;
                        self.stack.push(timestamp);
                    }
                    2 => {
                        // Get random number
                        use rand::Rng;
                        let value = rand::thread_rng().gen::<u32>();
                        self.stack.push(value);
                    }
                    _ => {
                        return Err(anyhow::anyhow!("Unknown syscall ID: {}", syscall_id));
                    }
                }
                
                self.pc += 1;
                self.gas_used += 100;
            }
            
            // SIMD - Vector load - Load 4 consecutive values into vector register
            0x60 => {
                self.pc += 1;
                if self.pc >= self.memory.len() {
                    return Err(anyhow::anyhow!("{}", VMError::OutOfMemory(self.pc)));
                }
                
                // Get vector register idx (0-3) and memory address
                let vreg_idx = (self.memory[self.pc] >> 4) & 0x3; // Top 4 bits (but use only 2)
                
                // Get memory address from stack
                if self.stack.is_empty() {
                    return Err(anyhow::anyhow!("{}", VMError::StackUnderflow));
                }
                let addr = self.stack.pop().unwrap() as usize;
                
                // Check if all addresses are in range
                if addr + 3 >= self.memory.len() {
                    return Err(anyhow::anyhow!("{}", VMError::OutOfMemory(addr + 3)));
                }
                
                // Load 4 consecutive values into the vector register
                for i in 0..4 {
                    self.vector_registers[vreg_idx as usize][i] = self.memory[addr + i] as u32;
                }
                
                self.pc += 1;
                self.gas_used += 20;
            }
            
            // SIMD - Vector store - Store vector register to 4 consecutive memory locations
            0x61 => {
                self.pc += 1;
                if self.pc >= self.memory.len() {
                    return Err(anyhow::anyhow!("{}", VMError::OutOfMemory(self.pc)));
                }
                
                // Get vector register idx (0-3)
                let vreg_idx = (self.memory[self.pc] >> 4) & 0x3; // Top 4 bits (but use only 2)
                
                // Get memory address from stack
                if self.stack.is_empty() {
                    return Err(anyhow::anyhow!("{}", VMError::StackUnderflow));
                }
                let addr = self.stack.pop().unwrap() as usize;
                
                // Check if all addresses are in range
                if addr + 3 >= self.memory.len() {
                    return Err(anyhow::anyhow!("{}", VMError::OutOfMemory(addr + 3)));
                }
                
                // Store vector register to consecutive memory locations
                for i in 0..4 {
                    self.memory[addr + i] = (self.vector_registers[vreg_idx as usize][i] & 0xFF) as u8;
                }
                
                self.pc += 1;
                self.gas_used += 20;
            }
            
            // SIMD - Vector add
            0x62 => {
                self.pc += 1;
                if self.pc >= self.memory.len() {
                    return Err(anyhow::anyhow!("{}", VMError::OutOfMemory(self.pc)));
                }
                
                // Get vector register indices
                let reg_byte = self.memory[self.pc];
                let dest_reg = (reg_byte >> 4) & 0x3; // Top 4 bits (but use only 2)
                let src_reg = reg_byte & 0x3; // Bottom 4 bits (but use only 2)
                
                // Perform vector addition
                for i in 0..4 {
                    self.vector_registers[dest_reg as usize][i] = 
                        self.vector_registers[dest_reg as usize][i].wrapping_add(self.vector_registers[src_reg as usize][i]);
                }
                
                self.pc += 1;
                self.gas_used += 15;
            }
            
            // SIMD - Vector subtract
            0x63 => {
                self.pc += 1;
                if self.pc >= self.memory.len() {
                    return Err(anyhow::anyhow!("{}", VMError::OutOfMemory(self.pc)));
                }
                
                // Get vector register indices
                let reg_byte = self.memory[self.pc];
                let dest_reg = (reg_byte >> 4) & 0x3; // Top 4 bits (but use only 2)
                let src_reg = reg_byte & 0x3; // Bottom 4 bits (but use only 2)
                
                // Perform vector subtraction
                for i in 0..4 {
                    self.vector_registers[dest_reg as usize][i] = 
                        self.vector_registers[dest_reg as usize][i].wrapping_sub(self.vector_registers[src_reg as usize][i]);
                }
                
                self.pc += 1;
                self.gas_used += 15;
            }
            
            // SIMD - Vector dot product
            0x64 => {
                self.pc += 1;
                if self.pc >= self.memory.len() {
                    return Err(anyhow::anyhow!("{}", VMError::OutOfMemory(self.pc)));
                }
                
                // Get vector register indices
                let reg_byte = self.memory[self.pc];
                let a_reg = (reg_byte >> 4) & 0x3; // Top 4 bits (but use only 2)
                let b_reg = reg_byte & 0x3; // Bottom 4 bits (but use only 2)
                
                // Calculate dot product: a·b = a₀b₀ + a₁b₁ + a₂b₂ + a₃b₃
                let mut dot_product: u32 = 0;
                for i in 0..4 {
                    dot_product = dot_product.wrapping_add(
                        self.vector_registers[a_reg as usize][i].wrapping_mul(self.vector_registers[b_reg as usize][i])
                    );
                }
                
                self.stack.push(dot_product);
                self.pc += 1;
                self.gas_used += 25;
            }

            // PUSHF - Push floating point value onto stack
            0x70 => {
                self.pc += 1;
                if self.pc + 7 >= self.memory.len() {
                    return Err(anyhow::anyhow!("{}", VMError::OutOfMemory(self.pc)));
                }
                
                // Read 8 bytes and convert to f64
                let mut bytes = [0u8; 8];
                for i in 0..8 {
                    bytes[i] = self.memory[self.pc + i];
                }
                let value = f64::from_le_bytes(bytes);
                
                // Store the raw bytes as u32 values on the stack (2 u32s for 1 f64)
                let bytes_as_u32: [u32; 2] = unsafe { std::mem::transmute(value) };
                self.stack.push(bytes_as_u32[0]);
                self.stack.push(bytes_as_u32[1]);
                
                self.pc += 8;
                self.gas_used += 5;
            }
            
            // LOADF - Load floating point value from register to stack
            0x71 => {
                self.pc += 1;
                if self.pc >= self.memory.len() {
                    return Err(anyhow::anyhow!("{}", VMError::OutOfMemory(self.pc)));
                }
                
                // Get register index
                let reg_idx = self.memory[self.pc] as usize;
                if reg_idx >= self.fp_registers.len() {
                    return Err(anyhow::anyhow!("Invalid floating point register: {}", reg_idx));
                }
                
                // Get the value and push it to the stack as 2 u32s
                let value = self.fp_registers[reg_idx];
                let bytes_as_u32: [u32; 2] = unsafe { std::mem::transmute(value) };
                self.stack.push(bytes_as_u32[0]);
                self.stack.push(bytes_as_u32[1]);
                
                self.pc += 1;
                self.gas_used += 3;
            }
            
            // STOREF - Store floating point value from stack to register
            0x72 => {
                self.pc += 1;
                if self.pc >= self.memory.len() {
                    return Err(anyhow::anyhow!("{}", VMError::OutOfMemory(self.pc)));
                }
                
                // Need at least 2 values on the stack
                if self.stack.len() < 2 {
                    return Err(anyhow::anyhow!("{}", VMError::StackUnderflow));
                }
                
                // Get register index
                let reg_idx = self.memory[self.pc] as usize;
                if reg_idx >= self.fp_registers.len() {
                    return Err(anyhow::anyhow!("Invalid floating point register: {}", reg_idx));
                }
                
                // Pop two u32 values and convert to f64
                let high = self.stack.pop().unwrap();
                let low = self.stack.pop().unwrap();
                let bytes_as_u32 = [low, high];
                let value = unsafe { std::mem::transmute(bytes_as_u32) };
                
                // Store the value in the register
                self.fp_registers[reg_idx] = value;
                
                self.pc += 1;
                self.gas_used += 3;
            }
            
            // ADDF - Floating point addition
            0x73 => {
                self.pc += 1;
                if self.pc >= self.memory.len() {
                    return Err(anyhow::anyhow!("{}", VMError::OutOfMemory(self.pc)));
                }
                
                // Get register indices
                let reg_byte = self.memory[self.pc];
                let dest_reg = (reg_byte >> 4) & 0x7; // Top 4 bits (but use only 3)
                let src_reg = reg_byte & 0x7; // Bottom 4 bits (but use only 3)
                
                // Perform floating point addition
                self.fp_registers[dest_reg as usize] += self.fp_registers[src_reg as usize];
                
                self.pc += 1;
                self.gas_used += 5;
            }
            
            // SUBF - Floating point subtraction
            0x74 => {
                self.pc += 1;
                if self.pc >= self.memory.len() {
                    return Err(anyhow::anyhow!("{}", VMError::OutOfMemory(self.pc)));
                }
                
                // Get register indices
                let reg_byte = self.memory[self.pc];
                let dest_reg = (reg_byte >> 4) & 0x7; // Top 4 bits (but use only 3)
                let src_reg = reg_byte & 0x7; // Bottom 4 bits (but use only 3)
                
                // Perform floating point subtraction
                self.fp_registers[dest_reg as usize] -= self.fp_registers[src_reg as usize];
                
                self.pc += 1;
                self.gas_used += 5;
            }
            
            // MULF - Floating point multiplication
            0x75 => {
                self.pc += 1;
                if self.pc >= self.memory.len() {
                    return Err(anyhow::anyhow!("{}", VMError::OutOfMemory(self.pc)));
                }
                
                // Get register indices
                let reg_byte = self.memory[self.pc];
                let dest_reg = (reg_byte >> 4) & 0x7; // Top 4 bits (but use only 3)
                let src_reg = reg_byte & 0x7; // Bottom 4 bits (but use only 3)
                
                // Perform floating point multiplication
                self.fp_registers[dest_reg as usize] *= self.fp_registers[src_reg as usize];
                
                self.pc += 1;
                self.gas_used += 8;
            }
            
            // DIVF - Floating point division
            0x76 => {
                self.pc += 1;
                if self.pc >= self.memory.len() {
                    return Err(anyhow::anyhow!("{}", VMError::OutOfMemory(self.pc)));
                }
                
                // Get register indices
                let reg_byte = self.memory[self.pc];
                let dest_reg = (reg_byte >> 4) & 0x7; // Top 4 bits (but use only 3)
                let src_reg = reg_byte & 0x7; // Bottom 4 bits (but use only 3)
                
                // Check for division by zero
                if self.fp_registers[src_reg as usize] == 0.0 {
                    self.fp_registers[dest_reg as usize] = f64::INFINITY;
                } else {
                    // Perform floating point division
                    self.fp_registers[dest_reg as usize] /= self.fp_registers[src_reg as usize];
                }
                
                self.pc += 1;
                self.gas_used += 10;
            }
            
            // MATMUL - Matrix multiplication
            0x80 => {
                self.pc += 1;
                if self.pc >= self.memory.len() {
                    return Err(anyhow::anyhow!("{}", VMError::OutOfMemory(self.pc)));
                }
                
                // Get matrix indices (we only have 2 matrix registers)
                let mat_indices = self.memory[self.pc];
                let dest_idx = (mat_indices >> 4) & 0x1; // Use only 1 bit
                let src_idx = mat_indices & 0x1; // Use only 1 bit
                
                // Create result matrix (initialized to zeros)
                let mut result = [[0.0f32; 4]; 4];
                
                // Perform matrix multiplication
                for i in 0..4 {
                    for j in 0..4 {
                        for k in 0..4 {
                            result[i][j] += self.matrix_registers[dest_idx as usize][i][k] * 
                                          self.matrix_registers[src_idx as usize][k][j];
                        }
                    }
                }
                
                // Store result back in destination matrix
                self.matrix_registers[dest_idx as usize] = result;
                
                self.pc += 1;
                self.gas_used += 40; // Matrix multiplication is expensive
            }
            
            // CRYPTOHASH - Compute SHA-256 hash of data in memory
            0x90 => {
                self.pc += 1;
                if self.pc + 1 >= self.memory.len() {
                    return Err(anyhow::anyhow!("{}", VMError::OutOfMemory(self.pc)));
                }
                
                // TODO: Implement SHA-256 hashing logic
                self.pc += 2; 
                self.gas_used += 100; // Cryptographic operations are expensive
            }

            // CPLXADD - Complex number addition
            0xA0 => {
                self.pc += 1;
                if self.pc >= self.memory.len() {
                    return Err(anyhow::anyhow!("{}", VMError::OutOfMemory(self.pc)));
                }
                
                // Get complex register indices
                let reg_byte = self.memory[self.pc];
                let dest_reg = (reg_byte >> 4) & 0x3; // Top 4 bits (but use only 2)
                let src_reg = reg_byte & 0x3; // Bottom 4 bits (but use only 2)
                
                // Perform complex addition (a+bi) + (c+di) = (a+c) + (b+d)i
                self.complex_registers[dest_reg as usize][0] += self.complex_registers[src_reg as usize][0]; // Real part
                self.complex_registers[dest_reg as usize][1] += self.complex_registers[src_reg as usize][1]; // Imaginary part
                
                self.pc += 1;
                self.gas_used += 8;
            }
            
            // CPLXMUL - Complex number multiplication
            0xA1 => {
                self.pc += 1;
                if self.pc >= self.memory.len() {
                    return Err(anyhow::anyhow!("{}", VMError::OutOfMemory(self.pc)));
                }
                
                // Get complex register indices
                let reg_byte = self.memory[self.pc];
                let dest_reg = (reg_byte >> 4) & 0x3; // Top 4 bits (but use only 2)
                let src_reg = reg_byte & 0x3; // Bottom 4 bits (but use only 2)
                
                // Perform complex multiplication (a+bi) * (c+di) = (ac-bd) + (ad+bc)i
                let a = self.complex_registers[dest_reg as usize][0]; // Real part of first number
                let b = self.complex_registers[dest_reg as usize][1]; // Imaginary part of first number
                let c = self.complex_registers[src_reg as usize][0]; // Real part of second number
                let d = self.complex_registers[src_reg as usize][1]; // Imaginary part of second number
                
                let real = (a * c) - (b * d);
                let imag = (a * d) + (b * c);

                // Store in temporary variables first to avoid overwriting values before using them
                let temp_real = real;
                let temp_imag = imag;
                
                self.complex_registers[dest_reg as usize][0] = temp_real; // Store real part
                self.complex_registers[dest_reg as usize][1] = imag;
                
                self.pc += 1;
                self.gas_used += 15;
            }

            // STRPUSH - Push a character to string buffer
            0xB0 => {
                self.pc += 1;
                if self.pc >= self.memory.len() {
                    return Err(anyhow::anyhow!("{}", VMError::OutOfMemory(self.pc)));
                }
                
                // Get the character from memory
                let ch = self.memory[self.pc] as char;
                
                // Push it to the string buffer
                self.string_buffer.push(ch);
                
                self.pc += 1;
                self.gas_used += 2;
            }
            
            // STRPOP - Pop a character from string buffer
            0xB1 => {
                // Check if string buffer is empty
                if self.string_buffer.is_empty() {
                    self.stack.push(0); // Push 0 if empty
                } else {
                    // Pop the last character
                    if let Some(ch) = self.string_buffer.pop() {
                        self.stack.push(ch as u32);
                    } else {
                        self.stack.push(0);
                    }
                }
                
                self.pc += 1;
                self.gas_used += 2;
            }
            
            // STRCLEAR - Clear the string buffer
            0xB2 => {
                self.string_buffer.clear();
                
                self.pc += 1;
                self.gas_used += 1;
            }
            
            // STRLEN - Get string length onto stack
            0xB3 => {
                self.stack.push(self.string_buffer.len() as u32);
                
                self.pc += 1;
                self.gas_used += 1;
            }

            // JSONPARSE - Parse string buffer as JSON and extract value
            0xC0 => {
                self.pc += 1;
                if self.pc >= self.memory.len() {
                    return Err(anyhow::anyhow!("{}", VMError::OutOfMemory(self.pc)));
                }
                
                // In a real implementation, this would parse the JSON in the string buffer
                // and extract the value at the specified path.
                // For now, we'll just return a dummy value
                self.stack.push(42);
                
                self.pc += 1;
                self.gas_used += 25;
            }
            
            // MAPINIT - Initialize a key-value map 
            0xD0 => {
                // In a real implementation, this would allocate a map data structure
                // For this prototype, we'll just modify our state to indicate a map is active
                
                self.pc += 1;
                self.gas_used += 10;
            }
            
            // MAPSET - Set a key-value pair in the map
            0xD1 => {
                // Check if we have at least 2 values on the stack (key and value)
                if self.stack.len() < 2 {
                    return Err(anyhow::anyhow!("{}", VMError::StackUnderflow));
                }
                
                // Pop value and key from stack
                let value = self.stack.pop().unwrap();
                let key = self.stack.pop().unwrap();
                
                // In a real implementation, this would add the key-value pair to a map
                // For this prototype, we'll just log it
                log::info!("Map set: key={}, value={}", key, value);
                
                self.pc += 1;
                self.gas_used += 5;
            }
            
            // MAPGET - Get a value from the map by key
            0xD2 => {
                // Check if we have at least 1 value on the stack (key)
                if self.stack.is_empty() {
                    return Err(anyhow::anyhow!("{}", VMError::StackUnderflow));
                }
                
                // Pop key from stack
                let key = self.stack.pop().unwrap();
                
                // In a real implementation, this would look up the key in a map
                // For this prototype, we'll just return a dummy value
                self.stack.push(key); // Echo back the key as a dummy value
                
                self.pc += 1;
                self.gas_used += 5;
            }
            
            // RAND - Generate a cryptographically secure random number
            0xE0 => {
                // In a real implementation, this would use a CSPRNG
                self.stack.push(42); // Dummy value
                self.pc += 1;
                self.gas_used += 20;
            }
            
            // HALT - Stop execution
            0xFF => {
                self.state = VMState::Halted;
                self.pc += 1;
                self.gas_used += 1;
                return Ok(false);
            }
            
            // Invalid opcode
            _ => {
                return Err(anyhow::anyhow!("{}", VMError::InvalidOpcode(opcode)));
            }
        }
        
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_vm() {
        let vm = VM::new();
        assert_eq!(vm.pc, 0);
        assert_eq!(vm.stack.len(), 0);
        assert_eq!(vm.gas_used(), 0);
        
        match vm.state {
            VMState::Ready => {}
            _ => panic!("VM should be in Ready state"),
        }
    }
    
    #[test]
    fn test_push_pop() {
        let mut vm = VM::new();
        
        // Create test program: PUSH1 42, PUSH1 21, ADD, HALT
        let bytecode = vec![0x01, 42, 0x01, 21, 0x10, 0xFF];
        vm.load_bytecode(&bytecode).unwrap();
        
        // Run the program
        vm.run(10).unwrap();
        
        // Check the result
        assert_eq!(vm.stack.len(), 1);
        assert_eq!(vm.stack[0], 63);
        
        match vm.state {
            VMState::Halted => {}
            _ => panic!("VM should be in Halted state"),
        }
    }
    
    #[test]
    fn test_jump() {
        let mut vm = VM::new();
        
        // Create test program: 
        // 0: PUSH1 6  (Jump target - after HALT)
        // 2: PUSH1 0  (Condition)
        // 4: JUMPIF 0
        // 6: PUSH1 42
        // 8: HALT
        let bytecode = vec![0x01, 6, 0x01, 0, 0x31, 0, 0x01, 42, 0xFF];
        vm.load_bytecode(&bytecode).unwrap();
        
        // Run the program
        vm.run(10).unwrap();

        // The stack should contain [6, 42] because:
        // 1. PUSH1 6 - Pushes 6 to the stack
        // 2. PUSH1 0 - Pushes 0 to the stack
        // 3. JUMPIF 0 - Pops 0 (condition) but does not jump because condition is 0
        //               Jump target 6 is still on stack
        // 4. PUSH1 42 - Pushes 42 to the stack
        // 5. HALT

        // Check the result
        assert_eq!(vm.stack.len(), 2);
        assert_eq!(vm.stack[1], 42);
        match vm.state {
            VMState::Halted => {}
            _ => panic!("VM should be in Halted state"),
        }
    }
    
    #[test]
    fn test_simd_instructions() {
        let mut vm = VM::new();
        
        // Create a program that uses SIMD instructions:
        // 1. Push memory address 100 to stack
        // 2. Load 4 values at address 100 into vector register 0
        // 3. Push memory address 104 to stack
        // 4. Load 4 values at address 104 into vector register 1
        // 5. Add vector registers 0 and 1, store result in vector register 0
        // 6. Push memory address 108 to stack 
        // 7. Store vector register 0 to memory at address 108
        // 8. Calculate dot product of vector registers 0 and 1
        // 9. Halt
        let program = vec![
            0x01, 100,            // PUSH1 100 (address for first vector)
            0x60, 0x00,           // VLOAD register 0
            0x01, 104,            // PUSH1 104 (address for second vector)
            0x60, 0x10,           // VLOAD register 1
            0x62, 0x01,           // VADD registers 0 and 1, result in 0
            0x01, 108,            // PUSH1 108 (store result address)
            0x61, 0x00,           // VSTORE register 0
            0x64, 0x01,           // VDOT registers 0 and 1
            0xFF                  // HALT
        ];
        
        // Set up test data in memory
        vm.load_bytecode(&program).unwrap();
        
        // Initialize memory with test vectors
        // First vector: [1, 2, 3, 4]
        vm.memory[100] = 1;
        vm.memory[101] = 2;
        vm.memory[102] = 3;
        vm.memory[103] = 4;
        
        // Second vector: [5, 6, 7, 8]
        vm.memory[104] = 5;
        vm.memory[105] = 6;
        vm.memory[106] = 7;
        vm.memory[107] = 8;
        
        // Run the program
        vm.run(20).unwrap();
        
        // Check that vector addition worked: [1,2,3,4] + [5,6,7,8] = [6,8,10,12]
        assert_eq!(vm.memory[108], 6);
        assert_eq!(vm.memory[109], 8);
        assert_eq!(vm.memory[110], 10);
        assert_eq!(vm.memory[111], 12);
        
        // Check dot product of [6,8,10,12] and [5,6,7,8]:
        // 6*5 + 8*6 + 10*7 + 12*8 = 30 + 48 + 70 + 96 = 244
        assert_eq!(vm.stack.len(), 1);
        assert_eq!(vm.stack[0], 244);
    }
    
    #[test]
    fn test_floating_point_operations() {
        let mut vm = VM::new();
        
        // Create a program that uses floating point operations:
        // 1. PUSHF 3.14159 (pi)
        // 2. STOREF 0 (store in fp register 0)
        // 3. PUSHF 2.71828 (e)
        // 4. STOREF 1 (store in fp register 1)
        // 5. ADDF 0, 1 (0 += 1) - now register 0 contains pi + e
        // 6. LOADF 0 (load result to stack)
        // 7. HALT
        
        // Set up our pi and e constants
        let pi_bytes = 3.14159f64.to_le_bytes();
        let e_bytes = 2.71828f64.to_le_bytes();
        
        let program = vec![
            0x70, pi_bytes[0], pi_bytes[1], pi_bytes[2], pi_bytes[3], pi_bytes[4], pi_bytes[5], pi_bytes[6], pi_bytes[7], // PUSHF 3.14159
            0x72, 0, // STOREF 0
            0x70, e_bytes[0], e_bytes[1], e_bytes[2], e_bytes[3], e_bytes[4], e_bytes[5], e_bytes[6], e_bytes[7], // PUSHF 2.71828
            0x72, 1, // STOREF 1
            0x73, 0x01, // ADDF 0, 1 
            0x71, 0, // LOADF 0
            0xFF // HALT
        ];
        
        vm.load_bytecode(&program).unwrap();
        vm.run(20).unwrap();
        
        // Check that we have two u32 values on the stack (representing one f64)
        assert_eq!(vm.stack.len(), 2);
        
        // Convert the two u32 values back to an f64
        let bytes_as_u32 = [vm.stack[0], vm.stack[1]];
        let result: f64 = unsafe { std::mem::transmute(bytes_as_u32) };
        
        // The result should be approximately pi + e
        let expected = 3.14159 + 2.71828;
        assert!((result - expected).abs() < 0.0001);
    }
    
    #[test]
    fn test_complex_number_operations() {
        let mut vm = VM::new();
        
        // Initialize complex numbers directly
        vm.complex_registers[0][0] = 3.0; // Real part of complex number 0
        vm.complex_registers[0][1] = 4.0; // Imaginary part of complex number 0
        
        vm.complex_registers[1][0] = 1.0; // Real part of complex number 1
        vm.complex_registers[1][1] = 2.0; // Imaginary part of complex number 1
        
        // Verify initial values before multiplication
        assert_eq!(vm.complex_registers[0][0], 3.0);
        assert_eq!(vm.complex_registers[0][1], 4.0);
        assert_eq!(vm.complex_registers[1][0], 1.0);
        assert_eq!(vm.complex_registers[1][1], 2.0);

        // For debugging: print a message
        println!("Initial complex values: [{}, {}i] and [{}, {}i]", vm.complex_registers[0][0], vm.complex_registers[0][1], vm.complex_registers[1][0], vm.complex_registers[1][1]);
        // Create a program that performs complex number multiplication:
        // 1. CPLXMUL 0, 1 (perform (3+4i) * (1+2i) = (3-8) + (4+6)i = -5 + 10i)
        // 2. HALT
        let program = vec![
            0xA1, 0x01, // CPLXMUL 0, 1
            0xFF // HALT
        ];
        
        vm.load_bytecode(&program).unwrap();
        vm.run(10).unwrap();
        
        // For debugging: print result values
        println!("After CPLXMUL: [{}, {}i]", 
                 vm.complex_registers[0][0], 
                 vm.complex_registers[0][1]);
        
        // Check the result: (3+4i) * (1+2i) = -5 + 10i
        assert_eq!(vm.complex_registers[0][0], -5.0); // Real part
        assert_eq!(vm.complex_registers[0][1], 10.0); // Imaginary part
    }
    
    #[test]
    fn test_string_operations() {
        let mut vm = VM::new();
        
        // Create a program that tests string operations:
        // 1. STRPUSH 'H'
        // 2. STRPUSH 'e'
        // 3. STRPUSH 'l'
        // 4. STRPUSH 'l'
        // 5. STRPUSH 'o'
        // 6. STRLEN (should push 5 onto stack)
        // 7. STRPOP (should pop 'o' and push 111 (ASCII for 'o') onto stack)
        // 8. HALT
        let program = vec![
            0xB0, 'H' as u8, // STRPUSH 'H'
            0xB0, 'e' as u8, // STRPUSH 'e'
            0xB0, 'l' as u8, // STRPUSH 'l'
            0xB0, 'l' as u8, // STRPUSH 'l'
            0xB0, 'o' as u8, // STRPUSH 'o'
            0xB3, // STRLEN
            0xB1, // STRPOP
            0xFF // HALT
        ];
        
        vm.load_bytecode(&program).unwrap();
        vm.run(10).unwrap();
        
        // Check the stack has 2 values: length (5) and last char ('o' = 111)
        assert_eq!(vm.stack.len(), 2);
        assert_eq!(vm.stack[0], 5); // String length
        assert_eq!(vm.stack[1], 'o' as u32); // Popped character
        
        // Check that the string buffer now contains "Hell"
        assert_eq!(vm.string_buffer, "Hell");
    }
    
    #[test]
    fn test_matrix_operations() {
        let mut vm = VM::new();
        
        // Initialize identity matrix in matrix register 0
        vm.matrix_registers[0] = [[1.0, 0.0, 0.0, 0.0], 
                                  [0.0, 1.0, 0.0, 0.0],
                                  [0.0, 0.0, 1.0, 0.0],
                                  [0.0, 0.0, 0.0, 1.0]];
        
        // Set up another matrix in register 1 with some values
        vm.matrix_registers[1] = [[2.0, 3.0, 0.0, 0.0],
                                  [1.0, 4.0, 0.0, 0.0],
                                  [0.0, 0.0, 5.0, 6.0],
                                  [0.0, 0.0, 7.0, 8.0]];
                                  
        // Create a program that multiplies matrices
        let program = vec![
            0x80, 0x01, // MATMUL 0, 1 (multiply matrices and store in register 0)
            0xFF // HALT
        ];
        
        vm.load_bytecode(&program).unwrap();
        vm.run(10).unwrap();
        
        // Check results: identity matrix * any matrix = that matrix
        // So result in register 0 should be the same as what was in register 1
        for i in 0..4 {
            for j in 0..4 {
                assert_eq!(vm.matrix_registers[0][i][j], vm.matrix_registers[1][i][j]);
            }
        }
    }
    
    #[test]
    fn test_map_operations() {
        let mut vm = VM::new();
        
        // Create a program that tests map operations:
        // 1. MAPINIT (initialize a map)
        // 2. PUSH1 42 (value)
        // 3. PUSH1 100 (key)
        // 4. MAPSET (set key 100 to value 42)
        // 5. PUSH1 100 (key to look up)
        // 6. MAPGET (get value for key 100 - should be 42)
        // 7. HALT
        let program = vec![
            0xD0, // MAPINIT
            0x01, 42, // PUSH1 42 (value)
            0x01, 100, // PUSH1 100 (key)
            0xD1, // MAPSET
            0x01, 100, // PUSH1 100 (key)
            0xD2, // MAPGET
            0xFF // HALT
        ];
        
        vm.load_bytecode(&program).unwrap();
        vm.run(10).unwrap();
        
        // In our implementation, MAPGET just echoes back the key as a dummy value
        assert_eq!(vm.stack.len(), 1);
        assert_eq!(vm.stack[0], 100);
    }
}