import { useState, useEffect } from 'react';
import { X, Plus, Trash2, User, Users } from 'lucide-react';
import { api } from '../api';

interface PlayerIdentityModalProps {
  isOpen: boolean;
  onClose: () => void;
}

export function PlayerIdentityModal({ isOpen, onClose }: PlayerIdentityModalProps) {
  const [playerNames, setPlayerNames] = useState<string[]>([]);
  const [clanTags, setClanTags] = useState<string[]>([]);
  const [enemyNames, setEnemyNames] = useState<string[]>([]);
  const [enemyClans, setEnemyClans] = useState<string[]>([]);
  const [newPlayerName, setNewPlayerName] = useState('');
  const [newClanTag, setNewClanTag] = useState('');
  const [newEnemyName, setNewEnemyName] = useState('');
  const [newEnemyClan, setNewEnemyClan] = useState('');
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    if (isOpen) {
      loadData();
    }
  }, [isOpen]);

  const loadData = async () => {
    try {
      const [names, tags, enemyNamesList, enemyClansList] = await Promise.all([
        api.getPlayerNames(),
        api.getClanTags(),
        api.getEnemyNames(),
        api.getEnemyClans()
      ]);
      setPlayerNames(names);
      setClanTags(tags);
      setEnemyNames(enemyNamesList);
      setEnemyClans(enemyClansList);
    } catch (error) {
      console.error('Failed to load player identity:', error);
    }
  };

  const handleAddPlayerName = () => {
    const trimmed = newPlayerName.trim();
    if (trimmed && !playerNames.includes(trimmed)) {
      setPlayerNames([...playerNames, trimmed]);
      setNewPlayerName('');
    }
  };

  const handleRemovePlayerName = (index: number) => {
    setPlayerNames(playerNames.filter((_, i) => i !== index));
  };

  const handleAddClanTag = () => {
    const trimmed = newClanTag.trim();
    if (trimmed && !clanTags.includes(trimmed)) {
      setClanTags([...clanTags, trimmed]);
      setNewClanTag('');
    }
  };

  const handleRemoveClanTag = (index: number) => {
    setClanTags(clanTags.filter((_, i) => i !== index));
  };

  const handleAddEnemyName = () => {
    const trimmed = newEnemyName.trim();
    if (trimmed && !enemyNames.includes(trimmed)) {
      setEnemyNames([...enemyNames, trimmed]);
      setNewEnemyName('');
    }
  };

  const handleRemoveEnemyName = (index: number) => {
    setEnemyNames(enemyNames.filter((_, i) => i !== index));
  };

  const handleAddEnemyClan = () => {
    const trimmed = newEnemyClan.trim();
    if (trimmed && !enemyClans.includes(trimmed)) {
      setEnemyClans([...enemyClans, trimmed]);
      setNewEnemyClan('');
    }
  };

  const handleRemoveEnemyClan = (index: number) => {
    setEnemyClans(enemyClans.filter((_, i) => i !== index));
  };

  const handleSave = async () => {
    try {
      setLoading(true);
      await Promise.all([
        api.setPlayerNames(playerNames),
        api.setClanTags(clanTags),
        api.setEnemyNames(enemyNames),
        api.setEnemyClans(enemyClans)
      ]);
      if ((window as any).debugLog) {
        (window as any).debugLog('success', `🎮 Player identity saved: ${playerNames.length} names, ${clanTags.length} clans | 🎯 Enemies: ${enemyNames.length} names, ${enemyClans.length} clans`);
      }
      onClose();
    } catch (error: any) {
      if ((window as any).debugLog) {
        (window as any).debugLog('error', `❌ Save failed: ${error.message}`);
      }
    } finally {
      setLoading(false);
    }
  };

  if (!isOpen) return null;

  return (
    <div style={{
      position: 'fixed',
      top: 0,
      left: 0,
      right: 0,
      bottom: 0,
      background: 'rgba(0, 0, 0, 0.75)',
      display: 'flex',
      alignItems: 'center',
      justifyContent: 'center',
      zIndex: 1000,
      padding: '20px'
    }}>
      <div style={{
        background: 'var(--bg-secondary)',
        borderRadius: '12px',
        border: '1px solid var(--border)',
        maxWidth: '600px',
        width: '100%',
        maxHeight: '80vh',
        overflow: 'hidden',
        display: 'flex',
        flexDirection: 'column'
      }}>
        {/* Header */}
        <div style={{
          padding: '20px',
          borderBottom: '1px solid var(--border)',
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'space-between'
        }}>
          <div style={{ display: 'flex', alignItems: 'center', gap: '10px' }}>
            <User size={22} style={{ color: 'var(--primary)' }} />
            <h2 style={{ fontSize: '18px', fontWeight: 600, margin: 0 }}>
              Player Identity
            </h2>
          </div>
          <button
            onClick={onClose}
            style={{
              background: 'transparent',
              border: 'none',
              color: 'var(--text-muted)',
              cursor: 'pointer',
              padding: '4px'
            }}
          >
            <X size={20} />
          </button>
        </div>

        {/* Content */}
        <div style={{
          padding: '20px',
          overflowY: 'auto',
          flex: 1
        }}>
          {/* Player Names Section */}
          <div style={{ marginBottom: '24px' }}>
            <div style={{ display: 'flex', alignItems: 'center', gap: '8px', marginBottom: '12px' }}>
              <User size={16} style={{ color: 'var(--primary)' }} />
              <h3 style={{ fontSize: '14px', fontWeight: 600, margin: 0 }}>
                Nicknames
              </h3>
            </div>

            <div style={{ display: 'flex', gap: '8px', marginBottom: '12px' }}>
              <input
                type="text"
                value={newPlayerName}
                onChange={(e) => setNewPlayerName(e.target.value)}
                onKeyPress={(e) => e.key === 'Enter' && handleAddPlayerName()}
                placeholder="e.g. YourNickname123"
                style={{
                  flex: 1,
                  padding: '10px 12px',
                  background: 'var(--bg-tertiary)',
                  border: '1px solid var(--border)',
                  borderRadius: '6px',
                  color: 'var(--text)',
                  fontSize: '13px',
                  outline: 'none'
                }}
              />
              <button
                onClick={handleAddPlayerName}
                disabled={!newPlayerName.trim()}
                style={{
                  padding: '10px 14px',
                  background: newPlayerName.trim() ? 'var(--primary)' : 'var(--bg-tertiary)',
                  color: newPlayerName.trim() ? 'white' : 'var(--text-muted)',
                  border: 'none',
                  borderRadius: '6px',
                  cursor: newPlayerName.trim() ? 'pointer' : 'default',
                  display: 'flex',
                  alignItems: 'center',
                  gap: '6px',
                  fontSize: '13px',
                  fontWeight: 500
                }}
              >
                <Plus size={16} />
                Add
              </button>
            </div>

            {/* List of player names */}
            <div style={{ display: 'flex', flexDirection: 'column', gap: '6px' }}>
              {playerNames.length === 0 ? (
                <div style={{
                  padding: '12px',
                  background: 'rgba(255, 255, 255, 0.02)',
                  borderRadius: '6px',
                  fontSize: '12px',
                  color: 'var(--text-muted)',
                  textAlign: 'center'
                }}>
                  No nicknames added yet
                </div>
              ) : (
                playerNames.map((name, index) => (
                  <div
                    key={index}
                    style={{
                      display: 'flex',
                      alignItems: 'center',
                      justifyContent: 'space-between',
                      padding: '10px 12px',
                      background: 'rgba(99, 102, 241, 0.1)',
                      border: '1px solid rgba(99, 102, 241, 0.2)',
                      borderRadius: '6px'
                    }}
                  >
                    <span style={{ fontSize: '13px', fontWeight: 500 }}>{name}</span>
                    <button
                      onClick={() => handleRemovePlayerName(index)}
                      style={{
                        background: 'transparent',
                        border: 'none',
                        color: '#ef4444',
                        cursor: 'pointer',
                        padding: '4px',
                        display: 'flex',
                        alignItems: 'center'
                      }}
                    >
                      <Trash2 size={16} />
                    </button>
                  </div>
                ))
              )}
            </div>
          </div>

          {/* Clan Tags Section */}
          <div>
            <div style={{ display: 'flex', alignItems: 'center', gap: '8px', marginBottom: '12px' }}>
              <Users size={16} style={{ color: 'var(--primary)' }} />
              <h3 style={{ fontSize: '14px', fontWeight: 600, margin: 0 }}>
                Clan Tags (optional)
              </h3>
            </div>

            <div style={{ display: 'flex', gap: '8px', marginBottom: '12px' }}>
              <input
                type="text"
                value={newClanTag}
                onChange={(e) => setNewClanTag(e.target.value)}
                onKeyPress={(e) => e.key === 'Enter' && handleAddClanTag()}
                placeholder="e.g. [CLAN]"
                style={{
                  flex: 1,
                  padding: '10px 12px',
                  background: 'var(--bg-tertiary)',
                  border: '1px solid var(--border)',
                  borderRadius: '6px',
                  color: 'var(--text)',
                  fontSize: '13px',
                  outline: 'none'
                }}
              />
              <button
                onClick={handleAddClanTag}
                disabled={!newClanTag.trim()}
                style={{
                  padding: '10px 14px',
                  background: newClanTag.trim() ? 'var(--primary)' : 'var(--bg-tertiary)',
                  color: newClanTag.trim() ? 'white' : 'var(--text-muted)',
                  border: 'none',
                  borderRadius: '6px',
                  cursor: newClanTag.trim() ? 'pointer' : 'default',
                  display: 'flex',
                  alignItems: 'center',
                  gap: '6px',
                  fontSize: '13px',
                  fontWeight: 500
                }}
              >
                <Plus size={16} />
                Add
              </button>
            </div>

            {/* List of clan tags */}
            <div style={{ display: 'flex', flexDirection: 'column', gap: '6px' }}>
              {clanTags.length === 0 ? (
                <div style={{
                  padding: '12px',
                  background: 'rgba(255, 255, 255, 0.02)',
                  borderRadius: '6px',
                  fontSize: '12px',
                  color: 'var(--text-muted)',
                  textAlign: 'center'
                }}>
                  No clan tags added yet
                </div>
              ) : (
                clanTags.map((tag, index) => (
                  <div
                    key={index}
                    style={{
                      display: 'flex',
                      alignItems: 'center',
                      justifyContent: 'space-between',
                      padding: '10px 12px',
                      background: 'rgba(34, 197, 94, 0.1)',
                      border: '1px solid rgba(34, 197, 94, 0.2)',
                      borderRadius: '6px'
                    }}
                  >
                    <span style={{ fontSize: '13px', fontWeight: 500 }}>{tag}</span>
                    <button
                      onClick={() => handleRemoveClanTag(index)}
                      style={{
                        background: 'transparent',
                        border: 'none',
                        color: '#ef4444',
                        cursor: 'pointer',
                        padding: '4px',
                        display: 'flex',
                        alignItems: 'center'
                      }}
                    >
                      <Trash2 size={16} />
                    </button>
                  </div>
                ))
              )}
            </div>
          </div>

          {/* Enemy Nicknames Section */}
          <div style={{ marginTop: '24px' }}>
            <div style={{
              display: 'flex',
              alignItems: 'center',
              gap: '8px',
              marginBottom: '12px'
            }}>
              <User size={18} color="#ef4444" />
              <h3 style={{ margin: 0, fontSize: '14px', fontWeight: 600, color: '#ef4444' }}>
                🎯 Enemy Nicknames
              </h3>
            </div>

            <div style={{
              display: 'flex',
              gap: '8px',
              marginBottom: '12px'
            }}>
              <input
                type="text"
                value={newEnemyName}
                onChange={(e) => setNewEnemyName(e.target.value)}
                onKeyPress={(e) => e.key === 'Enter' && handleAddEnemyName()}
                placeholder="e.g. EnemyPlayer123"
                style={{
                  flex: 1,
                  padding: '10px 12px',
                  background: 'var(--bg-tertiary)',
                  border: '1px solid var(--border)',
                  borderRadius: '6px',
                  color: 'var(--text)',
                  fontSize: '13px'
                }}
              />
              <button
                onClick={handleAddEnemyName}
                style={{
                  padding: '10px 16px',
                  background: 'rgba(239, 68, 68, 0.1)',
                  border: '1px solid rgba(239, 68, 68, 0.3)',
                  borderRadius: '6px',
                  color: '#ef4444',
                  cursor: 'pointer',
                  display: 'flex',
                  alignItems: 'center',
                  gap: '6px',
                  fontSize: '13px',
                  fontWeight: 500
                }}
              >
                <Plus size={16} /> Add
              </button>
            </div>

            <div style={{
              display: 'flex',
              flexDirection: 'column',
              gap: '8px'
            }}>
              {enemyNames.length === 0 ? (
                <div style={{
                  padding: '20px',
                  color: 'var(--text-muted)',
                  fontSize: '12px',
                  textAlign: 'center'
                }}>
                  No enemy names added yet
                </div>
              ) : (
                enemyNames.map((name, index) => (
                  <div
                    key={index}
                    style={{
                      display: 'flex',
                      alignItems: 'center',
                      justifyContent: 'space-between',
                      padding: '10px 12px',
                      background: 'rgba(239, 68, 68, 0.1)',
                      border: '1px solid rgba(239, 68, 68, 0.2)',
                      borderRadius: '6px'
                    }}
                  >
                    <span style={{ fontSize: '13px', fontWeight: 500 }}>{name}</span>
                    <button
                      onClick={() => handleRemoveEnemyName(index)}
                      style={{
                        background: 'transparent',
                        border: 'none',
                        color: '#ef4444',
                        cursor: 'pointer',
                        padding: '4px',
                        display: 'flex',
                        alignItems: 'center'
                      }}
                    >
                      <Trash2 size={16} />
                    </button>
                  </div>
                ))
              )}
            </div>
          </div>

          {/* Enemy Clans Section */}
          <div style={{ marginTop: '24px' }}>
            <div style={{
              display: 'flex',
              alignItems: 'center',
              gap: '8px',
              marginBottom: '12px'
            }}>
              <Users size={18} color="#ef4444" />
              <h3 style={{ margin: 0, fontSize: '14px', fontWeight: 600, color: '#ef4444' }}>
                ☠️ Enemy Clans
              </h3>
            </div>

            <div style={{
              display: 'flex',
              gap: '8px',
              marginBottom: '12px'
            }}>
              <input
                type="text"
                value={newEnemyClan}
                onChange={(e) => setNewEnemyClan(e.target.value)}
                onKeyPress={(e) => e.key === 'Enter' && handleAddEnemyClan()}
                placeholder="e.g. [ENEMY]"
                style={{
                  flex: 1,
                  padding: '10px 12px',
                  background: 'var(--bg-tertiary)',
                  border: '1px solid var(--border)',
                  borderRadius: '6px',
                  color: 'var(--text)',
                  fontSize: '13px'
                }}
              />
              <button
                onClick={handleAddEnemyClan}
                style={{
                  padding: '10px 16px',
                  background: 'rgba(239, 68, 68, 0.1)',
                  border: '1px solid rgba(239, 68, 68, 0.3)',
                  borderRadius: '6px',
                  color: '#ef4444',
                  cursor: 'pointer',
                  display: 'flex',
                  alignItems: 'center',
                  gap: '6px',
                  fontSize: '13px',
                  fontWeight: 500
                }}
              >
                <Plus size={16} /> Add
              </button>
            </div>

            <div style={{
              display: 'flex',
              flexDirection: 'column',
              gap: '8px'
            }}>
              {enemyClans.length === 0 ? (
                <div style={{
                  padding: '20px',
                  color: 'var(--text-muted)',
                  fontSize: '12px',
                  textAlign: 'center'
                }}>
                  No enemy clans added yet
                </div>
              ) : (
                enemyClans.map((clan, index) => (
                  <div
                    key={index}
                    style={{
                      display: 'flex',
                      alignItems: 'center',
                      justifyContent: 'space-between',
                      padding: '10px 12px',
                      background: 'rgba(239, 68, 68, 0.1)',
                      border: '1px solid rgba(239, 68, 68, 0.2)',
                      borderRadius: '6px'
                    }}
                  >
                    <span style={{ fontSize: '13px', fontWeight: 500 }}>{clan}</span>
                    <button
                      onClick={() => handleRemoveEnemyClan(index)}
                      style={{
                        background: 'transparent',
                        border: 'none',
                        color: '#ef4444',
                        cursor: 'pointer',
                        padding: '4px',
                        display: 'flex',
                        alignItems: 'center'
                      }}
                    >
                      <Trash2 size={16} />
                    </button>
                  </div>
                ))
              )}
            </div>
          </div>

          {/* Info */}
          <div style={{
            marginTop: '20px',
            padding: '12px',
            background: 'rgba(99, 102, 241, 0.05)',
            border: '1px solid rgba(99, 102, 241, 0.15)',
            borderRadius: '6px',
            fontSize: '11px',
            color: 'var(--text-muted)',
            lineHeight: '1.5'
          }}>
            💡 <strong>Player identity:</strong> filters your own events (kills, achievements)<br/>
            🎯 <strong>Enemy list:</strong> tracks kills of specific enemies for priority triggers
          </div>
        </div>

        {/* Footer */}
        <div style={{
          padding: '16px 20px',
          borderTop: '1px solid var(--border)',
          display: 'flex',
          gap: '12px',
          justifyContent: 'flex-end'
        }}>
          <button
            onClick={onClose}
            style={{
              padding: '10px 20px',
              background: 'var(--bg-tertiary)',
              color: 'var(--text)',
              border: '1px solid var(--border)',
              borderRadius: '6px',
              cursor: 'pointer',
              fontSize: '13px',
              fontWeight: 500
            }}
          >
            Cancel
          </button>
          <button
            onClick={handleSave}
            disabled={loading}
            style={{
              padding: '10px 24px',
              background: loading ? 'var(--bg-tertiary)' : 'var(--primary)',
              color: loading ? 'var(--text-muted)' : 'white',
              border: 'none',
              borderRadius: '6px',
              cursor: loading ? 'default' : 'pointer',
              fontSize: '13px',
              fontWeight: 500,
              opacity: loading ? 0.6 : 1
            }}
          >
            {loading ? 'Saving...' : 'Save'}
          </button>
        </div>
      </div>
    </div>
  );
}

