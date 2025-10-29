import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { 
  X, 
  Search, 
  Zap, 
  Activity, 
  AlertTriangle, 
  Wind, 
  Sparkles 
} from 'lucide-react';
import { 
  patternTemplates, 
  getTemplatesByCategory, 
  searchTemplates,
  createPatternFromTemplate,
  type PatternTemplate 
} from '../utils/patternTemplates';
import { usePatterns } from '../hooks/usePatterns';
import { showToast } from '../utils/toast';
import './PatternTemplatesModal.css';

interface PatternTemplatesModalProps {
  isOpen: boolean;
  onClose: () => void;
}

const categoryIcons = {
  combat: Zap,
  engine: Activity,
  damage: AlertTriangle,
  environmental: Wind,
  custom: Sparkles,
};

const categoryColors = {
  combat: '#ef4444',
  engine: '#8b5cf6',
  damage: '#f59e0b',
  environmental: '#3b82f6',
  custom: '#10b981',
};

export function PatternTemplatesModal({ isOpen, onClose }: PatternTemplatesModalProps) {
  const { t } = useTranslation();
  const { addFullPattern } = usePatterns();
  const [searchQuery, setSearchQuery] = useState('');
  const [selectedCategory, setSelectedCategory] = useState<PatternTemplate['category'] | 'all'>('all');

  if (!isOpen) return null;

  const filteredTemplates = searchQuery
    ? searchTemplates(searchQuery)
    : selectedCategory === 'all'
    ? patternTemplates
    : getTemplatesByCategory(selectedCategory);

  const handleUseTemplate = async (template: PatternTemplate) => {
    try {
      const newPattern = createPatternFromTemplate(template);
      await addFullPattern(newPattern);
      showToast.success(`Template "${template.name}" added!`);
      onClose();
    } catch (error) {
      showToast.error('Failed to add template');
      console.error('Failed to add template:', error);
    }
  };

  return (
    <div className="modal-overlay" onClick={onClose}>
      <div className="modal-container pattern-templates-modal" onClick={(e) => e.stopPropagation()}>
        <div className="modal-header">
          <h2>
            <Sparkles size={24} />
            Pattern Templates
          </h2>
          <button className="modal-close" onClick={onClose}>
            <X size={24} />
          </button>
        </div>

        <div className="modal-body">
          {/* Search */}
          <div className="search-bar">
            <Search size={18} />
            <input
              type="text"
              placeholder="Search templates..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
            />
          </div>

          {/* Category Filter */}
          <div className="category-filter">
            <button
              className={`category-btn ${selectedCategory === 'all' ? 'active' : ''}`}
              onClick={() => setSelectedCategory('all')}
            >
              All
            </button>
            {Object.entries(categoryIcons).map(([category, Icon]) => (
              <button
                key={category}
                className={`category-btn ${selectedCategory === category ? 'active' : ''}`}
                onClick={() => setSelectedCategory(category as PatternTemplate['category'])}
                style={{
                  borderColor: selectedCategory === category 
                    ? categoryColors[category as keyof typeof categoryColors]
                    : 'transparent'
                }}
              >
                <Icon size={16} style={{ 
                  color: categoryColors[category as keyof typeof categoryColors] 
                }} />
                {category.charAt(0).toUpperCase() + category.slice(1)}
              </button>
            ))}
          </div>

          {/* Templates Grid */}
          <div className="templates-grid">
            {filteredTemplates.length === 0 ? (
              <div className="no-results">
                <p>No templates found</p>
              </div>
            ) : (
              filteredTemplates.map((template) => {
                const Icon = categoryIcons[template.category];
                const color = categoryColors[template.category];
                
                return (
                  <div key={template.id} className="template-card">
                    <div className="template-header">
                      <div className="template-icon" style={{ color }}>
                        <Icon size={20} />
                      </div>
                      <div className="template-category" style={{ borderColor: color, color }}>
                        {template.category}
                      </div>
                    </div>
                    
                    <h3>{template.name}</h3>
                    <p>{template.description}</p>
                    
                    <div className="template-tags">
                      {template.tags.map((tag) => (
                        <span key={tag} className="tag">
                          {tag}
                        </span>
                      ))}
                    </div>
                    
                    <div className="template-meta">
                      <span>{template.pattern.nodes.length - 2} nodes</span>
                    </div>
                    
                    <button
                      className="btn btn-primary btn-sm"
                      onClick={() => handleUseTemplate(template)}
                    >
                      Use Template
                    </button>
                  </div>
                );
              })
            )}
          </div>
        </div>
      </div>
    </div>
  );
}

