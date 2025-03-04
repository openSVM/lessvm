# Agent Kit Implementation Guide

This guide outlines the steps needed to implement and integrate the Sonic Agent Kit and Solana Agent Kit into the LessVM IDE's AI Assistant.

## Prerequisites

- Node.js v16+ 
- npm or yarn package manager
- Access to Solana and/or Sonic testnets/mainnets

## Installation Steps

1. Install the required dependencies:

```bash
cd ideless-js
npm install @solana/web3.js @sendaifun/sonic-agent-kit @sendaifun/solana-agent-kit
```

Or if using yarn:

```bash
cd ideless-js
yarn add @solana/web3.js @sendaifun/sonic-agent-kit @sendaifun/solana-agent-kit
```

## Configuration

The implementation includes:

1. **AgentService**: A service layer that provides unified blockchain operations
2. **Redux Integration**: State management for blockchain connections and transactions
3. **UI Components**: Enhanced AI Assistant with blockchain agent capabilities

### Environment Variables (Optional)

You may want to provide default RPC URLs for both networks in your environment configuration:

```
SOLANA_RPC_URL=https://api.mainnet-beta.solana.com
SONIC_RPC_URL=https://api.testnet.sonic.game
```

## Usage

### Connecting Wallets

Users can connect their Solana and Sonic wallets by:

1. Clicking the settings icon (⚙️) in the AI Assistant panel
2. Entering their private key and RPC URL
3. Clicking "Connect Solana Wallet" or "Connect Sonic Wallet"

> **Security Note**: When implementing in production, consider using a more secure wallet connection method such as wallet adapters rather than directly entering private keys.

### Using Blockchain Commands

Once connected, users can use slash commands in the AI Assistant to interact with the blockchain:

- `/balance [token_address]` - Check wallet balance
- `/transfer <recipient_address> <amount> [token_address]` - Send tokens
- `/switch <solana|sonic>` - Switch between blockchains
- `/history` - View recent transactions
- `/help` - Show available commands

## Testing

The implementation includes test files for core functionality:

- `AgentService.test.ts` - Tests for the blockchain service layer

To run tests:

```bash
cd ideless-js
npm test
```

## Troubleshooting

### Common Issues

1. **Connection Errors**: Ensure RPC URLs are correct and accessible from the user's network
2. **Invalid Private Key**: Private keys must be in the correct format (base58 encoded string)
3. **Transaction Failures**: Check that the wallet has sufficient funds for the transaction

### Error Handling

The implementation includes error handling for common blockchain operations:
- Connection failures
- Insufficient funds
- Invalid addresses
- Transaction timeouts

## Future Enhancements

Potential areas for future development:

1. Secure wallet connection using wallet adapters
2. Advanced transaction support (NFTs, DeFi operations)
3. Cross-chain operations
4. Enhanced analytics for blockchain activities
5. Smart contract deployment and interaction

## Resources

- [Solana Documentation](https://docs.solana.com/)
- [Sonic Documentation](https://docs.sonic.game/)
- [Sonic Agent Kit Repository](https://github.com/sendaifun/sonic-agent-kit)
- [Solana Agent Kit Repository](https://github.com/sendaifun/solana-agent-kit)
