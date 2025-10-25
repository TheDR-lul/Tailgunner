import { useState, useEffect } from "react";
import { useTranslation } from "react-i18next";
import "./App.css";
import "./styles/nodes.css";
import "./styles/modal.css";
import "./styles/scrollbar.css";
import { Dashboard } from "./components/Dashboard";
import { DeviceList } from "./components/DeviceList";
import { PatternManager } from "./components/PatternManager";
import { PatternEditorModal } from "./components/PatternEditorModal";
import { DebugConsole } from "./components/DebugConsole";
import { LanguageSwitcher } from "./components/LanguageSwitcher";
import { GameStatus } from "./components/GameStatus";
import { EventConfiguration } from "./components/EventConfiguration";
import { VehicleInfoCard } from "./components/VehicleInfoCard";
import { PlayerIdentityModal } from "./components/PlayerIdentityModal";
import { User } from "lucide-react";
import { api } from "./api";
import { usePatterns, Pattern } from "./hooks/usePatterns";

function App() {
  const { t } = useTranslation();
  const [isRunning, setIsRunning] = useState(false);
  const [isEditorOpen, setIsEditorOpen] = useState(false);
  const [isPlayerModalOpen, setIsPlayerModalOpen] = useState(false);
  const [editingPattern, setEditingPattern] = useState<Pattern | undefined>();
  
  const { addPattern, updatePattern } = usePatterns();

  // Auto-initialize datamine on startup
  useEffect(() => {
    const initDatamine = async () => {
      try {
        console.log("[Datamine] Auto-initializing...");
        const result = await api.datamineAutoInit();
        console.log("[Datamine]", result);
      } catch (error) {
        console.warn("[Datamine] Auto-init failed:", error);
      }
    };
    
    initDatamine();
  }, []);

  // Check system status (NOT auto-connect!)
  useEffect(() => {
    const interval = setInterval(async () => {
      try {
        const running = await api.isRunning();
        setIsRunning(running);
      } catch (error) {
        // Silently ignore
      }
    }, 3000);

    return () => clearInterval(interval);
  }, []);

  const handleEditPattern = (pattern?: Pattern) => {
    // Explicitly set to undefined when creating new pattern, or to the pattern when editing
    setEditingPattern(pattern || undefined);
    setIsEditorOpen(true);
  };

  const handleSavePattern = async (name: string, nodes: any[], edges: any[]) => {
    if (editingPattern) {
      await updatePattern(editingPattern.id, { name, nodes, edges });
    } else {
      await addPattern(name, nodes, edges);
    }
  };

  return (
    <div className="app-container">
      {/* Modern header */}
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
            <button
              onClick={() => setIsPlayerModalOpen(true)}
              className="player-identity-btn"
              title="Player Identity"
            >
              <User size={18} />
            </button>
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

      {/* Content grid */}
      <main className="app-main">
        <div className="content-grid">
          {/* Left panel - controls */}
          <aside className="sidebar-left">
            <Dashboard />
            <DeviceList />
            <GameStatus />
            <VehicleInfoCard />
          </aside>

              {/* Main area - patterns and events */}
              <section className="main-area">
                <PatternManager onEditPattern={handleEditPattern} />
                <EventConfiguration />
              </section>
            </div>
          </main>

      {/* Debug console */}
      <DebugConsole />

      {/* Pattern editor modal */}
      <PatternEditorModal
        isOpen={isEditorOpen}
        onClose={() => {
          setIsEditorOpen(false);
          setEditingPattern(undefined);
        }}
        onSave={handleSavePattern}
        initialData={editingPattern}
      />

      {/* Player identity modal */}
      <PlayerIdentityModal 
        isOpen={isPlayerModalOpen}
        onClose={() => setIsPlayerModalOpen(false)}
      />
    </div>
  );
}

export default App;
