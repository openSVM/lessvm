import { useEffect } from 'react';
import { Outlet, useLocation, useNavigate, useParams } from 'react-router-dom';
import { useSelector, useDispatch } from 'react-redux';
import { RootState } from '../store';
import { 
  toggleSidebar, 
  toggleBottomPanel, 
  toggleRightPanel,
  setActiveTab
} from '../store/slices/uiSlice';
import { setActiveProject } from '../store/slices/projectSlice';

// Components
import Sidebar from '../components/layout/Sidebar';
import Header from '../components/layout/Header';
import BottomPanel from '../components/layout/BottomPanel';
import RightPanel from '../components/layout/RightPanel';
import AiAssistant from '../components/ai/AiAssistant';

const MainLayout = () => {
  const dispatch = useDispatch();
  const location = useLocation();
  const navigate = useNavigate();
  const { projectId } = useParams<{ projectId: string }>();
  
  const { 
    sidebarOpen, 
    bottomPanelOpen, 
    rightPanelOpen, 
    aiAssistantOpen,
    activeTab
  } = useSelector((state: RootState) => state.ui);
  
  const { activeProjectId, projects } = useSelector((state: RootState) => state.project);

  // Set active project based on URL
  useEffect(() => {
    if (projectId && projectId !== activeProjectId) {
      dispatch(setActiveProject(projectId));
    }
  }, [projectId, activeProjectId, dispatch]);

  // Set active tab based on URL
  useEffect(() => {
    const path = location.pathname;
    
    if (path.includes('/editor')) {
      dispatch(setActiveTab('editor'));
    } else if (path.includes('/debugger')) {
      dispatch(setActiveTab('debugger'));
    } else if (path.includes('/simulator')) {
      dispatch(setActiveTab('simulator'));
    } else if (path.includes('/deploy')) {
      dispatch(setActiveTab('deploy'));
    } else if (path.includes('/settings')) {
      dispatch(setActiveTab('settings'));
    }
  }, [location, dispatch]);

  // Navigate to the correct tab when activeTab changes
  useEffect(() => {
    if (activeProjectId && activeTab) {
      const currentPath = location.pathname;
      const expectedPath = `/project/${activeProjectId}/${activeTab}`;
      
      // Only navigate if we're in a project and the path doesn't already match
      if (currentPath.includes(`/project/${activeProjectId}`) && !currentPath.includes(`/${activeTab}`)) {
        navigate(expectedPath);
      }
    }
  }, [activeTab, activeProjectId, location.pathname, navigate]);

  return (
    <div className="flex h-screen overflow-hidden">
      {/* Sidebar */}
      <Sidebar 
        isOpen={sidebarOpen} 
        onToggle={() => dispatch(toggleSidebar())} 
      />
      
      {/* Main content */}
      <div className="flex flex-col flex-1 overflow-hidden">
        {/* Header */}
        <Header 
          onToggleSidebar={() => dispatch(toggleSidebar())}
          onToggleBottomPanel={() => dispatch(toggleBottomPanel())}
          onToggleRightPanel={() => dispatch(toggleRightPanel())}
        />
        
        {/* Content area */}
        <div className="flex flex-1 overflow-hidden">
          {/* Main content */}
          <div className="flex-1 overflow-auto">
            <Outlet />
          </div>
          
          {/* Right panel */}
          {rightPanelOpen && (
            <RightPanel />
          )}
          
          {/* AI Assistant */}
          {aiAssistantOpen && (
            <AiAssistant />
          )}
        </div>
        
        {/* Bottom panel */}
        {bottomPanelOpen && (
          <BottomPanel />
        )}
      </div>
    </div>
  );
};

export default MainLayout;