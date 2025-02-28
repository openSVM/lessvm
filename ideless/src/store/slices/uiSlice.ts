import { createSlice, PayloadAction } from '@reduxjs/toolkit';

export type ThemeMode = 'light' | 'dark' | 'system';
export type PanelSize = 'small' | 'medium' | 'large';
export type LayoutMode = 'default' | 'editor-focus' | 'debugger-focus' | 'simulator-focus';

interface UiState {
  theme: ThemeMode;
  sidebarOpen: boolean;
  sidebarWidth: number;
  bottomPanelOpen: boolean;
  bottomPanelHeight: number;
  rightPanelOpen: boolean;
  rightPanelWidth: number;
  activeTab: 'editor' | 'debugger' | 'simulator' | 'deploy' | 'settings';
  editorFontSize: number;
  editorFontFamily: string;
  editorTabSize: number;
  editorWordWrap: boolean;
  editorMinimap: boolean;
  editorLineNumbers: boolean;
  terminalOpen: boolean;
  terminalHeight: number;
  aiAssistantOpen: boolean;
  aiAssistantWidth: number;
  layoutMode: LayoutMode;
  notifications: {
    id: string;
    type: 'info' | 'success' | 'warning' | 'error';
    message: string;
    timestamp: number;
    read: boolean;
  }[];
  modal: {
    open: boolean;
    type: string | null;
    data: any;
  };
}

const initialState: UiState = {
  theme: 'dark',
  sidebarOpen: true,
  sidebarWidth: 250,
  bottomPanelOpen: true,
  bottomPanelHeight: 300,
  rightPanelOpen: true,
  rightPanelWidth: 300,
  activeTab: 'editor',
  editorFontSize: 14,
  editorFontFamily: 'IBM Plex Mono',
  editorTabSize: 2,
  editorWordWrap: false,
  editorMinimap: true,
  editorLineNumbers: true,
  terminalOpen: false,
  terminalHeight: 200,
  aiAssistantOpen: true,
  aiAssistantWidth: 300,
  layoutMode: 'default',
  notifications: [],
  modal: {
    open: false,
    type: null,
    data: null,
  },
};

export const uiSlice = createSlice({
  name: 'ui',
  initialState,
  reducers: {
    setTheme: (state, action: PayloadAction<ThemeMode>) => {
      state.theme = action.payload;
    },
    toggleSidebar: (state) => {
      state.sidebarOpen = !state.sidebarOpen;
    },
    setSidebarWidth: (state, action: PayloadAction<number>) => {
      state.sidebarWidth = action.payload;
    },
    toggleBottomPanel: (state) => {
      state.bottomPanelOpen = !state.bottomPanelOpen;
    },
    setBottomPanelHeight: (state, action: PayloadAction<number>) => {
      state.bottomPanelHeight = action.payload;
    },
    toggleRightPanel: (state) => {
      state.rightPanelOpen = !state.rightPanelOpen;
    },
    setRightPanelWidth: (state, action: PayloadAction<number>) => {
      state.rightPanelWidth = action.payload;
    },
    setActiveTab: (state, action: PayloadAction<UiState['activeTab']>) => {
      state.activeTab = action.payload;
    },
    setEditorFontSize: (state, action: PayloadAction<number>) => {
      state.editorFontSize = action.payload;
    },
    setEditorFontFamily: (state, action: PayloadAction<string>) => {
      state.editorFontFamily = action.payload;
    },
    setEditorTabSize: (state, action: PayloadAction<number>) => {
      state.editorTabSize = action.payload;
    },
    toggleEditorWordWrap: (state) => {
      state.editorWordWrap = !state.editorWordWrap;
    },
    toggleEditorMinimap: (state) => {
      state.editorMinimap = !state.editorMinimap;
    },
    toggleEditorLineNumbers: (state) => {
      state.editorLineNumbers = !state.editorLineNumbers;
    },
    toggleTerminal: (state) => {
      state.terminalOpen = !state.terminalOpen;
    },
    setTerminalHeight: (state, action: PayloadAction<number>) => {
      state.terminalHeight = action.payload;
    },
    toggleAiAssistant: (state) => {
      state.aiAssistantOpen = !state.aiAssistantOpen;
    },
    setAiAssistantWidth: (state, action: PayloadAction<number>) => {
      state.aiAssistantWidth = action.payload;
    },
    setLayoutMode: (state, action: PayloadAction<LayoutMode>) => {
      state.layoutMode = action.payload;
    },
    addNotification: (state, action: PayloadAction<{ type: 'info' | 'success' | 'warning' | 'error'; message: string }>) => {
      const id = `notification-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
      state.notifications.push({
        id,
        type: action.payload.type,
        message: action.payload.message,
        timestamp: Date.now(),
        read: false,
      });
    },
    markNotificationAsRead: (state, action: PayloadAction<string>) => {
      const notification = state.notifications.find(n => n.id === action.payload);
      if (notification) {
        notification.read = true;
      }
    },
    clearNotifications: (state) => {
      state.notifications = [];
    },
    openModal: (state, action: PayloadAction<{ type: string; data?: any }>) => {
      state.modal = {
        open: true,
        type: action.payload.type,
        data: action.payload.data || null,
      };
    },
    closeModal: (state) => {
      state.modal = {
        open: false,
        type: null,
        data: null,
      };
    },
    resetUi: (state) => {
      return initialState;
    },
  },
});

export const {
  setTheme,
  toggleSidebar,
  setSidebarWidth,
  toggleBottomPanel,
  setBottomPanelHeight,
  toggleRightPanel,
  setRightPanelWidth,
  setActiveTab,
  setEditorFontSize,
  setEditorFontFamily,
  setEditorTabSize,
  toggleEditorWordWrap,
  toggleEditorMinimap,
  toggleEditorLineNumbers,
  toggleTerminal,
  setTerminalHeight,
  toggleAiAssistant,
  setAiAssistantWidth,
  setLayoutMode,
  addNotification,
  markNotificationAsRead,
  clearNotifications,
  openModal,
  closeModal,
  resetUi,
} = uiSlice.actions;

export default uiSlice.reducer;