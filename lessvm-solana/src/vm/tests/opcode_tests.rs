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

// Test for Solana account operations
#[test]
fn test_solana_account_operations() {
    let program_id = Pubkey::new_unique();
    let (_, _, account1) = create_test_account(1000000);
    let (_, _, account2) = create_test_account(500000);
    let accounts = vec![account1, account2];
    
    let mut vm = VM::new(&program_id, &accounts, &[]);
    
    // Test GetBalance
    vm.stack.push(Value(0)).unwrap(); // account index 0
    let result = vm.execute(&[OpCode::GetBalance as u8]);
    assert!(result.is_ok());
    assert_eq!(vm.stack.pop().unwrap().0, 1000000);
    
    // Test IsWritable
    vm.stack.push(Value(1)).unwrap(); // account index 1
    let result = vm.execute(&[OpCode::IsWritable as u8]);
    assert!(result.is_ok());
    assert_eq!(vm.stack.pop().unwrap().0, 1); // account is writable
    
    // Test IsSigner
    vm.stack.push(Value(0)).unwrap(); // account index 0
    let result = vm.execute(&[OpCode::IsSigner as u8]);
    assert!(result.is_ok());
    assert_eq!(vm.stack.pop().unwrap().0, 1); // account is signer
}

// Test for Graph operations
#[test]
fn test_graph_operations() {
    let program_id = Pubkey::new_unique();
    let (_, _, account) = create_test_account(1000000);
    let accounts = vec![account];
    
    let mut vm = VM::new(&program_id, &accounts, &[]);
    
    // Create a graph
    vm.stack.push(Value(0)).unwrap(); // graph id 0
    let result = vm.execute(&[OpCode::GraphCreate as u8]);
    assert!(result.is_ok());
    
    // Add nodes
    vm.stack.push(Value(0)).unwrap(); // graph id
    vm.stack.push(Value(1)).unwrap(); // node id
    vm.stack.push(Value(100)).unwrap(); // node value
    let result = vm.execute(&[OpCode::GraphAddNode as u8]);
    assert!(result.is_ok());
    
    vm.stack.push(Value(0)).unwrap(); // graph id
    vm.stack.push(Value(2)).unwrap(); // node id
    vm.stack.push(Value(200)).unwrap(); // node value
    let result = vm.execute(&[OpCode::GraphAddNode as u8]);
    assert!(result.is_ok());
    
    vm.stack.push(Value(0)).unwrap(); // graph id
    vm.stack.push(Value(3)).unwrap(); // node id
    vm.stack.push(Value(300)).unwrap(); // node value
    let result = vm.execute(&[OpCode::GraphAddNode as u8]);
    assert!(result.is_ok());
    
    // Add edges
    vm.stack.push(Value(0)).unwrap(); // graph id
    vm.stack.push(Value(1)).unwrap(); // from node
    vm.stack.push(Value(2)).unwrap(); // to node
    vm.stack.push(Value(10)).unwrap(); // weight
    let result = vm.execute(&[OpCode::GraphAddEdge as u8]);
    assert!(result.is_ok());
    
    vm.stack.push(Value(0)).unwrap(); // graph id
    vm.stack.push(Value(1)).unwrap(); // from node
    vm.stack.push(Value(3)).unwrap(); // to node
    vm.stack.push(Value(20)).unwrap(); // weight
    let result = vm.execute(&[OpCode::GraphAddEdge as u8]);
    assert!(result.is_ok());
    
    // Get node value
    vm.stack.push(Value(0)).unwrap(); // graph id
    vm.stack.push(Value(2)).unwrap(); // node id
    let result = vm.execute(&[OpCode::GraphGetNode as u8]);
    assert!(result.is_ok());
    assert_eq!(vm.stack.pop().unwrap().0, 200);
    
    // Set node value
    vm.stack.push(Value(0)).unwrap(); // graph id
    vm.stack.push(Value(2)).unwrap(); // node id
    vm.stack.push(Value(250)).unwrap(); // new value
    let result = vm.execute(&[OpCode::GraphSetNode as u8]);
    assert!(result.is_ok());
    
    // Verify the new value
    vm.stack.push(Value(0)).unwrap(); // graph id
    vm.stack.push(Value(2)).unwrap(); // node id
    let result = vm.execute(&[OpCode::GraphGetNode as u8]);
    assert!(result.is_ok());
    assert_eq!(vm.stack.pop().unwrap().0, 250);
    
    // Get neighbors
    vm.stack.push(Value(0)).unwrap(); // graph id
    vm.stack.push(Value(1)).unwrap(); // node id
    let result = vm.execute(&[OpCode::GraphGetNeighbors as u8]);
    assert!(result.is_ok());
    
    // Stack should have: [count, neighbor1, weight1, neighbor2, weight2]
    let count = vm.stack.pop().unwrap().0;
    assert_eq!(count, 2);
    
    // Pop in reverse order of how they were pushed
    let neighbor2 = vm.stack.pop().unwrap().0;
    let weight2 = vm.stack.pop().unwrap().0;
    let neighbor1 = vm.stack.pop().unwrap().0;
    let weight1 = vm.stack.pop().unwrap().0;
    
    // Check the neighbors (order may vary)
    if neighbor1 == 2 {
        assert_eq!(weight1, 10);
        assert_eq!(neighbor2, 3);
        assert_eq!(weight2, 20);
    } else {
        assert_eq!(neighbor1, 3);
        assert_eq!(weight1, 20);
        assert_eq!(neighbor2, 2);
        assert_eq!(weight2, 10);
    }
    
    // Test BFS
    vm.stack.push(Value(0)).unwrap(); // graph id
    vm.stack.push(Value(1)).unwrap(); // start node
    let result = vm.execute(&[OpCode::GraphBfs as u8]);
    assert!(result.is_ok());
    
    // Stack should have: [count, node1, node2, node3]
    let count = vm.stack.pop().unwrap().0;
    assert_eq!(count, 3);
    
    // Pop nodes in BFS order
    let node3 = vm.stack.pop().unwrap().0;
    let node2 = vm.stack.pop().unwrap().0;
    let node1 = vm.stack.pop().unwrap().0;
    
    assert_eq!(node1, 1); // Start node
    // node2 and node3 could be in either order depending on implementation
    assert!(
        (node2 == 2 && node3 == 3) || 
        (node2 == 3 && node3 == 2)
    );
    
    // Test GraphClear
    vm.stack.push(Value(0)).unwrap(); // graph id
    let result = vm.execute(&[OpCode::GraphClear as u8]);
    assert!(result.is_ok());
    
    // Verify the graph is empty
    vm.stack.push(Value(0)).unwrap(); // graph id
    vm.stack.push(Value(1)).unwrap(); // node id
    let result = vm.execute(&[OpCode::GraphGetNode as u8]);
    assert!(result.is_ok());
    assert_eq!(vm.stack.pop().unwrap().0, 0); // Should return 0 for non-existent node
}

