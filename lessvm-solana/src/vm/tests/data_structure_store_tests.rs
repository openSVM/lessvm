use super::super::*;
use super::super::data_structures::*;
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
fn test_data_structure_store_new() {
    let program_id = Pubkey::new_unique();
    let (_, _, account) = create_test_account(1000000);
    let accounts = vec![account];
    
    let vm = VM::new(&program_id, &accounts, &[]);
    
    // Verify the data structures are initialized with the correct capacity
    assert_eq!(vm.data_structures.btrees.capacity(), MAX_DATA_STRUCTURES);
    assert_eq!(vm.data_structures.tries.capacity(), MAX_DATA_STRUCTURES);
    assert_eq!(vm.data_structures.graphs.capacity(), MAX_DATA_STRUCTURES);
    assert_eq!(vm.data_structures.ohlcvs.capacity(), MAX_DATA_STRUCTURES);
    assert_eq!(vm.data_structures.hypergraphs.capacity(), MAX_DATA_STRUCTURES);
    
    // Verify the data structures are initially empty
    assert_eq!(vm.data_structures.btrees.len(), 0);
    assert_eq!(vm.data_structures.tries.len(), 0);
    assert_eq!(vm.data_structures.graphs.len(), 0);
    assert_eq!(vm.data_structures.ohlcvs.len(), 0);
    assert_eq!(vm.data_structures.hypergraphs.len(), 0);
}

#[test]
fn test_ensure_capacity_btree() {
    let program_id = Pubkey::new_unique();
    let (_, _, account) = create_test_account(1000000);
    let accounts = vec![account];
    
    let mut vm = VM::new(&program_id, &accounts, &[]);
    
    // Initially the btrees vector is empty
    assert_eq!(vm.data_structures.btrees.len(), 0);
    
    // Ensure capacity for index 5
    vm.data_structures.ensure_capacity(DataStructureType::BTreeMap, 5);
    
    // The vector should now have length 6 (indices 0-5)
    assert_eq!(vm.data_structures.btrees.len(), 6);
    
    // All elements should be None
    for i in 0..6 {
        assert!(vm.data_structures.btrees[i].is_none());
    }
    
    // Create a BTreeMap at index 3
    vm.data_structures.btrees[3] = Some(BTreeMapDS::new());
    
    // Ensure capacity for index 10
    vm.data_structures.ensure_capacity(DataStructureType::BTreeMap, 10);
    
    // The vector should now have length 11 (indices 0-10)
    assert_eq!(vm.data_structures.btrees.len(), 11);
    
    // The BTreeMap at index 3 should still exist
    assert!(vm.data_structures.btrees[3].is_some());
    
    // Other elements should be None
    for i in 0..11 {
        if i != 3 {
            assert!(vm.data_structures.btrees[i].is_none());
        }
    }
}

#[test]
fn test_ensure_capacity_all_types() {
    let program_id = Pubkey::new_unique();
    let (_, _, account) = create_test_account(1000000);
    let accounts = vec![account];
    
    let mut vm = VM::new(&program_id, &accounts, &[]);
    
    // Test ensure_capacity for all data structure types
    vm.data_structures.ensure_capacity(DataStructureType::BTreeMap, 3);
    vm.data_structures.ensure_capacity(DataStructureType::Trie, 4);
    vm.data_structures.ensure_capacity(DataStructureType::Graph, 5);
    vm.data_structures.ensure_capacity(DataStructureType::OHLCV, 6);
    vm.data_structures.ensure_capacity(DataStructureType::Hypergraph, 7);
    
    // Verify the lengths
    assert_eq!(vm.data_structures.btrees.len(), 4);
    assert_eq!(vm.data_structures.tries.len(), 5);
    assert_eq!(vm.data_structures.graphs.len(), 6);
    assert_eq!(vm.data_structures.ohlcvs.len(), 7);
    assert_eq!(vm.data_structures.hypergraphs.len(), 8);
}

#[test]
fn test_data_structure_operations_with_ensure_capacity() {
    let program_id = Pubkey::new_unique();
    let (_, _, account) = create_test_account(1000000);
    let accounts = vec![account];
    
    let mut vm = VM::new(&program_id, &accounts, &[]);
    
    // Test BTreeClear with ensure_capacity
    vm.stack.push(Value(10)).unwrap(); // id = 10
    let result = vm.execute(&[OpCode::BTreeClear as u8]);
    assert!(result.is_ok());
    
    // Verify that the BTreeMap was created at index 10
    assert_eq!(vm.data_structures.btrees.len(), 11);
    assert!(vm.data_structures.btrees[10].is_some());
    
    // Test GraphCreate with ensure_capacity
    vm.stack.push(Value(15)).unwrap(); // id = 15
    let result = vm.execute(&[OpCode::GraphCreate as u8]);
    assert!(result.is_ok());
    
    // Verify that the Graph was created at index 15
    assert_eq!(vm.data_structures.graphs.len(), 16);
    assert!(vm.data_structures.graphs[15].is_some());
}