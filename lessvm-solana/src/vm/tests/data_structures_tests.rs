// Import test framework items explicitly
use crate::vm::data_structures::*;
use crate::vm::Value;

// Simple smoke test to verify test framework setup
#[test]
fn test_sanity_check() {
    println!("Sanity check test is running!");
    assert!(true);
}

// Tests for BTreeMapDS
#[test]
fn test_btreemap_basic_operations() {
    let mut btree = BTreeMapDS::new();
    
    // Test that new map is empty
    assert!(btree.is_empty());
    assert_eq!(btree.len(), 0);
    assert_eq!(btree.first_key(), None);
    assert_eq!(btree.last_key(), None);
    
    // Test insert
    assert_eq!(btree.insert(10, 100), None);
    assert_eq!(btree.insert(20, 200), None);
    assert_eq!(btree.insert(30, 300), None);
    
    // Test len and is_empty after insertion
    assert!(!btree.is_empty());
    assert_eq!(btree.len(), 3);
    
    // Test get
    assert_eq!(btree.get(10), Some(100));
    assert_eq!(btree.get(20), Some(200));
    assert_eq!(btree.get(30), Some(300));
    assert_eq!(btree.get(40), None); // Non-existent key
    
    // Test contains_key
    assert!(btree.contains_key(10));
    assert!(btree.contains_key(30));
    assert!(!btree.contains_key(40)); // Non-existent key
    
    // Test first_key and last_key
    assert_eq!(btree.first_key(), Some(10));
    assert_eq!(btree.last_key(), Some(30));
    
    // Test overwriting a key
    assert_eq!(btree.insert(20, 250), Some(200));
    assert_eq!(btree.get(20), Some(250));
    
    // Test remove
    assert_eq!(btree.remove(20), Some(250));
    assert_eq!(btree.len(), 2);
    assert_eq!(btree.get(20), None);
    
    // Test clear
    btree.clear();
    assert!(btree.is_empty());
    assert_eq!(btree.len(), 0);
    assert_eq!(btree.get(10), None);
}

// Tests for TrieDS
#[test]
fn test_trie_basic_operations() {
    let mut trie = TrieDS::new();
    
    // Test insertion and retrieval
    trie.insert(b"hello", Value(42)).unwrap();
    trie.insert(b"world", Value(100)).unwrap();
    trie.insert(b"help", Value(30)).unwrap();
    
    // Test successful retrieval
    assert_eq!(trie.get(b"hello"), Some(Value(42)));
    assert_eq!(trie.get(b"world"), Some(Value(100)));
    assert_eq!(trie.get(b"help"), Some(Value(30)));
    
    // Test non-existent keys
    assert_eq!(trie.get(b"hell"), None);
    assert_eq!(trie.get(b"helper"), None);
    assert_eq!(trie.get(b"world!"), None);
    
    // Test contains
    assert!(trie.contains(b"hello"));
    assert!(!trie.contains(b"hell"));
    
    // Test clear
    trie.clear();
    assert_eq!(trie.get(b"hello"), None);
    assert_eq!(trie.get(b"world"), None);
}

// Tests for GraphDS
#[test]
fn test_graph_basic_operations() {
    let mut graph = GraphDS::new();
    
    // Test add_node
    graph.add_node(1, 100).unwrap();
    graph.add_node(2, 200).unwrap();
    graph.add_node(3, 300).unwrap();
    
    // Test get_node_value
    assert_eq!(graph.get_node_value(1), Some(100));
    assert_eq!(graph.get_node_value(2), Some(200));
    assert_eq!(graph.get_node_value(4), None); // Non-existent node
    
    // Test set_node_value
    graph.set_node_value(1, 150).unwrap();
    assert_eq!(graph.get_node_value(1), Some(150));
    assert!(graph.set_node_value(4, 400).is_err()); // Should fail for non-existent node
    
    // Test add_edge
    graph.add_edge(1, 2, 10).unwrap();
    graph.add_edge(1, 3, 20).unwrap();
    graph.add_edge(2, 3, 30).unwrap();
    
    // Test has_edge
    assert!(graph.has_edge(1, 2));
    assert!(graph.has_edge(1, 3));
    assert!(!graph.has_edge(3, 1)); // Directed graph
    
    // Test get_edge_weight
    assert_eq!(graph.get_edge_weight(1, 2), Some(10));
    assert_eq!(graph.get_edge_weight(1, 3), Some(20));
    assert_eq!(graph.get_edge_weight(3, 1), None); // No edge in this direction
    
    // Test get_neighbors
    let neighbors = graph.get_neighbors(1);
    assert_eq!(neighbors.len(), 2);
    assert!(neighbors.contains(&(2, 10)));
    assert!(neighbors.contains(&(3, 20)));
    
    // Test BFS
    let bfs_order = graph.bfs(1);
    assert_eq!(bfs_order, vec![1, 2, 3]); // Expected BFS traversal
    
    // Test auto-creation of nodes when adding edges
    graph.add_edge(4, 5, 40).unwrap();
    assert_eq!(graph.get_node_value(4), Some(0)); // Default value
    assert_eq!(graph.get_node_value(5), Some(0)); // Default value
    
    // Test clear
    graph.clear();
    assert_eq!(graph.get_node_value(1), None);
    assert_eq!(graph.get_neighbors(1).len(), 0);
}

