//! Advanced data structures for LessVM
//! 
//! This module implements several advanced data structures:
//! - BTreeMap: An ordered map with O(log n) operations
//! - Trie: A prefix tree for efficient string operations
//! - Graph: A directed graph for relationship modeling
//! - OHLCV: Open-High-Low-Close-Volume for time series financial data
//! - Hypergraph: A graph where edges can connect multiple vertices

use super::{VMError, Value};
use std::collections::{BTreeMap as StdBTreeMap, HashMap, HashSet};
use std::collections::VecDeque;
use solana_program::msg;

// Constants for data structure IDs
pub const MAX_DATA_STRUCTURES: usize = 32;
pub const MAX_GRAPH_NODES: usize = 1024; // Maximum number of nodes in a graph

/// The type of data structure
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DataStructureType {
    BTreeMap = 0,
    Trie = 1,
    Graph = 2,
    OHLCV = 3,
    Hypergraph = 4,
}

impl DataStructureType {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(DataStructureType::BTreeMap),
            1 => Some(DataStructureType::Trie),
            2 => Some(DataStructureType::Graph),
            3 => Some(DataStructureType::OHLCV),
            4 => Some(DataStructureType::Hypergraph),
            _ => None,
        }
    }
}

/// An OHLCV entry represents price and volume data for a time period
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OHLCVEntry {
    pub timestamp: u64,
    pub open: u64,
    pub high: u64,
    pub low: u64,
    pub close: u64,
    pub volume: u64,
}

/// A node in a trie structure
#[derive(Debug, Clone)]
pub struct TrieNode {
    pub children: HashMap<u8, usize>, // Map from character to child node index
    pub is_end_of_word: bool,
    pub value: Option<Value>,
}

/// BTreeMap implementation
#[derive(Debug, Clone)]
pub struct BTreeMapDS {
    data: StdBTreeMap<u64, u64>,
}

impl BTreeMapDS {
    pub fn new() -> Self {
        Self {
            data: StdBTreeMap::new(),
        }
    }

    pub fn insert(&mut self, key: u64, value: u64) -> Option<u64> {
        self.data.insert(key, value)
    }

    pub fn get(&self, key: u64) -> Option<u64> {
        self.data.get(&key).copied()
    }

    pub fn remove(&mut self, key: u64) -> Option<u64> {
        self.data.remove(&key)
    }

    pub fn contains_key(&self, key: u64) -> bool {
        self.data.contains_key(&key)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn first_key(&self) -> Option<u64> {
        self.data.keys().next().copied()
    }

    pub fn last_key(&self) -> Option<u64> {
        self.data.keys().next_back().copied()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }
}

/// Trie implementation
#[derive(Debug, Clone)]
pub struct TrieDS {
    nodes: Vec<TrieNode>,
    root: usize,
}

impl TrieDS {
    pub fn new() -> Self {
        let root_node = TrieNode {
            children: HashMap::new(),
            is_end_of_word: false,
            value: None,
        };

        let mut nodes = Vec::new();
        nodes.push(root_node);

        Self {
            nodes,
            root: 0,
        }
    }

    pub fn insert(&mut self, key: &[u8], value: Value) -> Result<(), VMError> {
        let mut current_node = self.root;

        for &byte in key {
            let next_node = match self.nodes[current_node].children.get(&byte) {
                Some(&next) => next,
                None => {
                    // Create new node
                    let new_node = TrieNode {
                        children: HashMap::new(),
                        is_end_of_word: false,
                        value: None,
                    };
                    let new_index = self.nodes.len();
                    self.nodes.push(new_node);
                    
                    // Link from parent
                    self.nodes[current_node].children.insert(byte, new_index);
                    new_index
                }
            };
            current_node = next_node;
        }

        self.nodes[current_node].is_end_of_word = true;
        self.nodes[current_node].value = Some(value);

        Ok(())
    }

    pub fn get(&self, key: &[u8]) -> Option<Value> {
        let mut current_node = self.root;

        for &byte in key {
            match self.nodes[current_node].children.get(&byte) {
                Some(&next) => current_node = next,
                None => return None,
            }
        }

        if self.nodes[current_node].is_end_of_word {
            self.nodes[current_node].value
        } else {
            None
        }
    }

    pub fn contains(&self, key: &[u8]) -> bool {
        self.get(key).is_some()
    }

