import { useState } from 'react';
import { useSelector } from 'react-redux';
import { RootState } from '../../store';
import { FiX, FiInfo, FiFileText, FiSettings, FiHelpCircle } from 'react-icons/fi';

interface TabProps {
  id: string;
  label: string;
  icon: React.ReactNode;
}

const RightPanel = () => {
  const [activeTab, setActiveTab] = useState('properties');
  const { rightPanelWidth, activeTab: mainTab } = useSelector((state: RootState) => state.ui);
  const { activeProjectId, projects } = useSelector((state: RootState) => state.project);
  
  const activeProject = projects.find(p => p.id === activeProjectId);
  
  const tabs: TabProps[] = [
    { id: 'properties', label: 'Properties', icon: <FiSettings size={16} /> },
    { id: 'documentation', label: 'Documentation', icon: <FiFileText size={16} /> },
    { id: 'help', label: 'Help', icon: <FiHelpCircle size={16} /> },
  ];
  
  // Get contextual documentation based on the active tab
  const getDocumentation = () => {
    if (!mainTab) return null;
    
    switch (mainTab) {
      case 'editor':
        return (
          <div>
            <h3 className="text-lg font-medium mb-3">LessVM Editor</h3>
            <p className="mb-3">
              The editor provides a powerful environment for writing and editing LessVM code.
            </p>
            <h4 className="text-md font-medium mb-2">Key Features</h4>
            <ul className="list-disc pl-5 mb-3 space-y-1">
              <li>Syntax highlighting for LessVM bytecode</li>
              <li>Auto-completion with context-aware suggestions</li>
              <li>Real-time error checking and linting</li>
              <li>Split-view editing for multiple files</li>
            </ul>
            <h4 className="text-md font-medium mb-2">Keyboard Shortcuts</h4>
            <div className="grid grid-cols-2 gap-2 mb-3">
              <div className="text-gray-300">Save File</div>
              <div className="text-gray-400">Ctrl+S</div>
              <div className="text-gray-300">Find</div>
              <div className="text-gray-400">Ctrl+F</div>
              <div className="text-gray-300">Replace</div>
              <div className="text-gray-400">Ctrl+H</div>
            </div>
          </div>
        );
      case 'debugger':
        return (
          <div>
            <h3 className="text-lg font-medium mb-3">LessVM Debugger</h3>
            <p className="mb-3">
              The debugger allows you to step through your LessVM code, inspect variables, and track execution.
            </p>
            <h4 className="text-md font-medium mb-2">Key Features</h4>
            <ul className="list-disc pl-5 mb-3 space-y-1">
              <li>Step-by-step execution control</li>
              <li>Breakpoint management</li>
              <li>Stack and memory visualization</li>
              <li>Gas usage monitoring</li>
            </ul>
            <h4 className="text-md font-medium mb-2">Keyboard Shortcuts</h4>
            <div className="grid grid-cols-2 gap-2 mb-3">
              <div className="text-gray-300">Start/Continue</div>
              <div className="text-gray-400">F5</div>
              <div className="text-gray-300">Step Over</div>
              <div className="text-gray-400">F10</div>
              <div className="text-gray-300">Step Into</div>
              <div className="text-gray-400">F11</div>
              <div className="text-gray-300">Step Out</div>
              <div className="text-gray-400">Shift+F11</div>
            </div>
          </div>
        );
      case 'simulator':
        return (
          <div>
            <h3 className="text-lg font-medium mb-3">LessVM Simulator</h3>
            <p className="mb-3">
              The simulator provides a controlled environment to test your LessVM programs without deploying to the blockchain.
            </p>
            <h4 className="text-md font-medium mb-2">Key Features</h4>
            <ul className="list-disc pl-5 mb-3 space-y-1">
              <li>Local execution of LessVM programs</li>
              <li>Configurable initial state</li>
              <li>Performance profiling</li>
              <li>Gas usage optimization</li>
            </ul>
            <h4 className="text-md font-medium mb-2">Simulation Controls</h4>
            <div className="grid grid-cols-2 gap-2 mb-3">
              <div className="text-gray-300">Run Simulation</div>
              <div className="text-gray-400">Ctrl+R</div>
              <div className="text-gray-300">Pause Simulation</div>
              <div className="text-gray-400">Ctrl+P</div>
              <div className="text-gray-300">Reset Simulation</div>
              <div className="text-gray-400">Ctrl+Shift+R</div>
            </div>
          </div>
        );
      case 'deploy':
        return (
          <div>
            <h3 className="text-lg font-medium mb-3">LessVM Deployment</h3>
            <p className="mb-3">
              The deployment tool allows you to deploy your LessVM programs to the Solana blockchain.
            </p>
            <h4 className="text-md font-medium mb-2">Key Features</h4>
            <ul className="list-disc pl-5 mb-3 space-y-1">
              <li>One-click deployment to Solana networks</li>
              <li>Frontend deployment to Arweave</li>
              <li>Deployment configuration management</li>
              <li>Deployment verification and monitoring</li>
            </ul>
            <h4 className="text-md font-medium mb-2">Network Options</h4>
            <div className="grid grid-cols-2 gap-2 mb-3">
              <div className="text-gray-300">Mainnet</div>
              <div className="text-gray-400">Production environment</div>
              <div className="text-gray-300">Testnet</div>
              <div className="text-gray-400">Testing environment</div>
              <div className="text-gray-300">Devnet</div>
              <div className="text-gray-400">Development environment</div>
              <div className="text-gray-300">Localnet</div>
              <div className="text-gray-400">Local development</div>
            </div>
          </div>
        );
      default:
        return (
          <div className="text-gray-400">
            Select a tool to view documentation
          </div>
        );
    }
  };
  
  // Get properties based on the active project
  const getProperties = () => {
    if (!activeProject) {
      return (
        <div className="text-gray-400">
          No active project
        </div>
      );
    }
    
    return (
      <div>
        <h3 className="text-lg font-medium mb-3">Project Properties</h3>
        <div className="grid grid-cols-2 gap-2 mb-3">
          <div className="text-gray-300">Name</div>
          <div className="text-gray-400">{activeProject.name}</div>
          <div className="text-gray-300">Template</div>
          <div className="text-gray-400">{activeProject.template}</div>
          <div className="text-gray-300">Created</div>
          <div className="text-gray-400">{new Date(activeProject.createdAt).toLocaleDateString()}</div>
          <div className="text-gray-300">Last Modified</div>
          <div className="text-gray-400">{new Date(activeProject.lastModified).toLocaleDateString()}</div>
          <div className="text-gray-300">Network</div>
          <div className="text-gray-400">{activeProject.settings.solanaNetwork}</div>
          <div className="text-gray-300">RPC Endpoint</div>
          <div className="text-gray-400 truncate">{activeProject.settings.rpcEndpoint}</div>
          <div className="text-gray-300">Gas Limit</div>
          <div className="text-gray-400">{activeProject.settings.gasLimit.toLocaleString()}</div>
          <div className="text-gray-300">Deployment Target</div>
          <div className="text-gray-400">{activeProject.settings.deploymentTarget}</div>
        </div>
      </div>
    );
  };
  
  // Get help content
  const getHelp = () => {
    return (
      <div>
        <h3 className="text-lg font-medium mb-3">LessVM Help</h3>
        <p className="mb-3">
          LessVM is a lightweight virtual machine designed for the Solana blockchain.
        </p>
        <h4 className="text-md font-medium mb-2">Resources</h4>
        <ul className="list-disc pl-5 mb-3 space-y-1">
          <li>
            <a href="https://docs.lessvm.org" className="text-primary-400 hover:text-primary-300">
              LessVM Documentation
            </a>
          </li>
          <li>
            <a href="https://github.com/openSVM/lessvm" className="text-primary-400 hover:text-primary-300">
              GitHub Repository
            </a>
          </li>
          <li>
            <a href="https://discord.gg/lessvm" className="text-primary-400 hover:text-primary-300">
              Discord Community
            </a>
          </li>
        </ul>
        <h4 className="text-md font-medium mb-2">Getting Started</h4>
        <p className="mb-3">
          To get started with LessVM, create a new project using one of the available templates.
          Then, use the editor to write your code, the debugger to test it, and the simulator to
          run it locally before deploying to the Solana blockchain.
        </p>
      </div>
    );
  };
  
  // Get content based on active tab
  const getContent = () => {
    switch (activeTab) {
      case 'properties':
        return getProperties();
      case 'documentation':
        return getDocumentation();
      case 'help':
        return getHelp();
      default:
        return null;
    }
  };
  
  return (
    <div 
      className="right-panel bg-dark-900 border-l border-dark-700 flex flex-col"
      style={{ width: `${rightPanelWidth}px` }}
    >
      {/* Tabs */}
      <div className="flex items-center border-b border-dark-700">
        <div className="flex-1 flex">
          {tabs.map(tab => (
            <button
              key={tab.id}
              className={`flex items-center px-4 py-2 text-sm transition-colors ${
                activeTab === tab.id 
                  ? 'text-white border-t-2 border-primary-500 bg-dark-800' 
                  : 'text-gray-400 hover:text-white hover:bg-dark-800'
              }`}
              onClick={() => setActiveTab(tab.id)}
            >
              <span className="mr-2">{tab.icon}</span>
              <span>{tab.label}</span>
            </button>
          ))}
        </div>
        
        <div className="flex items-center">
          <button className="p-2 text-gray-400 hover:text-white transition-colors">
            <FiX size={16} />
          </button>
        </div>
      </div>
      
      {/* Content */}
      <div className="flex-1 overflow-auto p-4">
        {getContent()}
      </div>
    </div>
  );
};

export default RightPanel;