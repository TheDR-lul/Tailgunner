import { useTranslation } from 'react-i18next';
import './ConfirmDialog.css';

interface ConfirmDialogProps {
  isOpen: boolean;
  title?: string;
  message: string;
  onConfirm: () => void;
  onCancel: () => void;
}

export function ConfirmDialog({ isOpen, title, message, onConfirm, onCancel }: ConfirmDialogProps) {
  const { t } = useTranslation();

  if (!isOpen) return null;

  return (
    <div className="confirm-overlay" onClick={onCancel}>
      <div className="confirm-dialog" onClick={(e) => e.stopPropagation()}>
        {title && <h3 className="confirm-title">{title}</h3>}
        <p className="confirm-message">{message}</p>
        <div className="confirm-actions">
          <button className="btn btn-secondary" onClick={onCancel}>
            {t('common.cancel')}
          </button>
          <button className="btn btn-danger" onClick={onConfirm}>
            {t('common.delete')}
          </button>
        </div>
      </div>
    </div>
  );
}


