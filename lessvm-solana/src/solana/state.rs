use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct ProgramState {
    pub is_initialized: bool,
    pub authority: Pubkey,
    pub total_executions: u64,
    pub total_gas_used: u64,
    pub last_execution_timestamp: i64,
}

impl Sealed for ProgramState {}

impl IsInitialized for ProgramState {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for ProgramState {
    const LEN: usize = 1 + 32 + 8 + 8 + 8; // bool + Pubkey + u64 + u64 + i64

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        if src.len() != Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }

        let mut current = src;
        
        // Unpack is_initialized
        let is_initialized = current[0] != 0;
        current = &current[1..];

        // Unpack authority
        let mut authority = [0u8; 32];
        authority.copy_from_slice(&current[..32]);
        let authority = Pubkey::new_from_array(authority);
        current = &current[32..];

        // Unpack total_executions
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&current[..8]);
        let total_executions = u64::from_le_bytes(bytes);
        current = &current[8..];

        // Unpack total_gas_used
        bytes.copy_from_slice(&current[..8]);
        let total_gas_used = u64::from_le_bytes(bytes);
        current = &current[8..];

        // Unpack last_execution_timestamp
        bytes.copy_from_slice(&current[..8]);
        let last_execution_timestamp = i64::from_le_bytes(bytes);

        Ok(ProgramState {
            is_initialized,
            authority,
            total_executions,
            total_gas_used,
            last_execution_timestamp,
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        if dst.len() != Self::LEN {
            panic!("Invalid account data length");
        }

        let mut current = dst;

        // Pack is_initialized
        current[0] = self.is_initialized as u8;
        current = &mut current[1..];

        // Pack authority
        current[..32].copy_from_slice(self.authority.as_ref());
        current = &mut current[32..];

        // Pack total_executions
        current[..8].copy_from_slice(&self.total_executions.to_le_bytes());
        current = &mut current[8..];

        // Pack total_gas_used
        current[..8].copy_from_slice(&self.total_gas_used.to_le_bytes());
        current = &mut current[8..];

        // Pack last_execution_timestamp
        current[..8].copy_from_slice(&self.last_execution_timestamp.to_le_bytes());
    }
}

impl Default for ProgramState {
    fn default() -> Self {
        Self {
            is_initialized: false,
            authority: Pubkey::default(),
            total_executions: 0,
            total_gas_used: 0,
            last_execution_timestamp: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_pack() {
        let state = ProgramState {
            is_initialized: true,
            authority: Pubkey::new_unique(),
            total_executions: 42,
            total_gas_used: 1000000,
            last_execution_timestamp: 1234567890,
        };

        let mut packed = vec![0u8; ProgramState::LEN];
        state.pack_into_slice(&mut packed);

        let unpacked = ProgramState::unpack_from_slice(&packed).unwrap();

        assert_eq!(state.is_initialized, unpacked.is_initialized);
        assert_eq!(state.authority, unpacked.authority);
        assert_eq!(state.total_executions, unpacked.total_executions);
        assert_eq!(state.total_gas_used, unpacked.total_gas_used);
        assert_eq!(state.last_execution_timestamp, unpacked.last_execution_timestamp);
    }

    #[test]
    fn test_state_pack_unpack_errors() {
        let state = ProgramState::default();
        
        // Test packing into too small buffer
        let mut small_buf = vec![0u8; ProgramState::LEN - 1];
        assert!(std::panic::catch_unwind(|| state.pack_into_slice(&mut small_buf)).is_err());

        // Test unpacking from too small buffer
        assert!(ProgramState::unpack_from_slice(&small_buf).is_err());

        // Test packing into too large buffer
        let mut large_buf = vec![0u8; ProgramState::LEN + 1];
        assert!(std::panic::catch_unwind(|| state.pack_into_slice(&mut large_buf)).is_err());

        // Test unpacking from too large buffer
        assert!(ProgramState::unpack_from_slice(&large_buf).is_err());
    }
}