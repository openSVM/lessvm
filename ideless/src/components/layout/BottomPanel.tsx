import { useState } from 'react';
import { useSelector } from 'react-redux';
import { RootState } from '../../store';
import { FiX, FiMaximize2, FiMinimize2, FiTerminal, FiAlertCircle, FiInfo } from 'react-icons/fi';

interface TabProps {
  id: string;
  label: string;
  icon: React.ReactNode;
  count?: number;
}

const BottomPanel = () => {
  const [activeTab, setActiveTab] = useState('terminal');
  const { bottomPanelHeight } = useSelector((state: RootState) => state.ui);
  const { logs } = useSelector((state: RootState) => state.debugger);
  
  const tabs: TabProps[] = [
    { id: 'terminal', label: 'Terminal', icon: <FiTerminal size={16} /> },
    { id: 'output', label: 'Output', icon: <FiInfo size={16} /> },
    { id: 'problems', label: 'Problems', icon: <FiAlertCircle size={16} />, count: 0 },
  ];
  
  return (
    <div 
      className="bottom-panel bg-dark-900 border-t border-dark-700 flex flex-col"
      style={{ height: `${bottomPanelHeight}px` }}
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
              {tab.count !== undefined && tab.count > 0 && (
                <span className="ml-2 px-1.5 py-0.5 text-xs bg-primary-600 text-white rounded-full">
                  {tab.count}
                </span>
              )}
            </button>
          ))}
        </div>
        
        <div className="flex items-center">
          <button className="p-2 text-gray-400 hover:text-white transition-colors">
            <FiMinimize2 size={16} />
          </button>
          <button className="p-2 text-gray-400 hover:text-white transition-colors">
            <FiX size={16} />
          </button>
        </div>
      </div>
      
      {/* Content */}
      <div className="flex-1 overflow-auto p-2 font-mono text-sm">
        {activeTab === 'terminal' && (
          <div className="terminal text-gray-300">
            <div className="mb-2 text-gray-500">$ lessvm --version</div>
            <div className="mb-2">LessVM CLI v1.0.0</div>
            <div className="mb-2 text-gray-500">$ </div>
          </div>
        )}
        
        {activeTab === 'output' && (
          <div className="output text-gray-300">
            {logs.map((log, index) => (
              <div 
                key={index} 
                className={`mb-1 ${
                  log.type === 'error' 
                    ? 'text-red-400' 
                    : log.type === 'warning' 
                      ? 'text-yellow-400' 
                      : 'text-gray-300'
                }`}
              >
                [{new Date(log.timestamp).toLocaleTimeString()}] {log.message}
              </div>
            ))}
            {logs.length === 0 && (
              <div className="text-gray-500">No output to display</div>
            )}
          </div>
        )}
        
        {activeTab === 'problems' && (
          <div className="problems text-gray-300">
            <div className="text-gray-500">No problems detected</div>
          </div>
        )}
      </div>
    </div>
  );
};

export default BottomPanel;