// Test for OHLCV operations
#[test]
fn test_ohlcv_operations() {
    let program_id = Pubkey::new_unique();
    let (_, _, account) = create_test_account(1000000);
    let accounts = vec![account];
    
    let mut vm = VM::new(&program_id, &accounts, &[]);
    
    // Create an OHLCV
    vm.stack.push(Value(0)).unwrap(); // ohlcv id 0
    let result = vm.execute(&[OpCode::OhlcvCreate as u8]);
    assert!(result.is_ok());
    
    // Add bars
    // Bar 1: timestamp=1000, open=100, high=110, low=90, close=105, volume=1000
    vm.stack.push(Value(0)).unwrap(); // ohlcv id
    vm.stack.push(Value(1000)).unwrap(); // timestamp
    vm.stack.push(Value(100)).unwrap(); // open
    vm.stack.push(Value(110)).unwrap(); // high
    vm.stack.push(Value(90)).unwrap(); // low
    vm.stack.push(Value(105)).unwrap(); // close
    vm.stack.push(Value(1000)).unwrap(); // volume
    let result = vm.execute(&[OpCode::OhlcvAddBar as u8]);
    assert!(result.is_ok());
    
    // Bar 2: timestamp=2000, open=105, high=120, low=100, close=115, volume=1500
    vm.stack.push(Value(0)).unwrap(); // ohlcv id
    vm.stack.push(Value(2000)).unwrap(); // timestamp
    vm.stack.push(Value(105)).unwrap(); // open
    vm.stack.push(Value(120)).unwrap(); // high
    vm.stack.push(Value(100)).unwrap(); // low
    vm.stack.push(Value(115)).unwrap(); // close
    vm.stack.push(Value(1500)).unwrap(); // volume
    let result = vm.execute(&[OpCode::OhlcvAddBar as u8]);
    assert!(result.is_ok());
    
    // Bar 3: timestamp=3000, open=115, high=130, low=110, close=125, volume=2000
    vm.stack.push(Value(0)).unwrap(); // ohlcv id
    vm.stack.push(Value(3000)).unwrap(); // timestamp
    vm.stack.push(Value(115)).unwrap(); // open
    vm.stack.push(Value(130)).unwrap(); // high
    vm.stack.push(Value(110)).unwrap(); // low
    vm.stack.push(Value(125)).unwrap(); // close
    vm.stack.push(Value(2000)).unwrap(); // volume
    let result = vm.execute(&[OpCode::OhlcvAddBar as u8]);
    assert!(result.is_ok());
    
    // Get bar
    vm.stack.push(Value(0)).unwrap(); // ohlcv id
    vm.stack.push(Value(1)).unwrap(); // index
    let result = vm.execute(&[OpCode::OhlcvGetBar as u8]);
    assert!(result.is_ok());
    
    // Stack should have: [timestamp, open, high, low, close, volume]
    let volume = vm.stack.pop().unwrap().0;
    let close = vm.stack.pop().unwrap().0;
    let low = vm.stack.pop().unwrap().0;
    let high = vm.stack.pop().unwrap().0;
    let open = vm.stack.pop().unwrap().0;
    let timestamp = vm.stack.pop().unwrap().0;
    
    assert_eq!(timestamp, 2000);
    assert_eq!(open, 105);
    assert_eq!(high, 120);
    assert_eq!(low, 100);
    assert_eq!(close, 115);
    assert_eq!(volume, 1500);
    
    // Calculate SMA with period 2
    vm.stack.push(Value(0)).unwrap(); // ohlcv id
    vm.stack.push(Value(2)).unwrap(); // period
    let result = vm.execute(&[OpCode::OhlcvSma as u8]);
    assert!(result.is_ok());
    
    // Stack should have: [count, timestamp1, value1, timestamp2, value2]
    let count = vm.stack.pop().unwrap().0;
    assert_eq!(count, 2);
    
    // Pop SMA values
    let timestamp2 = vm.stack.pop().unwrap().0;
    let value2 = vm.stack.pop().unwrap().0;
    let timestamp1 = vm.stack.pop().unwrap().0;
    let value1 = vm.stack.pop().unwrap().0;
    
    assert_eq!(timestamp1, 2000);
    assert_eq!(value1, (105 + 115) / 2); // (bar1.close + bar2.close) / 2
    assert_eq!(timestamp2, 3000);
    assert_eq!(value2, (115 + 125) / 2); // (bar2.close + bar3.close) / 2
}

