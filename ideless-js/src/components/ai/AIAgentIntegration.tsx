import React, { useState, useEffect } from 'react';
import { useSelector, useDispatch } from 'react-redux';
import { RootState } from '../../store';
import { PublicKey } from '@solana/web3.js';
import { FiSend, FiCopy, FiExternalLink } from 'react-icons/fi';
import { agentService, BlockchainType } from '../../services/agent/AgentService';
import { 
  setBalance, 
  addTransaction, 
  Transaction,
  updateTransactionStatus 
} from '../../store/slices/agentSlice';

interface AIAgentIntegrationProps {
  onSendMessage: (message: string) => void;
}

const AIAgentIntegration: React.FC<AIAgentIntegrationProps> = ({ onSendMessage }) => {
  const dispatch = useDispatch();
  const [command, setCommand] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const [blockchainResponse, setBlockchainResponse] = useState<string | null>(null);
  const [selectedBlockchain, setSelectedBlockchain] = useState<BlockchainType>('solana');
  const [error, setError] = useState<string | null>(null);
  
  const { balances, isInitialized, transactions } = useSelector((state: RootState) => state.agent);
  const isAgentInitialized = isInitialized[selectedBlockchain];
  
  // Generate a unique ID for transactions
  const generateTransactionId = () => {
    return `tx-${Date.now()}-${Math.random().toString(36).substring(2, 9)}`;
  };

  // Function to detect blockchain commands
  const parseCommand = (input: string): { command: string; args: string[] } | null => {
    // Basic command format: /command arg1 arg2 ...
    const match = input.match(/^\/(\w+)\s*(.*)/);
    if (!match) return null;
    
    const command = match[1].toLowerCase();
    const argsString = match[2].trim();
    const args = argsString ? argsString.split(/\s+/) : [];
    
    return { command, args };
  };

  // Execute blockchain command
  const executeCommand = async (parsedCommand: { command: string; args: string[] }) => {
    const { command, args } = parsedCommand;
    setIsLoading(true);
    setError(null);
    
    try {
      let response: string;
      
      switch (command) {
        case 'balance': {
          const tokenAddress = args.length > 0 ? args[0] : undefined;
          const balance = await agentService.getBalance(selectedBlockchain, tokenAddress);
          response = `Balance: ${balance} ${tokenAddress ? 'tokens' : selectedBlockchain === 'solana' ? 'SOL' : 'SONIC'}`;
          
          // Update balance in Redux
          dispatch(setBalance({ 
            blockchain: selectedBlockchain, 
            balance: balance || 0 
          }));
          break;
        }
        
        case 'transfer': {
          if (args.length < 2) {
            throw new Error('Missing required arguments: /transfer <recipient_address> <amount> [token_address]');
          }
          
          const recipientAddress = args[0];
          const amount = parseFloat(args[1]);
          const tokenAddress = args.length > 2 ? args[2] : undefined;
          
          if (isNaN(amount) || amount <= 0) {
            throw new Error('Invalid amount');
          }
          
          // Validate address
          try {
            new PublicKey(recipientAddress);
          } catch (e) {
            throw new Error('Invalid recipient address');
          }
          
          // Create transaction record
          const txId = generateTransactionId();
          const txRecord: Transaction = {
            id: txId,
            timestamp: Date.now(),
            type: 'transfer',
            amount,
            recipient: recipientAddress,
            blockchain: selectedBlockchain,
            status: 'pending',
            signature: '',
            details: tokenAddress ? { tokenAddress } : undefined
          };
          
          // Add transaction to Redux
          dispatch(addTransaction(txRecord));
          
          // Execute transfer
          const signature = await agentService.transfer(
            selectedBlockchain,
            recipientAddress,
            amount,
            tokenAddress
          );
          
          if (signature) {
            // Update transaction record
            dispatch(updateTransactionStatus({ 
              id: txId, 
              status: 'confirmed' 
            }));
            
            response = `Transfer successful! Transaction signature: ${signature}`;
            
            // Update balance after transfer
            const newBalance = await agentService.getBalance(selectedBlockchain);
            dispatch(setBalance({ 
              blockchain: selectedBlockchain, 
              balance: newBalance || 0 
            }));
          } else {
            dispatch(updateTransactionStatus({ 
              id: txId, 
              status: 'failed' 
            }));
            throw new Error('Transfer failed');
          }
          break;
        }
        
        case 'switch': {
          if (args.length < 1) {
            throw new Error('Missing blockchain name: /switch <solana|sonic>');
          }
          
          const blockchain = args[0].toLowerCase() as BlockchainType;
          if (blockchain !== 'solana' && blockchain !== 'sonic') {
            throw new Error('Invalid blockchain. Choose either "solana" or "sonic"');
          }
          
          if (!isInitialized[blockchain]) {
            throw new Error(`${blockchain} agent is not initialized`);
          }
          
          setSelectedBlockchain(blockchain);
          response = `Switched to ${blockchain} blockchain`;
          break;
        }
        
        case 'history': {
          if (transactions.length === 0) {
            response = 'No transaction history available';
          } else {
            const filteredTransactions = transactions
              .filter(tx => tx.blockchain === selectedBlockchain)
              .slice(0, 5); // Show only the last 5 transactions
              
            if (filteredTransactions.length === 0) {
              response = `No transactions found for ${selectedBlockchain}`;
            } else {
              const txList = filteredTransactions.map(tx => {
                const date = new Date(tx.timestamp).toLocaleString();
                const statusEmoji = tx.status === 'confirmed' ? '✅' : tx.status === 'pending' ? '⏳' : '❌';
                return `${statusEmoji} ${date} - ${tx.type} ${tx.amount} to ${tx.recipient?.substring(0, 8)}...`;
              }).join('\n');
              
              response = `Recent transactions for ${selectedBlockchain}:\n${txList}`;
            }
          }
          break;
        }
        
        case 'help': {
          response = `
Available commands:
/balance [token_address] - Check your balance
/transfer <recipient_address> <amount> [token_address] - Send tokens
/switch <solana|sonic> - Switch between blockchains
/history - View recent transactions
/help - Show this help menu
          `.trim();
          break;
        }
        
        default:
          throw new Error(`Unknown command: ${command}. Type /help for available commands.`);
      }
      
      setBlockchainResponse(response);
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'An unknown error occurred';
      setError(errorMessage);
      setBlockchainResponse(null);
    } finally {
      setIsLoading(false);
    }
  };

  // Handle command submission
  const handleSubmit = () => {
    if (!command.trim()) return;
    
    const parsedCommand = parseCommand(command);
    
    if (parsedCommand) {
      executeCommand(parsedCommand);
    } else {
      // If not a blockchain command, pass to the regular AI assistant
      onSendMessage(command);
      setCommand('');
    }
  };

  // Handle key press (Enter to send)
  const handleKeyPress = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      handleSubmit();
    }
  };

  // Effect to update balances when blockchain changes
  useEffect(() => {
    if (isAgentInitialized) {
      const fetchBalance = async () => {
        try {
          const balance = await agentService.getBalance(selectedBlockchain);
          dispatch(setBalance({ 
            blockchain: selectedBlockchain, 
            balance: balance || 0 
          }));
        } catch (error) {
          console.error('Failed to fetch balance:', error);
        }
      };
      
      fetchBalance();
    }
  }, [selectedBlockchain, isAgentInitialized, dispatch]);

  const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text);
  };

  return (
    <div className="blockchain-agent-panel p-3 bg-dark-800 rounded-lg border border-dark-600">
      <div className="agent-header flex items-center justify-between mb-3">
        <h3 className="text-primary-400 font-medium">Blockchain Agent</h3>
        <div className="flex space-x-2">
          <button
            onClick={() => setSelectedBlockchain('solana')}
            className={`px-2 py-1 rounded text-xs ${
              selectedBlockchain === 'solana' 
                ? 'bg-primary-600 text-white' 
                : 'bg-dark-700 text-gray-300'
            }`}
          >
            Solana
          </button>
          <button
            onClick={() => setSelectedBlockchain('sonic')}
            className={`px-2 py-1 rounded text-xs ${
              selectedBlockchain === 'sonic' 
                ? 'bg-primary-600 text-white' 
                : 'bg-dark-700 text-gray-300'
            }`}
          >
            Sonic
          </button>
        </div>
      </div>
      
      {/* Status Indicator */}
      <div className="status-card mb-3 p-2 bg-dark-700 rounded flex justify-between items-center">
        <div className="flex items-center">
          <div 
            className={`h-2 w-2 rounded-full mr-2 ${
              isAgentInitialized ? 'bg-green-500' : 'bg-red-500'
            }`} 
          />
          <span className="text-xs text-gray-300">
            {isAgentInitialized 
              ? `${selectedBlockchain} Connected` 
              : `${selectedBlockchain} Not Connected`}
          </span>
        </div>
        {isAgentInitialized && (
          <div className="text-xs text-gray-300">
            Balance: {balances[selectedBlockchain] || '0'} 
            {selectedBlockchain === 'solana' ? ' SOL' : ' SONIC'}
          </div>
        )}
      </div>
      
      {/* Response Area */}
      {(blockchainResponse || error) && (
        <div className={`response-area mb-3 p-3 rounded text-sm ${
          error ? 'bg-red-900/30 text-red-300' : 'bg-dark-700 text-gray-200'
        }`}>
          {error ? (
            <div className="error-message">
              <strong>Error:</strong> {error}
            </div>
          ) : (
            <div className="blockchain-response whitespace-pre-line">
              {blockchainResponse}
              {blockchainResponse && blockchainResponse.includes('signature') && (
                <div className="flex mt-2 space-x-2">
                  <button 
                    onClick={() => blockchainResponse && copyToClipboard(
                      blockchainResponse.split('signature: ')[1]
                    )}
                    className="text-primary-400 hover:text-primary-300"
                    title="Copy Transaction Signature"
                  >
                    <FiCopy size={14} />
                  </button>
                  <a 
                    href={`https://solscan.io/tx/${blockchainResponse?.split('signature: ')[1]}`}
                    target="_blank"
                    rel="noopener noreferrer"
                    className="text-primary-400 hover:text-primary-300"
                    title="View on Explorer"
                  >
                    <FiExternalLink size={14} />
                  </a>
                </div>
              )}
            </div>
          )}
        </div>
      )}
      
      {/* Command Input */}
      <div className="command-input flex items-center bg-dark-700 rounded-lg overflow-hidden">
        <input
          type="text"
          value={command}
          onChange={(e) => setCommand(e.target.value)}
          onKeyPress={handleKeyPress}
          placeholder={isAgentInitialized 
            ? "Type /help for available commands..." 
            : `${selectedBlockchain} agent is not initialized`}
          className="flex-1 bg-transparent border-none outline-none p-2 text-gray-200 text-sm"
          disabled={!isAgentInitialized || isLoading}
        />
        <button 
          onClick={handleSubmit}
          className="p-2 text-primary-400 hover:text-primary-300 transition-colors disabled:opacity-50"
          disabled={!command.trim() || !isAgentInitialized || isLoading}
        >
          <FiSend size={16} />
        </button>
      </div>
      
      {!isAgentInitialized && (
        <div className="mt-2 text-xs text-red-400">
          Please connect your wallet to use blockchain features
        </div>
      )}
    </div>
  );
};

export default AIAgentIntegration;
