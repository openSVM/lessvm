#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum OpCode {
    // Stack Operations (0x0*)
    Nop = 0x00,
    Push1 = 0x01,
    Push8 = 0x02,
    Pop = 0x03,
    Dup = 0x04,
    Swap = 0x05,

    // Math Operations (0x1*)
    Add = 0x10,
    Sub = 0x11,
    Mul = 0x12,
    Div = 0x13,
    MulDiv = 0x14,
    Mod = 0x15,
    Exp = 0x16,
    SignExtend = 0x17,

    // Bitwise Operations (0x18-0x1F)
    And = 0x18,
    Or = 0x19,
    Xor = 0x1A,
    Not = 0x1B,
    Byte = 0x1C,
    Shl = 0x1D,
    Shr = 0x1E,
    Sar = 0x1F,

    // Memory Operations (0x2*)
    Load = 0x20,
    Store = 0x21,
    LoadN = 0x22,
    StoreN = 0x23,
    Msize = 0x24,
    Mload8 = 0x25,
    Mstore8 = 0x26,

    // Control Flow (0x3*)
    Jump = 0x30,
    JumpI = 0x31,
    Call = 0x32,
    Return = 0x33,
    Revert = 0x34,

    // Solana Operations (0x4*)
    Transfer = 0x40,
    SPLTransfer = 0x41,
    CPI = 0x42,
    Log = 0x43,
    GetBalance = 0x44,
    GetOwner = 0x45,
    IsWritable = 0x46,
    IsSigner = 0x47,

    // Data Structure Operations (0x5*)
    // BTreeMap operations
    BTreeCreate = 0x50,
    BTreeInsert = 0x51,
    BTreeGet = 0x52,
    BTreeRemove = 0x53,
    BTreeContains = 0x54,
    BTreeLen = 0x55,
    BTreeFirstKey = 0x56,
    BTreeLastKey = 0x57,
    BTreeClear = 0x58,

    // Trie operations
    TrieCreate = 0x59,
    TrieInsert = 0x5A,
    TrieGet = 0x5B,
    TrieContains = 0x5C,
    TrieClear = 0x5D,

    // Graph operations
    GraphCreate = 0x60,
    GraphAddNode = 0x61,
    GraphAddEdge = 0x62,
    GraphGetNode = 0x63,
    GraphSetNode = 0x64,
    GraphGetNeighbors = 0x65,
    GraphBfs = 0x66,
    GraphClear = 0x67,

    // OHLCV operations
    OhlcvCreate = 0x68,
    OhlcvAddBar = 0x69,
    OhlcvGetBar = 0x6A,
    OhlcvSma = 0x6B,

    // Hypergraph operations
    HyperCreate = 0x6C,
    HyperAddNode = 0x6D,
    HyperAddEdge = 0x6E,
    HyperAddNodeToEdge = 0x6F,

    // System Operations (0xF*)
    Halt = 0xFF,
}

impl OpCode {
    pub fn gas_cost(&self) -> u64 {
        match self {
            // Stack operations
            OpCode::Nop => 1,
            OpCode::Push1 | OpCode::Pop => 2,
            OpCode::Push8 => 3,
            OpCode::Dup | OpCode::Swap => 3,

            // Math operations
            OpCode::Add | OpCode::Sub => 3,
            OpCode::Mul => 5,
            OpCode::Div | OpCode::Mod => 8,
            OpCode::MulDiv => 10,
            OpCode::Exp => 50,
            OpCode::SignExtend => 5,

            // Bitwise operations
            OpCode::And | OpCode::Or | OpCode::Xor | OpCode::Not => 3,
            OpCode::Byte => 3,
            OpCode::Shl | OpCode::Shr | OpCode::Sar => 3,

            // Memory operations
            OpCode::Load | OpCode::Store => 3,
            OpCode::LoadN | OpCode::StoreN => 6,
            OpCode::Msize => 2,
            OpCode::Mload8 | OpCode::Mstore8 => 3,

            // Control flow
            OpCode::Jump => 8,
            OpCode::JumpI => 10,
            OpCode::Call => 40,
            OpCode::Return => 0,
            OpCode::Revert => 0,

            // Solana operations
            OpCode::Transfer | OpCode::SPLTransfer => 100,
            OpCode::CPI => 200,
            OpCode::Log => 8,
            OpCode::GetBalance | OpCode::GetOwner => 20,
            OpCode::IsWritable | OpCode::IsSigner => 5,

            // Data Structure operations - gas costs reflect complexity
            OpCode::BTreeCreate | OpCode::TrieCreate | OpCode::GraphCreate | 
            OpCode::OhlcvCreate | OpCode::HyperCreate => 5,
            OpCode::BTreeInsert | OpCode::BTreeGet | OpCode::BTreeRemove | 
            OpCode::BTreeContains | OpCode::BTreeFirstKey | OpCode::BTreeLastKey => 15,
            OpCode::TrieInsert | OpCode::TrieGet | OpCode::TrieContains => 20, 
            OpCode::GraphAddNode | OpCode::GraphGetNode | OpCode::GraphSetNode => 10,
            OpCode::GraphAddEdge | OpCode::GraphGetNeighbors => 15,
            OpCode::GraphBfs => 50,
            OpCode::OhlcvAddBar | OpCode::OhlcvGetBar => 8,
            OpCode::OhlcvSma => 30,
            OpCode::HyperAddNode | OpCode::HyperAddEdge | OpCode::HyperAddNodeToEdge => 20,
            OpCode::BTreeLen | OpCode::BTreeClear | OpCode::TrieClear | OpCode::GraphClear => 5,

            // System operations
            OpCode::Halt => 0,
        }
    }

    pub fn from_byte(byte: u8) -> Option<Self> {
        use std::mem::transmute;
        match byte {
            0x00..=0x05 | // Stack ops
            0x10..=0x1F | // Math and bitwise ops
            0x20..=0x26 | // Memory ops
            0x30..=0x34 | // Control flow
            0x40..=0x47 | // Solana ops
            0x50..=0x5D | // BTreeMap and Trie ops
            0x60..=0x6F | // Graph, OHLCV, and Hypergraph ops
            0xFF => unsafe { Some(transmute(byte)) }, // Safe because we check valid ranges
            _ => None
        }
    }
} 