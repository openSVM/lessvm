import { useState, useEffect } from 'react';
import { useSelector, useDispatch } from 'react-redux';
import { useParams } from 'react-router-dom';
import { RootState } from '../store';
import { 
  startDebugging, 
  stopDebugging, 
  pauseDebugging, 
  resumeDebugging,
  stepOver,
  stepInto,
  stepOut,
  addBreakpoint,
  removeBreakpoint,
  toggleBreakpoint
} from '../store/slices/debuggerSlice';
import { setActiveFile } from '../store/slices/editorSlice';
import { 
  FiPlay, 
  FiPause, 
  FiStop, 
  FiSkipForward, 
  FiCornerDownRight, 
  FiCornerUpRight,
  FiList,
  FiCpu,
  FiDatabase,
  FiAlertCircle
} from 'react-icons/fi';
import Editor from '@monaco-editor/react';

const DebuggerPage = () => {
  const dispatch = useDispatch();
  const { projectId } = useParams<{ projectId: string }>();
  const { files, activeFileId } = useSelector((state: RootState) => state.editor);
  const { 
    isDebugging, 
    isPaused, 
    currentFileId, 
    currentLine, 
    breakpoints, 
    callStack,
    stack,
    memory,
    gasUsed,
    gasLimit,
    logs
  } = useSelector((state: RootState) => state.debugger);
  const { projects } = useSelector((state: RootState) => state.project);
  
  const activeProject = projects.find(p => p.id === projectId);
  const activeFile = files.find(f => f.id === activeFileId);
  const currentFile = files.find(f => f.id === currentFileId);
  
  // State for active tab in the bottom panel
  const [activeTab, setActiveTab] = useState('stack');
  
  // Handle file selection
  const handleFileSelect = (fileId: string) => {
    dispatch(setActiveFile(fileId));
  };
  
  // Handle start debugging
  const handleStartDebugging = () => {
    if (activeFileId) {
      dispatch(startDebugging({ fileId: activeFileId }));
    }
  };
  
  // Handle stop debugging
  const handleStopDebugging = () => {
    dispatch(stopDebugging());
  };
  
  // Handle pause/resume debugging
  const handlePauseResumeDebugging = () => {
    if (isPaused) {
      dispatch(resumeDebugging());
    } else {
      dispatch(pauseDebugging());
    }
  };
  
  // Handle step over
  const handleStepOver = () => {
    dispatch(stepOver());
  };
  
  // Handle step into
  const handleStepInto = () => {
    dispatch(stepInto());
  };
  
  // Handle step out
  const handleStepOut = () => {
    dispatch(stepOut());
  };
  
  // Handle breakpoint toggle
  const handleBreakpointToggle = (lineNumber: number) => {
    if (activeFileId) {
      const existingBreakpoint = breakpoints.find(
        bp => bp.fileId === activeFileId && bp.line === lineNumber
      );
      
      if (existingBreakpoint) {
        dispatch(toggleBreakpoint(existingBreakpoint.id));
      } else {
        dispatch(addBreakpoint({
          id: `bp-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`,
          fileId: activeFileId,
          line: lineNumber,
          enabled: true
        }));
      }
    }
  };
  
  // Get language for Monaco editor based on file extension
  const getLanguage = (fileName: string) => {
    const extension = fileName.split('.').pop()?.toLowerCase();
    
    switch (extension) {
      case 'rs':
        return 'rust';
      case 'js':
        return 'javascript';
      case 'ts':
        return 'typescript';
      case 'json':
        return 'json';
      case 'md':
        return 'markdown';
      default:
        return 'plaintext';
    }
  };
  
  // Format memory address
  const formatAddress = (address: string) => {
    return `0x${address.padStart(4, '0')}`;
  };
  
  // Format memory value
  const formatValue = (value: number) => {
    return `0x${value.toString(16).padStart(2, '0')}`;
  };
  
  // If no project is active, show a message
  if (!activeProject) {
    return (
      <div className="flex items-center justify-center h-full">
        <div className="text-center">
          <h2 className="text-xl font-semibold mb-2">No Project Selected</h2>
          <p className="text-gray-400">Select a project from the sidebar to start debugging.</p>
        </div>
      </div>
    );
  }
  
  return (
    <div className="flex flex-col h-full">
      {/* Toolbar */}
      <div className="bg-dark-800 border-b border-dark-700 p-2 flex items-center">
        <div className="flex items-center space-x-2">
          {!isDebugging ? (
            <button 
              onClick={handleStartDebugging}
              className="p-2 bg-green-600 text-white rounded hover:bg-green-700 transition-colors flex items-center"
              disabled={!activeFileId}
            >
              <FiPlay size={18} className="mr-1" />
              <span>Start</span>
            </button>
          ) : (
            <>
              <button 
                onClick={handleStopDebugging}
                className="p-2 bg-red-600 text-white rounded hover:bg-red-700 transition-colors flex items-center"
              >
                <FiStop size={18} className="mr-1" />
                <span>Stop</span>
              </button>
              
              <button 
                onClick={handlePauseResumeDebugging}
                className={`p-2 ${
                  isPaused 
                    ? 'bg-green-600 hover:bg-green-700' 
                    : 'bg-yellow-600 hover:bg-yellow-700'
                } text-white rounded transition-colors flex items-center`}
              >
                {isPaused ? (
                  <>
                    <FiPlay size={18} className="mr-1" />
                    <span>Resume</span>
                  </>
                ) : (
                  <>
                    <FiPause size={18} className="mr-1" />
                    <span>Pause</span>
                  </>
                )}
              </button>
              
              <button 
                onClick={handleStepOver}
                className="p-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors flex items-center"
                disabled={!isPaused}
              >
                <FiSkipForward size={18} className="mr-1" />
                <span>Step Over</span>
              </button>
              
              <button 
                onClick={handleStepInto}
                className="p-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors flex items-center"
                disabled={!isPaused}
              >
                <FiCornerDownRight size={18} className="mr-1" />
                <span>Step Into</span>
              </button>
              
              <button 
                onClick={handleStepOut}
                className="p-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors flex items-center"
                disabled={!isPaused}
              >
                <FiCornerUpRight size={18} className="mr-1" />
                <span>Step Out</span>
              </button>
            </>
          )}
        </div>
        
        <div className="ml-auto flex items-center space-x-4">
          <div className="flex items-center">
            <span className="text-gray-400 mr-2">Gas:</span>
            <div className="bg-dark-700 rounded px-2 py-1">
              <span className="text-white">{gasUsed}</span>
              <span className="text-gray-400"> / {gasLimit}</span>
            </div>
          </div>
        </div>
      </div>
      
      {/* Main content */}
      <div className="flex-1 flex">
        {/* Editor */}
        <div className="flex-1">
          {activeFile ? (
            <Editor
              height="100%"
              language={getLanguage(activeFile.name)}
              value={activeFile.content}
              theme="vs-dark"
              options={{
                minimap: { enabled: true },
                scrollBeyondLastLine: false,
                fontSize: 14,
                fontFamily: 'IBM Plex Mono, monospace',
                tabSize: 2,
                readOnly: isDebugging,
                lineNumbers: 'on',
                glyphMargin: true,
              }}
              onMount={(editor, monaco) => {
                // Add breakpoint handling
                editor.onMouseDown((e) => {
                  if (e.target.type === monaco.editor.MouseTargetType.GUTTER_GLYPH_MARGIN) {
                    const lineNumber = e.target.position?.lineNumber;
                    if (lineNumber) {
                      handleBreakpointToggle(lineNumber);
                    }
                  }
                });
                
                // Add decorations for breakpoints and current line
                const updateDecorations = () => {
                  const fileBreakpoints = breakpoints.filter(bp => bp.fileId === activeFileId && bp.enabled);
                  
                  const breakpointDecorations = fileBreakpoints.map(bp => ({
                    range: new monaco.Range(bp.line, 1, bp.line, 1),
                    options: {
                      isWholeLine: false,
                      glyphMarginClassName: 'breakpoint-glyph',
                      glyphMarginHoverMessage: { value: 'Breakpoint' },
                    },
                  }));
                  
                  editor.deltaDecorations([], breakpointDecorations);
                  
                  // Add current line decoration if debugging
                  if (isDebugging && currentFileId === activeFileId && currentLine !== null) {
                    const currentLineDecorations = [{
                      range: new monaco.Range(currentLine, 1, currentLine, 1),
                      options: {
                        isWholeLine: true,
                        className: 'current-line-highlight',
                        glyphMarginClassName: 'current-line-glyph',
                      },
                    }];
                    
                    editor.deltaDecorations([], currentLineDecorations);
                    
                    // Scroll to current line
                    editor.revealLineInCenter(currentLine);
                  }
                };
                
                updateDecorations();
                
                // Add CSS for decorations
                const styleElement = document.createElement('style');
                styleElement.textContent = `
                  .breakpoint-glyph {
                    background-color: #e51400;
                    border-radius: 50%;
                    width: 8px !important;
                    height: 8px !important;
                    margin-left: 5px;
                  }
                  .current-line-highlight {
                    background-color: rgba(58, 150, 221, 0.15);
                  }
                  .current-line-glyph {
                    background-color: #3a96dd;
                    width: 0 !important;
                    height: 0 !important;
                    border-left: 6px solid transparent;
                    border-right: 6px solid transparent;
                    border-top: 6px solid #3a96dd;
                    margin-left: 5px;
                  }
                `;
                document.head.appendChild(styleElement);
              }}
            />
          ) : (
            <div className="flex items-center justify-center h-full">
              <div className="text-center">
                <h2 className="text-xl font-semibold mb-2">No File Selected</h2>
                <p className="text-gray-400">Select a file from the explorer to start debugging.</p>
              </div>
            </div>
          )}
        </div>
        
        {/* Debug panels */}
        <div className="w-80 border-l border-dark-700 flex flex-col">
          {/* Tabs */}
          <div className="flex border-b border-dark-700">
            <button 
              className={`flex-1 py-2 px-4 text-sm flex items-center justify-center ${
                activeTab === 'stack' ? 'bg-dark-700 text-white' : 'text-gray-400 hover:text-white'
              }`}
              onClick={() => setActiveTab('stack')}
            >
              <FiList size={16} className="mr-2" />
              Stack
            </button>
            <button 
              className={`flex-1 py-2 px-4 text-sm flex items-center justify-center ${
                activeTab === 'memory' ? 'bg-dark-700 text-white' : 'text-gray-400 hover:text-white'
              }`}
              onClick={() => setActiveTab('memory')}
            >
              <FiDatabase size={16} className="mr-2" />
              Memory
            </button>
            <button 
              className={`flex-1 py-2 px-4 text-sm flex items-center justify-center ${
                activeTab === 'callstack' ? 'bg-dark-700 text-white' : 'text-gray-400 hover:text-white'
              }`}
              onClick={() => setActiveTab('callstack')}
            >
              <FiCpu size={16} className="mr-2" />
              Call Stack
            </button>
          </div>
          
          {/* Panel content */}
          <div className="flex-1 overflow-auto">
            {activeTab === 'stack' && (
              <div className="p-2">
                <h3 className="text-sm font-medium mb-2 text-gray-300">Stack ({stack.length} items)</h3>
                {stack.length > 0 ? (
                  <div className="space-y-1">
                    {stack.map((item, index) => (
                      <div 
                        key={index} 
                        className="flex items-center justify-between p-2 bg-dark-700 rounded"
                      >
                        <div className="flex items-center">
                          <span className="text-gray-400 mr-2">{stack.length - 1 - index}:</span>
                          <span className="text-white font-mono">{item.value}</span>
                        </div>
                        <span className="text-xs text-gray-500">{item.type}</span>
                      </div>
                    ))}
                  </div>
                ) : (
                  <div className="text-gray-500 text-center py-4">
                    Stack is empty
                  </div>
                )}
              </div>
            )}
            
            {activeTab === 'memory' && (
              <div className="p-2">
                <h3 className="text-sm font-medium mb-2 text-gray-300">Memory</h3>
                {Object.keys(memory).length > 0 ? (
                  <div className="grid grid-cols-2 gap-1">
                    {Object.entries(memory).map(([address, data]) => (
                      <div 
                        key={address} 
                        className={`flex items-center justify-between p-2 rounded ${
                          data.lastModified ? 'bg-blue-900 bg-opacity-30' : 'bg-dark-700'
                        }`}
                      >
                        <span className="text-gray-400 font-mono">{formatAddress(address)}</span>
                        <span className="text-white font-mono">{formatValue(data.value)}</span>
                      </div>
                    ))}
                  </div>
                ) : (
                  <div className="text-gray-500 text-center py-4">
                    No memory data available
                  </div>
                )}
              </div>
            )}
            
            {activeTab === 'callstack' && (
              <div className="p-2">
                <h3 className="text-sm font-medium mb-2 text-gray-300">Call Stack</h3>
                {callStack.length > 0 ? (
                  <div className="space-y-1">
                    {callStack.map((frame, index) => (
                      <div 
                        key={index} 
                        className="p-2 bg-dark-700 rounded cursor-pointer hover:bg-dark-600"
                        onClick={() => {
                          dispatch(setActiveFile(frame.fileId));
                        }}
                      >
                        <div className="flex items-center justify-between">
                          <span className="text-white">{frame.functionName}</span>
                          <span className="text-xs text-gray-400">Line {frame.line}</span>
                        </div>
                        <div className="text-xs text-gray-500 mt-1">
                          {files.find(f => f.id === frame.fileId)?.name || 'Unknown file'}
                        </div>
                      </div>
                    ))}
                  </div>
                ) : (
                  <div className="text-gray-500 text-center py-4">
                    Call stack is empty
                  </div>
                )}
              </div>
            )}
          </div>
          
          {/* Logs */}
          <div className="border-t border-dark-700">
            <div className="p-2 flex items-center justify-between">
              <h3 className="text-sm font-medium text-gray-300">Logs</h3>
              <span className="text-xs bg-dark-700 px-2 py-1 rounded text-gray-400">
                {logs.length} entries
              </span>
            </div>
            <div className="h-32 overflow-auto p-2 bg-dark-900 font-mono text-xs">
              {logs.length > 0 ? (
                <div className="space-y-1">
                  {logs.map((log, index) => (
                    <div 
                      key={index} 
                      className={`flex items-start ${
                        log.type === 'error' 
                          ? 'text-red-400' 
                          : log.type === 'warning' 
                            ? 'text-yellow-400' 
                            : 'text-gray-300'
                      }`}
                    >
                      <span className="mr-2">
                        {log.type === 'error' && <FiAlertCircle size={14} />}
                        {log.type === 'warning' && <FiAlertCircle size={14} />}
                      </span>
                      <div>
                        <span className="text-gray-500">[{new Date(log.timestamp).toLocaleTimeString()}]</span>{' '}
                        {log.message}
                      </div>
                    </div>
                  ))}
                </div>
              ) : (
                <div className="text-gray-500 text-center py-2">
                  No logs to display
                </div>
              )}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default DebuggerPage;