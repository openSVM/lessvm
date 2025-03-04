import { AgentService, BlockchainType, WalletConfig } from './AgentService';
import { PublicKey } from '@solana/web3.js';

// Mock the external dependencies
jest.mock('@sendaifun/sonic-agent-kit', () => {
  return {
    SonicAgentKit: jest.fn().mockImplementation((privateKey, rpcUrl) => {
      return {
        wallet_address: new PublicKey('8YLKoCu7NwqHNS8GzuvA3Vv5J1UNpaf1rnxJwjhUdQjN'),
        getBalance: jest.fn().mockResolvedValue(100),
        transfer: jest.fn().mockResolvedValue('mock-transaction-signature'),
      };
    }),
  };
});

jest.mock('@sendaifun/solana-agent-kit', () => {
  return {
    SolanaAgentKit: jest.fn().mockImplementation((privateKey, rpcUrl) => {
      return {
        wallet_address: new PublicKey('HU2RGX5XVqaVpPJHUW8MnJPqnd9EZB5A2BgW7RNFjDUJ'),
        getBalance: jest.fn().mockResolvedValue(200),
        transfer: jest.fn().mockResolvedValue('mock-solana-tx-signature'),
      };
    }),
  };
});

describe('AgentService', () => {
  let agentService: AgentService;
  const mockSolanaConfig: WalletConfig = {
    privateKey: 'mock-solana-private-key',
    rpcUrl: 'https://api.mainnet-beta.solana.com',
    openaiApiKey: 'mock-openai-api-key',
  };
  
  const mockSonicConfig: WalletConfig = {
    privateKey: 'mock-sonic-private-key',
    rpcUrl: 'https://api.testnet.sonic.game',
    openaiApiKey: 'mock-openai-api-key',
  };

  beforeEach(() => {
    // Reset mocks
    jest.clearAllMocks();
  });

  describe('initialization', () => {
    it('should initialize without configs', () => {
      agentService = new AgentService();
      expect(agentService.isAgentInitialized('solana')).toBe(false);
      expect(agentService.isAgentInitialized('sonic')).toBe(false);
    });

    it('should initialize with Solana config only', () => {
      agentService = new AgentService({ solanaConfig: mockSolanaConfig });
      expect(agentService.isAgentInitialized('solana')).toBe(true);
      expect(agentService.isAgentInitialized('sonic')).toBe(false);
    });

    it('should initialize with Sonic config only', () => {
      agentService = new AgentService({ sonicConfig: mockSonicConfig });
      expect(agentService.isAgentInitialized('solana')).toBe(false);
      expect(agentService.isAgentInitialized('sonic')).toBe(true);
    });

    it('should initialize with both configs', () => {
      agentService = new AgentService({
        solanaConfig: mockSolanaConfig,
        sonicConfig: mockSonicConfig,
      });
      expect(agentService.isAgentInitialized('solana')).toBe(true);
      expect(agentService.isAgentInitialized('sonic')).toBe(true);
    });
  });

  describe('wallet operations', () => {
    beforeEach(() => {
      agentService = new AgentService({
        solanaConfig: mockSolanaConfig,
        sonicConfig: mockSonicConfig,
      });
    });

    it('should get wallet address for Solana', () => {
      const address = agentService.getWalletAddress('solana');
      expect(address).toBe('HU2RGX5XVqaVpPJHUW8MnJPqnd9EZB5A2BgW7RNFjDUJ');
    });

    it('should get wallet address for Sonic', () => {
      const address = agentService.getWalletAddress('sonic');
      expect(address).toBe('8YLKoCu7NwqHNS8GzuvA3Vv5J1UNpaf1rnxJwjhUdQjN');
    });

    it('should return null for uninitialized agent', () => {
      agentService = new AgentService();
      const address = agentService.getWalletAddress('solana');
      expect(address).toBeNull();
    });
  });

  describe('balance operations', () => {
    beforeEach(() => {
      agentService = new AgentService({
        solanaConfig: mockSolanaConfig,
        sonicConfig: mockSonicConfig,
      });
    });

    it('should get balance for Solana', async () => {
      const balance = await agentService.getBalance('solana');
      expect(balance).toBe(200);
    });

    it('should get balance for Sonic', async () => {
      const balance = await agentService.getBalance('sonic');
      expect(balance).toBe(100);
    });

    it('should return null for uninitialized agent', async () => {
      agentService = new AgentService();
      const balance = await agentService.getBalance('solana');
      expect(balance).toBeNull();
    });
  });

  describe('transfer operations', () => {
    beforeEach(() => {
      agentService = new AgentService({
        solanaConfig: mockSolanaConfig,
        sonicConfig: mockSonicConfig,
      });
    });

    it('should transfer SOL on Solana', async () => {
      const txSig = await agentService.transfer(
        'solana',
        'HU2RGX5XVqaVpPJHUW8MnJPqnd9EZB5A2BgW7RNFjDUJ',
        1.0
      );
      expect(txSig).toBe('mock-solana-tx-signature');
    });

    it('should transfer tokens on Sonic', async () => {
      const txSig = await agentService.transfer(
        'sonic',
        '8YLKoCu7NwqHNS8GzuvA3Vv5J1UNpaf1rnxJwjhUdQjN',
        10.0
      );
      expect(txSig).toBe('mock-transaction-signature');
    });

    it('should return null for uninitialized agent', async () => {
      agentService = new AgentService();
      const txSig = await agentService.transfer(
        'solana',
        'HU2RGX5XVqaVpPJHUW8MnJPqnd9EZB5A2BgW7RNFjDUJ',
        1.0
      );
      expect(txSig).toBeNull();
    });
  });

  describe('agent instances', () => {
    beforeEach(() => {
      agentService = new AgentService({
        solanaConfig: mockSolanaConfig,
        sonicConfig: mockSonicConfig,
      });
    });

    it('should return Solana agent instance', () => {
      const agent = agentService.getSolanaAgent();
      expect(agent).not.toBeNull();
    });

    it('should return Sonic agent instance', () => {
      const agent = agentService.getSonicAgent();
      expect(agent).not.toBeNull();
    });

    it('should return null for uninitialized agent', () => {
      agentService = new AgentService();
      const solanaAgent = agentService.getSolanaAgent();
      const sonicAgent = agentService.getSonicAgent();
      expect(solanaAgent).toBeNull();
      expect(sonicAgent).toBeNull();
    });
  });
});
