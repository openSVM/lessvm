use solana_program::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::Pubkey,
    msg,
};
use super::{
    OpCode, Stack, Memory, Gas, Value, VMError,
    data_structures::{
        BTreeMapDS,
        TrieDS,
        GraphDS,
        OHLCVDS,
        HypergraphDS,
        DataStructureType,
        OHLCVEntry
    },
    debug::{Tracer, DefaultTracer, ExecutionTrace},
};

/// Current version of the VM runtime
pub const VERSION: u64 = 2;

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

pub struct AccountsView<'a> {
    accounts: &'a [AccountInfo<'a>],
    current: usize,
}

pub struct ReentrancyGuard {
    entered: bool,
}

impl ReentrancyGuard {
    fn new() -> Self {
        Self { entered: false }
    }

    fn enter(&mut self) -> Result<(), VMError> {
        if self.entered {
            return Err(VMError::ReentrancyDetected);
        }
        self.entered = true;
        Ok(())
    }

    fn exit(&mut self) {
        self.entered = false;
    }
}

/// Maximum number of data structures that can be created
const MAX_DATA_STRUCTURES: usize = 16;

#[repr(C, align(64))]
struct DataStructureStore {
    btrees: Vec<Option<BTreeMapDS>>,
    tries: Vec<Option<TrieDS>>,
    graphs: Vec<Option<GraphDS>>,
    ohlcvs: Vec<Option<OHLCVDS>>,
    hypergraphs: Vec<Option<HypergraphDS>>,
}

impl DataStructureStore {
    fn new() -> Self {
        Self {
            btrees: Vec::with_capacity(MAX_DATA_STRUCTURES),
            tries: Vec::with_capacity(MAX_DATA_STRUCTURES),
            graphs: Vec::with_capacity(MAX_DATA_STRUCTURES),
            ohlcvs: Vec::with_capacity(MAX_DATA_STRUCTURES),
            hypergraphs: Vec::with_capacity(MAX_DATA_STRUCTURES),
        }
    }

    fn ensure_capacity(&mut self, ds_type: DataStructureType, id: usize) {
        match ds_type {
            DataStructureType::BTreeMap => {
                if id >= self.btrees.len() {
                    self.btrees.resize_with(id + 1, || None);
                }
            },
            DataStructureType::Trie => {
                if id >= self.tries.len() {
                    self.tries.resize_with(id + 1, || None);
                }
            },
            DataStructureType::Graph => {
                if id >= self.graphs.len() {
                    self.graphs.resize_with(id + 1, || None);
                }
            },
            DataStructureType::OHLCV => {
                if id >= self.ohlcvs.len() {
                    self.ohlcvs.resize_with(id + 1, || None);
                }
            },
            DataStructureType::Hypergraph => {
                if id >= self.hypergraphs.len() {
                    self.hypergraphs.resize_with(id + 1, || None);
                }
            },
        }
    }
}

#[repr(C, align(64))]
pub struct VM<'a> {
    pc: usize,
    gas: Gas,
    stack: Stack,
    memory: Memory,
    accounts: AccountsView<'a>,
    program_id: &'a Pubkey,
    data_structures: DataStructureStore,
    reentrancy_guard: ReentrancyGuard,
    tracer: Box<dyn Tracer>,
}

impl<'a> VM<'a> {
    /// Returns the current VM version
    pub fn get_version() -> u64 {
        VERSION
    }

