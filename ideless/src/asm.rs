use std::fmt;

/// Instruction representation with metadata
#[derive(Debug, Clone)]
pub struct Instruction {
    pub opcode: u8,
    pub name: String,
    pub description: String,
    pub params: Vec<u8>,
    pub offset: usize,
    pub size: usize,
    pub issues: Vec<String>,
}

/// Disassembled line with metadata
#[derive(Debug, Clone)]
pub struct DisassembledLine {
    pub offset: usize,
    pub raw_bytes: Vec<u8>,
    pub instruction: Option<String>,
    pub description: Option<String>,
    pub issues: Vec<String>,
}

/// LessVM Disassembler
pub struct Disassembler {
    pub bytecode: Vec<u8>,
    pub disassembled: Vec<DisassembledLine>,
    pub has_issues: bool,
}

impl Disassembler {
    /// Create a new disassembler for the given bytecode
    pub fn new(bytecode: Vec<u8>) -> Self {
        Disassembler {
            bytecode,
            disassembled: Vec::new(),
            has_issues: false,
        }
    }
    
    /// Run the disassembler
    pub fn run(&mut self) {
        let mut offset = 0;
        
        while offset < self.bytecode.len() {
            let (line, next_offset) = self.disassemble_instruction(offset);
            self.disassembled.push(line);
            
            // Move to next instruction
            offset = next_offset;
        }
    }
    
    /// Get opcode name
    fn get_opcode_name(&self, opcode: u8) -> String {
        match opcode {
            0x01 => "PUSH1".to_string(),
            0x02 => "PUSH2".to_string(),
            0x03 => "PUSH4".to_string(),
            0x04 => "POP".to_string(),
            0x05 => "DUP".to_string(),
            0x06 => "SWAP".to_string(),
            0x10 => "ADD".to_string(),
            0x11 => "SUB".to_string(),
            0x12 => "MUL".to_string(),
            0x13 => "DIV".to_string(),
            0x20 => "AND".to_string(),
            0x21 => "OR".to_string(),
            0x22 => "XOR".to_string(),
            0x23 => "NOT".to_string(),
            0x30 => "JUMP".to_string(),
            0x31 => "JUMPIF".to_string(),
            0x32 => "CALL".to_string(),
            0x33 => "RETURN".to_string(),
            0x40 => "LOAD".to_string(),
            0x41 => "STORE".to_string(),
            0x50 => "SOLTRANSFER".to_string(),
            0x51 => "TOKENTRANSFER".to_string(),
            0x52 => "SYSCALL".to_string(),
            0xFF => "HALT".to_string(),
            0x60 => "VLOAD".to_string(),
            0x61 => "VSTORE".to_string(),
            0x62 => "VADD".to_string(),
            0x63 => "VSUB".to_string(),
            0x64 => "VDOT".to_string(),
            0x70 => "PUSHF".to_string(),
            0x71 => "LOADF".to_string(),
            0x72 => "STOREF".to_string(),
            0x73 => "ADDF".to_string(),
            0x74 => "SUBF".to_string(),
            0x75 => "MULF".to_string(),
            0x76 => "DIVF".to_string(),
            0x80 => "MATMUL".to_string(),
            0x90 => "CRYPTOHASH".to_string(),
            0xA0 => "CPLXADD".to_string(),
            0xA1 => "CPLXMUL".to_string(),
            0xB0 => "STRPUSH".to_string(),
            0xB1 => "STRPOP".to_string(),
            0xB2 => "STRCLEAR".to_string(),
            0xB3 => "STRLEN".to_string(),
            0xC0 => "JSONPARSE".to_string(),
            0xD0 => "MAPINIT".to_string(),
            0xD1 => "MAPSET".to_string(),
            0xD2 => "MAPGET".to_string(),
            0xE0 => "RAND".to_string(),
            _ => format!("UNKNOWN(0x{:02X})", opcode),
        }
    }
    
