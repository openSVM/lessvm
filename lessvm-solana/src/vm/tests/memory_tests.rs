use super::super::*;
use solana_program::clock::Epoch;
use solana_program::pubkey::Pubkey;
use solana_program::account_info::AccountInfo;

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
fn test_memory_initial_capacity() {
    // Test that memory is initialized with the correct capacity
    let memory = Memory::new();
    assert_eq!(memory.capacity(), 1024);
    assert_eq!(memory.size(), 0);
}

#[test]
fn test_memory_growth_strategy() {
    // Test the memory growth strategy
    let mut memory = Memory::new();
    
    // Initial capacity is 1024 bytes
    assert_eq!(memory.capacity(), 1024);
    
    // Store data that fits within the initial capacity
    let data = vec![1; 1000];
    memory.store(0, &data).unwrap();
    assert_eq!(memory.capacity(), 1024);
    
    // Store data that exceeds the initial capacity
    let data = vec![2; 1024];
    memory.store(1000, &data).unwrap();
    
    // Capacity should grow by 50% (1024 * 1.5 = 1536) or to the required size (1000 + 1024 = 2024),
    // whichever is larger
    assert_eq!(memory.capacity(), 2024);
    
    // Store more data to trigger another growth
    let data = vec![3; 1000];
    memory.store(2024, &data).unwrap();
    
    // New capacity should be 2024 * 1.5 = 3036
    assert_eq!(memory.capacity(), 3036);
}

#[test]
fn test_memory_operations_with_growth() {
    let mut memory = Memory::new();
    
    // Fill memory with a pattern
    for i in 0..1024 {
        memory.store8(i, (i % 256) as u8).unwrap();
    }
    
    // Verify the pattern
    for i in 0..1024 {
        assert_eq!(memory.load8(i).unwrap(), (i % 256) as u8);
    }
    
    // Store beyond the initial capacity
    let large_data = vec![0xAA; 2000];
    memory.store(1024, &large_data).unwrap();
    
    // Verify the original data is intact
    for i in 0..1024 {
        assert_eq!(memory.load8(i).unwrap(), (i % 256) as u8);
    }
    
    // Verify the new data
    for i in 0..2000 {
        assert_eq!(memory.load8(1024 + i).unwrap(), 0xAA);
    }
    
    // Test copy operation across the growth boundary
    memory.copy(3000, 500, 100).unwrap();
    
    // Verify the copied data
    for i in 0..100 {
        assert_eq!(memory.load8(3000 + i).unwrap(), ((500 + i) % 256) as u8);
    }
}

#[test]
fn test_memory_with_vm_operations() {
    let program_id = Pubkey::new_unique();
    let (_, _, account) = create_test_account(1000000);
    let accounts = vec![account];
    
    let mut vm = VM::new(&program_id, &accounts, &[]);
    
    // Create a program that stores data beyond the initial memory capacity
    // push8 <value>, push8 <offset>, store, halt
    let mut bytecode = vec![];
    
    // Store a value at offset 2000 (beyond initial capacity)
    bytecode.extend_from_slice(&[
        OpCode::Push8 as u8,
    ]);
    bytecode.extend_from_slice(&0x1234567890ABCDEFu64.to_le_bytes());
    bytecode.extend_from_slice(&[
        OpCode::Push8 as u8,
    ]);
    bytecode.extend_from_slice(&2000u64.to_le_bytes());
    bytecode.extend_from_slice(&[
        OpCode::Store as u8,
        OpCode::Halt as u8,
    ]);
    
    // Execute the program
    let result = vm.execute(&bytecode);
    assert!(result.is_ok());
    
    // Verify the memory capacity has grown
    assert!(vm.memory.capacity() > 1024);
    
    // Verify the stored value
    vm.stack.push(Value(2000)).unwrap();
    let result = vm.execute(&[OpCode::Load as u8]);
    assert!(result.is_ok());
    assert_eq!(vm.stack.pop().unwrap().0, 0x1234567890ABCDEF);
}