## Visualization

```mermaid
graph TD
    VM --> Stack
    VM --> Memory
    VM --> GasMetering
    VM --> Debugger
    VM --> SecurityChecker
    VM --> SPLOperations
    Opcodes --> Stack
    Opcodes --> Memory
    Opcodes --> Accounts
    SecurityChecker --> Accounts
    SPLOperations --> SPLTokenTransfers
    VM --> Errors