    /// Get opcode description
    fn get_opcode_description(&self, opcode: u8) -> String {
        match opcode {
            0x01 => "Push 1-byte value onto stack".to_string(),
            0x02 => "Push 2-byte value onto stack".to_string(),
            0x03 => "Push 4-byte value onto stack".to_string(),
            0x04 => "Remove top item from stack".to_string(),
            0x05 => "Duplicate nth stack item".to_string(),
            0x06 => "Swap nth stack item with top".to_string(),
            0x10 => "Addition".to_string(),
            0x11 => "Subtraction".to_string(),
            0x12 => "Multiplication".to_string(),
            0x13 => "Division".to_string(),
            0x20 => "Bitwise AND".to_string(),
            0x21 => "Bitwise OR".to_string(),
            0x22 => "Bitwise XOR".to_string(),
            0x23 => "Bitwise NOT".to_string(),
            0x30 => "Unconditional jump".to_string(),
            0x31 => "Conditional jump".to_string(),
            0x32 => "Function call".to_string(),
            0x33 => "Return from function".to_string(),
            0x40 => "Load from memory".to_string(),
            0x41 => "Store to memory".to_string(),
            0x50 => "Transfer SOL".to_string(),
            0x51 => "Transfer tokens".to_string(),
            0x52 => "System call".to_string(),
            0xFF => "Halt execution".to_string(),
            0x60 => "Load 4 values into vector register".to_string(),
            0x61 => "Store vector register to memory".to_string(),
            0x62 => "Vector addition".to_string(),
            0x63 => "Vector subtraction".to_string(),
            0x64 => "Vector dot product".to_string(),
            0x70 => "Push 64-bit floating point value onto stack".to_string(),
            0x71 => "Load floating point value from register to stack".to_string(),
            0x72 => "Store floating point value from stack to register".to_string(),
            0x73 => "Floating point addition".to_string(),
            0x74 => "Floating point subtraction".to_string(),
            0x75 => "Floating point multiplication".to_string(),
            0x76 => "Floating point division".to_string(),
            0x80 => "Matrix multiplication".to_string(),
            0x90 => "Compute SHA-256 hash".to_string(),
            0xA0 => "Complex number addition".to_string(),
            0xA1 => "Complex number multiplication".to_string(),
            0xB0 => "Push character to string buffer".to_string(),
            0xB1 => "Pop character from string buffer".to_string(),
            0xB2 => "Clear string buffer".to_string(),
            0xB3 => "Get string length".to_string(),
            0xC0 => "Parse JSON from string buffer".to_string(),
            0xD0 => "Initialize key-value map".to_string(),
            0xD1 => "Set key-value pair in map".to_string(),
            0xD2 => "Get value from map by key".to_string(),
            0xE0 => "Generate cryptographically secure random number".to_string(),
            _ => "Unknown opcode".to_string(),
        }
    }
    
    /// Get number of parameters for opcode
    fn get_opcode_param_count(&self, opcode: u8) -> usize {
        match opcode {
            0x01 => 1, // PUSH1
            0x02 => 2, // PUSH2
            0x03 => 4, // PUSH4
            0x04 => 0, // POP
            0x05 => 1, // DUP
            0x06 => 1, // SWAP
            0x10 => 0, // ADD
            0x11 => 0, // SUB
            0x12 => 0, // MUL
            0x13 => 0, // DIV
            0x20 => 0, // AND
            0x21 => 0, // OR
            0x22 => 0, // XOR
            0x23 => 0, // NOT
            0x30 => 0, // JUMP
            0x31 => 1, // JUMPIF
            0x32 => 0, // CALL
            0x33 => 0, // RETURN
            0x40 => 1, // LOAD
            0x41 => 1, // STORE
            0x50 => 0, // SOLTRANSFER
            0x51 => 0, // TOKENTRANSFER
            0x52 => 0, // SYSCALL
            0x60 => 1, // VLOAD
            0x61 => 1, // VSTORE
            0x62 => 1, // VADD
            0x63 => 1, // VSUB
            0x64 => 1, // VDOT
            0x70 => 8, // PUSHF (8 bytes)
            0x71 => 1, // LOADF
            0x72 => 1, // STOREF
            0x73 => 1, // ADDF
            0x74 => 1, // SUBF
            0x75 => 1, // MULF
            0x76 => 1, // DIVF
            0x80 => 1, // MATMUL
            0x90 => 2, // CRYPTOHASH (destination address + length)
            0xA0 => 1, // CPLXADD
            0xA1 => 1, // CPLXMUL
            0xB0 => 1, // STRPUSH
            0xB1 => 0, // STRPOP
            0xB2 => 0, // STRCLEAR
            0xB3 => 0, // STRLEN
            0xC0 => 1, // JSONPARSE
            0xD0 => 0, // MAPINIT
            0xD1 => 0, // MAPSET
            0xD2 => 0, // MAPGET
            0xE0 => 0, // RAND
            0xFF => 0, // HALT
            _ => 0,    // UNKNOWN
        }
    }
    
