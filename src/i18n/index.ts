import i18n from 'i18next';
import { initReactI18next } from 'react-i18next';
import ruTranslations from './locales/ru.json';
import enTranslations from './locales/en.json';

// Get language from localStorage, browser, or use Russian as default
const browserLanguage = navigator.language.toLowerCase();
const defaultLanguage = browserLanguage.startsWith('ru') || browserLanguage.startsWith('be') || browserLanguage.startsWith('uk') ? 'ru' : 'en';
const savedLanguage = localStorage.getItem('language') || defaultLanguage;

i18n
  .use(initReactI18next)
  .init({
    resources: {
      ru: { translation: ruTranslations },
      en: { translation: enTranslations },
    },
    lng: savedLanguage,
    fallbackLng: 'en',
    interpolation: {
      escapeValue: false, // React already protects from XSS
    },
  });

export default i18n;
