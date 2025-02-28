import { useSelector } from 'react-redux';
import { RootState } from '../../store';
import { FiMenu, FiMaximize2, FiMinimize2, FiTerminal, FiSidebar } from 'react-icons/fi';

interface HeaderProps {
  onToggleSidebar: () => void;
  onToggleBottomPanel: () => void;
  onToggleRightPanel: () => void;
}

const Header = ({ onToggleSidebar, onToggleBottomPanel, onToggleRightPanel }: HeaderProps) => {
  const { activeProjectId, projects } = useSelector((state: RootState) => state.project);
  const { activeTab } = useSelector((state: RootState) => state.ui);
  
  const activeProject = projects.find(p => p.id === activeProjectId);
  
  // Get the title based on the active tab
  const getTitle = () => {
    if (!activeProject) return 'LessVM ideless';
    
    switch (activeTab) {
      case 'editor':
        return `${activeProject.name} - Editor`;
      case 'debugger':
        return `${activeProject.name} - Debugger`;
      case 'simulator':
        return `${activeProject.name} - Simulator`;
      case 'deploy':
        return `${activeProject.name} - Deploy`;
      case 'settings':
        return `${activeProject.name} - Settings`;
      default:
        return activeProject.name;
    }
  };
  
  return (
    <header className="bg-dark-800 border-b border-dark-700 h-14 flex items-center px-4 justify-between">
      <div className="flex items-center">
        <button
          onClick={onToggleSidebar}
          className="mr-4 text-gray-400 hover:text-white transition-colors"
          aria-label="Toggle sidebar"
        >
          <FiMenu size={20} />
        </button>
        
        <h1 className="text-lg font-medium text-white">{getTitle()}</h1>
      </div>
      
      <div className="flex items-center space-x-2">
        {/* Network indicator - only show when a project is active */}
        {activeProject && (
          <div className="flex items-center mr-4">
            <div className="w-2 h-2 rounded-full bg-green-500 mr-2"></div>
            <span className="text-sm text-gray-300">
              {activeProject.settings.solanaNetwork}
            </span>
          </div>
        )}
        
        {/* Bottom panel toggle */}
        <button
          onClick={onToggleBottomPanel}
          className="p-2 text-gray-400 hover:text-white hover:bg-dark-700 rounded transition-colors"
          aria-label="Toggle terminal"
        >
          <FiTerminal size={18} />
        </button>
        
        {/* Right panel toggle */}
        <button
          onClick={onToggleRightPanel}
          className="p-2 text-gray-400 hover:text-white hover:bg-dark-700 rounded transition-colors"
          aria-label="Toggle right panel"
        >
          <FiSidebar size={18} />
        </button>
      </div>
    </header>
  );
};

export default Header;