    /// Disassemble a single instruction
    fn disassemble_instruction(&self, offset: usize) -> (DisassembledLine, usize) {
        let mut line = DisassembledLine {
            offset,
            raw_bytes: Vec::new(),
            instruction: None,
            description: None,
            issues: Vec::new(),
        };
        
        // Check if we're past the end of the bytecode
        if offset >= self.bytecode.len() {
            line.issues.push("Offset beyond end of bytecode".to_string());
            return (line, offset + 1);
        }
        
        // Read the opcode
        let opcode = self.bytecode[offset];
        line.raw_bytes.push(opcode);
        
        // Get information about the opcode
        let opcode_name = self.get_opcode_name(opcode);
        let opcode_desc = self.get_opcode_description(opcode);
        let param_count = self.get_opcode_param_count(opcode);
        
        // Read parameters
        let mut params = Vec::new();
        let mut param_str = String::new();
        
        for i in 0..param_count {
            let param_offset = offset + 1 + i;
            
            if param_offset >= self.bytecode.len() {
                line.issues.push(format!("Incomplete instruction: missing parameter {}", i+1));
                break;
            }
            
            let param = self.bytecode[param_offset];
            params.push(param);
            line.raw_bytes.push(param);
            
            if !param_str.is_empty() {
                param_str.push_str(", ");
            }
            param_str.push_str(&format!("0x{:02X}", param));
        }
        
        // Build instruction string
        let mut instr_str = opcode_name.clone();
        if !param_str.is_empty() {
            instr_str.push_str(&format!(" {}", param_str));
        }
        
        // Set instruction and description
        line.instruction = Some(instr_str);
        line.description = Some(opcode_desc.clone());
        
        // Check for potential issues with the instruction
        if opcode_name.starts_with("UNKNOWN") {
            line.issues.push(format!("Unknown opcode: 0x{:02X}", opcode));
        }
        
        // Return the disassembled line and the next offset
        (line, offset + 1 + param_count)
    }
    
    /// Analyze compute units usage
    pub fn analyze_compute_units(&self) -> u64 {
        let mut total_cu = 0u64;
        let mut offset = 0;
        
        while offset < self.bytecode.len() {
            let opcode = self.bytecode[offset];
            let param_count = self.get_opcode_param_count(opcode);
            total_cu += self.compute_units_for_opcode(opcode);
            offset += 1 + param_count;
        }
        
        total_cu
    }
    
    /// Get detailed compute units analysis
    pub fn get_detailed_cu_analysis(&self) -> Vec<(usize, String, u64)> {
        let mut analysis = Vec::new();
        let mut offset = 0;
        
        while offset < self.bytecode.len() {
            let opcode = self.bytecode[offset];
            let cu = self.compute_units_for_opcode(opcode);
            let name = self.get_opcode_name(opcode);
            let param_count = self.get_opcode_param_count(opcode);
            
            analysis.push((offset, name, cu));
            offset += 1 + param_count;
        }
        
        analysis
    }
    