// Tests for OHLCVDS
#[test]
fn test_ohlcv_basic_operations() {
    let mut ohlcv = OHLCVDS::new();
    
    // Test is_empty and len for new instance
    assert!(ohlcv.is_empty());
    assert_eq!(ohlcv.len(), 0);
    
    // Create test entries
    let entry1 = OHLCVEntry {
        timestamp: 1000,
        open: 100,
        high: 110,
        low: 90,
        close: 105,
        volume: 1000,
    };
    
    let entry2 = OHLCVEntry {
        timestamp: 2000,
        open: 105,
        high: 120,
        low: 100,
        close: 115,
        volume: 1500,
    };
    
    let entry3 = OHLCVEntry {
        timestamp: 3000,
        open: 115,
        high: 130,
        low: 110,
        close: 125,
        volume: 2000,
    };
    
    // Test add_entry
    ohlcv.add_entry(entry2).unwrap(); // Add out of order
    ohlcv.add_entry(entry1).unwrap(); // Earlier timestamp
    ohlcv.add_entry(entry3).unwrap(); // Later timestamp
    
    // Test len after additions
    assert_eq!(ohlcv.len(), 3);
    
    // Test get_entry
    assert_eq!(ohlcv.get_entry(0), Some(entry1)); // Should be ordered by timestamp
    assert_eq!(ohlcv.get_entry(1), Some(entry2));
    assert_eq!(ohlcv.get_entry(2), Some(entry3));
    assert_eq!(ohlcv.get_entry(3), None); // Out of range
    
    // Test get_by_timestamp
    assert_eq!(ohlcv.get_by_timestamp(1000), Some(entry1));
    assert_eq!(ohlcv.get_by_timestamp(2000), Some(entry2));
    assert_eq!(ohlcv.get_by_timestamp(1500), None); // No entry at this timestamp
    
    // Test calculate_sma
    let sma = ohlcv.calculate_sma(2);
    assert_eq!(sma.len(), 2);
    assert_eq!(sma[0], (2000, (105 + 115) / 2)); // (entry1.close + entry2.close) / 2
    assert_eq!(sma[1], (3000, (115 + 125) / 2)); // (entry2.close + entry3.close) / 2
    
    // Test SMA with period larger than data
    assert_eq!(ohlcv.calculate_sma(4).len(), 0);
    
    // Test clear
    ohlcv.clear();
    assert!(ohlcv.is_empty());
    assert_eq!(ohlcv.len(), 0);
}

// Tests for HypergraphDS
#[test]
fn test_hypergraph_basic_operations() {
    let mut hypergraph = HypergraphDS::new();
    
    // Test add_node
    hypergraph.add_node(1, 100).unwrap();
    hypergraph.add_node(2, 200).unwrap();
    hypergraph.add_node(3, 300).unwrap();
    hypergraph.add_node(4, 400).unwrap();
    
    // Test create_hyperedge
    hypergraph.create_hyperedge(101, 10).unwrap();
    hypergraph.create_hyperedge(102, 20).unwrap();
    
    // Test add_node_to_edge
    hypergraph.add_node_to_edge(101, 1).unwrap();
    hypergraph.add_node_to_edge(101, 2).unwrap();
    hypergraph.add_node_to_edge(101, 3).unwrap();
    
    hypergraph.add_node_to_edge(102, 2).unwrap();
    hypergraph.add_node_to_edge(102, 3).unwrap();
    hypergraph.add_node_to_edge(102, 4).unwrap();
    
    // Test get_nodes_in_edge
    let nodes_in_edge_101 = hypergraph.get_nodes_in_edge(101).unwrap();
    assert_eq!(nodes_in_edge_101.len(), 3);
    assert!(nodes_in_edge_101.contains(&1));
    assert!(nodes_in_edge_101.contains(&2));
    assert!(nodes_in_edge_101.contains(&3));
    
    let nodes_in_edge_102 = hypergraph.get_nodes_in_edge(102).unwrap();
    assert_eq!(nodes_in_edge_102.len(), 3);
    assert!(nodes_in_edge_102.contains(&2));
    assert!(nodes_in_edge_102.contains(&3));
    assert!(nodes_in_edge_102.contains(&4));
    
    // Test get_edges_containing_node
    let edges_with_node_1 = hypergraph.get_edges_containing_node(1);
    assert_eq!(edges_with_node_1.len(), 1);
    assert!(edges_with_node_1.contains(&101));
    
    let edges_with_node_3 = hypergraph.get_edges_containing_node(3);
    assert_eq!(edges_with_node_3.len(), 2);
    assert!(edges_with_node_3.contains(&101));
    assert!(edges_with_node_3.contains(&102));
    
    // Test auto-creation when adding to non-existent edge/node
    hypergraph.add_node_to_edge(103, 5).unwrap(); // New edge and new node
    assert!(hypergraph.get_nodes_in_edge(103).unwrap().contains(&5));
    
    // Test clear
    hypergraph.clear();
    assert_eq!(hypergraph.get_nodes_in_edge(101), None);
    assert_eq!(hypergraph.get_edges_containing_node(1).len(), 0);
}