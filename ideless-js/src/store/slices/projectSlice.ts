import { createSlice, PayloadAction } from '@reduxjs/toolkit';

export interface Project {
  id: string;
  name: string;
  description: string;
  createdAt: number;
  lastModified: number;
  template: string;
  rootDirectory: string;
  mainFile: string;
  settings: {
    solanaNetwork: 'mainnet-beta' | 'testnet' | 'devnet' | 'localnet';
    rpcEndpoint: string;
    autoSave: boolean;
    gasLimit: number;
    deploymentTarget: 'solana' | 'arweave' | 'both';
  };
}

interface ProjectState {
  projects: Project[];
  activeProjectId: string | null;
  isLoading: boolean;
  error: string | null;
}

const initialState: ProjectState = {
  projects: [],
  activeProjectId: null,
  isLoading: false,
  error: null,
};

// Default OpenSVM RPC endpoints
const DEFAULT_RPC_ENDPOINTS = {
  'mainnet-beta': 'https://api.opensvm.com/mainnet',
  'testnet': 'https://api.opensvm.com/testnet',
  'devnet': 'https://api.opensvm.com/devnet',
  'localnet': 'http://localhost:8899',
};

export const projectSlice = createSlice({
  name: 'project',
  initialState,
  reducers: {
    setProjects: (state, action: PayloadAction<Project[]>) => {
      state.projects = action.payload;
    },
    addProject: (state, action: PayloadAction<Project>) => {
      // Ensure we're using OpenSVM RPC endpoints by default
      const project = {
        ...action.payload,
        settings: {
          ...action.payload.settings,
          rpcEndpoint: DEFAULT_RPC_ENDPOINTS[action.payload.settings.solanaNetwork],
        },
      };
      state.projects.push(project);
    },
    updateProject: (state, action: PayloadAction<{ id: string; updates: Partial<Project> }>) => {
      const { id, updates } = action.payload;
      const project = state.projects.find(p => p.id === id);
      if (project) {
        // If updating network, ensure we use the default OpenSVM endpoint
        if (updates.settings?.solanaNetwork) {
          updates.settings.rpcEndpoint = DEFAULT_RPC_ENDPOINTS[updates.settings.solanaNetwork];
        }
        
        Object.assign(project, { ...updates, lastModified: Date.now() });
      }
    },
    deleteProject: (state, action: PayloadAction<string>) => {
      const projectId = action.payload;
      state.projects = state.projects.filter(p => p.id !== projectId);
      if (state.activeProjectId === projectId) {
        state.activeProjectId = state.projects[0]?.id || null;
      }
    },
    setActiveProject: (state, action: PayloadAction<string>) => {
      state.activeProjectId = action.payload;
    },
    setLoading: (state, action: PayloadAction<boolean>) => {
      state.isLoading = action.payload;
    },
    setError: (state, action: PayloadAction<string | null>) => {
      state.error = action.payload;
    },
    updateProjectSettings: (state, action: PayloadAction<{ id: string; settings: Partial<Project['settings']> }>) => {
      const { id, settings } = action.payload;
      const project = state.projects.find(p => p.id === id);
      if (project) {
        // If updating network, ensure we use the default OpenSVM endpoint
        if (settings.solanaNetwork) {
          settings.rpcEndpoint = DEFAULT_RPC_ENDPOINTS[settings.solanaNetwork];
        }
        
        project.settings = { ...project.settings, ...settings };
        project.lastModified = Date.now();
      }
    },
    createProjectFromTemplate: (state, action: PayloadAction<{ name: string; template: string; description?: string }>) => {
      const { name, template, description = '' } = action.payload;
      const id = `project-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
      const timestamp = Date.now();
      
      const newProject: Project = {
        id,
        name,
        description,
        createdAt: timestamp,
        lastModified: timestamp,
        template,
        rootDirectory: `/${id}`,
        mainFile: 'src/main.rs', // Default main file, will be updated based on template
        settings: {
          solanaNetwork: 'devnet',
          rpcEndpoint: DEFAULT_RPC_ENDPOINTS['devnet'],
          autoSave: true,
          gasLimit: 1000000,
          deploymentTarget: 'solana',
        },
      };
      
      state.projects.push(newProject);
      state.activeProjectId = id;
    },
  },
});

export const {
  setProjects,
  addProject,
  updateProject,
  deleteProject,
  setActiveProject,
  setLoading,
  setError,
  updateProjectSettings,
  createProjectFromTemplate,
} = projectSlice.actions;

export default projectSlice.reducer;