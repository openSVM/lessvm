import { useEffect } from 'react';
import { useSelector, useDispatch } from 'react-redux';
import { RootState } from '../../store';
import { closeModal } from '../../store/slices/uiSlice';
import { FiX } from 'react-icons/fi';

// Modal Components
import NewProjectModal from '../modals/NewProjectModal';
import SettingsModal from '../modals/SettingsModal';
import DeploymentModal from '../modals/DeploymentModal';
import ConfirmationModal from '../modals/ConfirmationModal';

const ModalContainer = () => {
  const dispatch = useDispatch();
  const { modal } = useSelector((state: RootState) => state.ui);
  
  // Close modal on escape key
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'Escape' && modal.open) {
        dispatch(closeModal());
      }
    };
    
    window.addEventListener('keydown', handleKeyDown);
    
    return () => {
      window.removeEventListener('keydown', handleKeyDown);
    };
  }, [modal.open, dispatch]);
  
  // Prevent scrolling when modal is open
  useEffect(() => {
    if (modal.open) {
      document.body.style.overflow = 'hidden';
    } else {
      document.body.style.overflow = 'auto';
    }
    
    return () => {
      document.body.style.overflow = 'auto';
    };
  }, [modal.open]);
  
  if (!modal.open) {
    return null;
  }
  
  // Render the appropriate modal based on type
  const renderModal = () => {
    switch (modal.type) {
      case 'new-project':
        return <NewProjectModal />;
      case 'settings':
        return <SettingsModal />;
      case 'deployment':
        return <DeploymentModal />;
      case 'confirmation':
        return <ConfirmationModal data={modal.data} />;
      default:
        return (
          <div className="p-6">
            <h2 className="text-xl font-semibold mb-4">Unknown Modal Type</h2>
            <p>Modal type "{modal.type}" is not recognized.</p>
          </div>
        );
    }
  };
  
  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black bg-opacity-50">
      <div className="bg-dark-800 rounded-lg shadow-xl max-w-2xl w-full max-h-[90vh] flex flex-col">
        {/* Modal Header */}
        <div className="flex items-center justify-between p-4 border-b border-dark-700">
          <h2 className="text-xl font-semibold text-white">
            {modal.type === 'new-project' && 'Create New Project'}
            {modal.type === 'settings' && 'Settings'}
            {modal.type === 'deployment' && 'Deploy Project'}
            {modal.type === 'confirmation' && 'Confirmation'}
          </h2>
          <button
            onClick={() => dispatch(closeModal())}
            className="text-gray-400 hover:text-white transition-colors"
          >
            <FiX size={24} />
          </button>
        </div>
        
        {/* Modal Content */}
        <div className="flex-1 overflow-auto">
          {renderModal()}
        </div>
      </div>
    </div>
  );
};

export default ModalContainer;