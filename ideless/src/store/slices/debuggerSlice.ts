import { createSlice, PayloadAction } from '@reduxjs/toolkit';

export interface Breakpoint {
  id: string;
  fileId: string;
  line: number;
  condition?: string;
  enabled: boolean;
}

export interface StackItem {
  value: string;
  type: 'number' | 'address' | 'boolean' | 'string';
  description?: string;
}

export interface MemoryState {
  [address: string]: {
    value: number;
    lastModified?: number;
  };
}

interface DebuggerState {
  isDebugging: boolean;
  isPaused: boolean;
  currentFileId: string | null;
  currentLine: number | null;
  breakpoints: Breakpoint[];
  callStack: { functionName: string; fileId: string; line: number }[];
  stack: StackItem[];
  memory: MemoryState;
  gasUsed: number;
  gasLimit: number;
  logs: { message: string; type: 'info' | 'warning' | 'error'; timestamp: number }[];
}

const initialState: DebuggerState = {
  isDebugging: false,
  isPaused: false,
  currentFileId: null,
  currentLine: null,
  breakpoints: [],
  callStack: [],
  stack: [],
  memory: {},
  gasUsed: 0,
  gasLimit: 1000000,
  logs: [],
};

export const debuggerSlice = createSlice({
  name: 'debugger',
  initialState,
  reducers: {
    startDebugging: (state, action: PayloadAction<{ fileId: string }>) => {
      state.isDebugging = true;
      state.isPaused = false;
      state.currentFileId = action.payload.fileId;
      state.currentLine = 0;
      state.callStack = [];
      state.stack = [];
      state.memory = {};
      state.gasUsed = 0;
      state.logs = [];
    },
    stopDebugging: (state) => {
      state.isDebugging = false;
      state.isPaused = false;
      state.currentFileId = null;
      state.currentLine = null;
    },
    pauseDebugging: (state) => {
      state.isPaused = true;
    },
    resumeDebugging: (state) => {
      state.isPaused = false;
    },
    stepOver: (state) => {
      // Logic for stepping over would be implemented in a thunk
      // This is just a placeholder for the action
    },
    stepInto: (state) => {
      // Logic for stepping into would be implemented in a thunk
    },
    stepOut: (state) => {
      // Logic for stepping out would be implemented in a thunk
    },
    setCurrentLine: (state, action: PayloadAction<{ fileId: string; line: number }>) => {
      state.currentFileId = action.payload.fileId;
      state.currentLine = action.payload.line;
    },
    addBreakpoint: (state, action: PayloadAction<Breakpoint>) => {
      state.breakpoints.push(action.payload);
    },
    removeBreakpoint: (state, action: PayloadAction<string>) => {
      state.breakpoints = state.breakpoints.filter(bp => bp.id !== action.payload);
    },
    toggleBreakpoint: (state, action: PayloadAction<string>) => {
      const breakpoint = state.breakpoints.find(bp => bp.id === action.payload);
      if (breakpoint) {
        breakpoint.enabled = !breakpoint.enabled;
      }
    },
    updateBreakpoint: (state, action: PayloadAction<{ id: string; condition?: string }>) => {
      const breakpoint = state.breakpoints.find(bp => bp.id === action.payload.id);
      if (breakpoint && action.payload.condition !== undefined) {
        breakpoint.condition = action.payload.condition;
      }
    },
    updateStack: (state, action: PayloadAction<StackItem[]>) => {
      state.stack = action.payload;
    },
    pushStack: (state, action: PayloadAction<StackItem>) => {
      state.stack.push(action.payload);
    },
    popStack: (state) => {
      state.stack.pop();
    },
    updateMemory: (state, action: PayloadAction<MemoryState>) => {
      state.memory = action.payload;
    },
    updateMemoryValue: (state, action: PayloadAction<{ address: string; value: number }>) => {
      state.memory[action.payload.address] = {
        value: action.payload.value,
        lastModified: Date.now(),
      };
    },
    updateGasUsed: (state, action: PayloadAction<number>) => {
      state.gasUsed = action.payload;
    },
    addLog: (state, action: PayloadAction<{ message: string; type: 'info' | 'warning' | 'error' }>) => {
      state.logs.push({
        ...action.payload,
        timestamp: Date.now(),
      });
    },
    clearLogs: (state) => {
      state.logs = [];
    },
    resetDebugger: (state) => {
      return initialState;
    },
  },
});

export const {
  startDebugging,
  stopDebugging,
  pauseDebugging,
  resumeDebugging,
  stepOver,
  stepInto,
  stepOut,
  setCurrentLine,
  addBreakpoint,
  removeBreakpoint,
  toggleBreakpoint,
  updateBreakpoint,
  updateStack,
  pushStack,
  popStack,
  updateMemory,
  updateMemoryValue,
  updateGasUsed,
  addLog,
  clearLogs,
  resetDebugger,
} = debuggerSlice.actions;

export default debuggerSlice.reducer;