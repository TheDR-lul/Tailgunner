import type { Pattern } from '../hooks/PatternsProvider';

export interface PatternTemplate {
  id: string;
  name: string;
  description: string;
  category: 'combat' | 'engine' | 'damage' | 'environmental' | 'custom';
  pattern: {
    name: string;
    nodes: any[];
    edges: any[];
  };
  tags: string[];
}

export const patternTemplates: PatternTemplate[] = [
  // ===== COMBAT TEMPLATES =====
  {
    id: 'hit-impact',
    name: 'Hit Impact',
    description: 'Quick vibration spike when getting hit by enemy fire',
    category: 'combat',
    tags: ['combat', 'damage', 'hit'],
    pattern: {
      name: 'Hit Impact',
      nodes: [
        { 
          id: '1', 
          type: 'event', 
          position: { x: 100, y: 150 }, 
          data: { 
            event: 'Hit',
            filter_type: 'any'
          } 
        },
        { 
          id: '2', 
          type: 'vibration', 
          position: { x: 350, y: 150 }, 
          data: { 
            duration: 0.15,
            curve: [
              { x: 0.0, y: 1.0 },
              { x: 0.3, y: 0.8 },
              { x: 1.0, y: 0.0 }
            ],
            mode: 'once'
          }
        },
        { 
          id: '3', 
          type: 'output', 
          position: { x: 600, y: 150 }, 
          data: { 
            deviceMode: 'all'
          }
        },
      ],
      edges: [
        { id: 'e1-2', source: '1', target: '2' },
        { id: 'e2-3', source: '2', target: '3' },
      ],
    },
  },
  
  {
    id: 'target-destroyed',
    name: 'Target Destroyed',
    description: 'Satisfying burst when you destroy an enemy',
    category: 'combat',
    tags: ['combat', 'kill', 'victory'],
    pattern: {
      name: 'Target Destroyed',
      nodes: [
        { 
          id: '1', 
          type: 'event', 
          position: { x: 100, y: 150 }, 
          data: { 
            event: 'TargetDestroyed',
            filter_type: 'any'
          } 
        },
        { 
          id: '2', 
          type: 'vibration', 
          position: { x: 350, y: 150 }, 
          data: { 
            duration: 0.6,
            curve: [
              { x: 0.0, y: 0.8 },
              { x: 0.2, y: 1.0 },
              { x: 0.5, y: 0.7 },
              { x: 1.0, y: 0.0 }
            ],
            mode: 'once'
          }
        },
        { 
          id: '3', 
          type: 'output', 
          position: { x: 600, y: 150 }, 
          data: { 
            deviceMode: 'all'
          }
        },
      ],
      edges: [
        { id: 'e1-2', source: '1', target: '2' },
        { id: 'e2-3', source: '2', target: '3' },
      ],
    },
  },
  
  {
    id: 'critical-hit',
    name: 'Critical Hit',
    description: 'Intense vibration for critical damage',
    category: 'combat',
    tags: ['combat', 'damage', 'critical'],
    pattern: {
      name: 'Critical Hit',
      nodes: [
        { 
          id: '1', 
          type: 'event', 
          position: { x: 100, y: 150 }, 
          data: { 
            event: 'CriticalHit',
            filter_type: 'any'
          } 
        },
        { 
          id: '2', 
          type: 'vibration', 
          position: { x: 350, y: 150 }, 
          data: { 
            duration: 0.3,
            curve: [
              { x: 0.0, y: 1.0 },
              { x: 0.5, y: 0.9 },
              { x: 1.0, y: 0.0 }
            ],
            mode: 'once'
          }
        },
        { 
          id: '3', 
          type: 'output', 
          position: { x: 600, y: 150 }, 
          data: { 
            deviceMode: 'all'
          }
        },
      ],
      edges: [
        { id: 'e1-2', source: '1', target: '2' },
        { id: 'e2-3', source: '2', target: '3' },
      ],
    },
  },
  
  // ===== ENGINE TEMPLATES =====
  {
    id: 'engine-overheat',
    name: 'Engine Overheat',
    description: 'Warning pulse when engine overheats',
    category: 'engine',
    tags: ['engine', 'warning', 'overheat'],
    pattern: {
      name: 'Engine Overheat',
      nodes: [
        { 
          id: '1', 
          type: 'event', 
          position: { x: 100, y: 150 }, 
          data: { 
            event: 'EngineOverheat',
            filter_type: 'any'
          } 
        },
        { 
          id: '2', 
          type: 'vibration', 
          position: { x: 350, y: 150 }, 
          data: { 
            duration: 0.5,
            curve: [
              { x: 0.0, y: 0.6 },
              { x: 0.5, y: 0.8 },
              { x: 1.0, y: 0.6 }
            ],
            mode: 'repeat',
            repeatCount: 3
          }
        },
        { 
          id: '3', 
          type: 'output', 
          position: { x: 600, y: 150 }, 
          data: { 
            deviceMode: 'all'
          }
        },
      ],
      edges: [
        { id: 'e1-2', source: '1', target: '2' },
        { id: 'e2-3', source: '2', target: '3' },
      ],
    },
  },
  
  {
    id: 'high-g-load',
    name: 'High G-Load',
    description: 'Increasing intensity with G-forces above 6G',
    category: 'environmental',
    tags: ['aircraft', 'g-force', 'maneuver'],
    pattern: {
      name: 'High G-Load',
      nodes: [
        { 
          id: '1', 
          type: 'input', 
          position: { x: 100, y: 100 }, 
          data: { 
            indicator: 'g_load'
          } 
        },
        { 
          id: '2', 
          type: 'condition', 
          position: { x: 300, y: 100 }, 
          data: { 
            operator: '>',
            value: 6.0
          }
        },
        { 
          id: '3', 
          type: 'vibration', 
          position: { x: 500, y: 100 }, 
          data: { 
            duration: 1.0,
            curve: [
              { x: 0.0, y: 0.5 },
              { x: 0.5, y: 0.8 },
              { x: 1.0, y: 0.5 }
            ],
            mode: 'while_true'
          }
        },
        { 
          id: '4', 
          type: 'output', 
          position: { x: 750, y: 100 }, 
          data: { 
            deviceMode: 'all'
          }
        },
      ],
      edges: [
        { id: 'e1-2', source: '1', target: '2' },
        { id: 'e2-3', source: '2', target: '3' },
        { id: 'e3-4', source: '3', target: '4' },
      ],
    },
  },
  
  {
    id: 'high-speed',
    name: 'High Speed Vibration',
    description: 'Gentle rumble when flying above 700 km/h',
    category: 'environmental',
    tags: ['aircraft', 'speed', 'rumble'],
    pattern: {
      name: 'High Speed',
      nodes: [
        { 
          id: '1', 
          type: 'input', 
          position: { x: 100, y: 100 }, 
          data: { 
            indicator: 'ias'
          } 
        },
        { 
          id: '2', 
          type: 'condition', 
          position: { x: 300, y: 100 }, 
          data: { 
            operator: '>',
            value: 700
          }
        },
        { 
          id: '3', 
          type: 'vibration', 
          position: { x: 500, y: 100 }, 
          data: { 
            duration: 2.0,
            curve: [
              { x: 0.0, y: 0.3 },
              { x: 0.5, y: 0.4 },
              { x: 1.0, y: 0.3 }
            ],
            mode: 'while_true'
          }
        },
        { 
          id: '4', 
          type: 'output', 
          position: { x: 750, y: 100 }, 
          data: { 
            deviceMode: 'all'
          }
        },
      ],
      edges: [
        { id: 'e1-2', source: '1', target: '2' },
        { id: 'e2-3', source: '2', target: '3' },
        { id: 'e3-4', source: '3', target: '4' },
      ],
    },
  },
  
  // ===== DAMAGE TEMPLATES =====
  {
    id: 'fire-warning',
    name: 'Fire Warning',
    description: 'Urgent pulsing when aircraft is set on fire',
    category: 'damage',
    tags: ['fire', 'damage', 'warning'],
    pattern: {
      name: 'Fire Warning',
      nodes: [
        { 
          id: '1', 
          type: 'event', 
          position: { x: 100, y: 150 }, 
          data: { 
            event: 'TargetSetOnFire',
            filter_type: 'any'
          } 
        },
        { 
          id: '2', 
          type: 'vibration', 
          position: { x: 350, y: 150 }, 
          data: { 
            duration: 0.3,
            curve: [
              { x: 0.0, y: 0.8 },
              { x: 0.5, y: 0.9 },
              { x: 1.0, y: 0.8 }
            ],
            mode: 'repeat',
            repeatCount: 5
          }
        },
        { 
          id: '3', 
          type: 'output', 
          position: { x: 600, y: 150 }, 
          data: { 
            deviceMode: 'all'
          }
        },
      ],
      edges: [
        { id: 'e1-2', source: '1', target: '2' },
        { id: 'e2-3', source: '2', target: '3' },
      ],
    },
  },
  
  // ===== CUSTOM/SIMPLE TEMPLATES =====
  {
    id: 'simple-pulse',
    name: 'Simple Pulse',
    description: 'Basic event-triggered pulse pattern',
    category: 'custom',
    tags: ['basic', 'simple', 'pulse'],
    pattern: {
      name: 'Simple Pulse',
      nodes: [
        { 
          id: '1', 
          type: 'event', 
          position: { x: 100, y: 150 }, 
          data: { 
            event: 'Hit',
            filter_type: 'any'
          } 
        },
        { 
          id: '2', 
          type: 'vibration', 
          position: { x: 350, y: 150 }, 
          data: { 
            duration: 0.5,
            curve: [
              { x: 0.3, y: 0.6 },
              { x: 0.7, y: 0.6 }
            ],
            mode: 'once'
          }
        },
        { 
          id: '3', 
          type: 'output', 
          position: { x: 600, y: 150 }, 
          data: { 
            deviceMode: 'all'
          }
        },
      ],
      edges: [
        { id: 'e1-2', source: '1', target: '2' },
        { id: 'e2-3', source: '2', target: '3' },
      ],
    },
  },
  
  {
    id: 'shooting-feedback',
    name: 'Shooting Feedback',
    description: 'Short pulse when firing weapons',
    category: 'combat',
    tags: ['combat', 'shooting', 'weapons'],
    pattern: {
      name: 'Shooting Feedback',
      nodes: [
        { 
          id: '1', 
          type: 'event', 
          position: { x: 100, y: 150 }, 
          data: { 
            event: 'Shooting',
            filter_type: 'any'
          } 
        },
        { 
          id: '2', 
          type: 'vibration', 
          position: { x: 350, y: 150 }, 
          data: { 
            duration: 0.1,
            curve: [
              { x: 0.0, y: 0.4 },
              { x: 0.5, y: 0.5 },
              { x: 1.0, y: 0.0 }
            ],
            mode: 'once'
          }
        },
        { 
          id: '3', 
          type: 'output', 
          position: { x: 600, y: 150 }, 
          data: { 
            deviceMode: 'all'
          }
        },
      ],
      edges: [
        { id: 'e1-2', source: '1', target: '2' },
        { id: 'e2-3', source: '2', target: '3' },
      ],
    },
  },
];

export function getTemplatesByCategory(category: PatternTemplate['category']): PatternTemplate[] {
  return patternTemplates.filter(t => t.category === category);
}

export function getTemplateById(id: string): PatternTemplate | undefined {
  return patternTemplates.find(t => t.id === id);
}

export function searchTemplates(query: string): PatternTemplate[] {
  const lowerQuery = query.toLowerCase();
  return patternTemplates.filter(
    t => 
      t.name.toLowerCase().includes(lowerQuery) ||
      t.description.toLowerCase().includes(lowerQuery) ||
      t.tags.some(tag => tag.toLowerCase().includes(lowerQuery))
  );
}

export function createPatternFromTemplate(template: PatternTemplate): Pattern {
  const pattern: Pattern = {
    id: `pattern-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`,
    name: template.pattern.name,
    nodes: template.pattern.nodes,
    edges: template.pattern.edges,
    enabled: true,
    createdAt: new Date(),
  };
  
  console.log('[Template] Created pattern:', pattern.name, 'with', pattern.nodes.length, 'nodes');
  
  return pattern;
}
