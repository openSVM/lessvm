import { createSlice, PayloadAction } from '@reduxjs/toolkit';

export interface File {
  id: string;
  name: string;
  path: string;
  content: string;
  language: string;
  lastModified: number;
}

interface EditorState {
  files: File[];
  activeFileId: string | null;
  openFileIds: string[];
  unsavedFileIds: string[];
}

const initialState: EditorState = {
  files: [],
  activeFileId: null,
  openFileIds: [],
  unsavedFileIds: [],
};

export const editorSlice = createSlice({
  name: 'editor',
  initialState,
  reducers: {
    setFiles: (state, action: PayloadAction<File[]>) => {
      state.files = action.payload;
    },
    addFile: (state, action: PayloadAction<File>) => {
      state.files.push(action.payload);
    },
    updateFile: (state, action: PayloadAction<{ id: string; content: string }>) => {
      const file = state.files.find(f => f.id === action.payload.id);
      if (file) {
        file.content = action.payload.content;
        file.lastModified = Date.now();
        if (!state.unsavedFileIds.includes(file.id)) {
          state.unsavedFileIds.push(file.id);
        }
      }
    },
    saveFile: (state, action: PayloadAction<string>) => {
      const fileId = action.payload;
      state.unsavedFileIds = state.unsavedFileIds.filter(id => id !== fileId);
    },
    deleteFile: (state, action: PayloadAction<string>) => {
      const fileId = action.payload;
      state.files = state.files.filter(f => f.id !== fileId);
      state.openFileIds = state.openFileIds.filter(id => id !== fileId);
      state.unsavedFileIds = state.unsavedFileIds.filter(id => id !== fileId);
      if (state.activeFileId === fileId) {
        state.activeFileId = state.openFileIds[0] || null;
      }
    },
    setActiveFile: (state, action: PayloadAction<string>) => {
      const fileId = action.payload;
      state.activeFileId = fileId;
      if (!state.openFileIds.includes(fileId)) {
        state.openFileIds.push(fileId);
      }
    },
    closeFile: (state, action: PayloadAction<string>) => {
      const fileId = action.payload;
      state.openFileIds = state.openFileIds.filter(id => id !== fileId);
      if (state.activeFileId === fileId) {
        state.activeFileId = state.openFileIds[0] || null;
      }
    },
  },
});

export const {
  setFiles,
  addFile,
  updateFile,
  saveFile,
  deleteFile,
  setActiveFile,
  closeFile,
} = editorSlice.actions;

export default editorSlice.reducer;