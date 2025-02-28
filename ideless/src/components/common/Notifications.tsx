import { useEffect } from 'react';
import { useSelector, useDispatch } from 'react-redux';
import { RootState } from '../../store';
import { FiX, FiInfo, FiAlertCircle, FiAlertTriangle, FiCheckCircle } from 'react-icons/fi';
import { markNotificationAsRead } from '../../store/slices/uiSlice';

const Notifications = () => {
  const dispatch = useDispatch();
  const { notifications } = useSelector((state: RootState) => state.ui);
  
  // Auto-dismiss notifications after 5 seconds
  useEffect(() => {
    const timers: NodeJS.Timeout[] = [];
    
    notifications.forEach(notification => {
      if (!notification.read) {
        const timer = setTimeout(() => {
          dispatch(markNotificationAsRead(notification.id));
        }, 5000);
        
        timers.push(timer);
      }
    });
    
    return () => {
      timers.forEach(timer => clearTimeout(timer));
    };
  }, [notifications, dispatch]);
  
  // Get icon based on notification type
  const getIcon = (type: 'info' | 'success' | 'warning' | 'error') => {
    switch (type) {
      case 'info':
        return <FiInfo size={20} className="text-blue-400" />;
      case 'success':
        return <FiCheckCircle size={20} className="text-green-400" />;
      case 'warning':
        return <FiAlertTriangle size={20} className="text-yellow-400" />;
      case 'error':
        return <FiAlertCircle size={20} className="text-red-400" />;
      default:
        return <FiInfo size={20} className="text-blue-400" />;
    }
  };
  
  // Get background color based on notification type
  const getBackgroundColor = (type: 'info' | 'success' | 'warning' | 'error') => {
    switch (type) {
      case 'info':
        return 'bg-blue-500 bg-opacity-10 border-blue-500';
      case 'success':
        return 'bg-green-500 bg-opacity-10 border-green-500';
      case 'warning':
        return 'bg-yellow-500 bg-opacity-10 border-yellow-500';
      case 'error':
        return 'bg-red-500 bg-opacity-10 border-red-500';
      default:
        return 'bg-blue-500 bg-opacity-10 border-blue-500';
    }
  };
  
  // Only show unread notifications
  const unreadNotifications = notifications.filter(notification => !notification.read);
  
  if (unreadNotifications.length === 0) {
    return null;
  }
  
  return (
    <div className="fixed top-4 right-4 z-50 space-y-2 max-w-md">
      {unreadNotifications.map(notification => (
        <div 
          key={notification.id}
          className={`flex items-start p-3 rounded-lg border ${getBackgroundColor(notification.type)} shadow-lg transition-all duration-300 animate-fadeIn`}
        >
          <div className="flex-shrink-0 mr-3 mt-0.5">
            {getIcon(notification.type)}
          </div>
          <div className="flex-1 mr-2">
            <p className="text-white">{notification.message}</p>
            <p className="text-xs text-gray-400 mt-1">
              {new Date(notification.timestamp).toLocaleTimeString()}
            </p>
          </div>
          <button
            onClick={() => dispatch(markNotificationAsRead(notification.id))}
            className="text-gray-400 hover:text-white transition-colors"
          >
            <FiX size={18} />
          </button>
        </div>
      ))}
    </div>
  );
};

export default Notifications;