// Test for Hypergraph operations
#[test]
fn test_hypergraph_operations() {
    let program_id = Pubkey::new_unique();
    let (_, _, account) = create_test_account(1000000);
    let accounts = vec![account];
    
    let mut vm = VM::new(&program_id, &accounts, &[]);
    
    // Create a hypergraph
    vm.stack.push(Value(0)).unwrap(); // hypergraph id 0
    let result = vm.execute(&[OpCode::HyperCreate as u8]);
    assert!(result.is_ok());
    
    // Add nodes
    vm.stack.push(Value(0)).unwrap(); // hypergraph id
    vm.stack.push(Value(1)).unwrap(); // node id
    vm.stack.push(Value(100)).unwrap(); // node value
    let result = vm.execute(&[OpCode::HyperAddNode as u8]);
    assert!(result.is_ok());
    
    vm.stack.push(Value(0)).unwrap(); // hypergraph id
    vm.stack.push(Value(2)).unwrap(); // node id
    vm.stack.push(Value(200)).unwrap(); // node value
    let result = vm.execute(&[OpCode::HyperAddNode as u8]);
    assert!(result.is_ok());
    
    vm.stack.push(Value(0)).unwrap(); // hypergraph id
    vm.stack.push(Value(3)).unwrap(); // node id
    vm.stack.push(Value(300)).unwrap(); // node value
    let result = vm.execute(&[OpCode::HyperAddNode as u8]);
    assert!(result.is_ok());
    
    // Create hyperedges
    vm.stack.push(Value(0)).unwrap(); // hypergraph id
    vm.stack.push(Value(101)).unwrap(); // edge id
    vm.stack.push(Value(10)).unwrap(); // weight
    let result = vm.execute(&[OpCode::HyperAddEdge as u8]);
    assert!(result.is_ok());
    
    vm.stack.push(Value(0)).unwrap(); // hypergraph id
    vm.stack.push(Value(102)).unwrap(); // edge id
    vm.stack.push(Value(20)).unwrap(); // weight
    let result = vm.execute(&[OpCode::HyperAddEdge as u8]);
    assert!(result.is_ok());
    
    // Add nodes to edges
    vm.stack.push(Value(0)).unwrap(); // hypergraph id
    vm.stack.push(Value(101)).unwrap(); // edge id
    vm.stack.push(Value(1)).unwrap(); // node id
    let result = vm.execute(&[OpCode::HyperAddNodeToEdge as u8]);
    assert!(result.is_ok());
    
    vm.stack.push(Value(0)).unwrap(); // hypergraph id
    vm.stack.push(Value(101)).unwrap(); // edge id
    vm.stack.push(Value(2)).unwrap(); // node id
    let result = vm.execute(&[OpCode::HyperAddNodeToEdge as u8]);
    assert!(result.is_ok());
    
    vm.stack.push(Value(0)).unwrap(); // hypergraph id
    vm.stack.push(Value(102)).unwrap(); // edge id
    vm.stack.push(Value(2)).unwrap(); // node id
    let result = vm.execute(&[OpCode::HyperAddNodeToEdge as u8]);
    assert!(result.is_ok());
    
    vm.stack.push(Value(0)).unwrap(); // hypergraph id
    vm.stack.push(Value(102)).unwrap(); // edge id
    vm.stack.push(Value(3)).unwrap(); // node id
    let result = vm.execute(&[OpCode::HyperAddNodeToEdge as u8]);
    assert!(result.is_ok());
    
    // Test auto-creation of nodes and edges
    vm.stack.push(Value(0)).unwrap(); // hypergraph id
    vm.stack.push(Value(103)).unwrap(); // new edge id
    vm.stack.push(Value(4)).unwrap(); // new node id
    let result = vm.execute(&[OpCode::HyperAddNodeToEdge as u8]);
    assert!(result.is_ok());
}

// Test for Revert opcode
#[test]
fn test_revert_opcode() {
    let program_id = Pubkey::new_unique();
    let (_, _, account) = create_test_account(1000000);
    let accounts = vec![account];
    
    let mut vm = VM::new(&program_id, &accounts, &[]);
    
    // Push error code 42
    vm.stack.push(Value(42)).unwrap();
    
    // Execute Revert
    let result = vm.execute(&[OpCode::Revert as u8]);
    
    // Should return a ProgramError::Custom with code 42
    assert!(result.is_err());
    if let Err(err) = result {
        if let solana_program::program_error::ProgramError::Custom(code) = err {
            assert_eq!(code, 42);
        } else {
            panic!("Expected ProgramError::Custom, got {:?}", err);
        }
    }
}