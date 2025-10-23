import { useTranslation } from 'react-i18next';
import { Globe } from 'lucide-react';

export function LanguageSwitcher() {
  const { i18n } = useTranslation();

  const toggleLanguage = () => {
    const newLang = i18n.language === 'ru' ? 'en' : 'ru';
    i18n.changeLanguage(newLang);
    localStorage.setItem('language', newLang);
  };

  return (
    <button
      onClick={toggleLanguage}
      className="btn btn-secondary"
      style={{
        display: 'flex',
        alignItems: 'center',
        gap: '0.5rem',
        padding: '0.5rem 1rem',
      }}
      title={i18n.language === 'en' ? 'Switch to Russian' : 'Switch to English'}
    >
      <Globe size={18} />
      {i18n.language === 'ru' ? 'EN' : 'RU'}
    </button>
  );
}