    pub fn clear(&mut self) {
        self.nodes.clear();
        let root_node = TrieNode {
            children: HashMap::new(),
            is_end_of_word: false,
            value: None,
        };
        self.nodes.push(root_node);
        self.root = 0;
    }
}

/// Graph implementation - directed graph with edge weights
#[derive(Debug, Clone)]
pub struct GraphDS {
    // Adjacency list: map from node ID to list of (target node, weight) pairs
    edges: HashMap<u64, Vec<(u64, u64)>>, 
    node_values: HashMap<u64, u64>,
}

impl GraphDS {
    pub fn new() -> Self {
        Self {
            edges: HashMap::new(),
            node_values: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node_id: u64, value: u64) -> Result<(), VMError> {
        if self.node_values.len() >= MAX_GRAPH_NODES {
            return Err(VMError::StackOverflow); // Reuse error for memory limit
        }
        
        self.node_values.insert(node_id, value);
        if !self.edges.contains_key(&node_id) {
            self.edges.insert(node_id, Vec::new());
        }
        
        Ok(())
    }

    pub fn add_edge(&mut self, from: u64, to: u64, weight: u64) -> Result<(), VMError> {
        // Create nodes if they don't exist
        if !self.node_values.contains_key(&from) {
            self.add_node(from, 0)?;
        }
        
        if !self.node_values.contains_key(&to) {
            self.add_node(to, 0)?;
        }
        
        // Add edge
        self.edges.entry(from)
            .or_insert_with(Vec::new)
            .push((to, weight));
        
        Ok(())
    }

    pub fn get_node_value(&self, node_id: u64) -> Option<u64> {
        self.node_values.get(&node_id).copied()
    }

    pub fn set_node_value(&mut self, node_id: u64, value: u64) -> Result<(), VMError> {
        if !self.node_values.contains_key(&node_id) {
            return Err(VMError::InvalidMemoryAccess); // Node doesn't exist
        }
        
        self.node_values.insert(node_id, value);
        Ok(())
    }

    pub fn get_neighbors(&self, node_id: u64) -> Vec<(u64, u64)> {
        self.edges.get(&node_id)
            .cloned()
            .unwrap_or_default()
    }

    pub fn has_edge(&self, from: u64, to: u64) -> bool {
        self.edges.get(&from)
            .map(|edges| edges.iter().any(|(t, _)| *t == to))
            .unwrap_or(false)
    }

    pub fn get_edge_weight(&self, from: u64, to: u64) -> Option<u64> {
        self.edges.get(&from)
            .and_then(|edges| edges.iter()
                .find(|(t, _)| *t == to)
                .map(|(_, w)| *w))
    }

    // BFS traversal - returns nodes in BFS order
    pub fn bfs(&self, start: u64) -> Vec<u64> {
        let mut result = Vec::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        
        if !self.node_values.contains_key(&start) {
            return result;
        }
        
        queue.push_back(start);
        visited.insert(start);
        
        while let Some(node) = queue.pop_front() {
            result.push(node);
            
            if let Some(neighbors) = self.edges.get(&node) {
                for &(neighbor, _) in neighbors {
                    if !visited.contains(&neighbor) {
                        visited.insert(neighbor);
                        queue.push_back(neighbor);
                    }
                }
            }
        }
        
        result
    }

    pub fn clear(&mut self) {
        self.edges.clear();
        self.node_values.clear();
    }
}

/// OHLCV implementation for financial data
#[derive(Debug, Clone)]
pub struct OHLCVDS {
    data: Vec<OHLCVEntry>,
}

impl OHLCVDS {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, entry: OHLCVEntry) -> Result<(), VMError> {
        // Typically OHLCV data should be sorted by timestamp
        // Find the right position to insert
        let pos = self.data.binary_search_by_key(&entry.timestamp, |e| e.timestamp)
            .unwrap_or_else(|e| e);
            
        self.data.insert(pos, entry);
        Ok(())
    }

    pub fn get_entry(&self, index: usize) -> Option<OHLCVEntry> {
        self.data.get(index).cloned()
    }

    pub fn get_by_timestamp(&self, timestamp: u64) -> Option<OHLCVEntry> {
        self.data.binary_search_by_key(&timestamp, |e| e.timestamp)
            .ok()
            .map(|idx| self.data[idx])
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Calculate Simple Moving Average (SMA) of close prices
    pub fn calculate_sma(&self, period: usize) -> Vec<(u64, u64)> {
        if period == 0 || self.data.len() < period {
            return Vec::new();
        }

        let mut result = Vec::new();
        let mut sum = 0;

        // Initialize sum with first 'period' entries
        for i in 0..period {
            sum += self.data[i].close;
        }

        // Add first SMA
        result.push((self.data[period-1].timestamp, sum / period as u64));

        // Calculate remaining SMAs
        for i in period..self.data.len() {
            sum = sum + self.data[i].close - self.data[i-period].close;
            result.push((self.data[i].timestamp, sum / period as u64));
        }

        result
    }
}

/// Hypergraph implementation
#[derive(Debug, Clone)]
pub struct HypergraphDS {
    // Map from hyperedge ID to the set of nodes it connects
    edges: HashMap<u64, HashSet<u64>>,
    // Map from node ID to value
    node_values: HashMap<u64, u64>,
    // Map from hyperedge ID to edge weight
    edge_weights: HashMap<u64, u64>,
}

impl HypergraphDS {
    pub fn new() -> Self {
        Self {
            edges: HashMap::new(),
            node_values: HashMap::new(),
            edge_weights: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node_id: u64, value: u64) -> Result<(), VMError> {
        self.node_values.insert(node_id, value);
        Ok(())
    }

    pub fn create_hyperedge(&mut self, edge_id: u64, weight: u64) -> Result<(), VMError> {
        self.edges.insert(edge_id, HashSet::new());
        self.edge_weights.insert(edge_id, weight);
        Ok(())
    }

    pub fn add_node_to_edge(&mut self, edge_id: u64, node_id: u64) -> Result<(), VMError> {
        // Create node if it doesn't exist
        if !self.node_values.contains_key(&node_id) {
            self.add_node(node_id, 0)?;
        }
        
        // Create edge if it doesn't exist
        if !self.edges.contains_key(&edge_id) {
            self.create_hyperedge(edge_id, 1)?;
        }
        
        // Add node to edge
        self.edges.get_mut(&edge_id).unwrap().insert(node_id);
        Ok(())
    }

    pub fn get_nodes_in_edge(&self, edge_id: u64) -> Option<Vec<u64>> {
        self.edges.get(&edge_id)
            .map(|nodes| nodes.iter().copied().collect())
    }

    pub fn get_edges_containing_node(&self, node_id: u64) -> Vec<u64> {
        self.edges.iter()
            .filter_map(|(&edge_id, nodes)| {
                if nodes.contains(&node_id) {
                    Some(edge_id)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn clear(&mut self) {
        self.edges.clear();
        self.node_values.clear();
        self.edge_weights.clear();
    }
}