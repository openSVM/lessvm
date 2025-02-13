use crate::vm::OpCode;

/// Test program that adds two numbers and logs the result
pub fn addition_test() -> Vec<u8> {
    vec![
        OpCode::Push1 as u8, 5,  // Push 5 onto stack
        OpCode::Push1 as u8, 3,  // Push 3 onto stack
        OpCode::Add as u8,       // Add them (5 + 3 = 8)
        OpCode::Log as u8,       // Log the result
        OpCode::Halt as u8,      // Halt execution
    ]
}

/// Test program that performs a token transfer
pub fn token_transfer_test() -> Vec<u8> {
    vec![
        OpCode::Push8 as u8, 100, 0, 0, 0, 0, 0, 0, 0,  // Amount: 100 tokens
        OpCode::Push1 as u8, 1,                          // Destination account index
        OpCode::Transfer as u8,                          // Perform transfer
        OpCode::Log as u8,                               // Log completion
        OpCode::Halt as u8,                              // Halt execution
    ]
}

/// Test program that demonstrates memory operations
pub fn memory_test() -> Vec<u8> {
    vec![
        // Store value 42 at memory position 0
        OpCode::Push1 as u8, 42,
        OpCode::Push1 as u8, 0,
        OpCode::Store as u8,
        
        // Load value from memory position 0
        OpCode::Push1 as u8, 0,
        OpCode::Load as u8,
        
        // Log the loaded value
        OpCode::Log as u8,
        OpCode::Halt as u8,
    ]
}

/// Test program that demonstrates stack operations
pub fn stack_test() -> Vec<u8> {
    vec![
        OpCode::Push1 as u8, 1,    // Push 1
        OpCode::Push1 as u8, 2,    // Push 2
        OpCode::Push1 as u8, 3,    // Push 3
        OpCode::Dup as u8, 0,      // Duplicate top value (3)
        OpCode::Swap as u8, 1,     // Swap top two values
        OpCode::Add as u8,         // Add them
        OpCode::Log as u8,         // Log result
        OpCode::Halt as u8,        // Halt execution
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::VM;
    use solana_program::{
        account_info::AccountInfo,
        pubkey::Pubkey,
        clock::Epoch,
    };

    fn setup_test_accounts() -> (Pubkey, Vec<AccountInfo>) {
        let program_id = Pubkey::new_unique();
        let mut lamports1 = 1000000;
        let mut lamports2 = 1000000;
        let mut data1 = vec![0; 32];
        let mut data2 = vec![0; 32];

        let account1 = AccountInfo::new(
            &Pubkey::new_unique(),
            true,
            true,
            &mut lamports1,
            &mut data1,
            &program_id,
            false,
            Epoch::default(),
        );

        let account2 = AccountInfo::new(
            &Pubkey::new_unique(),
            true,
            true,
            &mut lamports2,
            &mut data2,
            &program_id,
            false,
            Epoch::default(),
        );

        (program_id, vec![account1, account2])
    }

    #[test]
    fn test_addition_program() {
        let (program_id, accounts) = setup_test_accounts();
        let program = addition_test();
        let mut vm = VM::new(&program_id, &accounts, &program);
        assert!(vm.execute(&program).is_ok());
    }

    #[test]
    fn test_token_transfer_program() {
        let (program_id, accounts) = setup_test_accounts();
        let program = token_transfer_test();
        let mut vm = VM::new(&program_id, &accounts, &program);
        assert!(vm.execute(&program).is_ok());
    }

    #[test]
    fn test_memory_operations() {
        let (program_id, accounts) = setup_test_accounts();
        let program = memory_test();
        let mut vm = VM::new(&program_id, &accounts, &program);
        assert!(vm.execute(&program).is_ok());
    }

    #[test]
    fn test_stack_operations() {
        let (program_id, accounts) = setup_test_accounts();
        let program = stack_test();
        let mut vm = VM::new(&program_id, &accounts, &program);
        assert!(vm.execute(&program).is_ok());
    }
}