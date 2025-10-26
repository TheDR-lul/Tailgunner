import { Node, Edge } from 'reactflow';

export interface ValidationError {
  type: 'error' | 'warning';
  message: string;
}

export function validatePattern(nodes: Node[], edges: Edge[]): ValidationError[] {
  const errors: ValidationError[] = [];

  // Check if pattern has any nodes
  if (nodes.length === 0) {
    errors.push({
      type: 'error',
      message: 'validation.no_nodes'
    });
    return errors;
  }

  // Check for input nodes (input OR event - both are valid sources)
  const inputNodes = nodes.filter(n => n.type === 'input' || n.type === 'event');
  if (inputNodes.length === 0) {
    errors.push({
      type: 'error',
      message: 'validation.no_input_nodes'
    });
  }

  // Check for vibration nodes
  const vibrationNodes = nodes.filter(n => n.type === 'vibration' || n.type === 'linear' || n.type === 'rotate');
  if (vibrationNodes.length === 0) {
    errors.push({
      type: 'error',
      message: 'validation.no_vibration_nodes'
    });
  }

  // Check if there are any connections
  if (edges.length === 0 && nodes.length > 1) {
    errors.push({
      type: 'warning',
      message: 'validation.not_connected'
    });
  }

  // Check for disconnected nodes
  const connectedNodeIds = new Set<string>();
  edges.forEach(edge => {
    connectedNodeIds.add(edge.source);
    connectedNodeIds.add(edge.target);
  });

  const disconnectedNodes = nodes.filter(n => !connectedNodeIds.has(n.id));
  if (disconnectedNodes.length > 0 && edges.length > 0) {
    errors.push({
      type: 'warning',
      message: 'validation.disconnected_nodes'
    });
  }

  return errors;
}

export function hasPatternErrors(nodes: Node[], edges: Edge[]): boolean {
  const errors = validatePattern(nodes, edges);
  return errors.some(e => e.type === 'error');
}

export function hasPatternWarnings(nodes: Node[], edges: Edge[]): boolean {
  const errors = validatePattern(nodes, edges);
  return errors.some(e => e.type === 'warning');
}

