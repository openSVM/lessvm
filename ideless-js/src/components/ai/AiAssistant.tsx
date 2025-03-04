import { useState, useRef, useEffect } from 'react';
import { useSelector, useDispatch } from 'react-redux';
import { RootState } from '../../store';
import { FiSend, FiX, FiMaximize2, FiMinimize2, FiMessageSquare, FiSettings } from 'react-icons/fi';
import AIAgentIntegration from './AIAgentIntegration';
import { setSolanaConfig, setSonicConfig, setInitialized } from '../../store/slices/agentSlice';
import agentService from '../../services/agent/AgentService';

interface WalletConfig {
  privateKey: string;
  rpcUrl: string;
  openaiApiKey?: string;
}

const AiAssistant = () => {
  const dispatch = useDispatch();
  const [input, setInput] = useState('');
  const [messages, setMessages] = useState<{ role: 'user' | 'assistant'; content: string }[]>([
    { 
      role: 'assistant', 
      content: 'Hello! I\'m your LessVM AI Assistant. I can help you with coding, debugging, and deploying your LessVM applications. How can I assist you today?' 
    }
  ]);
  const [isTyping, setIsTyping] = useState(false);
  const messagesEndRef = useRef<HTMLDivElement>(null);
  const { aiAssistantWidth } = useSelector((state: RootState) => state.ui);
  const { activeProjectId, projects } = useSelector((state: RootState) => state.project);
  const { activeTab } = useSelector((state: RootState) => state.ui);
  
  const activeProject = projects.find(p => p.id === activeProjectId);
  
  // Scroll to bottom of messages
  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages]);
  
  // Settings for wallet integration
  const [showSettings, setShowSettings] = useState(false);
  const [solanaPrivateKey, setSolanaPrivateKey] = useState('');
  const [solanaRpcUrl, setSolanaRpcUrl] = useState('https://api.mainnet-beta.solana.com');
  const [sonicPrivateKey, setSonicPrivateKey] = useState('');
  const [sonicRpcUrl, setSonicRpcUrl] = useState('https://api.testnet.sonic.game');
  const [openaiApiKey, setOpenaiApiKey] = useState('');

  // Handle sending a message
  const handleSendMessage = (inputMessage: string = input) => {
    if (!inputMessage.trim()) return;
    
    // Add user message
    setMessages(prev => [...prev, { role: 'user', content: inputMessage }]);
    setInput('');
    
    // Simulate AI typing
    setIsTyping(true);
    
    // Get contextual response based on active tab and project
    setTimeout(() => {
      let response = '';
      
      if (activeTab === 'editor') {
        response = 'I see you\'re working in the editor. Would you like help with writing LessVM code, understanding the instruction set, or optimizing your code?';
      } else if (activeTab === 'debugger') {
        response = 'I notice you\'re in the debugger. I can help you understand what\'s happening with your code, identify issues, or explain how to use the debugging tools.';
      } else if (activeTab === 'simulator') {
        response = 'You\'re in the simulator. I can help you set up test scenarios, understand the execution results, or optimize your code for better performance.';
      } else if (activeTab === 'deploy') {
        response = 'I see you\'re preparing to deploy. I can help you understand the deployment process, configure your deployment settings, or troubleshoot deployment issues.';
      } else {
        response = 'How can I help you with your LessVM development today?';
      }
      
      setMessages(prev => [...prev, { role: 'assistant', content: response }]);
      setIsTyping(false);
    }, 1000);
  };

  // Connect wallet handlers
  const handleConnectSolana = () => {
    if (!solanaPrivateKey) {
      return;
    }

    try {
      const config = {
        privateKey: solanaPrivateKey,
        rpcUrl: solanaRpcUrl,
        openaiApiKey: openaiApiKey || undefined
      };

      dispatch(setSolanaConfig(config));
      agentService.initSolanaAgent(config);
      dispatch(setInitialized({ blockchain: 'solana', isInitialized: true }));
      
      // Add success message
      setMessages(prev => [...prev, { 
        role: 'assistant', 
        content: 'Successfully connected to Solana. You can now use Solana blockchain commands in this chat.' 
      }]);
      
      setShowSettings(false);
    } catch (error) {
      console.error('Failed to connect Solana wallet:', error);
      setMessages(prev => [...prev, { 
        role: 'assistant', 
        content: `Failed to connect Solana wallet: ${error instanceof Error ? error.message : String(error)}` 
      }]);
    }
  };

  const handleConnectSonic = () => {
    if (!sonicPrivateKey) {
      return;
    }

    try {
      const config = {
        privateKey: sonicPrivateKey,
        rpcUrl: sonicRpcUrl,
        openaiApiKey: openaiApiKey || undefined
      };

      dispatch(setSonicConfig(config));
      agentService.initSonicAgent(config);
      dispatch(setInitialized({ blockchain: 'sonic', isInitialized: true }));
      
      // Add success message
      setMessages(prev => [...prev, { 
        role: 'assistant', 
        content: 'Successfully connected to Sonic. You can now use Sonic blockchain commands in this chat.' 
      }]);
      
      setShowSettings(false);
    } catch (error) {
      console.error('Failed to connect Sonic wallet:', error);
      setMessages(prev => [...prev, { 
        role: 'assistant', 
        content: `Failed to connect Sonic wallet: ${error instanceof Error ? error.message : String(error)}` 
      }]);
    }
  };
  
  // Handle key press (Enter to send)
  const handleKeyPress = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      handleSendMessage();
    }
  };
  
  return (
    <div 
      className="ai-assistant bg-dark-900 border-l border-dark-700 flex flex-col"
      style={{ width: `${aiAssistantWidth}px` }}
    >
      {/* Header */}
      <div className="flex items-center justify-between p-3 border-b border-dark-700">
        <div className="flex items-center">
          <FiMessageSquare className="text-primary-400 mr-2" size={18} />
          <h3 className="text-white font-medium">AI Assistant</h3>
        </div>
        <div className="flex items-center">
          <button 
            className="p-1 text-gray-400 hover:text-white transition-colors mr-1"
            onClick={() => setShowSettings(!showSettings)}
            title="Agent Settings"
          >
            <FiSettings size={16} />
          </button>
          <button className="p-1 text-gray-400 hover:text-white transition-colors">
            <FiMinimize2 size={16} />
          </button>
          <button className="p-1 text-gray-400 hover:text-white transition-colors ml-1">
            <FiX size={16} />
          </button>
        </div>
      </div>
      
      {/* Settings Panel */}
      {showSettings && (
        <div className="settings-panel bg-dark-800 p-3 border-b border-dark-700">
          <h4 className="text-primary-400 text-sm font-medium mb-2">Agent Settings</h4>
          
          <div className="mb-4">
            <h5 className="text-white text-xs font-medium mb-2">Solana</h5>
            <input
              type="password"
              placeholder="Solana Private Key"
              value={solanaPrivateKey}
              onChange={(e) => setSolanaPrivateKey(e.target.value)}
              className="w-full bg-dark-700 border border-dark-600 rounded p-2 text-sm text-gray-200 mb-2"
            />
            <input
              type="text"
              placeholder="Solana RPC URL"
              value={solanaRpcUrl}
              onChange={(e) => setSolanaRpcUrl(e.target.value)}
              className="w-full bg-dark-700 border border-dark-600 rounded p-2 text-sm text-gray-200 mb-2"
            />
            <button
              onClick={handleConnectSolana}
              disabled={!solanaPrivateKey}
              className="bg-primary-600 text-white rounded px-3 py-1 text-sm disabled:opacity-50"
            >
              Connect Solana Wallet
            </button>
          </div>
          
          <div className="mb-4">
            <h5 className="text-white text-xs font-medium mb-2">Sonic</h5>
            <input
              type="password"
              placeholder="Sonic Private Key"
              value={sonicPrivateKey}
              onChange={(e) => setSonicPrivateKey(e.target.value)}
              className="w-full bg-dark-700 border border-dark-600 rounded p-2 text-sm text-gray-200 mb-2"
            />
            <input
              type="text"
              placeholder="Sonic RPC URL"
              value={sonicRpcUrl}
              onChange={(e) => setSonicRpcUrl(e.target.value)}
              className="w-full bg-dark-700 border border-dark-600 rounded p-2 text-sm text-gray-200 mb-2"
            />
            <button
              onClick={handleConnectSonic}
              disabled={!sonicPrivateKey}
              className="bg-primary-600 text-white rounded px-3 py-1 text-sm disabled:opacity-50"
            >
              Connect Sonic Wallet
            </button>
          </div>
          
          <div className="mb-2">
            <h5 className="text-white text-xs font-medium mb-2">OpenAI</h5>
            <input
              type="password"
              placeholder="OpenAI API Key (Optional)"
              value={openaiApiKey}
              onChange={(e) => setOpenaiApiKey(e.target.value)}
              className="w-full bg-dark-700 border border-dark-600 rounded p-2 text-sm text-gray-200"
            />
          </div>
        </div>
      )}
      
      {/* Messages */}
      <div className="flex-1 overflow-auto p-4 space-y-4">
        {messages.map((message, index) => (
          <div 
            key={index} 
            className={`flex ${message.role === 'user' ? 'justify-end' : 'justify-start'}`}
          >
            <div 
              className={`max-w-[80%] rounded-lg p-3 ${
                message.role === 'user' 
                  ? 'bg-primary-600 text-white' 
                  : 'bg-dark-700 text-gray-200'
              }`}
            >
              {message.content}
            </div>
          </div>
        ))}
        
        {isTyping && (
          <div className="flex justify-start">
            <div className="bg-dark-700 text-gray-200 rounded-lg p-3 max-w-[80%]">
              <div className="flex space-x-2">
                <div className="w-2 h-2 rounded-full bg-gray-400 animate-bounce"></div>
                <div className="w-2 h-2 rounded-full bg-gray-400 animate-bounce delay-100"></div>
                <div className="w-2 h-2 rounded-full bg-gray-400 animate-bounce delay-200"></div>
              </div>
            </div>
          </div>
        )}
        
        <div ref={messagesEndRef} />
      </div>
      
      {/* Blockchain Agent Panel */}
      <AIAgentIntegration onSendMessage={handleSendMessage} />
      
      {/* Input */}
      <div className="p-3 border-t border-dark-700">
        <div className="flex items-center bg-dark-700 rounded-lg">
          <textarea
            value={input}
            onChange={(e) => setInput(e.target.value)}
            onKeyPress={handleKeyPress}
            placeholder="Ask me anything about LessVM..."
            className="flex-1 bg-transparent border-none outline-none p-3 text-gray-200 resize-none"
            rows={1}
          />
          <button 
            onClick={() => handleSendMessage()}
            className="p-3 text-primary-400 hover:text-primary-300 transition-colors"
            disabled={!input.trim()}
          >
            <FiSend size={18} />
          </button>
        </div>
        
        {/* Context indicator */}
        {activeProject && (
          <div className="mt-2 text-xs text-gray-500">
            Context: {activeProject.name} {activeTab && `/ ${activeTab}`}
          </div>
        )}
      </div>
    </div>
  );
};

export default AiAssistant;
