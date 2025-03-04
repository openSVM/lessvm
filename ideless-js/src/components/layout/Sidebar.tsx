import { useState } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { useSelector, useDispatch } from 'react-redux';
import { RootState } from '../../store';
import { setActiveProject } from '../../store/slices/projectSlice';

// Icons
import { 
  FiHome, 
  FiCode, 
  FiTerminal, 
  FiPlay, 
  FiUploadCloud, 
  FiSettings,
  FiPlus,
  FiFolder,
  FiChevronRight,
  FiChevronDown,
  FiMenu,
  FiX
} from 'react-icons/fi';

interface SidebarProps {
  isOpen: boolean;
  onToggle: () => void;
}

const Sidebar = ({ isOpen, onToggle }: SidebarProps) => {
  const dispatch = useDispatch();
  const navigate = useNavigate();
  const [projectsExpanded, setProjectsExpanded] = useState(true);
  
  const { projects, activeProjectId } = useSelector((state: RootState) => state.project);
  const activeProject = projects.find(p => p.id === activeProjectId);
  
  const handleProjectClick = (projectId: string) => {
    dispatch(setActiveProject(projectId));
    navigate(`/project/${projectId}/editor`);
  };
  
  return (
    <div 
      className={`sidebar bg-dark-900 border-r border-dark-700 h-full flex flex-col transition-all duration-300 ${
        isOpen ? 'w-64' : 'w-0 -ml-64'
      }`}
    >
      <div className="flex items-center justify-between p-4 border-b border-dark-700">
        <div className="flex items-center">
          <span className="text-xl font-semibold text-primary-400">less<span className="text-white">VM</span></span>
          <span className="ml-2 text-xs bg-dark-700 px-2 py-0.5 rounded text-gray-400">ideless</span>
        </div>
        <button 
          onClick={onToggle}
          className="text-gray-400 hover:text-white transition-colors"
        >
          <FiX size={20} />
        </button>
      </div>
      
      <div className="flex-1 overflow-y-auto py-2">
        <nav className="px-2 space-y-1">
          {/* Home */}
          <Link 
            to="/"
            className="flex items-center px-3 py-2 text-gray-300 hover:bg-dark-700 rounded-md transition-colors"
          >
            <FiHome className="mr-3" size={18} />
            <span>Home</span>
          </Link>
          
          {/* Projects section */}
          <div>
            <button 
              onClick={() => setProjectsExpanded(!projectsExpanded)}
              className="w-full flex items-center justify-between px-3 py-2 text-gray-300 hover:bg-dark-700 rounded-md transition-colors"
            >
              <div className="flex items-center">
                <FiFolder className="mr-3" size={18} />
                <span>Projects</span>
              </div>
              {projectsExpanded ? <FiChevronDown size={16} /> : <FiChevronRight size={16} />}
            </button>
            
            {projectsExpanded && (
              <div className="ml-6 mt-1 space-y-1">
                {projects.map(project => (
                  <button
                    key={project.id}
                    onClick={() => handleProjectClick(project.id)}
                    className={`w-full text-left px-3 py-2 rounded-md transition-colors ${
                      project.id === activeProjectId 
                        ? 'bg-primary-600 text-white' 
                        : 'text-gray-300 hover:bg-dark-700'
                    }`}
                  >
                    {project.name}
                  </button>
                ))}
                
                <Link
                  to="/new-project"
                  className="flex items-center px-3 py-2 text-gray-300 hover:bg-dark-700 rounded-md transition-colors"
                >
                  <FiPlus className="mr-2" size={16} />
                  <span>New Project</span>
                </Link>
              </div>
            )}
          </div>
          
          {/* Project navigation - only show when a project is active */}
          {activeProject && (
            <div className="pt-2 mt-2 border-t border-dark-700">
              <div className="px-3 py-2 text-sm text-gray-400">
                {activeProject.name}
              </div>
              
              <Link
                to={`/project/${activeProjectId}/editor`}
                className="flex items-center px-3 py-2 text-gray-300 hover:bg-dark-700 rounded-md transition-colors"
              >
                <FiCode className="mr-3" size={18} />
                <span>Editor</span>
              </Link>
              
              <Link
                to={`/project/${activeProjectId}/debugger`}
                className="flex items-center px-3 py-2 text-gray-300 hover:bg-dark-700 rounded-md transition-colors"
              >
                <FiTerminal className="mr-3" size={18} />
                <span>Debugger</span>
              </Link>
              
              <Link
                to={`/project/${activeProjectId}/simulator`}
                className="flex items-center px-3 py-2 text-gray-300 hover:bg-dark-700 rounded-md transition-colors"
              >
                <FiPlay className="mr-3" size={18} />
                <span>Simulator</span>
              </Link>
              
              <Link
                to={`/project/${activeProjectId}/deploy`}
                className="flex items-center px-3 py-2 text-gray-300 hover:bg-dark-700 rounded-md transition-colors"
              >
                <FiUploadCloud className="mr-3" size={18} />
                <span>Deploy</span>
              </Link>
              
              <Link
                to={`/project/${activeProjectId}/settings`}
                className="flex items-center px-3 py-2 text-gray-300 hover:bg-dark-700 rounded-md transition-colors"
              >
                <FiSettings className="mr-3" size={18} />
                <span>Project Settings</span>
              </Link>
            </div>
          )}
        </nav>
      </div>
      
      {/* Bottom section */}
      <div className="p-4 border-t border-dark-700">
        <Link
          to="/settings"
          className="flex items-center px-3 py-2 text-gray-300 hover:bg-dark-700 rounded-md transition-colors"
        >
          <FiSettings className="mr-3" size={18} />
          <span>Settings</span>
        </Link>
      </div>
    </div>
  );
};

export default Sidebar;