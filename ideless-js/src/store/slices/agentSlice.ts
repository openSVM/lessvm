import { createSlice, PayloadAction } from '@reduxjs/toolkit';
import { BlockchainType, WalletConfig } from '../../services/agent/AgentService';

export interface Transaction {
  id: string;
  timestamp: number;
  type: 'transfer' | 'deploy' | 'mint' | 'stake' | 'other';
  amount: number;
  recipient?: string;
  blockchain: BlockchainType;
  status: 'pending' | 'confirmed' | 'failed';
  signature: string;
  details?: Record<string, any>;
}

export interface AgentState {
  solanaConfig: WalletConfig | null;
  sonicConfig: WalletConfig | null;
  isInitialized: {
    solana: boolean;
    sonic: boolean;
  };
  activeBlockchain: BlockchainType | null;
  transactions: Transaction[];
  balances: {
    solana: number | null;
    sonic: number | null;
  };
}

const initialState: AgentState = {
  solanaConfig: null,
  sonicConfig: null,
  isInitialized: {
    solana: false,
    sonic: false,
  },
  activeBlockchain: null,
  transactions: [],
  balances: {
    solana: null,
    sonic: null,
  },
};

export const agentSlice = createSlice({
  name: 'agent',
  initialState,
  reducers: {
    setSolanaConfig: (state, action: PayloadAction<WalletConfig | null>) => {
      state.solanaConfig = action.payload;
    },
    setSonicConfig: (state, action: PayloadAction<WalletConfig | null>) => {
      state.sonicConfig = action.payload;
    },
    setInitialized: (
      state,
      action: PayloadAction<{ blockchain: BlockchainType; isInitialized: boolean }>
    ) => {
      const { blockchain, isInitialized } = action.payload;
      state.isInitialized[blockchain] = isInitialized;
    },
    setActiveBlockchain: (state, action: PayloadAction<BlockchainType | null>) => {
      state.activeBlockchain = action.payload;
    },
    addTransaction: (state, action: PayloadAction<Transaction>) => {
      state.transactions.unshift(action.payload);
    },
    updateTransactionStatus: (
      state,
      action: PayloadAction<{ id: string; status: Transaction['status'] }>
    ) => {
      const { id, status } = action.payload;
      const transaction = state.transactions.find(tx => tx.id === id);
      if (transaction) {
        transaction.status = status;
      }
    },
    setBalance: (
      state,
      action: PayloadAction<{ blockchain: BlockchainType; balance: number | null }>
    ) => {
      const { blockchain, balance } = action.payload;
      state.balances[blockchain] = balance;
    },
    resetAgent: () => initialState,
  },
});

export const {
  setSolanaConfig,
  setSonicConfig,
  setInitialized,
  setActiveBlockchain,
  addTransaction,
  updateTransactionStatus,
  setBalance,
  resetAgent,
} = agentSlice.actions;

export default agentSlice.reducer;
