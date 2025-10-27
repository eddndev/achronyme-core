/**
 * Utility functions for parsing and formatting values
 */

import { ComplexNumber, AchronymeValueType } from './types.js';
import { AchronymeTypeError } from './errors.js';

/**
 * Parse a C++ result string into JavaScript types
 */
export function parseResult(result: string): any {
  const trimmed = result.trim();

  // Check for complex number (contains 'i')
  if (trimmed.includes('i')) {
    return parseComplex(trimmed);
  }

  // Check for vector (starts with '[')
  if (trimmed.startsWith('[')) {
    return parseVector(trimmed);
  }

  // Check for matrix (contains nested brackets)
  if (trimmed.includes('[[')) {
    return parseMatrix(trimmed);
  }

  // Check for function (contains 'function' or '=>')
  if (trimmed.includes('function') || trimmed.includes('=>')) {
    return trimmed;
  }

  // Try to parse as number
  const num = parseFloat(trimmed);
  if (!isNaN(num)) {
    return num;
  }

  // Return as string if nothing else matches
  return trimmed;
}

/**
 * Parse complex number string to ComplexNumber object
 * Examples: "3i", "2+3i", "2-3i", "2.5 + 3.7i"
 */
export function parseComplex(str: string): ComplexNumber {
  const trimmed = str.trim();

  // Handle pure imaginary (e.g., "3i")
  if (!trimmed.includes('+') && !trimmed.includes('-', 1)) {
    if (trimmed.endsWith('i')) {
      const im = parseFloat(trimmed.slice(0, -1)) || 1;
      return { re: 0, im };
    }
  }

  // Handle "a + bi" or "a - bi" format
  const regex = /([+-]?\d+\.?\d*)\s*([+-])\s*(\d+\.?\d*)i/;
  const match = trimmed.match(regex);

  if (match) {
    const re = parseFloat(match[1]);
    const sign = match[2] === '+' ? 1 : -1;
    const im = parseFloat(match[3]) * sign;
    return { re, im };
  }

  // Try to extract real and imaginary parts more flexibly
  const parts = trimmed.split(/\s+/);
  let re = 0;
  let im = 0;

  for (const part of parts) {
    if (part.endsWith('i')) {
      im = parseFloat(part.slice(0, -1)) || 0;
    } else if (part !== '+' && part !== '-') {
      re = parseFloat(part) || 0;
    }
  }

  return { re, im };
}

/**
 * Parse vector string to number array
 * Example: "[1, 2, 3, 4]" => [1, 2, 3, 4]
 */
export function parseVector(str: string): number[] {
  try {
    // Remove brackets and split by comma
    const cleaned = str.trim().slice(1, -1);
    if (!cleaned) return [];

    return cleaned.split(',').map(s => {
      const num = parseFloat(s.trim());
      if (isNaN(num)) {
        throw new AchronymeTypeError(`Invalid number in vector: ${s.trim()}`);
      }
      return num;
    });
  } catch (e) {
    throw new AchronymeTypeError(`Failed to parse vector: ${str}`, 'vector', typeof str);
  }
}

/**
 * Parse matrix string to 2D number array
 * Example: "[[1, 2], [3, 4]]" => [[1, 2], [3, 4]]
 */
export function parseMatrix(str: string): number[][] {
  try {
    // Basic parsing - could be enhanced
    const cleaned = str.trim();

    // Remove outer brackets
    let content = cleaned.slice(1, -1).trim();

    // Split into rows
    const rows: number[][] = [];
    let depth = 0;
    let currentRow = '';

    for (let i = 0; i < content.length; i++) {
      const char = content[i];

      if (char === '[') {
        depth++;
        if (depth === 1) continue;
      } else if (char === ']') {
        depth--;
        if (depth === 0) {
          if (currentRow.trim()) {
            rows.push(parseVector('[' + currentRow + ']'));
          }
          currentRow = '';
          continue;
        }
      }

      if (depth > 0) {
        currentRow += char;
      }
    }

    return rows;
  } catch (e) {
    throw new AchronymeTypeError(`Failed to parse matrix: ${str}`, 'matrix', typeof str);
  }
}

/**
 * Format a JavaScript value into a C++ expression string
 */
export function formatValue(value: any): string {
  if (typeof value === 'number') {
    return value.toString();
  }

  if (typeof value === 'string') {
    return value;
  }

  if (Array.isArray(value)) {
    // Check if it's a matrix (2D array)
    if (Array.isArray(value[0])) {
      return formatMatrix(value as number[][]);
    }
    return formatVector(value);
  }

  if (typeof value === 'object' && value !== null) {
    // Check if it's a complex number
    if ('re' in value && 'im' in value) {
      return formatComplex(value as ComplexNumber);
    }
  }

  return String(value);
}

/**
 * Format vector as C++ expression
 */
export function formatVector(vec: number[]): string {
  return `[${vec.join(', ')}]`;
}

/**
 * Format matrix as C++ expression
 */
export function formatMatrix(mat: number[][]): string {
  const rows = mat.map(row => `[${row.join(', ')}]`);
  return `[${rows.join(', ')}]`;
}

/**
 * Format complex number as C++ expression
 */
export function formatComplex(c: ComplexNumber): string {
  if (c.re === 0) {
    return `${c.im}i`;
  }
  if (c.im === 0) {
    return `${c.re}`;
  }
  const sign = c.im >= 0 ? '+' : '';
  return `${c.re}${sign}${c.im}i`;
}

/**
 * Detect the type of a result string
 */
export function detectType(result: string): AchronymeValueType {
  const trimmed = result.trim();

  if (trimmed.includes('function') || trimmed.includes('=>')) {
    return 'function';
  }

  if (trimmed.includes('[[')) {
    return 'matrix';
  }

  if (trimmed.startsWith('[')) {
    return 'vector';
  }

  if (trimmed.includes('i')) {
    return 'complex';
  }

  if (!isNaN(parseFloat(trimmed))) {
    return 'number';
  }

  return 'unknown';
}

/**
 * Escape special characters in strings for C++ eval
 */
export function escapeString(str: string): string {
  return str.replace(/\\/g, '\\\\').replace(/"/g, '\\"').replace(/'/g, "\\'");
}

/**
 * Validate variable name
 */
export function isValidVariableName(name: string): boolean {
  return /^[a-zA-Z_][a-zA-Z0-9_]*$/.test(name);
}
