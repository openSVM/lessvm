use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction as SolanaInstruction},
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum Instruction {
    /// Initialize a new VM instance
    Initialize,
    
    /// Execute bytecode
    Execute {
        /// Program bytecode
        bytecode: Vec<u8>,
    },
    
    /// Execute SPL token operation
    TokenOperation {
        /// Token instruction type
        instruction_type: TokenInstructionType,
        /// Amount of tokens
        amount: u64,
    },
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum TokenInstructionType {
    Transfer,
    Mint,
    Burn,
}

impl Instruction {
    pub fn to_instruction(
        self,
        program_id: &Pubkey,
        accounts: Vec<AccountMeta>,
    ) -> Result<SolanaInstruction, ProgramError> {
        let data = self.try_to_vec()
            .map_err(|_| ProgramError::InvalidInstructionData)?;

        Ok(SolanaInstruction {
            program_id: *program_id,
            accounts,
            data,
        })
    }

    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        Self::try_from_slice(input)
            .map_err(|_| ProgramError::InvalidInstructionData)
    }
}

pub fn create_initialize_instruction(
    program_id: &Pubkey,
    program_account: &Pubkey,
) -> Result<SolanaInstruction, ProgramError> {
    let accounts = vec![
        AccountMeta::new(*program_account, true),
    ];

    Instruction::Initialize.to_instruction(program_id, accounts)
}

pub fn create_execute_instruction(
    program_id: &Pubkey,
    program_account: &Pubkey,
    bytecode: Vec<u8>,
    accounts: Vec<AccountMeta>,
) -> Result<SolanaInstruction, ProgramError> {
    let mut instruction_accounts = vec![
        AccountMeta::new(*program_account, true),
    ];
    instruction_accounts.extend(accounts);

    Instruction::Execute { bytecode }
        .to_instruction(program_id, instruction_accounts)
}

pub fn create_token_operation_instruction(
    program_id: &Pubkey,
    instruction_type: TokenInstructionType,
    amount: u64,
    source: &Pubkey,
    destination: &Pubkey,
    authority: &Pubkey,
    token_program: &Pubkey,
) -> Result<SolanaInstruction, ProgramError> {
    let accounts = vec![
        AccountMeta::new(*source, false),
        AccountMeta::new(*destination, false),
        AccountMeta::new_readonly(*authority, true),
        AccountMeta::new_readonly(*token_program, false),
    ];

    Instruction::TokenOperation {
        instruction_type,
        amount,
    }.to_instruction(program_id, accounts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruction_serialization() {
        let program_id = Pubkey::new_unique();
        let account = Pubkey::new_unique();

        // Test Initialize instruction
        let init_ix = create_initialize_instruction(&program_id, &account).unwrap();
        let unpacked = Instruction::unpack(&init_ix.data).unwrap();
        match unpacked {
            Instruction::Initialize => {},
            _ => panic!("Wrong instruction type"),
        }

        // Test Execute instruction
        let bytecode = vec![0x01, 0x02, 0x03];
        let exec_ix = create_execute_instruction(
            &program_id,
            &account,
            bytecode.clone(),
            vec![],
        ).unwrap();
        let unpacked = Instruction::unpack(&exec_ix.data).unwrap();
        match unpacked {
            Instruction::Execute { bytecode: unpacked_bytecode } => {
                assert_eq!(bytecode, unpacked_bytecode);
            },
            _ => panic!("Wrong instruction type"),
        }

        // Test Token Operation instruction
        let token_ix = create_token_operation_instruction(
            &program_id,
            TokenInstructionType::Transfer,
            1000,
            &Pubkey::new_unique(),
            &Pubkey::new_unique(),
            &Pubkey::new_unique(),
            &Pubkey::new_unique(),
        ).unwrap();
        let unpacked = Instruction::unpack(&token_ix.data).unwrap();
        match unpacked {
            Instruction::TokenOperation { instruction_type, amount } => {
                match instruction_type {
                    TokenInstructionType::Transfer => {},
                    _ => panic!("Wrong token instruction type"),
                }
                assert_eq!(amount, 1000);
            },
            _ => panic!("Wrong instruction type"),
        }
    }
} 