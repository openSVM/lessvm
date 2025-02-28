import { useEffect } from 'react';
import { Routes, Route, Navigate } from 'react-router-dom';
import { useSelector, useDispatch } from 'react-redux';
import { RootState } from './store';
import { setTheme } from './store/slices/uiSlice';

// Layout components
import MainLayout from './layouts/MainLayout';

// Pages
import EditorPage from './pages/EditorPage';
import DebuggerPage from './pages/DebuggerPage';
import SimulatorPage from './pages/SimulatorPage';
import DeployPage from './pages/DeployPage';
import SettingsPage from './pages/SettingsPage';
import ProjectsPage from './pages/ProjectsPage';
import NewProjectPage from './pages/NewProjectPage';
import NotFoundPage from './pages/NotFoundPage';

// Components
import Notifications from './components/common/Notifications';
import ModalContainer from './components/common/ModalContainer';

const App = () => {
  const dispatch = useDispatch();
  const { theme } = useSelector((state: RootState) => state.ui);
  const { activeProjectId } = useSelector((state: RootState) => state.project);

  // Apply theme
  useEffect(() => {
    if (theme === 'dark') {
      document.documentElement.classList.add('dark');
    } else if (theme === 'light') {
      document.documentElement.classList.remove('dark');
    } else if (theme === 'system') {
      const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
      if (prefersDark) {
        document.documentElement.classList.add('dark');
      } else {
        document.documentElement.classList.remove('dark');
      }
    }
  }, [theme]);

  // Listen for system theme changes
  useEffect(() => {
    if (theme === 'system') {
      const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
      
      const handleChange = (e: MediaQueryListEvent) => {
        if (e.matches) {
          document.documentElement.classList.add('dark');
        } else {
          document.documentElement.classList.remove('dark');
        }
      };
      
      mediaQuery.addEventListener('change', handleChange);
      
      return () => {
        mediaQuery.removeEventListener('change', handleChange);
      };
    }
  }, [theme]);

  return (
    <div className="app min-h-screen bg-dark-800 text-gray-100">
      <Routes>
        <Route path="/" element={<MainLayout />}>
          <Route index element={<ProjectsPage />} />
          <Route path="projects" element={<ProjectsPage />} />
          <Route path="new-project" element={<NewProjectPage />} />
          
          {/* Project routes - require an active project */}
          <Route 
            path="project/:projectId" 
            element={activeProjectId ? <EditorPage /> : <Navigate to="/" replace />} 
          />
          <Route 
            path="project/:projectId/editor" 
            element={activeProjectId ? <EditorPage /> : <Navigate to="/" replace />} 
          />
          <Route 
            path="project/:projectId/debugger" 
            element={activeProjectId ? <DebuggerPage /> : <Navigate to="/" replace />} 
          />
          <Route 
            path="project/:projectId/simulator" 
            element={activeProjectId ? <SimulatorPage /> : <Navigate to="/" replace />} 
          />
          <Route 
            path="project/:projectId/deploy" 
            element={activeProjectId ? <DeployPage /> : <Navigate to="/" replace />} 
          />
          <Route 
            path="project/:projectId/settings" 
            element={activeProjectId ? <SettingsPage /> : <Navigate to="/" replace />} 
          />
          
          {/* Settings route */}
          <Route path="settings" element={<SettingsPage />} />
          
          {/* 404 route */}
          <Route path="*" element={<NotFoundPage />} />
        </Route>
      </Routes>
      
      <Notifications />
      <ModalContainer />
    </div>
  );
};

export default App;