    /// Calculate compute units for an opcode
    fn compute_units_for_opcode(&self, opcode: u8) -> u64 {
        match opcode {
            // Stack operations - low cost
            0x01 | 0x02 | 0x03 => 3,  // PUSH
            0x04 => 5,                // POP
            0x05 | 0x06 => 5,         // DUP, SWAP
            
            // Basic arithmetic - medium cost
            0x10 | 0x11 => 10,        // ADD, SUB
            0x12 | 0x13 => 20,        // MUL, DIV
            
            // Bitwise operations - medium cost
            0x20 | 0x21 | 0x22 | 0x23 => 10, // AND, OR, XOR, NOT
            
            // Control flow - higher cost
            0x30 | 0x31 => 15,        // JUMP, JUMPIF
            0x32 | 0x33 => 25,        // CALL, RETURN
            
            // Memory operations - high cost
            0x40 | 0x41 => 30,        // LOAD, STORE
            
            // Solana operations - very high cost
            0x50 | 0x51 | 0x52 => 100, // SOLTRANSFER, TOKENTRANSFER, SYSCALL
            
            // SIMD operations - medium to high cost
            0x60 | 0x61 => 20,         // VLOAD, VSTORE
            0x62 | 0x63 => 15,         // VADD, VSUB
            0x64 => 25,                // VDOT
            
            // Floating point operations
            0x70 => 5,                 // PUSHF
            0x71 | 0x72 => 3,          // LOADF, STOREF
            0x73 | 0x74 => 5,          // ADDF, SUBF
            0x75 => 8,                 // MULF
            0x76 => 10,                // DIVF
            
            // Advanced operations
            0x80 => 40,                // MATMUL
            0x90 => 100,               // CRYPTOHASH
            0xA0 => 8,                 // CPLXADD
            0xA1 => 15,                // CPLXMUL
            0xB0 | 0xB1 => 2,          // STRPUSH, STRPOP
            0xB2 | 0xB3 => 1,          // STRCLEAR, STRLEN
            0xC0 => 25,                // JSONPARSE
            0xD0 => 10,                // MAPINIT
            0xD1 | 0xD2 => 5,          // MAPSET, MAPGET
            0xE0 => 20,                // RAND
            
            // Misc
            0xFF => 1,                 // HALT
            
            // Unknown opcode - default cost
            _ => 10,
        }
    }
}

impl fmt::Display for Disassembler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Offset | Bytes            | Instruction")?;
        writeln!(f, "-------|------------------|--------------------------------------------------")?;
        
        for line in &self.disassembled {
            // Format raw bytes
            let mut bytes_str = String::new();
            for byte in &line.raw_bytes {
                bytes_str.push_str(&format!("{:02X} ", byte));
            }
            
            // Pad bytes string to fixed width
            while bytes_str.len() < 18 {
                bytes_str.push(' ');
            }
            
            // Write the line
            writeln!(f, "{:06X} | {} | {}", 
                line.offset, 
                bytes_str,
                line.instruction.as_ref().unwrap_or(&"".to_string()))?;
            
            // If there are issues, print them indented
            for issue in &line.issues {
                writeln!(f, "       |                  | --> {}", issue)?;
            }
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_disassemble_simple_program() {
        // Create a simple program: PUSH1 42, PUSH1 21, ADD, HALT
        let bytecode = vec![0x01, 42, 0x01, 21, 0x10, 0xFF];
        
        let mut disasm = Disassembler::new(bytecode);
        disasm.run();
        
        assert_eq!(disasm.disassembled.len(), 4);
        assert_eq!(disasm.disassembled[0].instruction, Some("PUSH1 0x2A".to_string()));
        assert_eq!(disasm.disassembled[1].instruction, Some("PUSH1 0x15".to_string()));
        assert_eq!(disasm.disassembled[2].instruction, Some("ADD".to_string()));
        assert_eq!(disasm.disassembled[3].instruction, Some("HALT".to_string()));
    }
    
    #[test]
    fn test_analyze_compute_units() {
        // Create a simple program: PUSH1 42, PUSH1 21, ADD, HALT
        let bytecode = vec![0x01, 42, 0x01, 21, 0x10, 0xFF];
        
        let mut disasm = Disassembler::new(bytecode);
        let cu = disasm.analyze_compute_units();
        
        // Expected CU: 2 * PUSH1 (3) + ADD (10) + HALT (1) = 17
        assert_eq!(cu, 17);
    }
}
