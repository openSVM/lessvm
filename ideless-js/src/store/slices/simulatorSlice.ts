import { createSlice, PayloadAction } from '@reduxjs/toolkit';

export interface SimulationState {
  memory: Uint8Array;
  stack: bigint[];
  programCounter: number;
  gasUsed: number;
  gasLimit: number;
  running: boolean;
  paused: boolean;
  speed: 'slow' | 'normal' | 'fast';
  error: string | null;
  logs: { message: string; timestamp: number }[];
  executionHistory: {
    opcode: number;
    programCounter: number;
    gasUsed: number;
    timestamp: number;
  }[];
  accounts: {
    [address: string]: {
      lamports: number;
      owner: string;
      data: Uint8Array;
    };
  };
}

const initialState: SimulationState = {
  memory: new Uint8Array(1024), // 1KB of memory
  stack: [],
  programCounter: 0,
  gasUsed: 0,
  gasLimit: 1000000,
  running: false,
  paused: false,
  speed: 'normal',
  error: null,
  logs: [],
  executionHistory: [],
  accounts: {},
};

export const simulatorSlice = createSlice({
  name: 'simulator',
  initialState,
  reducers: {
    startSimulation: (state) => {
      state.running = true;
      state.paused = false;
      state.programCounter = 0;
      state.gasUsed = 0;
      state.error = null;
      state.logs = [];
      state.executionHistory = [];
    },
    pauseSimulation: (state) => {
      state.paused = true;
    },
    resumeSimulation: (state) => {
      state.paused = false;
    },
    stopSimulation: (state) => {
      state.running = false;
      state.paused = false;
    },
    setSimulationSpeed: (state, action: PayloadAction<'slow' | 'normal' | 'fast'>) => {
      state.speed = action.payload;
    },
    setMemory: (state, action: PayloadAction<{ memory: Uint8Array }>) => {
      state.memory = action.payload.memory;
    },
    updateMemoryAt: (state, action: PayloadAction<{ address: number; value: number }>) => {
      if (action.payload.address >= 0 && action.payload.address < state.memory.length) {
        state.memory[action.payload.address] = action.payload.value;
      }
    },
    setStack: (state, action: PayloadAction<{ stack: bigint[] }>) => {
      state.stack = action.payload.stack;
    },
    pushStack: (state, action: PayloadAction<{ value: bigint }>) => {
      state.stack.push(action.payload.value);
    },
    popStack: (state) => {
      state.stack.pop();
    },
    setProgramCounter: (state, action: PayloadAction<{ pc: number }>) => {
      state.programCounter = action.payload.pc;
    },
    incrementProgramCounter: (state) => {
      state.programCounter += 1;
    },
    setGasUsed: (state, action: PayloadAction<{ gas: number }>) => {
      state.gasUsed = action.payload.gas;
    },
    incrementGasUsed: (state, action: PayloadAction<{ amount: number }>) => {
      state.gasUsed += action.payload.amount;
    },
    setGasLimit: (state, action: PayloadAction<{ limit: number }>) => {
      state.gasLimit = action.payload.limit;
    },
    setError: (state, action: PayloadAction<{ error: string | null }>) => {
      state.error = action.payload.error;
    },
    addLog: (state, action: PayloadAction<{ message: string }>) => {
      state.logs.push({
        message: action.payload.message,
        timestamp: Date.now(),
      });
    },
    clearLogs: (state) => {
      state.logs = [];
    },
    addExecutionHistoryEntry: (state, action: PayloadAction<{ opcode: number; programCounter: number; gasUsed: number }>) => {
      state.executionHistory.push({
        ...action.payload,
        timestamp: Date.now(),
      });
    },
    clearExecutionHistory: (state) => {
      state.executionHistory = [];
    },
    setAccounts: (state, action: PayloadAction<{ accounts: SimulationState['accounts'] }>) => {
      state.accounts = action.payload.accounts;
    },
    updateAccount: (state, action: PayloadAction<{ address: string; lamports?: number; data?: Uint8Array }>) => {
      const { address, lamports, data } = action.payload;
      if (!state.accounts[address]) {
        state.accounts[address] = {
          lamports: 0,
          owner: '',
          data: new Uint8Array(0),
        };
      }
      
      if (lamports !== undefined) {
        state.accounts[address].lamports = lamports;
      }
      
      if (data !== undefined) {
        state.accounts[address].data = data;
      }
    },
    resetSimulator: (state) => {
      return initialState;
    },
  },
});

export const {
  startSimulation,
  pauseSimulation,
  resumeSimulation,
  stopSimulation,
  setSimulationSpeed,
  setMemory,
  updateMemoryAt,
  setStack,
  pushStack,
  popStack,
  setProgramCounter,
  incrementProgramCounter,
  setGasUsed,
  incrementGasUsed,
  setGasLimit,
  setError,
  addLog,
  clearLogs,
  addExecutionHistoryEntry,
  clearExecutionHistory,
  setAccounts,
  updateAccount,
  resetSimulator,
} = simulatorSlice.actions;

export default simulatorSlice.reducer;