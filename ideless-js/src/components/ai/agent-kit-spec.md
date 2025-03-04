# Agent Kit Integration Specification

## Overview
This document outlines the integration of Sonic Agent Kit and Solana Agent Kit into the LessVM IDE's AI Assistant sidebar.

## Goals
- Enhance the AI Assistant with blockchain capabilities
- Allow users to interact with Sonic and Solana blockchains directly from the IDE
- Provide a seamless experience for LessVM developers working with blockchain technologies

## Implementation Details

### Dependencies
- Sonic Agent Kit: https://github.com/sendaifun/sonic-agent-kit
- Solana Agent Kit: https://github.com/sendaifun/solana-agent-kit

### Architecture

1. **Agent Service Layer**
   - Create a service that initializes and manages the agent kit instances
   - Handle connection configuration and wallet management
   - Provide a unified API for AI Assistant to interact with both blockchains

2. **UI Components**
   - Extend the AI Assistant interface to include blockchain operations
   - Add visual indicators for blockchain state (connection status, balance, etc.)
   - Create command suggestions specific to blockchain operations

3. **State Management**
   - Add new state slices for blockchain connections and operations
   - Track operation history and state for both Sonic and Solana networks
   - Persist necessary configuration securely

4. **Message Processing**
   - Detect blockchain-related queries in user messages
   - Route commands to appropriate agent kit methods
   - Format and display responses from blockchain operations

## Features

### Common Features
- Wallet connection and management
- Balance checking
- Token transfers
- Transaction history viewing

### Sonic-specific Features
- Token deployment
- Collection management
- NFT minting operations
- Game-specific features (e.g., Rock-Paper-Scissors)

### Solana-specific Features
- Domain name registration and resolution (SNS)
- DEX interactions (trading, liquidity provision)
- NFT marketplace operations
- DeFi operations (lending, staking)

## User Experience

### Command Recognition
The AI Assistant will recognize and suggest blockchain commands based on user input:
- Explicit commands (e.g., "Deploy a new token on Sonic")
- Natural language processing for intent detection
- Context-aware suggestions based on current workspace

### Response Formatting
- Transaction results will be formatted in a readable manner
- Links to block explorers for transactions
- Visual feedback for transaction status

## Security Considerations
- Wallet private keys will be stored securely
- User confirmation required for all transactions
- Clear warning for operations that cost gas/fees
- Option to use simulation mode for testing

## Future Extensions
- Support for additional blockchains
- Integration with smart contract development workflows
- Enhanced analytics for blockchain operations
