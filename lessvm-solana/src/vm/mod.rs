pub mod core;
pub mod opcodes;
pub mod memory;
pub mod stack;
pub mod gas;
pub mod debug;
pub mod data_structures;
pub mod tests;

pub use core::VM;
pub use opcodes::OpCode;
pub use memory::Memory;
pub use stack::{Stack, Value};
pub use gas::Gas;
pub use data_structures::{BTreeMapDS, TrieDS, GraphDS, OHLCVDS, HypergraphDS};

use thiserror::Error;
use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Clone, Copy)]
pub enum VMError {
    #[error("Stack overflow")]
    StackOverflow,
    #[error("Stack underflow")]
    StackUnderflow,
    #[error("Invalid memory access")]
    InvalidMemoryAccess,
    #[error("Out of gas")]
    OutOfGas,
    #[error("Invalid instruction")]
    InvalidInstruction,
    #[error("Invalid account")]
    InvalidAccount,
    #[error("Arithmetic overflow")]
    ArithmeticOverflow,
    #[error("Reentrancy detected")]
    ReentrancyDetected,
    #[error("Invalid data structure operation")]
    InvalidDataStructureOperation,
}

impl From<VMError> for ProgramError {
    fn from(e: VMError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

// Re-export common types
pub type VMResult<T> = Result<T, VMError>;