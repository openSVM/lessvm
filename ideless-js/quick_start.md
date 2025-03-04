# LessVM IDELess Quick Start Guide

This guide will help you get started with IDELess, the integrated development environment for LessVM on Solana. Follow these steps to create, debug, simulate, and deploy your first LessVM program.

## Table of Contents

1. [Installation](#installation)
2. [Creating Your First Project](#creating-your-first-project)
3. [Understanding the Interface](#understanding-the-interface)
4. [Writing Your First Program](#writing-your-first-program)
5. [Debugging Your Program](#debugging-your-program)
6. [Simulating Execution](#simulating-execution)
7. [Deploying to Solana](#deploying-to-solana)
8. [Using the AI Assistant](#using-the-ai-assistant)
9. [Next Steps](#next-steps)

## Installation

Before starting, ensure you have IDELess installed on your system:

1. Download and install IDELess from [our official website](https://lessvm.org/ideless/download)
2. Follow the [Installation Guide](./installation_guide.md) for detailed instructions

## Creating Your First Project

Let's create a simple "Hello World" project to get started:

1. **Launch IDELess** from your applications menu or desktop shortcut
2. From the welcome screen, click **New Project**
3. Select **LessVM Project** from the template options
4. Enter a **Project Name** (e.g., "HelloWorld")
5. Choose a **Location** for your project
6. Click **Create Project**

IDELess will create a new project with the following structure:

```
HelloWorld/
├── src/
│   └── main.lvm       # Main program file
├── tests/
│   └── main.test.lvm  # Test file
└── lessvm.toml        # Project configuration
```

## Understanding the Interface

The IDELess interface consists of several key areas:

![IDELess Interface](./screenshots/interface_overview.png)

1. **Editor**: Central area where you write and edit code
2. **Explorer**: Left sidebar showing project files and structure
3. **Debugger**: Tools for debugging your program
4. **Simulator**: Tools for simulating program execution
5. **Deployment**: Tools for deploying to Solana
6. **AI Assistant**: Right sidebar providing context-aware assistance
7. **Terminal**: Bottom panel for command output and logs
8. **Status Bar**: Bottom bar showing program status and information

## Writing Your First Program

Let's write a simple program that adds two numbers:

1. Open `src/main.lvm` in the editor
2. Replace the default content with the following code:

```
// Simple addition program
function add(a: i32, b: i32) -> i32 {
    return a + b;
}

function main() {
    // Initialize variables
    let x: i32 = 5;
    let y: i32 = 7;
    
    // Call add function
    let result: i32 = add(x, y);
    
    // Print result
    print("Result: " + to_string(result));
}
```

3. Save the file with **Ctrl+S** (or **⌘+S** on macOS)

Notice how IDELess provides:
- Syntax highlighting for LessVM code
- Code completion as you type
- Real-time error checking
- Function parameter hints

## Debugging Your Program

Now let's debug our program to understand its execution:

1. Set a breakpoint by clicking in the gutter next to line 10 (the `let result` line)
2. Click the **Debug** button in the toolbar or press **F5**
3. The program will start and pause at your breakpoint
4. Examine the **Variables** panel to see the values of `x` and `y`
5. Use the debug controls to:
   - **Step Over** (F10): Execute the current line and move to the next
   - **Step Into** (F11): Enter a function call
   - **Step Out** (Shift+F11): Exit the current function
   - **Continue** (F5): Resume execution until the next breakpoint
6. Continue execution to see the final result

## Simulating Execution

The Simulator allows you to analyze program performance and behavior:

1. Click the **Simulator** tab in the right panel
2. Click **Run Simulation**
3. Observe the execution visualization showing:
   - Memory state changes
   - Stack operations
   - Gas usage
   - Execution path
4. Review the **Gas Analysis** to understand computational costs
5. Experiment with different inputs using the **Simulation Parameters** panel

## Deploying to Solana

Now let's deploy our program to the Solana blockchain:

1. Click the **Deploy** tab in the right panel
2. Select a network:
   - **Localnet**: For local testing (default)
   - **Devnet**: For development testing
   - **Testnet**: For pre-production testing
   - **Mainnet**: For production deployment
3. Ensure **OpenSVM RPC** is selected as the RPC provider
4. Configure your deployment:
   - **Program ID**: Generate a new one or use existing
   - **Payer**: Select your wallet for transaction fees
   - **Upgrade Authority**: Set who can upgrade the program
5. Click **Build & Deploy**
6. Monitor the deployment progress in the terminal
7. Once complete, you'll see the deployment details including:
   - Program ID
   - Transaction signature
   - Block explorer link

## Using the AI Assistant

The AI Assistant can help you learn and improve your code:

1. Click the **AI Assistant** tab in the right panel
2. Try these useful commands:
   - **Explain Code**: Select code and ask "Explain this code"
   - **Optimize**: Ask "How can I optimize this function?"
   - **Fix Errors**: When you have an error, ask "How do I fix this?"
   - **Learn Concepts**: Ask "Explain how memory works in LessVM"
   - **Generate Code**: Ask "Write a function to sort an array"

The AI Assistant is context-aware and understands your code, providing relevant suggestions and explanations.

## Next Steps

Congratulations! You've created, debugged, simulated, and deployed your first LessVM program. Here are some next steps to continue your journey:

1. **Explore Sample Projects**:
   - Go to **File > Open Sample** to explore example projects
   - Study different patterns and techniques

2. **Learn LessVM**:
   - Review the [LessVM Documentation](https://docs.lessvm.org)
   - Experiment with different opcodes and features
   - Use the AI Assistant to explain concepts

3. **Customize Your Environment**:
   - Explore **Settings** to customize the editor
   - Set up keyboard shortcuts for your workflow
   - Configure themes and layout

4. **Join the Community**:
   - Join our [Discord community](https://discord.gg/lessvm)
   - Share your projects and get feedback
   - Participate in challenges and hackathons

## Common Tasks Reference

### Project Management
- **New Project**: File > New Project
- **Open Project**: File > Open Project
- **Save All**: File > Save All
- **Close Project**: File > Close Project

### Editing
- **Find**: Ctrl+F (⌘+F on macOS)
- **Replace**: Ctrl+H (⌘+Option+F on macOS)
- **Format Document**: Alt+Shift+F (Option+Shift+F on macOS)
- **Toggle Comment**: Ctrl+/ (⌘+/ on macOS)
- **Code Completion**: Ctrl+Space (⌘+Space on macOS)

### Debugging
- **Start/Continue Debugging**: F5
- **Stop Debugging**: Shift+F5
- **Step Over**: F10
- **Step Into**: F11
- **Step Out**: Shift+F11
- **Toggle Breakpoint**: F9

### Simulation
- **Start Simulation**: Alt+S (Option+S on macOS)
- **Stop Simulation**: Alt+X (Option+X on macOS)
- **Reset Simulation**: Alt+R (Option+R on macOS)
- **Show Gas Analysis**: Alt+G (Option+G on macOS)

### Deployment
- **Build**: Ctrl+Shift+B (⌘+Shift+B on macOS)
- **Deploy**: Ctrl+Shift+D (⌘+Shift+D on macOS)
- **Verify Deployment**: Ctrl+Shift+V (⌘+Shift+V on macOS)

### AI Assistant
- **Show Assistant**: Alt+A (Option+A on macOS)
- **Explain Selection**: Alt+E (Option+E on macOS)
- **Optimize Selection**: Alt+O (Option+O on macOS)
- **Generate Code**: Alt+G (Option+G on macOS)

For a complete list of keyboard shortcuts, see the [Keyboard Shortcuts](./keyboard_shortcuts.md) document.

## Troubleshooting

If you encounter issues:

1. Check the **Terminal** panel for error messages
2. Consult the **Problems** panel for code issues
3. Use the AI Assistant for help with errors
4. Refer to the [Documentation](https://docs.lessvm.org/ideless)
5. Ask the community on [Discord](https://discord.gg/lessvm)

Happy coding with IDELess!