pub mod test_programs;
pub mod vm;
pub mod solana;

use solana_program::{
    account_info::AccountInfo,
    clock::Clock,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    sysvar::Sysvar,
};

use vm::VM;
use solana::{
    account::AccountManager,
    instruction::{Instruction, TokenInstructionType},
    state::ProgramState,
};

#[derive(Debug, Clone)]
pub struct ProgramAccountInfo {
    is_signer: bool,
    is_writable: bool,
    key: Pubkey,
}

// Program entrypoint
entrypoint!(process_instruction);

pub fn process_instruction<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &'a [u8],
) -> ProgramResult {
    msg!("LessVM: Processing instruction");

    let instruction = Instruction::unpack(instruction_data)?;
    let mut account_manager = AccountManager::new(accounts);

    match instruction {
        Instruction::Initialize => {
            msg!("LessVM: Initializing");
            process_initialize(program_id, &mut account_manager)
        }
        Instruction::Execute { bytecode } => {
            msg!("LessVM: Executing bytecode");
            let bytecode_owned = Box::leak(bytecode.into_boxed_slice());
            process_execute(program_id, accounts, bytecode_owned)
        }
        Instruction::TokenOperation { instruction_type, amount } => {
            msg!("LessVM: Processing token operation");
            process_token_operation(&mut account_manager, instruction_type, amount)
        }
    }
}

fn process_initialize<'a>(
    program_id: &'a Pubkey,
    account_manager: &mut AccountManager<'a>,
) -> ProgramResult {
    let program_account = account_manager.get_account(0)?;

    // Verify account ownership
    account_manager.verify_owner(0, program_id)?;

    // Verify signer
    account_manager.verify_signer(0)?;

    // Verify rent exempt
    account_manager.verify_rent_exempt(0)?;

    // Initialize program state
    let clock = Clock::get()?;
    let mut state = ProgramState::default();
    state.is_initialized = true;
    state.authority = *program_account.key;
    state.last_execution_timestamp = clock.unix_timestamp;

    let mut data = program_account.try_borrow_mut_data()?;
    state.pack_into_slice(&mut data);

    msg!("LessVM: Initialized successfully");
    Ok(())
}

fn process_execute<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    bytecode: &'a [u8],
) -> ProgramResult {
    let mut account_manager = AccountManager::new(accounts);
    let program_account = account_manager.get_account(0)?;

    // Verify account ownership
    account_manager.verify_owner(0, program_id)?;

    // Load and verify program state
    let mut state = ProgramState::unpack(&program_account.try_borrow_data()?)?;
    if !state.is_initialized {
        return Err(ProgramError::UninitializedAccount);
    }

    // Create and execute VM
    let mut vm = VM::new(program_id, accounts, bytecode);
    let _result = vm.execute(bytecode)?;

    // Update program state
    state.total_executions += 1;
    state.total_gas_used += vm.gas_used();
    state.last_execution_timestamp = Clock::get()?.unix_timestamp;

    let mut data = program_account.try_borrow_mut_data()?;
    state.pack_into_slice(&mut data);

    Ok(())
}

fn process_token_operation<'a>(
    account_manager: &mut AccountManager<'a>,
    instruction_type: TokenInstructionType,
    amount: u64,
) -> ProgramResult {
    // Verify signer
    account_manager.verify_signer(2)?;

    match instruction_type {
        TokenInstructionType::Transfer => {
            account_manager.transfer_lamports(0, 1, amount)?;
        }
        TokenInstructionType::Mint => {
            return Err(ProgramError::InvalidInstructionData);
        }
        TokenInstructionType::Burn => {
            return Err(ProgramError::InvalidInstructionData);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_program::clock::Epoch;

    fn create_test_account(
        key: Pubkey,
        lamports: u64,
        space: usize,
        owner: Pubkey,
        is_signer: bool,
        is_writable: bool,
    ) -> AccountInfo<'static> {
        let mut lamports = lamports;
        let mut data = vec![0; space];
        
        AccountInfo::new(
            &key,
            is_signer,
            is_writable,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        )
    }

    #[test]
    fn test_initialize() {
        let program_id = Pubkey::new_unique();
        let account = create_test_account(
            Pubkey::new_unique(),
            1000000,
            ProgramState::LEN,
            program_id,
            true,
            true,
        );

        let accounts = vec![account];
        let instruction = Instruction::Initialize;
        let instruction_data = instruction.try_to_vec().unwrap();

        assert!(process_instruction(&program_id, &accounts, &instruction_data).is_ok());

        let state = ProgramState::unpack(&accounts[0].try_borrow_data().unwrap()).unwrap();
        assert!(state.is_initialized);
        assert_eq!(state.authority, *accounts[0].key);
    }

    #[test]
    fn test_execute() {
        let program_id = Pubkey::new_unique();
        let account = create_test_account(
            Pubkey::new_unique(),
            1000000,
            ProgramState::LEN,
            program_id,
            true,
            true,
        );

        // Initialize first
        let accounts = vec![account];
        let init_instruction = Instruction::Initialize;
        let init_data = init_instruction.try_to_vec().unwrap();
        assert!(process_instruction(&program_id, &accounts, &init_data).is_ok());

        // Then execute bytecode
        let bytecode = vec![0x01, 0x05, 0x01, 0x03, 0x10, 0xFF]; // push1 5, push1 3, add, halt
        let exec_instruction = Instruction::Execute { bytecode };
        let exec_data = exec_instruction.try_to_vec().unwrap();
        assert!(process_instruction(&program_id, &accounts, &exec_data).is_ok());

        let state = ProgramState::unpack(&accounts[0].try_borrow_data().unwrap()).unwrap();
        assert_eq!(state.total_executions, 1);
        assert!(state.total_gas_used > 0);
    }
}