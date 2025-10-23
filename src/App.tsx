import { useState, useEffect } from "react";
import { useTranslation } from "react-i18next";
import "./App.css";
import "./styles/nodes.css";
import "./styles/modal.css";
import "./styles/scrollbar.css";
import { Dashboard } from "./components/Dashboard";
import { DeviceList } from "./components/DeviceList";
import { ProfileList } from "./components/ProfileList";
import { PatternManager } from "./components/PatternManager";
import { PatternEditorModal } from "./components/PatternEditorModal";
import { DebugConsole } from "./components/DebugConsole";
import { LanguageSwitcher } from "./components/LanguageSwitcher";
import { api } from "./api";
import { usePatterns, Pattern } from "./hooks/usePatterns";

function App() {
  const { t } = useTranslation();
  const [isRunning, setIsRunning] = useState(false);
  const [isEditorOpen, setIsEditorOpen] = useState(false);
  const [editingPattern, setEditingPattern] = useState<Pattern | undefined>();
  
  const { addPattern, updatePattern } = usePatterns();

  // Проверка статуса системы (НЕ автоподключение!)
  useEffect(() => {
    const interval = setInterval(async () => {
      try {
        const running = await api.isRunning();
        setIsRunning(running);
      } catch (error) {
        // Тихо игнорируем
      }
    }, 3000);

    return () => clearInterval(interval);
  }, []);

  const handleEditPattern = (pattern?: Pattern) => {
    setEditingPattern(pattern);
    setIsEditorOpen(true);
  };

  const handleSavePattern = (name: string, nodes: any[], edges: any[]) => {
    if (editingPattern) {
      updatePattern(editingPattern.id, { name, nodes, edges });
    } else {
      addPattern(name, nodes, edges);
    }
  };

  return (
    <div className="app-container">
      {/* Современный хедер */}
      <header className="app-header">
        <div className="header-content">
          <div className="header-brand">
            <div className="brand-icon">⚡</div>
            <div className="brand-info">
              <h1>{t('app.title')}</h1>
              <span className="brand-subtitle">{t('app.subtitle')}</span>
            </div>
          </div>
          
          <div className="header-actions">
            <LanguageSwitcher />
            <div className={`status-chip ${isRunning ? 'running' : 'stopped'}`}>
              <span className="status-indicator"></span>
              <span className="status-text">
                {isRunning ? t('header.status_active') : t('header.status_stopped')}
              </span>
            </div>
          </div>
        </div>
      </header>

      {/* Контент с сеткой */}
      <main className="app-main">
        <div className="content-grid">
          {/* Левая панель - управление */}
          <aside className="sidebar-left">
            <Dashboard />
            <DeviceList />
          </aside>

          {/* Центральная область - паттерны и профили */}
          <section className="main-area">
            <PatternManager onEditPattern={handleEditPattern} />
            <ProfileList />
          </section>
        </div>
      </main>

      {/* Консоль отладки */}
      <DebugConsole />

      {/* Модальное окно редактора */}
      <PatternEditorModal
        isOpen={isEditorOpen}
        onClose={() => {
          setIsEditorOpen(false);
          setEditingPattern(undefined);
        }}
        onSave={handleSavePattern}
        initialData={editingPattern}
      />
    </div>
  );
}

export default App;
