import { SonicAgentKit } from '@sendaifun/sonic-agent-kit';
import { SolanaAgentKit } from '@sendaifun/solana-agent-kit';
import { PublicKey } from '@solana/web3.js';

export type BlockchainType = 'solana' | 'sonic';

export interface WalletConfig {
  privateKey: string;
  rpcUrl: string;
  openaiApiKey?: string;
}

export interface AgentServiceOptions {
  solanaConfig?: WalletConfig;
  sonicConfig?: WalletConfig;
}

/**
 * AgentService provides a unified interface for interacting with 
 * blockchain agent kits (Solana and Sonic)
 */
export class AgentService {
  private solanaAgent: SolanaAgentKit | null = null;
  private sonicAgent: SonicAgentKit | null = null;

  constructor(options: AgentServiceOptions = {}) {
    if (options.solanaConfig) {
      this.initSolanaAgent(options.solanaConfig);
    }
    
    if (options.sonicConfig) {
      this.initSonicAgent(options.sonicConfig);
    }
  }

  /**
   * Initialize the Solana agent with the provided configuration
   */
  initSolanaAgent(config: WalletConfig): void {
    try {
      this.solanaAgent = new SolanaAgentKit(
        config.privateKey,
        config.rpcUrl,
        { OPENAI_API_KEY: config.openaiApiKey || '' }
      );
      console.log('Solana agent initialized successfully');
    } catch (error) {
      console.error('Failed to initialize Solana agent:', error);
      throw new Error(`Failed to initialize Solana agent: ${error instanceof Error ? error.message : String(error)}`);
    }
  }

  /**
   * Initialize the Sonic agent with the provided configuration
   */
  initSonicAgent(config: WalletConfig): void {
    try {
      this.sonicAgent = new SonicAgentKit(
        config.privateKey,
        config.rpcUrl,
        { OPENAI_API_KEY: config.openaiApiKey || '' }
      );
      console.log('Sonic agent initialized successfully');
    } catch (error) {
      console.error('Failed to initialize Sonic agent:', error);
      throw new Error(`Failed to initialize Sonic agent: ${error instanceof Error ? error.message : String(error)}`);
    }
  }

  /**
   * Check if the specified agent is initialized
   */
  isAgentInitialized(type: BlockchainType): boolean {
    return type === 'solana' ? !!this.solanaAgent : !!this.sonicAgent;
  }

  /**
   * Get the wallet address for the specified blockchain
   */
  getWalletAddress(type: BlockchainType): string | null {
    try {
      if (type === 'solana' && this.solanaAgent) {
        return this.solanaAgent.wallet_address.toString();
      } else if (type === 'sonic' && this.sonicAgent) {
        return this.sonicAgent.wallet_address.toString();
      }
      return null;
    } catch (error) {
      console.error(`Failed to get wallet address for ${type}:`, error);
      return null;
    }
  }

  /**
   * Get the balance for the specified blockchain
   */
  async getBalance(type: BlockchainType, tokenAddress?: string): Promise<number | null> {
    try {
      if (type === 'solana' && this.solanaAgent) {
        return await this.solanaAgent.getBalance(
          tokenAddress ? new PublicKey(tokenAddress) : undefined
        );
      } else if (type === 'sonic' && this.sonicAgent) {
        return await this.sonicAgent.getBalance(
          tokenAddress ? new PublicKey(tokenAddress) : undefined
        );
      }
      return null;
    } catch (error) {
      console.error(`Failed to get balance for ${type}:`, error);
      return null;
    }
  }

  /**
   * Send a token transfer for the specified blockchain
   */
  async transfer(
    type: BlockchainType,
    recipientAddress: string,
    amount: number,
    tokenAddress?: string
  ): Promise<string | null> {
    try {
      const recipient = new PublicKey(recipientAddress);
      
      if (type === 'solana' && this.solanaAgent) {
        return await this.solanaAgent.transfer(
          recipient,
          amount,
          tokenAddress ? new PublicKey(tokenAddress) : undefined
        );
      } else if (type === 'sonic' && this.sonicAgent) {
        return await this.sonicAgent.transfer(
          recipient,
          amount,
          tokenAddress ? new PublicKey(tokenAddress) : undefined
        );
      }
      return null;
    } catch (error) {
      console.error(`Failed to transfer tokens for ${type}:`, error);
      return null;
    }
  }

  /**
   * Get the Solana agent instance
   */
  getSolanaAgent(): SolanaAgentKit | null {
    return this.solanaAgent;
  }

  /**
   * Get the Sonic agent instance
   */
  getSonicAgent(): SonicAgentKit | null {
    return this.sonicAgent;
  }
}

// Create a singleton instance
export const agentService = new AgentService();

export default agentService;
