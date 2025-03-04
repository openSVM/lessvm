# LessVM IDELess Keyboard Shortcuts

This document provides a comprehensive list of keyboard shortcuts for the LessVM IDELess development environment. Mastering these shortcuts will significantly enhance your productivity when developing LessVM programs on Solana.

## Table of Contents

- [General Shortcuts](#general-shortcuts)
- [Editor Shortcuts](#editor-shortcuts)
- [Debugger Shortcuts](#debugger-shortcuts)
- [Simulator Shortcuts](#simulator-shortcuts)
- [Deployment Shortcuts](#deployment-shortcuts)
- [AI Assistant Shortcuts](#ai-assistant-shortcuts)
- [Navigation Shortcuts](#navigation-shortcuts)
- [Panel Management Shortcuts](#panel-management-shortcuts)
- [Custom Shortcuts](#custom-shortcuts)
- [Platform-Specific Differences](#platform-specific-differences)

## General Shortcuts

### Application Control

| Shortcut | Action | Description |
|----------|--------|-------------|
| `Ctrl+N` / `⌘+N` | New File | Create a new file |
| `Ctrl+O` / `⌘+O` | Open File | Open an existing file |
| `Ctrl+S` / `⌘+S` | Save | Save the current file |
| `Ctrl+Shift+S` / `⌘+Shift+S` | Save As | Save the current file with a new name |
| `Ctrl+W` / `⌘+W` | Close Tab | Close the current tab |
| `Ctrl+Shift+W` / `⌘+Shift+W` | Close Window | Close the current window |
| `Ctrl+Q` / `⌘+Q` | Quit | Exit the application |
| `F11` | Toggle Fullscreen | Switch between fullscreen and windowed mode |
| `Ctrl+,` / `⌘+,` | Settings | Open settings panel |
| `Ctrl+Shift+P` / `⌘+Shift+P` | Command Palette | Open command palette |
| `Ctrl+R` / `⌘+R` | Reload Window | Reload the application window |
| `Ctrl+Shift+I` / `⌘+Option+I` | Developer Tools | Open developer tools for debugging the IDE itself |

### Project Management

| Shortcut | Action | Description |
|----------|--------|-------------|
| `Ctrl+Shift+N` / `⌘+Shift+N` | New Project | Create a new project |
| `Ctrl+Shift+O` / `⌘+Shift+O` | Open Project | Open an existing project |
| `Ctrl+Alt+C` / `⌘+Option+C` | Project Configuration | Open project configuration |
| `Ctrl+Shift+B` / `⌘+Shift+B` | Build Project | Build the current project |
| `Ctrl+Shift+E` / `⌘+Shift+E` | Export Project | Export the current project |
| `Ctrl+Alt+P` / `⌘+Option+P` | Project Properties | Show project properties |

## Editor Shortcuts

### Basic Editing

| Shortcut | Action | Description |
|----------|--------|-------------|
| `Ctrl+X` / `⌘+X` | Cut | Cut selected text |
| `Ctrl+C` / `⌘+C` | Copy | Copy selected text |
| `Ctrl+V` / `⌘+V` | Paste | Paste from clipboard |
| `Ctrl+Z` / `⌘+Z` | Undo | Undo last action |
| `Ctrl+Y` / `⌘+Shift+Z` | Redo | Redo last undone action |
| `Ctrl+A` / `⌘+A` | Select All | Select all text in the current file |
| `Ctrl+D` / `⌘+D` | Duplicate Line | Duplicate the current line |
| `Ctrl+Shift+K` / `⌘+Shift+K` | Delete Line | Delete the current line |
| `Alt+Up` / `Option+Up` | Move Line Up | Move the current line up |
| `Alt+Down` / `Option+Down` | Move Line Down | Move the current line down |
| `Ctrl+/` / `⌘+/` | Toggle Comment | Comment or uncomment the current line |
| `Tab` | Indent | Indent the current line |
| `Shift+Tab` | Outdent | Outdent the current line |
| `Ctrl+]` / `⌘+]` | Indent Line | Indent the current line |
| `Ctrl+[` / `⌘+[` | Outdent Line | Outdent the current line |
| `Ctrl+Enter` / `⌘+Enter` | Insert Line Below | Insert a new line below the current line |
| `Ctrl+Shift+Enter` / `⌘+Shift+Enter` | Insert Line Above | Insert a new line above the current line |

### Advanced Editing

| Shortcut | Action | Description |
|----------|--------|-------------|
| `Ctrl+F` / `⌘+F` | Find | Find text in the current file |
| `Ctrl+H` / `⌘+Option+F` | Replace | Find and replace text in the current file |
| `Ctrl+G` / `⌘+G` | Go to Line | Go to a specific line number |
| `Ctrl+Shift+F` / `⌘+Shift+F` | Find in Files | Find text across all project files |
| `Ctrl+Shift+H` / `⌘+Shift+Option+F` | Replace in Files | Find and replace text across all project files |
| `Alt+Click` / `Option+Click` | Multiple Cursors | Add a cursor at the clicked position |
| `Ctrl+Alt+Up` / `⌘+Option+Up` | Add Cursor Above | Add a cursor on the line above |
| `Ctrl+Alt+Down` / `⌘+Option+Down` | Add Cursor Below | Add a cursor on the line below |
| `Ctrl+U` / `⌘+U` | Undo Cursor | Undo the last cursor operation |
| `Ctrl+L` / `⌘+L` | Select Line | Select the current line |
| `Ctrl+Shift+L` / `⌘+Shift+L` | Select All Occurrences | Select all occurrences of the current selection |
| `Ctrl+F2` / `⌘+F2` | Select All Occurrences of Word | Select all occurrences of the current word |
| `Ctrl+Space` / `⌘+Space` | Trigger Suggestion | Show code completion suggestions |
| `Ctrl+Shift+Space` / `⌘+Shift+Space` | Trigger Parameter Hints | Show parameter hints |
| `Ctrl+K Ctrl+F` / `⌘+K ⌘+F` | Format Selection | Format the selected text |
| `Alt+F12` / `Option+F12` | Peek Definition | Show definition inline |
| `F12` | Go to Definition | Go to the definition of the symbol under cursor |
| `Shift+F12` / `Shift+F12` | Find References | Find all references to the symbol under cursor |
| `Ctrl+K Ctrl+X` / `⌘+K ⌘+X` | Trim Trailing Whitespace | Remove trailing whitespace |

### Code Folding

| Shortcut | Action | Description |
|----------|--------|-------------|
| `Ctrl+Shift+[` / `⌘+Option+[` | Fold | Fold the current region |
| `Ctrl+Shift+]` / `⌘+Option+]` | Unfold | Unfold the current region |
| `Ctrl+K Ctrl+0` / `⌘+K ⌘+0` | Fold All | Fold all regions |
| `Ctrl+K Ctrl+J` / `⌘+K ⌘+J` | Unfold All | Unfold all regions |
| `Ctrl+K Ctrl+1` / `⌘+K ⌘+1` | Fold Level 1 | Fold all regions of level 1 |
| `Ctrl+K Ctrl+2` / `⌘+K ⌘+2` | Fold Level 2 | Fold all regions of level 2 |
| `Ctrl+K Ctrl+3` / `⌘+K ⌘+3` | Fold Level 3 | Fold all regions of level 3 |
| `Ctrl+K Ctrl+[` / `⌘+K ⌘+[` | Fold Recursively | Fold the current region recursively |
| `Ctrl+K Ctrl+]` / `⌘+K ⌘+]` | Unfold Recursively | Unfold the current region recursively |

## Debugger Shortcuts

### Debugging Control

| Shortcut | Action | Description |
|----------|--------|-------------|
| `F5` | Start/Continue | Start debugging or continue execution |
| `Shift+F5` | Stop | Stop debugging |
| `Ctrl+Shift+F5` / `⌘+Shift+F5` | Restart | Restart debugging |
| `F6` | Pause | Pause execution |
| `F9` | Toggle Breakpoint | Set or remove a breakpoint at the current line |
| `F10` | Step Over | Execute the next line without stepping into functions |
| `F11` | Step Into | Step into the next function call |
| `Shift+F11` | Step Out | Step out of the current function |
| `Ctrl+F10` / `⌘+F10` | Run to Cursor | Run to the cursor position |
| `Ctrl+K Ctrl+I` / `⌘+K ⌘+I` | Show Hover | Show debug information for the symbol under cursor |

### Breakpoint Management

| Shortcut | Action | Description |
|----------|--------|-------------|
| `Ctrl+Shift+F9` / `⌘+Shift+F9` | Remove All Breakpoints | Clear all breakpoints |
| `Ctrl+F9` / `⌘+F9` | Enable/Disable Breakpoint | Toggle the enabled state of the current breakpoint |
| `Alt+F9` / `Option+F9` | Enable All Breakpoints | Enable all breakpoints |
| `Alt+Shift+F9` / `Option+Shift+F9` | Disable All Breakpoints | Disable all breakpoints |
| `Ctrl+Shift+B` / `⌘+Shift+B` | Edit Breakpoint | Edit the current breakpoint properties |
| `Ctrl+Alt+B` / `⌘+Option+B` | Add Conditional Breakpoint | Add a conditional breakpoint |
| `Ctrl+Alt+F9` / `⌘+Option+F9` | Add Logpoint | Add a logpoint |

### Debug Views

| Shortcut | Action | Description |
|----------|--------|-------------|
| `Ctrl+Shift+D` / `⌘+Shift+D` | Debug View | Show debug view |
| `Ctrl+Shift+Y` / `⌘+Shift+Y` | Debug Console | Show debug console |
| `Ctrl+Shift+V` / `⌘+Shift+V` | Variables | Show variables panel |
| `Ctrl+Shift+W` / `⌘+Shift+W` | Watch | Show watch panel |
| `Ctrl+Shift+A` / `⌘+Shift+A` | Call Stack | Show call stack panel |
| `Ctrl+Shift+M` / `⌘+Shift+M` | Memory | Show memory view |
| `Ctrl+Shift+R` / `⌘+Shift+R` | Registers | Show registers view |
| `Ctrl+Shift+K` / `⌘+Shift+K` | Breakpoints | Show breakpoints panel |

## Simulator Shortcuts

### Simulation Control

| Shortcut | Action | Description |
|----------|--------|-------------|
| `Ctrl+Alt+S` / `⌘+Option+S` | Start Simulation | Start the simulation |
| `Ctrl+Alt+X` / `⌘+Option+X` | Stop Simulation | Stop the simulation |
| `Ctrl+Alt+P` / `⌘+Option+P` | Pause Simulation | Pause the simulation |
| `Ctrl+Alt+C` / `⌘+Option+C` | Continue Simulation | Continue the paused simulation |
| `Ctrl+Alt+R` / `⌘+Option+R` | Reset Simulation | Reset the simulation to initial state |
| `Ctrl+Alt+F` / `⌘+Option+F` | Fast Forward | Increase simulation speed |
| `Ctrl+Alt+W` / `⌘+Option+W` | Slow Down | Decrease simulation speed |
| `Ctrl+Alt+N` / `⌘+Option+N` | Normal Speed | Reset to normal simulation speed |

### Simulation Views

| Shortcut | Action | Description |
|----------|--------|-------------|
| `Ctrl+Alt+G` / `⌘+Option+G` | Gas Analysis | Show gas usage analysis |
| `Ctrl+Alt+M` / `⌘+Option+M` | Memory View | Show memory state during simulation |
| `Ctrl+Alt+T` / `⌘+Option+T` | Stack View | Show stack state during simulation |
| `Ctrl+Alt+L` / `⌘+Option+L` | Log View | Show simulation logs |
| `Ctrl+Alt+V` / `⌘+Option+V` | Variables View | Show variable values during simulation |
| `Ctrl+Alt+H` / `⌘+Option+H` | Performance Heatmap | Show performance heatmap |

## Deployment Shortcuts

### Deployment Control

| Shortcut | Action | Description |
|----------|--------|-------------|
| `Ctrl+Shift+D` / `⌘+Shift+D` | Deploy | Deploy the current program |
| `Ctrl+Alt+D` / `⌘+Option+D` | Deployment Settings | Open deployment settings |
| `Ctrl+Shift+U` / `⌘+Shift+U` | Upgrade | Upgrade the deployed program |
| `Ctrl+Alt+V` / `⌘+Option+V` | Verify Deployment | Verify the deployed program |
| `Ctrl+Alt+N` / `⌘+Option+N` | Network Selection | Select deployment network |
| `Ctrl+Alt+K` / `⌘+Option+K` | Key Management | Open key management |
| `Ctrl+Alt+T` / `⌘+Option+T` | Transaction Builder | Open transaction builder |
| `Ctrl+Alt+L` / `⌘+Option+L` | Deployment Logs | Show deployment logs |

### Network Management

| Shortcut | Action | Description |
|----------|--------|-------------|
| `Ctrl+Alt+1` / `⌘+Option+1` | Local Network | Switch to local network |
| `Ctrl+Alt+2` / `⌘+Option+2` | Devnet | Switch to devnet |
| `Ctrl+Alt+3` / `⌘+Option+3` | Testnet | Switch to testnet |
| `Ctrl+Alt+4` / `⌘+Option+4` | Mainnet | Switch to mainnet |
| `Ctrl+Alt+C` / `⌘+Option+C` | Connection Status | Show connection status |
| `Ctrl+Alt+R` / `⌘+Option+R` | RPC Settings | Configure RPC settings |

## AI Assistant Shortcuts

### Assistant Interaction

| Shortcut | Action | Description |
|----------|--------|-------------|
| `Ctrl+Space` / `⌘+Space` | Trigger Suggestions | Show AI code suggestions |
| `Alt+/` / `Option+/` | Ask Assistant | Focus the AI assistant input |
| `Ctrl+Shift+A` / `⌘+Shift+A` | Assistant Panel | Toggle AI assistant panel |
| `Ctrl+Alt+E` / `⌘+Option+E` | Explain Code | Ask AI to explain selected code |
| `Ctrl+Alt+O` / `⌘+Option+O` | Optimize Code | Ask AI for optimization suggestions |
| `Ctrl+Alt+F` / `⌘+Option+F` | Fix Error | Ask AI to help fix the current error |
| `Ctrl+Alt+D` / `⌘+Option+D` | Document Code | Ask AI to generate documentation |
| `Ctrl+Alt+G` / `⌘+Option+G` | Generate Test | Ask AI to generate tests |
| `Ctrl+Alt+I` / `⌘+Option+I` | Implementation Suggestion | Ask AI for implementation suggestions |

### Learning Resources

| Shortcut | Action | Description |
|----------|--------|-------------|
| `Ctrl+Alt+L` / `⌘+Option+L` | Learn Concept | Ask AI to explain a concept |
| `Ctrl+Alt+T` / `⌘+Option+T` | Tutorial | Ask AI for a tutorial |
| `Ctrl+Alt+B` / `⌘+Option+B` | Best Practices | Ask AI for best practices |
| `Ctrl+Alt+P` / `⌘+Option+P` | Pattern Suggestion | Ask AI for design pattern suggestions |
| `Ctrl+Alt+S` / `⌘+Option+S` | Security Check | Ask AI to check for security issues |

## Navigation Shortcuts

### File Navigation

| Shortcut | Action | Description |
|----------|--------|-------------|
| `Ctrl+P` / `⌘+P` | Quick Open | Quickly open files |
| `Ctrl+Tab` / `⌘+Tab` | Next Tab | Switch to the next tab |
| `Ctrl+Shift+Tab` / `⌘+Shift+Tab` | Previous Tab | Switch to the previous tab |
| `Ctrl+PageDown` / `⌘+Option+Right` | Next Editor | Switch to the next editor |
| `Ctrl+PageUp` / `⌘+Option+Left` | Previous Editor | Switch to the previous editor |
| `Alt+Left` / `⌘+[` | Back | Navigate back |
| `Alt+Right` / `⌘+]` | Forward | Navigate forward |
| `Ctrl+G` / `⌘+G` | Go to Line | Go to a specific line |
| `Ctrl+T` / `⌘+T` | Go to Symbol | Go to a symbol in the current file |
| `Ctrl+Shift+O` / `⌘+Shift+O` | Go to Symbol in File | Go to a symbol in the current file |
| `Ctrl+Shift+M` / `⌘+Shift+M` | Problems Panel | Show problems panel |

### Workspace Navigation

| Shortcut | Action | Description |
|----------|--------|-------------|
| `Ctrl+0` / `⌘+0` | Focus Side Bar | Focus the side bar |
| `Ctrl+1` / `⌘+1` | Focus First Editor Group | Focus the first editor group |
| `Ctrl+2` / `⌘+2` | Focus Second Editor Group | Focus the second editor group |
| `Ctrl+3` / `⌘+3` | Focus Third Editor Group | Focus the third editor group |
| `Ctrl+Shift+E` / `⌘+Shift+E` | Explorer | Show explorer |
| `Ctrl+Shift+F` / `⌘+Shift+F` | Search | Show search |
| `Ctrl+Shift+G` / `⌘+Shift+G` | Source Control | Show source control |
| `Ctrl+Shift+D` / `⌘+Shift+D` | Debug | Show debug |
| `Ctrl+Shift+X` / `⌘+Shift+X` | Extensions | Show extensions |
| `Ctrl+Shift+Y` / `⌘+Shift+Y` | Debug Console | Show debug console |
| `Ctrl+Shift+U` / `⌘+Shift+U` | Output | Show output |
| `Ctrl+Shift+V` / `⌘+Shift+V` | Markdown Preview | Toggle Markdown preview |

## Panel Management Shortcuts

### Panel Control

| Shortcut | Action | Description |
|----------|--------|-------------|
| `Ctrl+J` / `⌘+J` | Toggle Panel | Toggle the visibility of the bottom panel |
| `Ctrl+B` / `⌘+B` | Toggle Sidebar | Toggle the visibility of the side bar |
| `Ctrl+Shift+B` / `⌘+Shift+B` | Toggle Activity Bar | Toggle the visibility of the activity bar |
| `Ctrl+\` / `⌘+\` | Split Editor | Split the editor |
| `Ctrl+K Ctrl+\` / `⌘+K ⌘+\` | Split Editor Orthogonal | Split the editor orthogonally |
| `Ctrl+W` / `⌘+W` | Close Editor | Close the current editor |
| `Ctrl+K W` / `⌘+K W` | Close All Editors | Close all editors |
| `Ctrl+K Ctrl+W` / `⌘+K ⌘+W` | Close All Editors | Close all editors |
| `Ctrl+K F` / `⌘+K F` | Close Folder | Close the current folder |
| `Ctrl+K M` / `⌘+K M` | Change Language Mode | Change the language mode of the current file |

### Layout Management

| Shortcut | Action | Description |
|----------|--------|-------------|
| `Ctrl+K Ctrl+Up` / `⌘+K ⌘+Up` | Focus Above Group | Focus the editor group above |
| `Ctrl+K Ctrl+Down` / `⌘+K ⌘+Down` | Focus Below Group | Focus the editor group below |
| `Ctrl+K Ctrl+Left` / `⌘+K ⌘+Left` | Focus Left Group | Focus the editor group to the left |
| `Ctrl+K Ctrl+Right` / `⌘+K ⌘+Right` | Focus Right Group | Focus the editor group to the right |
| `Ctrl+K Shift+Left` / `⌘+K Shift+Left` | Move Editor Left | Move the current editor to the left |
| `Ctrl+K Shift+Right` / `⌘+K Shift+Right` | Move Editor Right | Move the current editor to the right |
| `Ctrl+K Shift+Up` / `⌘+K Shift+Up` | Move Editor Up | Move the current editor up |
| `Ctrl+K Shift+Down` / `⌘+K Shift+Down` | Move Editor Down | Move the current editor down |
| `Ctrl+K Left` / `⌘+K Left` | Move Active Group Left | Move the active editor group left |
| `Ctrl+K Right` / `⌘+K Right` | Move Active Group Right | Move the active editor group right |
| `Ctrl+K Up` / `⌘+K Up` | Move Active Group Up | Move the active editor group up |
| `Ctrl+K Down` / `⌘+K Down` | Move Active Group Down | Move the active editor group down |

## Custom Shortcuts

IDELess allows you to customize keyboard shortcuts to match your preferences. To customize shortcuts:

1. Open Settings (`Ctrl+,` / `⌘+,`)
2. Navigate to "Keyboard Shortcuts"
3. Search for the command you want to customize
4. Click on the pencil icon to edit the shortcut
5. Press the desired key combination
6. Press Enter to save

### Common Custom Shortcut Patterns

| Task | Suggested Shortcut | Description |
|------|-------------------|-------------|
| Quick Build and Run | `Ctrl+R` / `⌘+R` | Build and run the current program |
| Format Document | `Alt+Shift+F` / `Option+Shift+F` | Format the current document |
| Toggle Terminal | `Ctrl+`` / `⌘+`` | Show/hide the terminal |
| Focus Editor | `Escape` | Focus the editor from any panel |
| Save All | `Ctrl+Alt+S` / `⌘+Option+S` | Save all open files |
| Close All But Current | `Ctrl+Alt+W` / `⌘+Option+W` | Close all editors except the current one |

## Platform-Specific Differences

### Windows/Linux vs. macOS

| Action | Windows/Linux | macOS |
|--------|--------------|-------|
| Copy | `Ctrl+C` | `⌘+C` |
| Paste | `Ctrl+V` | `⌘+V` |
| Cut | `Ctrl+X` | `⌘+X` |
| Undo | `Ctrl+Z` | `⌘+Z` |
| Redo | `Ctrl+Y` | `⌘+Shift+Z` |
| Save | `Ctrl+S` | `⌘+S` |
| Find | `Ctrl+F` | `⌘+F` |
| Replace | `Ctrl+H` | `⌘+Option+F` |
| Select All | `Ctrl+A` | `⌘+A` |
| New File | `Ctrl+N` | `⌘+N` |
| Open File | `Ctrl+O` | `⌘+O` |
| Close Tab | `Ctrl+W` | `⌘+W` |
| Settings | `Ctrl+,` | `⌘+,` |
| Command Palette | `Ctrl+Shift+P` | `⌘+Shift+P` |

### Key Symbol Legend

| Symbol | Key (Windows/Linux) | Key (macOS) |
|--------|---------------------|-------------|
| `Ctrl` | Control key | N/A |
| `Alt` | Alt key | N/A |
| `Shift` | Shift key | Shift key |
| `⌘` | N/A | Command key |
| `Option` | N/A | Option/Alt key |
| `⌃` | N/A | Control key |

## Tips for Efficient Keyboard Usage

1. **Learn incrementally**: Focus on a few shortcuts at a time until they become muscle memory.
2. **Use the Command Palette**: When you don't remember a shortcut, use `Ctrl+Shift+P` / `⌘+Shift+P` to search for commands.
3. **Customize for comfort**: Modify shortcuts to match your existing habits from other tools.
4. **Print a cheat sheet**: Keep a printed reference of your most-used shortcuts nearby.
5. **Practice deliberately**: Set aside time to practice using shortcuts instead of mouse actions.
6. **Use keyboard for navigation**: Try to navigate between files and panels using only the keyboard.
7. **Learn editor-specific shortcuts**: Pay special attention to code editing shortcuts for maximum productivity.
8. **Use multi-cursor editing**: Master the multi-cursor shortcuts for powerful text manipulation.
9. **Combine shortcuts**: Many powerful operations come from combining shortcuts in sequence.
10. **Observe experts**: Watch experienced users and note which shortcuts they use frequently.