    pub fn new(
        program_id: &'a Pubkey,
        accounts: &'a [AccountInfo<'a>],
        _instruction_data: &'a [u8],
    ) -> Self {
        Self {
            pc: 0,
            gas: Gas::new(200_000),
            stack: Stack::new(),
            memory: Memory::new(),
            accounts: AccountsView { accounts, current: 0 },
            program_id,
            data_structures: DataStructureStore::new(),
            reentrancy_guard: ReentrancyGuard::new(),
            tracer: Box::new(DefaultTracer),
        }
    }

    #[inline(always)]
    fn enter_function(&mut self) -> Result<(), VMError> {
        self.stack.push_frame()
    }

    #[inline(always)]
    fn exit_function(&mut self) -> Result<(), VMError> {
        self.stack.pop_frame()
    }

    pub fn gas_used(&self) -> u64 {
        let initial_gas = 200_000; // Same as in new()
        initial_gas - self.gas.remaining()
    }

    #[inline(always)]
    fn fetch_opcode(&mut self, code: &[u8]) -> Result<OpCode, VMError> {
        if self.pc >= code.len() {
            return Err(VMError::InvalidInstruction);
        }
        OpCode::from_byte(code[self.pc])
            .ok_or(VMError::InvalidInstruction)
    }

    #[inline(always)]
    fn fetch_u64(&mut self, code: &[u8]) -> Result<u64, VMError> {
        if self.pc + 8 > code.len() {
            return Err(VMError::InvalidInstruction);
        }
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&code[self.pc..self.pc + 8]);
        self.pc += 8;
        Ok(u64::from_le_bytes(bytes))
    }

    #[cfg(target_arch = "x86_64")]
    #[inline(always)]
    unsafe fn vector_add(&mut self) -> Result<(), VMError> {
        if self.stack.depth() < 8 {
            return Err(VMError::StackUnderflow);
        }

        // Get the second vector from the stack (4 values)
        let values2 = _mm256_loadu_si256(self.stack.as_simd_ptr()? as *const __m256i);
        
        // Pop the first 4 values to get to the first vector
        self.stack.pop()?;
        self.stack.pop()?;
        self.stack.pop()?;
        self.stack.pop()?;
        
        // Get the first vector from the stack (4 values)
        let values1 = _mm256_loadu_si256(self.stack.as_simd_ptr()? as *const __m256i);
        
        // Add the two vectors
        let result = _mm256_add_epi64(values1, values2);
        
        // Store the result back to the stack
        _mm256_storeu_si256(self.stack.as_simd_mut_ptr()? as *mut __m256i, result);
        Ok(())
    }

    pub fn execute(&mut self, code: &[u8]) -> Result<Option<Value>, ProgramError> {
        self.reentrancy_guard.enter()?;

        while self.pc < code.len() {
            let opcode = self.fetch_opcode(code)?;
            let gas_cost = opcode.gas_cost();
            self.gas.consume(gas_cost)?;

            self.tracer.trace_execution(ExecutionTrace {
                pc: self.pc,
                opcode,
                gas_used: gas_cost,
                gas_remaining: self.gas.remaining(), 
                stack_depth: self.stack.depth(),
                memory_size: self.memory.size(),
            });

            self.pc += 1;

            match opcode {
                // Stack Operations
                OpCode::Nop => {},
                OpCode::Push1 => {
                    if self.pc >= code.len() {
                        return Err(VMError::InvalidInstruction.into());
                    }
                    let value = code[self.pc] as u64;
                    self.stack.push(Value(value))?;
                    self.pc += 1;
                },
                OpCode::Push8 => {
                    let value = self.fetch_u64(code)?;
                    self.stack.push(Value(value))?;
                },
                OpCode::Pop => {
                    self.stack.pop()?;
                },
                OpCode::Dup => {
                    let n = code[self.pc] as usize;
                    self.stack.dup(n)?;
                    self.pc += 1;
                },
                OpCode::Swap => {
                    let n = code[self.pc] as usize;
                    self.stack.swap(n)?;
                    self.pc += 1;
                },

                // Math Operations
                OpCode::Add => {
                    let b = self.stack.pop()?;
                    let a = self.stack.pop()?;
                    let result = a.checked_add(&b).ok_or(VMError::ArithmeticOverflow)?;
                    self.stack.push(result)?;
                },
                OpCode::Sub => {
                    let b = self.stack.pop()?;
                    let a = self.stack.pop()?;
                    let result = a.checked_sub(&b).ok_or(VMError::ArithmeticOverflow)?;
                    self.stack.push(result)?;
                },
                OpCode::Mul => {
                    let b = self.stack.pop()?;
                    let a = self.stack.pop()?;
                    let result = a.checked_mul(&b).ok_or(VMError::ArithmeticOverflow)?;
                    self.stack.push(result)?;
                },
                OpCode::Div => {
                    let b = self.stack.pop()?;
                    let a = self.stack.pop()?;
                    let result = a.checked_div(&b).ok_or(VMError::ArithmeticOverflow)?;
                    self.stack.push(result)?;
                },
                OpCode::MulDiv => {
                    let c = self.stack.pop()?;
                    let b = self.stack.pop()?;
                    let a = self.stack.pop()?;
                    let mul = (a.0 as u128) * (b.0 as u128);
                    let div = mul / (c.0 as u128);
                    if div > u64::MAX as u128 {
                        return Err(VMError::ArithmeticOverflow.into());
                    }
                    self.stack.push(Value(div as u64))?;
                },
                OpCode::Mod => {
                    let b = self.stack.pop()?;
                    let a = self.stack.pop()?;
                    if b.0 == 0 {
                        return Err(VMError::DivisionByZero.into());
                    }
                    let result = Value(a.0 % b.0);
                    self.stack.push(result)?;
                },
                OpCode::Exp => {
                    let exponent = self.stack.pop()?.0;
                    let base = self.stack.pop()?.0;
                    
                    // Handle potential overflow
                    if exponent > 64 && base > 1 {
                        return Err(VMError::ArithmeticOverflow.into());
                    }
                    
                    let mut result = 1u64;
                    let mut base_power = base;
                    let mut exp = exponent;
                    
                    // Fast exponentiation algorithm
                    while exp > 0 {
                        if exp & 1 == 1 {
                            result = result.checked_mul(base_power)
                                .ok_or(VMError::ArithmeticOverflow)?;
                        }
                        base_power = base_power.checked_mul(base_power)
                            .ok_or(VMError::ArithmeticOverflow)?;
                        exp >>= 1;
                    }
                    
                    self.stack.push(Value(result))?;
                },
                OpCode::SignExtend => {
                    let byte_num = self.stack.pop()?.0 as usize;
                    let value = self.stack.pop()?.0;
                    
                    if byte_num >= 8 {
                        // No change if byte_num is out of range
                        self.stack.push(Value(value))?;
                        return Ok(());
                    }
                    
                    let bit_position = (byte_num + 1) * 8 - 1;
                    let mask = 1u64 << bit_position;
                    let sign_bit = value & mask;
                    
                    let result = if sign_bit != 0 {
                        // If sign bit is 1, set all higher bits to 1
                        let higher_bits_mask = !((1u64 << (bit_position + 1)) - 1);
                        value | higher_bits_mask
                    } else {
                        // If sign bit is 0, clear all higher bits
                        let lower_bits_mask = (1u64 << (bit_position + 1)) - 1;
                        value & lower_bits_mask
                    };
                    
                    self.stack.push(Value(result))?;
                },

                // Memory Operations
                OpCode::Load => {
                    let offset = self.stack.pop()?.0 as usize;
                    let data = self.memory.load(offset, 8)?;
                    let mut bytes = [0u8; 8];
                    bytes.copy_from_slice(data);
                    self.stack.push(Value(u64::from_le_bytes(bytes)))?;
                },
                OpCode::Store => {
                    let offset = self.stack.pop()?.0 as usize;
                    let value = self.stack.pop()?.0;
                    self.memory.store(offset, &value.to_le_bytes())?;
                },
                OpCode::LoadN => {
                    let len = self.stack.pop()?.0 as usize;
                    let offset = self.stack.pop()?.0 as usize;
                    let _data = self.memory.load(offset, len)?;
                    // Process loaded data as needed
                },
                OpCode::StoreN => {
                    let len = self.stack.pop()?.0 as usize;
                    let offset = self.stack.pop()?.0 as usize;
                    let value = self.stack.pop()?.0;
                    let bytes = value.to_le_bytes();
                    self.memory.store(offset, &bytes[..len.min(8)])?;
                },
                OpCode::Msize => {
                    self.stack.push(Value(self.memory.size() as u64))?;
                },
                OpCode::Mload8 => {
                    let offset = self.stack.pop()?.0 as usize;
                    let value = self.memory.load8(offset)? as u64;
                    self.stack.push(Value(value))?;
                },
                OpCode::Mstore8 => {
                    let offset = self.stack.pop()?.0 as usize;
                    let value = self.stack.pop()?.0 as u8;
                    self.memory.store8(offset, value)?;
                },

                // Control Flow
                OpCode::Jump => {
                    let target = self.stack.pop()?.0 as usize;
                    if target >= code.len() {
                        return Err(VMError::InvalidInstruction.into());
                    }
                    self.pc = target;
                },
                OpCode::JumpI => {
                    let target = self.stack.pop()?.0 as usize;
                    let condition = self.stack.pop()?.0;
                    if condition != 0 {
                        if target >= code.len() {
                            return Err(VMError::InvalidInstruction.into());
                        }
                        self.pc = target;
                    }
                },
                OpCode::Call => {
                    // Implement call logic
                    let target = self.stack.pop()?.0 as usize;
                    if target >= code.len() {
                        return Err(VMError::InvalidInstruction.into());
                    }
                    self.enter_function()?;
                    self.pc = target;
                },
                OpCode::Return => {
                    let return_value = self.stack.pop()?;
                    self.exit_function()?;
                    return Ok(Some(return_value));
                },

                // Solana Operations
                OpCode::Transfer => {
                    let amount = self.stack.pop()?.0;
                    let dest_idx = self.stack.pop()?.0 as usize;
                    let src_idx = self.stack.pop()?.0 as usize;

                    if src_idx >= self.accounts.accounts.len() || dest_idx >= self.accounts.accounts.len() {
                        return Err(VMError::InvalidAccount.into());
                    }

                    let source = &self.accounts.accounts[src_idx];
                    let destination = &self.accounts.accounts[dest_idx];

                    **source.try_borrow_mut_lamports()? = source
                        .lamports()
                        .checked_sub(amount)
                        .ok_or(VMError::ArithmeticOverflow)?;

                    **destination.try_borrow_mut_lamports()? = destination
                        .lamports()
                        .checked_add(amount)
                        .ok_or(VMError::ArithmeticOverflow)?;
                },
                OpCode::SPLTransfer => {
                    // Implement SPL token transfer
                    return Err(ProgramError::InvalidInstructionData);
                },
                OpCode::CPI => {
                    // Implement CPI
                    return Err(ProgramError::InvalidInstructionData);
                },
                OpCode::Log => {
                    let value = self.stack.pop()?;
                    msg!("VM Log: {}", value.0);
                },
                
                // Data Structure Operations - BTreeMap
                OpCode::BTreeCreate => {
                    let id = self.stack.pop()?.0 as usize;
                    if id >= self.data_structures.btrees.len() {
                        return Err(VMError::InvalidDataStructureOperation.into());
                    }
                    self.data_structures.btrees[id] = Some(BTreeMapDS::new());
                },
                OpCode::BTreeInsert => {
                    let value = self.stack.pop()?.0;
                    let key = self.stack.pop()?.0;
                    let id = self.stack.pop()?.0 as usize;
                    
                    if id >= self.data_structures.btrees.len() {
                        return Err(VMError::InvalidDataStructureOperation.into());
                    }
                    
                    if let Some(btree) = &mut self.data_structures.btrees[id] {
                        let old_value = btree.insert(key, value);
                        self.stack.push(Value(old_value.unwrap_or(0)))?;
                    } else {
                        return Err(VMError::InvalidDataStructureOperation.into());
                    }
                },
                OpCode::BTreeGet => {
                    let key = self.stack.pop()?.0;
                    let id = self.stack.pop()?.0 as usize;
                    
                    if id >= self.data_structures.btrees.len() {
                        return Err(VMError::InvalidDataStructureOperation.into());
                    }
                    
                    if let Some(btree) = &self.data_structures.btrees[id] {
                        match btree.get(key) {
                            Some(value) => self.stack.push(Value(value))?,
                            None => self.stack.push(Value(0))?, // Return 0 if key not found
                        }
                    } else {
                        return Err(VMError::InvalidDataStructureOperation.into());
                    }
                },
                OpCode::BTreeRemove => {
                    let key = self.stack.pop()?.0;
                    let id = self.stack.pop()?.0 as usize;
                    
                    if id >= self.data_structures.btrees.len() {
                        return Err(VMError::InvalidDataStructureOperation.into());
                    }
                    
                    if let Some(btree) = &mut self.data_structures.btrees[id] {
                        let old_value = btree.remove(key);
                        self.stack.push(Value(old_value.unwrap_or(0)))?;
                    } else {
                        return Err(VMError::InvalidDataStructureOperation.into());
                    }
                },
                OpCode::BTreeContains => {
                    let key = self.stack.pop()?.0;
                    let id = self.stack.pop()?.0 as usize;
                    
                    if id >= self.data_structures.btrees.len() {
                        return Err(VMError::InvalidDataStructureOperation.into());
                    }
                    
                    if let Some(btree) = &self.data_structures.btrees[id] {
                        self.stack.push(Value(if btree.contains_key(key) { 1 } else { 0 }))?;
                    } else {
                        return Err(VMError::InvalidDataStructureOperation.into());
                    }
                },
                OpCode::BTreeLen => {
                    let id = self.stack.pop()?.0 as usize;
                    
                    if id >= self.data_structures.btrees.len() {
                        return Err(VMError::InvalidDataStructureOperation.into());
                    }
                    
                    if let Some(btree) = &self.data_structures.btrees[id] {
                        self.stack.push(Value(btree.len() as u64))?;
                    } else {
                        return Err(VMError::InvalidDataStructureOperation.into());
                    }
                },
                OpCode::BTreeFirstKey => {
                    let id = self.stack.pop()?.0 as usize;
                    
                    if id >= self.data_structures.btrees.len() {
                        return Err(VMError::InvalidDataStructureOperation.into());
                    }
                    
                    if let Some(btree) = &self.data_structures.btrees[id] {
                        match btree.first_key() {
                            Some(key) => self.stack.push(Value(key))?,
                            None => self.stack.push(Value(0))?, // Return 0 if empty
                        }
                    } else {
                        return Err(VMError::InvalidDataStructureOperation.into());
                    }
                },
                OpCode::BTreeLastKey => {
                    let id = self.stack.pop()?.0 as usize;
                    
                    if id >= self.data_structures.btrees.len() {
                        return Err(VMError::InvalidDataStructureOperation.into());
                    }
                    
                    if let Some(btree) = &self.data_structures.btrees[id] {
                        match btree.last_key() {
                            Some(key) => self.stack.push(Value(key))?,
                            None => self.stack.push(Value(0))?, // Return 0 if empty
                        }
                    } else {
                        return Err(VMError::InvalidDataStructureOperation.into());
                    }
                },
                OpCode::BTreeClear => {
                    let id = self.stack.pop()?.0 as usize;
                    
                    self.data_structures.ensure_capacity(DataStructureType::BTreeMap, id);
                    
                    if let Some(btree) = &mut self.data_structures.btrees[id] {
                        btree.clear();
                    } else {
                        return Err(VMError::InvalidDataStructureOperation.into());
                    }
                },
                
                // Trie operations
                OpCode::TrieCreate => {
                    let id = self.stack.pop()?.0 as usize;
                    self.data_structures.ensure_capacity(DataStructureType::Trie, id);
                    self.data_structures.tries[id] = Some(TrieDS::new());
                },
                OpCode::TrieInsert => {
                    // Stack: [id, key_ptr, key_len, value]
                    let value = self.stack.pop()?;
                    let key_len = self.stack.pop()?.0 as usize;
                    let key_ptr = self.stack.pop()?.0 as usize;
                    let id = self.stack.pop()?.0 as usize;
                    
                    self.data_structures.ensure_capacity(DataStructureType::Trie, id);
                    
                    // Read key from memory
                    let key = self.memory.load(key_ptr, key_len)?;
                    
                    if let Some(trie) = &mut self.data_structures.tries[id] {
                        trie.insert(key, value)?;
                    } else {
                        return Err(VMError::InvalidDataStructureOperation.into());
                    }
                },
                OpCode::TrieGet => {
                    // Stack: [id, key_ptr, key_len]
                    let key_len = self.stack.pop()?.0 as usize;
                    let key_ptr = self.stack.pop()?.0 as usize;
                    let id = self.stack.pop()?.0 as usize;
                    
                    // Validate key length
                    if key_len == 0 {
                        return Err(VMError::InvalidDataStructureOperation.into());
                    }
                    
                    self.data_structures.ensure_capacity(DataStructureType::Trie, id);
                    
                    // Read key from memory with bounds check
                    let key = match self.memory.load(key_ptr, key_len) {
                        Ok(k) => k,
                        Err(_) => return Err(VMError::InvalidDataStructureOperation.into()),
                    };
                    
                    if let Some(trie) = &self.data_structures.tries[id] {
                        match trie.get(key) {
                            Some(value) => self.stack.push(value)?, // value is already a Value type
                            None => self.stack.push(Value(0))?, // Return 0 if key not found
                        }
                    } else {
                        return Err(VMError::InvalidDataStructureOperation.into());
                    }
                },
                OpCode::TrieContains => {
                    // Stack: [id, key_ptr, key_len]
                    let key_len = self.stack.pop()?.0 as usize;
                    let key_ptr = self.stack.pop()?.0 as usize;
                    let id = self.stack.pop()?.0 as usize;
                    
                    // Validate key length
                    if key_len == 0 {
                        return Err(VMError::InvalidDataStructureOperation.into());
                    }
                    
                    self.data_structures.ensure_capacity(DataStructureType::Trie, id);
                    
                    // Read key from memory with bounds check
                    let key = match self.memory.load(key_ptr, key_len) {
                        Ok(k) => k,
                        Err(_) => return Err(VMError::InvalidDataStructureOperation.into()),
                    };
                    
                    if let Some(trie) = &self.data_structures.tries[id] {
                        // Consistently return Value(1) for true and Value(0) for false
                        self.stack.push(Value(if trie.contains(key) { 1 } else { 0 }))?;
                    } else {
                        return Err(VMError::InvalidDataStructureOperation.into());
                    }
                },
                OpCode::TrieClear => {
                    let id = self.stack.pop()?.0 as usize;
                    
                    self.data_structures.ensure_capacity(DataStructureType::Trie, id);
                    
                    if let Some(trie) = &mut self.data_structures.tries[id] {
                        trie.clear();
                    } else {
                        return Err(VMError::InvalidDataStructureOperation.into());
                    }
                },
                
                // Graph operations - implementing basic ones
                OpCode::GraphCreate => {
                    let id = self.stack.pop()?.0 as usize;
                    if id >= self.data_structures.graphs.len() {
                        return Err(VMError::InvalidDataStructureOperation.into());
                    }
                    self.data_structures.graphs[id] = Some(GraphDS::new());
                },
                OpCode::GraphAddNode => {
                    let value = self.stack.pop()?.0;
                    let node_id = self.stack.pop()?.0;
                    let id = self.stack.pop()?.0 as usize;
                    
                    if id >= self.data_structures.graphs.len() {
                        return Err(VMError::InvalidDataStructureOperation.into());
                    }
                    
                    if let Some(graph) = &mut self.data_structures.graphs[id] {
                        graph.add_node(node_id, value)?;
                    } else {
                        return Err(VMError::InvalidDataStructureOperation.into());
                    }
                },
                
                // The remaining graph, OHLCV, and hypergraph operations can be 
                // implemented similarly as needed

                // OHLCV operations 
                OpCode::OhlcvCreate => {
                    let id = self.stack.pop()?.0 as usize;
                    self.data_structures.ensure_capacity(DataStructureType::OHLCV, id);
                    self.data_structures.ohlcvs[id] = Some(OHLCVDS::new());
                },

                // Hypergraph operations
                OpCode::HyperCreate => {
                    let id = self.stack.pop()?.0 as usize;
                    self.data_structures.ensure_capacity(DataStructureType::Hypergraph, id);
                    self.data_structures.hypergraphs[id] = Some(HypergraphDS::new());
                },

                // Bitwise Operations
                OpCode::And => {
                    let b = self.stack.pop()?.0;
                    let a = self.stack.pop()?.0;
                    self.stack.push(Value(a & b))?;
                },
                OpCode::Or => {
                    let b = self.stack.pop()?.0;
                    let a = self.stack.pop()?.0;
                    self.stack.push(Value(a | b))?;
                },
                OpCode::Xor => {
                    let b = self.stack.pop()?.0;
                    let a = self.stack.pop()?.0;
                    self.stack.push(Value(a ^ b))?;
                },
                OpCode::Not => {
                    let a = self.stack.pop()?.0;
                    self.stack.push(Value(!a))?;
                },
                OpCode::Byte => {
                    let byte_index = self.stack.pop()?.0 as usize;
                    let value = self.stack.pop()?.0;
                    
                    let result = if byte_index >= 32 {
                        0
                    } else {
                        // Extract the specified byte
                        let shift = (31 - byte_index) * 8;
                        (value >> shift) & 0xFF
                    };
                    
                    self.stack.push(Value(result))?;
                },
                OpCode::Shl => {
                    let shift = self.stack.pop()?.0;
                    let value = self.stack.pop()?.0;
                    
                    // Avoid undefined behavior with large shifts
                    let result = if shift >= 64 {
                        0
                    } else {
                        value << shift
                    };
                    
                    self.stack.push(Value(result))?;
                },
                OpCode::Shr => {
                    let shift = self.stack.pop()?.0;
                    let value = self.stack.pop()?.0;
                    
                    // Avoid undefined behavior with large shifts
                    let result = if shift >= 64 {
                        0
                    } else {
                        value >> shift
                    };
                    
                    self.stack.push(Value(result))?;
                },
                OpCode::Sar => {
                    let shift = self.stack.pop()?.0;
                    let value = self.stack.pop()?.0;
                    
                    // Arithmetic right shift (sign-extending)
                    let result = if shift >= 64 {
                        if (value as i64) < 0 {
                            u64::MAX  // All 1s for negative numbers
                        } else {
                            0  // All 0s for positive numbers
                        }
                    } else {
                        // Rust doesn't have arithmetic right shift for unsigned types
                        // Convert to signed, shift, then back to unsigned
                        ((value as i64) >> shift) as u64
                    };
                    
                    self.stack.push(Value(result))?;
                },
                
                OpCode::Halt => {
                    break;
                },
                // Solana Operations that were missing
                OpCode::GetBalance => {
                    let account_idx = self.stack.pop()?.0 as usize;
                    if account_idx >= self.accounts.accounts.len() {
                        return Err(VMError::InvalidAccount.into());
                    }
                    let account = &self.accounts.accounts[account_idx];
                    self.stack.push(Value(account.lamports()))?;
                },
                OpCode::GetOwner => {
                    let account_idx = self.stack.pop()?.0 as usize;
                    if account_idx >= self.accounts.accounts.len() {
                        return Err(VMError::InvalidAccount.into());
                    }
                    let account = &self.accounts.accounts[account_idx];
                    // Convert Pubkey to u64 for stack storage (using first 8 bytes)
                    let owner_bytes = account.owner.to_bytes();
                    let mut value_bytes = [0u8; 8];
                    value_bytes.copy_from_slice(&owner_bytes[0..8]);
                    self.stack.push(Value(u64::from_le_bytes(value_bytes)))?;
                },
                OpCode::IsWritable => {
                    let account_idx = self.stack.pop()?.0 as usize;
                    if account_idx >= self.accounts.accounts.len() {
                        return Err(VMError::InvalidAccount.into());
                    }
                    let account = &self.accounts.accounts[account_idx];
                    self.stack.push(Value(if account.is_writable() { 1 } else { 0 }))?;
                },
                OpCode::IsSigner => {
                    let account_idx = self.stack.pop()?.0 as usize;
                    if account_idx >= self.accounts.accounts.len() {
                        return Err(VMError::InvalidAccount.into());
                    }
                    let account = &self.accounts.accounts[account_idx];
                    self.stack.push(Value(if account.is_signer { 1 } else { 0 }))?;
                },
                // Control Flow
                OpCode::Revert => {
                    let error_code = self.stack.pop()?.0;
                    return Err(ProgramError::Custom(error_code as u32));
                },
                // Graph operations
                OpCode::GraphAddEdge => {
                    let weight = self.stack.pop()?.0;
                    let to = self.stack.pop()?.0;
                    let from = self.stack.pop()?.0;
                    let id = self.stack.pop()?.0 as usize;
                    
                    if id >= self.data_structures.graphs.len() {
                        return Err(VMError::InvalidDataStructureOperation.into());
                    }
                    
                    if let Some(graph) = &mut self.data_structures.graphs[id] {
                        graph.add_edge(from, to, weight)?;
                    } else {
                        return Err(VMError::InvalidDataStructureOperation.into());
                    }
                },
                OpCode::GraphGetNode => {
                    let node_id = self.stack.pop()?.0;
                    let id = self.stack.pop()?.0 as usize;
                    
                    if id >= self.data_structures.graphs.len() {
                        return Err(VMError::InvalidDataStructureOperation.into());
                    }
                    
                    if let Some(graph) = &self.data_structures.graphs[id] {
                        match graph.get_node_value(node_id) {
                            Some(value) => self.stack.push(Value(value))?,
                            None => self.stack.push(Value(0))?, // Return 0 if node not found
                        }
                    } else {
                        return Err(VMError::InvalidDataStructureOperation.into());
                    }
                },
                OpCode::GraphSetNode => {
                    let value = self.stack.pop()?.0;
                    let node_id = self.stack.pop()?.0;
                    let id = self.stack.pop()?.0 as usize;
                    
                    if id >= self.data_structures.graphs.len() {
                        return Err(VMError::InvalidDataStructureOperation.into());
                    }
                    
                    if let Some(graph) = &mut self.data_structures.graphs[id] {
                        graph.set_node_value(node_id, value)?;
                    } else {
                        return Err(VMError::InvalidDataStructureOperation.into());
                    }
                },
                // Graph operations
                OpCode::GraphGetNeighbors => {
                    let node_id = self.stack.pop()?.0;
                    let id = self.stack.pop()?.0 as usize;
                    
                    if id >= self.data_structures.graphs.len() {
                        return Err(VMError::InvalidDataStructureOperation.into());
                    }
                    
                    if let Some(graph) = &self.data_structures.graphs[id] {
                        let neighbors = graph.get_neighbors(node_id);
                        
                        // Store the number of neighbors on the stack
                        self.stack.push(Value(neighbors.len() as u64))?;
                        
                        // Store each neighbor and its weight on the stack
                        for (neighbor, weight) in neighbors.iter().rev() {
                            self.stack.push(Value(*weight))?;
                            self.stack.push(Value(*neighbor))?;
                        }
                    } else {
                        return Err(VMError::InvalidDataStructureOperation.into());
                    }
                },
                OpCode::GraphBfs => {
                    let start_node = self.stack.pop()?.0;
                    let id = self.stack.pop()?.0 as usize;
                    
                    if id >= self.data_structures.graphs.len() {
                        return Err(VMError::InvalidDataStructureOperation.into());
                    }
                    
                    if let Some(graph) = &self.data_structures.graphs[id] {
                        let bfs_result = graph.bfs(start_node);
                        
                        // Store the number of nodes in the BFS result on the stack
                        self.stack.push(Value(bfs_result.len() as u64))?;
                        
                        // Store each node in the BFS result on the stack (in reverse order so they come out in the right order when popped)
                        for node in bfs_result.iter().rev() {
                            self.stack.push(Value(*node))?;
                        }
                    } else {
                        return Err(VMError::InvalidDataStructureOperation.into());
                    }
                },
                OpCode::GraphClear => {
                    let id = self.stack.pop()?.0 as usize;
                    
                    if id >= self.data_structures.graphs.len() {
                        return Err(VMError::InvalidDataStructureOperation.into());
                    }
                    
                    if let Some(graph) = &mut self.data_structures.graphs[id] {
                        graph.clear();
                    } else {
                        return Err(VMError::InvalidDataStructureOperation.into());
                    }
                },
                // OHLCV operations
                OpCode::OhlcvAddBar => {
                    let volume = self.stack.pop()?.0;
                    let close = self.stack.pop()?.0;
                    let low = self.stack.pop()?.0;
                    let high = self.stack.pop()?.0;
                    let open = self.stack.pop()?.0;
                    let timestamp = self.stack.pop()?.0;
                    let id = self.stack.pop()?.0 as usize;
                    
                    if id >= self.data_structures.ohlcvs.len() {
                        return Err(VMError::InvalidDataStructureOperation.into());
                    }
                    
                    let entry = OHLCVEntry {
                        timestamp,
                        open,
                        high,
                        low,
                        close,
                        volume,
                    };
                    
                    if let Some(ohlcv) = &mut self.data_structures.ohlcvs[id] {
                        ohlcv.add_entry(entry)?;
                    } else {
                        return Err(VMError::InvalidDataStructureOperation.into());
                    }
                },
                OpCode::OhlcvGetBar => {
                    let index = self.stack.pop()?.0 as usize;
                    let id = self.stack.pop()?.0 as usize;
                    
                    if id >= self.data_structures.ohlcvs.len() {
                        return Err(VMError::InvalidDataStructureOperation.into());
                    }
                    
                    if let Some(ohlcv) = &self.data_structures.ohlcvs[id] {
                        match ohlcv.get_entry(index) {
                            Some(entry) => {
                                // Push all values in reverse order so they come out in the right order when popped
                                self.stack.push(Value(entry.volume))?;
                                self.stack.push(Value(entry.close))?;
                                self.stack.push(Value(entry.low))?;
                                self.stack.push(Value(entry.high))?;
                                self.stack.push(Value(entry.open))?;
                                self.stack.push(Value(entry.timestamp))?;
                            },
                            None => {
                                // Push zeros if entry not found
                                for _ in 0..6 {
                                    self.stack.push(Value(0))?;
                                }
                            }
                        }
                    } else {
                        return Err(VMError::InvalidDataStructureOperation.into());
                    }
                },
                OpCode::OhlcvSma => {
                    let period = self.stack.pop()?.0 as usize;
                    let id = self.stack.pop()?.0 as usize;
                    
                    if id >= self.data_structures.ohlcvs.len() {
                        return Err(VMError::InvalidDataStructureOperation.into());
                    }
                    
                    if let Some(ohlcv) = &self.data_structures.ohlcvs[id] {
                        let sma_result = ohlcv.calculate_sma(period);
                        
                        // Push the number of SMA values
                        self.stack.push(Value(sma_result.len() as u64))?;
                        
                        // Push each SMA value and timestamp in reverse order
                        for (timestamp, value) in sma_result.iter().rev() {
                            self.stack.push(Value(*value))?;
                            self.stack.push(Value(*timestamp))?;
                        }
                    } else {
                        return Err(VMError::InvalidDataStructureOperation.into());
                    }
                },
                // Hypergraph operations
                OpCode::HyperAddNode => {
                    let value = self.stack.pop()?.0;
                    let node_id = self.stack.pop()?.0;
                    let id = self.stack.pop()?.0 as usize;
                    
                    if id >= self.data_structures.hypergraphs.len() {
                        return Err(VMError::InvalidDataStructureOperation.into());
                    }
                    
                    if let Some(hypergraph) = &mut self.data_structures.hypergraphs[id] {
                        hypergraph.add_node(node_id, value)?;
                    } else {
                        return Err(VMError::InvalidDataStructureOperation.into());
                    }
                },
                OpCode::HyperAddEdge => {
                    let weight = self.stack.pop()?.0;
                    let edge_id = self.stack.pop()?.0;
                    let id = self.stack.pop()?.0 as usize;
                    
                    if id >= self.data_structures.hypergraphs.len() {
                        return Err(VMError::InvalidDataStructureOperation.into());
                    }
                    
                    if let Some(hypergraph) = &mut self.data_structures.hypergraphs[id] {
                        hypergraph.create_hyperedge(edge_id, weight)?;
                    } else {
                        return Err(VMError::InvalidDataStructureOperation.into());
                    }
                },
                OpCode::HyperAddNodeToEdge => {
                    let node_id = self.stack.pop()?.0;
                    let edge_id = self.stack.pop()?.0;
                    let id = self.stack.pop()?.0 as usize;
                    
                    if id >= self.data_structures.hypergraphs.len() {
                        return Err(VMError::InvalidDataStructureOperation.into());
                    }
                    
                    if let Some(hypergraph) = &mut self.data_structures.hypergraphs[id] {
                        hypergraph.add_node_to_edge(edge_id, node_id)?;
                    } else {
                        return Err(VMError::InvalidDataStructureOperation.into());
                    }
                },
                _ => return Err(VMError::InvalidInstruction.into()),
            }
        }

        self.reentrancy_guard.exit();
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_program::clock::Epoch;

    fn create_test_account(lamports: u64) -> (Pubkey, Vec<u8>, AccountInfo<'static>) {
        let key = Pubkey::new_unique();
        let mut lamports = lamports;
        let mut data = vec![0; 32];
        
        AccountInfo::new(
            &key,
            true,
            true,
            &mut lamports,
            &mut data,
            &Pubkey::new_unique(),
            false,
            Epoch::default(),
        )
    }

    #[test]
    fn test_basic_execution() {
        let program_id = Pubkey::new_unique();
        let (_, _, account) = create_test_account(1000000);
        let accounts = vec![account];
        
        // push1 5, push1 3, add, halt
        let bytecode = vec![0x01, 0x05, 0x01, 0x03, 0x10, 0xFF];
        
        let mut vm = VM::new(&program_id, &accounts, &bytecode);
        assert!(vm.execute(&bytecode).is_ok());
        
        let result = vm.stack.pop().unwrap();
        assert_eq!(result.0, 8);
    }

    #[test]
    fn test_transfer() {
        let program_id = Pubkey::new_unique();
        let (_, _, account1) = create_test_account(1000000);
        let (_, _, account2) = create_test_account(0);
        let accounts = vec![account1, account2];
        
        // push1 0, push1 1, push1 500000, transfer, halt
        let bytecode = vec![0x01, 0x00, 0x01, 0x01, 0x01, 0x07, 0xA1, 0x20, 0x40, 0xFF];
        
        let mut vm = VM::new(&program_id, &accounts, &bytecode);
        assert!(vm.execute(&bytecode).is_ok());
        
        assert_eq!(accounts[0].lamports(), 500000);
        assert_eq!(accounts[1].lamports(), 500000);
    }
} 