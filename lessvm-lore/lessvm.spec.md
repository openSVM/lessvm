# Specification for LESS Virtual Machine (lessVM)

---

## Overview

The LESS Virtual Machine (lessVM) is a minimalistic virtual machine designed for executing custom bytecode on the Solana blockchain. It is equipped with a set of stack operations, arithmetic instructions, memory access functionalities, control flow mechanisms, and Solana-specific operations. This specification provides an in-depth look into the architecture, instruction set, execution model, and error handling mechanisms of lessVM.

---

## Architecture

### Core Components

1. **Stack**: A fixed-size stack with 32 slots, each capable of holding a 64-bit unsigned integer (`u64`). Operations include pushing, popping, duplicating, and swapping elements.
   
2. **Memory**: A byte array of size 1024 bytes, used for program data storage. Operations include loading and storing values to/from memory.

3. **Program Counter (PC)**: Initializes at 0, pointing to the current instruction. It increments by default with each instruction unless altered by control flow operations.

4. **Gas Counter**: Tracks executed instructions to prevent resource exhaustion, with a maximum limit of 200,000 units.

5. **Accounts**: References to Solana accounts, enabling interactions such as transferring lamports (Solana's unit of value).

---

## Instruction Set

### Stack Operations

| Opcode (Hex) | Operation | Description |
|--------------|-----------|-------------|
| `0x00`       | **Nop**   | No operation; increments PC by 1. |
| `0x01`       | **Push1** | Pushes an 8-bit value onto the stack. |
| `0x02`       | **Push2** | Pushes a 16-bit value onto the stack. |
| `0x03`       | **Push4** | Pushes a 32-bit value onto the stack. |
| `0x04`       | **Push8** | Pushes a 64-bit value onto the stack. |
| `0x05`       | **Pop**   | Removes the top element from the stack. |
| `0x06`       | **Dup**   | Duplicates the nth element from the top of the stack. |
| `0x07`       | **Swap**  | Swaps the top two elements of the stack. |

### Math Operations

| Opcode (Hex) | Operation | Description |
|--------------|-----------|-------------|
| `0x10`       | **Add**   | Adds the top two elements, pushes the result. |
| `0x11`       | **Sub**   | Subtracts the top element from the second top element, pushes the result. |
| `0x12`       | **Mul**   | Multiplies the top two elements, pushes the result. |
| `0x13`       | **Div**   | Divides the second top element by the top element, pushes the result. |
| `0x14`       | **MulDiv**| Multiplies the top two elements, divides by the third top element, pushes the result. |
| `0x15`       | **Min**   | Pushes the minimum of the top two elements. |
| `0x16`       | **Max**   | Pushes the maximum of the top two elements. |

### Memory Operations

| Opcode (Hex) | Operation | Description |
|--------------|-----------|-------------|
| `0x20`       | **Load**  | Loads an 8-byte value from memory at the given offset, pushes it onto the stack. |
| `0x21`       | **Store** | Stores an 8-byte value from the stack to memory at the given offset. |
| `0x22`       | **LoadN** | Loads a variable number of bytes from memory, specified by the top stack value. |
| `0x23`       | **StoreN**| Stores a variable number of bytes to memory, specified by the top stack value. |
| `0x24`       | **Msize** | Pushes the current memory size onto the stack. |

### Control Flow

| Opcode (Hex) | Operation | Description |
|--------------|-----------|-------------|
| `0x30`       | **Jump**  | Unconditionally jumps to the specified instruction address. |
| `0x31`       | **JumpI** | Jumps to the specified address if the top stack value is non-zero. |
| `0x32`       | **Call**  | Calls a subroutine at the specified address. |
| `0x33`       | **Return**| Returns from a subroutine. |

### Solana Operations

| Opcode (Hex) | Operation | Description |
|--------------|-----------|-------------|
| `0x40`       | **Transfer** | Transfers lamports from the current account to another account. |
| `0x41`       | **SPLOp** | Placeholder for Solana-specific operations (not detailed here). |
| `0x42`       | **CPI**   | Invokes a cross-program invoke (CPI) to another program. |
| `0x43`       | **Log**   | Logs a value from the stack to the transaction logs. |

### System Operations

| Opcode (Hex) | Operation | Description |
|--------------|-----------|-------------|
| `0xFF`       | **Halt**  | Halts execution and returns control to the caller. |

---

## Execution Model

1. **Instruction Fetching**: The VM begins execution at the program counter (`PC`), incrementing it after each instruction unless modified by control flow operations.

2. **Opcode Execution**: Each opcode is executed according to its defined behavior, with the stack and memory manipulated accordingly. Error checking ensures safe operations.

3. **Gas Accounting**: Each executed instruction increments the gas counter by 1. Execution halts upon reaching the gas limit of 200,000 units, returning an error.

---

## Memory Management

- **Memory Size**: Fixed at 1024 bytes.
- **Data Alignment**: Load and Store operations work with 8-byte aligned data.
- **Bounds Checking**: All memory operations include bounds checking to prevent out-of-bounds access.

---

## Error Handling

The VM returns custom error codes for various conditions:

| Error Code | Description |
|------------|-------------|
| 1          | Stack overflow. |
| 2          | Stack underflow. |
| 3          | Invalid stack index for `Dup` or `Swap`. |
| 4          | Memory access out of bounds. |
| 5          | Invalid instruction pointer jump. |
| 6          | Out of gas. |
| 7          | Invalid opcode. |
| 8          | Arithmetic overflow or underflow. |

---

## Security Considerations

- **Memory Safety**: All operations include bounds checking to prevent buffer overflows.
- **Stack Safety**: Operations ensure prevention of overflows and underflows.
- **Gas Limitation**: Enforces resource constraints to prevent infinite loops.

---

## Example Programs

### Simple Addition

```rust
let program = vec![
    OpCode::Push1 as u8, 5,  // Push integer 5 onto the stack
    OpCode::Push1 as u8, 3,  // Push integer 3 onto the stack
    OpCode::Add as u8,       // Add the top two elements (5 + 3 = 8)
    OpCode::Halt as u8       // Halt execution
];
```

### Token Transfer

```rust
let program = vec![
    OpCode::Push1 as u8, 1,          // Push target account index (1)
    OpCode::Push8 as u8, 50, 0, 0, 0, 0, 0, 0, 0,  // Push amount (50 lamports)
    OpCode::Transfer as u8,          // Transfer lamports from current account to account 1
    OpCode::Halt as u8               // Halt execution
];
```

---

## Conclusion

The LESS Virtual Machine offers a minimalist yet functional environment for executing custom logic on the Solana blockchain. By carefully following this specification, developers can build secure and efficient programs that interact seamlessly with Solana's ecosystem, utilizing its high-performance capabilities effectively.