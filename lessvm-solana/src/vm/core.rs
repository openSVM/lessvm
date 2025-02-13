use solana_program::{
    account_info::AccountInfo,
    program_error::ProgramError, 
    pubkey::Pubkey,
    msg,
};
use super::{
    OpCode, Stack, Memory, Gas, Value, VMError,
    debug::{Tracer, DefaultTracer, ExecutionTrace},
};

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

#[repr(C, align(64))]
pub struct VM<'a> {
    pc: usize,
    gas: Gas,
    stack: Stack,
    memory: Memory,
    accounts: AccountsView<'a>,
    program_id: &'a Pubkey,
    reentrancy_guard: ReentrancyGuard,
    tracer: Box<dyn Tracer>,
}

impl<'a> VM<'a> {
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

        let values = _mm256_loadu_si256(self.stack.data.as_ptr().add(self.stack.depth() - 8) as *const __m256i);
        let result = _mm256_add_epi64(values, values); 
        _mm256_storeu_si256(self.stack.data.as_mut_ptr().add(self.stack.depth() - 8) as *mut __m256i, result);
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

                OpCode::Halt => {
                    break;
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