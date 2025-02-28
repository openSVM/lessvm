import { useState, useRef, useEffect } from 'react';
import { useSelector } from 'react-redux';
import { RootState } from '../../store';
import { FiSend, FiX, FiMaximize2, FiMinimize2, FiMessageSquare } from 'react-icons/fi';

const AiAssistant = () => {
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
  
  // Handle sending a message
  const handleSendMessage = () => {
    if (!input.trim()) return;
    
    // Add user message
    setMessages(prev => [...prev, { role: 'user', content: input }]);
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
          <button className="p-1 text-gray-400 hover:text-white transition-colors">
            <FiMinimize2 size={16} />
          </button>
          <button className="p-1 text-gray-400 hover:text-white transition-colors ml-1">
            <FiX size={16} />
          </button>
        </div>
      </div>
      
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
            onClick={handleSendMessage}
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