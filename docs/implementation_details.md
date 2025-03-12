# LessVM Implementation Details

This document provides details about the implementation of LessVM, focusing on recent changes and optimizations.

## Table of Contents

- [SIMD Vector Addition](#simd-vector-addition)
- [Data Structure Store](#data-structure-store)
- [Memory Management](#memory-management)
- [Opcode Implementations](#opcode-implementations)
  - [Solana Account Operations](#solana-account-operations)
  - [Graph Operations](#graph-operations)
  - [OHLCV Operations](#ohlcv-operations)
  - [Hypergraph Operations](#hypergraph-operations)

## SIMD Vector Addition

The `vector_add` function uses SIMD (Single Instruction, Multiple Data) instructions to efficiently add multiple values at once. The implementation loads two vectors from the stack and adds them together.

```mermaid
sequenceDiagram
    participant VM
    VM->>VM: _mm256_loadu_si256(stack[top] to values2)
    VM->>VM: stack.pop() x 4
    VM->>VM: _mm256_loadu_si256(stack[top] to values1)
    VM->>VM: result = _mm256_add_epi64(values1, values2)
    VM->>VM: stack.push(result) x 4
```

The function works as follows:
1. Load the second vector (values2) from the top of the stack
2. Pop the first 4 values from the stack
3. Load the first vector (values1) from the new top of the stack
4. Add the two vectors together using SIMD instructions
5. Push the result back onto the stack

This approach ensures that we're adding two different vectors together, rather than adding a vector to itself.

## Data Structure Store

The `DataStructureStore` manages various data structures used by the VM, including BTreeMaps, Tries, Graphs, OHLCV, and Hypergraphs.

```mermaid
classDiagram
    class DataStructureStore {
        btrees: Vec<Option<BTreeMapDS>>
        tries: Vec<Option<TrieDS>>
        graphs: Vec<Option<GraphDS>>
        ohlcvs: Vec<Option<OHLCVDS>>
        hypergraphs: Vec<Option<HypergraphDS>>
        +new() DataStructureStore
        +ensure_capacity(ds_type: DataStructureType, id: usize) void
    }
    note for DataStructureStore "Stores different types of data structures."
```

The `ensure_capacity` method ensures that the vectors have enough capacity to store a data structure at a specific index. If the index is beyond the current length of the vector, the vector is resized to accommodate the new index.

```mermaid
sequenceDiagram
    participant VM
    participant DataStructureStore
    VM->>DataStructureStore: ensure_capacity(DataStructureType::BTreeMap, 5)
    alt Vector length < 6
        DataStructureStore->>DataStructureStore: resize_with(6, || None)
    else Vector length >= 6
        DataStructureStore->>DataStructureStore: Do nothing
    end
```

## Memory Management

The memory management system has been optimized to use a more efficient growth strategy. Instead of doubling the capacity when more space is needed, the new implementation grows the memory by 50% or to the required size, whichever is larger.

```mermaid
sequenceDiagram
    participant Memory
    participant Vec
    Memory->>Memory: ensure_capacity(required_size)
    alt required_size > data.len()
        Memory->>Memory: new_capacity = (data.len() * 3 / 2).max(required_size)
        Memory->>Vec: resize(new_capacity, 0)
    else required_size <= data.len()
        Memory->>Memory: Do nothing
    end
```

This approach reduces memory waste while still providing amortized constant-time operations.

## Opcode Implementations

### Solana Account Operations

The following Solana account operations have been implemented:

- `GetBalance`: Gets the balance (lamports) of an account
- `GetOwner`: Gets the owner of an account
- `IsWritable`: Checks if an account is writable
- `IsSigner`: Checks if an account is a signer

```mermaid
sequenceDiagram
    participant VM
    participant Account
    VM->>VM: Pop account_idx from stack
    VM->>VM: Check if account_idx is valid
    alt Valid account_idx
        VM->>Account: Get account information
        Account->>VM: Return account information
        VM->>VM: Push result to stack
    else Invalid account_idx
        VM->>VM: Return InvalidAccount error
    end
```

### Graph Operations

The following graph operations have been implemented:

- `GraphAddEdge`: Adds an edge to a graph
- `GraphGetNode`: Gets the value of a node
- `GraphSetNode`: Sets the value of a node
- `GraphGetNeighbors`: Gets the neighbors of a node
- `GraphBfs`: Performs a breadth-first search starting from a node
- `GraphClear`: Clears a graph

```mermaid
sequenceDiagram
    participant VM
    participant DataStructureStore
    participant GraphDS
    VM->>VM: Pop weight, to, from, and id from stack
    VM->>DataStructureStore: Access graph with id
    alt Graph exists
        DataStructureStore->>GraphDS: add_edge(from, to, weight)
        GraphDS-->>DataStructureStore: Result
    else Graph does not exist
        DataStructureStore-->>VM: Error: InvalidDataStructureOperation
    end
```

### OHLCV Operations

The following OHLCV (Open-High-Low-Close-Volume) operations have been implemented:

- `OhlcvAddBar`: Adds a bar to an OHLCV
- `OhlcvGetBar`: Gets a bar from an OHLCV
- `OhlcvSma`: Calculates the Simple Moving Average (SMA) of an OHLCV

```mermaid
sequenceDiagram
    participant VM
    participant DataStructureStore
    participant OHLCVDS
    VM->>VM: Pop timestamp, open, high, low, close, volume, and id from stack
    VM->>DataStructureStore: Access ohlcv with id
    alt OHLCV exists
        DataStructureStore->>OHLCVDS: add_entry(timestamp, open, high, low, close, volume)
        OHLCVDS-->>DataStructureStore: Result
    else OHLCV does not exist
        DataStructureStore-->>VM: Error: InvalidDataStructureOperation
    end
```

### Hypergraph Operations

The following hypergraph operations have been implemented:

- `HyperAddNode`: Adds a node to a hypergraph
- `HyperAddEdge`: Adds an edge to a hypergraph
- `HyperAddNodeToEdge`: Adds a node to an edge in a hypergraph

```mermaid
sequenceDiagram
    participant VM
    participant DataStructureStore
    participant HypergraphDS
    VM->>VM: Pop node_id, edge_id, and id from stack
    VM->>DataStructureStore: Access hypergraph with id
    alt Hypergraph exists
        DataStructureStore->>HypergraphDS: add_node_to_edge(edge_id, node_id)
        HypergraphDS-->>DataStructureStore: Result
    else Hypergraph does not exist
        DataStructureStore-->>VM: Error: InvalidDataStructureOperation
    end
```