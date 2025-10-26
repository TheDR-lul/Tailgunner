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
import { MiniMap } from "./components/MiniMap";
import { VehicleModeCard } from "./components/VehicleModeCard";
import { GameChat } from "./components/GameChat";
import { MissionInfo } from "./components/MissionInfo";
import { User, Coffee } from "lucide-react";
import { api } from "./api";
import { usePatterns, Pattern } from "./hooks/usePatterns";

function App() {
  const { t } = useTranslation();
  const [isRunning, setIsRunning] = useState(false);
  const [isEditorOpen, setIsEditorOpen] = useState(false);
  const [isPlayerModalOpen, setIsPlayerModalOpen] = useState(false);
  const [editingPattern, setEditingPattern] = useState<Pattern | undefined>();
  const [supportHighlight, setSupportHighlight] = useState(false);
  
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

  // Periodic support button highlight (every 2 minutes)
  useEffect(() => {
    const highlightInterval = setInterval(() => {
      setSupportHighlight(true);
      setTimeout(() => setSupportHighlight(false), 3000); // Highlight for 3 seconds
    }, 120000); // Every 2 minutes

    return () => clearInterval(highlightInterval);
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

  const handleSupportClick = async () => {
    try {
      const { openUrl } = await import('@tauri-apps/plugin-opener');
      await openUrl('https://buymeacoffee.com/wingsofprey');
    } catch (error) {
      console.error('Failed to open link:', error);
      // Fallback to window.open
      window.open('https://buymeacoffee.com/wingsofprey', '_blank');
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
              onClick={handleSupportClick}
              className={`support-btn ${supportHighlight ? 'highlight' : ''}`}
              title="Support Developer"
            >
              <Coffee size={18} />
            </button>
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
            <VehicleModeCard />
            <VehicleInfoCard />
          </aside>

              {/* Main area - patterns and events */}
              <section className="main-area">
                <PatternManager onEditPattern={handleEditPattern} />
                <EventConfiguration />
                <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '16px' }}>
                  <MiniMap />
                  <div style={{ display: 'flex', flexDirection: 'column', gap: '16px' }}>
                    <MissionInfo />
                    <GameChat />
                  </div>
                </div>
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
