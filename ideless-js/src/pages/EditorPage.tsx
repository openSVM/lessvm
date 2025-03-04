import { useState, useEffect } from 'react';
import { useSelector, useDispatch } from 'react-redux';
import { useParams } from 'react-router-dom';
import { RootState } from '../store';
import { setActiveFile, updateFile } from '../store/slices/editorSlice';
import { FiCode, FiFile, FiFolder, FiChevronRight, FiChevronDown, FiPlus, FiTrash2 } from 'react-icons/fi';
import Editor from '@monaco-editor/react';

const EditorPage = () => {
  const dispatch = useDispatch();
  const { projectId } = useParams<{ projectId: string }>();
  const { files, activeFileId } = useSelector((state: RootState) => state.editor);
  const { projects } = useSelector((state: RootState) => state.project);
  
  const activeProject = projects.find(p => p.id === projectId);
  const activeFile = files.find(f => f.id === activeFileId);
  
  // State for file explorer
  const [expandedFolders, setExpandedFolders] = useState<string[]>(['/']);
  
  // Handle file selection
  const handleFileSelect = (fileId: string) => {
    dispatch(setActiveFile(fileId));
  };
  
  // Handle file content change
  const handleEditorChange = (value: string | undefined) => {
    if (activeFileId && value !== undefined) {
      dispatch(updateFile({ id: activeFileId, content: value }));
    }
  };
  
  // Toggle folder expansion
  const toggleFolder = (path: string) => {
    if (expandedFolders.includes(path)) {
      setExpandedFolders(expandedFolders.filter(f => f !== path));
    } else {
      setExpandedFolders([...expandedFolders, path]);
    }
  };
  
  // Get file icon based on file extension
  const getFileIcon = (fileName: string) => {
    const extension = fileName.split('.').pop()?.toLowerCase();
    
    switch (extension) {
      case 'rs':
        return <FiCode className="text-orange-400" />;
      case 'js':
      case 'ts':
        return <FiCode className="text-yellow-400" />;
      case 'json':
        return <FiCode className="text-green-400" />;
      case 'md':
        return <FiCode className="text-blue-400" />;
      default:
        return <FiFile className="text-gray-400" />;
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
  
  // Organize files into a tree structure
  const organizeFiles = () => {
    const fileTree: { [key: string]: any } = {};
    
    files.forEach(file => {
      const pathParts = file.path.split('/').filter(Boolean);
      let currentLevel = fileTree;
      
      // Create folder structure
      for (let i = 0; i < pathParts.length - 1; i++) {
        const part = pathParts[i];
        if (!currentLevel[part]) {
          currentLevel[part] = { __isFolder: true, __children: {} };
        }
        currentLevel = currentLevel[part].__children;
      }
      
      // Add file
      const fileName = pathParts[pathParts.length - 1];
      currentLevel[fileName] = { __isFile: true, __fileData: file };
    });
    
    return fileTree;
  };
  
  // Render file tree recursively
  const renderFileTree = (tree: { [key: string]: any }, path = '') => {
    return Object.keys(tree).map(key => {
      const item = tree[key];
      const itemPath = `${path}/${key}`;
      
      if (item.__isFolder) {
        const isExpanded = expandedFolders.includes(itemPath);
        
        return (
          <div key={itemPath}>
            <div 
              className="flex items-center py-1 px-2 hover:bg-dark-700 rounded cursor-pointer"
              onClick={() => toggleFolder(itemPath)}
            >
              {isExpanded ? (
                <FiChevronDown className="mr-1 text-gray-400" size={14} />
              ) : (
                <FiChevronRight className="mr-1 text-gray-400" size={14} />
              )}
              <FiFolder className="mr-2 text-yellow-400" size={16} />
              <span className="text-gray-300">{key}</span>
            </div>
            
            {isExpanded && (
              <div className="ml-4">
                {renderFileTree(item.__children, itemPath)}
              </div>
            )}
          </div>
        );
      } else if (item.__isFile) {
        const file = item.__fileData;
        
        return (
          <div 
            key={file.id}
            className={`flex items-center py-1 px-2 hover:bg-dark-700 rounded cursor-pointer ${
              file.id === activeFileId ? 'bg-dark-700' : ''
            }`}
            onClick={() => handleFileSelect(file.id)}
          >
            <div className="mr-2">{getFileIcon(file.name)}</div>
            <span className="text-gray-300">{file.name}</span>
          </div>
        );
      }
      
      return null;
    });
  };
  
  // If no project is active, show a message
  if (!activeProject) {
    return (
      <div className="flex items-center justify-center h-full">
        <div className="text-center">
          <h2 className="text-xl font-semibold mb-2">No Project Selected</h2>
          <p className="text-gray-400">Select a project from the sidebar to start editing.</p>
        </div>
      </div>
    );
  }
  
  return (
    <div className="flex h-full">
      {/* File Explorer */}
      <div className="w-64 bg-dark-800 border-r border-dark-700 flex flex-col">
        <div className="flex items-center justify-between p-3 border-b border-dark-700">
          <h3 className="font-medium">Explorer</h3>
          <button className="text-gray-400 hover:text-white transition-colors">
            <FiPlus size={18} />
          </button>
        </div>
        
        <div className="flex-1 overflow-auto p-2">
          {renderFileTree(organizeFiles())}
          
          {files.length === 0 && (
            <div className="text-center py-4 text-gray-500">
              <p>No files in project</p>
              <button className="mt-2 px-3 py-1 bg-primary-600 text-white rounded hover:bg-primary-700 transition-colors">
                Add File
              </button>
            </div>
          )}
        </div>
      </div>
      
      {/* Editor */}
      <div className="flex-1 flex flex-col">
        {activeFile ? (
          <Editor
            height="100%"
            language={getLanguage(activeFile.name)}
            value={activeFile.content}
            onChange={handleEditorChange}
            theme="vs-dark"
            options={{
              minimap: { enabled: true },
              scrollBeyondLastLine: false,
              fontSize: 14,
              fontFamily: 'IBM Plex Mono, monospace',
              tabSize: 2,
            }}
          />
        ) : (
          <div className="flex items-center justify-center h-full">
            <div className="text-center">
              <h2 className="text-xl font-semibold mb-2">No File Selected</h2>
              <p className="text-gray-400">Select a file from the explorer to start editing.</p>
            </div>
          </div>
        )}
      </div>
    </div>
  );
};

export default EditorPage;