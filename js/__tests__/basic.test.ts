/**
 * Basic arithmetic tests for TypeScript/JavaScript interface
 */

import { describe, it, expect, beforeAll } from 'vitest';
import { SOC } from '../src/index';

describe('SOC (Superior Order Calculator) - Phase 1', () => {
  const soc = new SOC();

  beforeAll(async () => {
    await soc.init();
  });

  describe('Basic Arithmetic', () => {
    it('should add two numbers', () => {
      expect(soc.eval('2 + 3')).toBe(5);
      expect(soc.eval('10 + 5')).toBe(15);
    });

    it('should subtract two numbers', () => {
      expect(soc.eval('5 - 3')).toBe(2);
      expect(soc.eval('10 - 15')).toBe(-5);
    });

    it('should multiply two numbers', () => {
      expect(soc.eval('2 * 3')).toBe(6);
      expect(soc.eval('5 * 7')).toBe(35);
    });

    it('should divide two numbers', () => {
      expect(soc.eval('6 / 2')).toBe(3);
      expect(soc.eval('10 / 4')).toBe(2.5);
    });

    it('should compute power', () => {
      expect(soc.eval('2 ^ 3')).toBe(8);
      expect(soc.eval('5 ^ 2')).toBe(25);
    });
  });

  describe('Operator Precedence', () => {
    it('should respect multiplication before addition', () => {
      expect(soc.eval('2 + 3 * 4')).toBe(14); // 2 + (3 * 4)
      expect(soc.eval('5 * 2 + 3')).toBe(13); // (5 * 2) + 3
    });

    it('should respect division before subtraction', () => {
      expect(soc.eval('10 - 6 / 2')).toBe(7); // 10 - (6 / 2)
    });

    it('should respect power before multiplication', () => {
      expect(soc.eval('2 * 3 ^ 2')).toBe(18); // 2 * (3 ^ 2)
    });

    it('should handle right-associative power', () => {
      expect(soc.eval('2 ^ 3 ^ 2')).toBe(512); // 2 ^ (3 ^ 2) = 2 ^ 9
    });
  });

  describe('Parentheses', () => {
    it('should override precedence with parentheses', () => {
      expect(soc.eval('(2 + 3) * 4')).toBe(20);
      expect(soc.eval('2 * (3 + 4)')).toBe(14);
    });

    it('should handle nested parentheses', () => {
      expect(soc.eval('((2 + 3) * 4)')).toBe(20);
      expect(soc.eval('2 * ((3 + 4) * 5)')).toBe(70);
    });
  });

  describe('Unary Minus', () => {
    it('should negate numbers', () => {
      expect(soc.eval('-5')).toBe(-5);
      expect(soc.eval('-3.14')).toBe(-3.14);
    });

    it('should handle double negation', () => {
      expect(soc.eval('--5')).toBe(5); // -(-5)
    });

    it('should work in expressions', () => {
      expect(soc.eval('-5 + 3')).toBe(-2);
      expect(soc.eval('2 * -3')).toBe(-6);
    });
  });

  describe('Decimal Numbers', () => {
    it('should handle decimal numbers', () => {
      expect(soc.eval('3.14 * 2')).toBeCloseTo(6.28);
      expect(soc.eval('0.1 + 0.2')).toBeCloseTo(0.3);
    });
  });

  describe('Scientific Notation', () => {
    it('should handle scientific notation', () => {
      expect(soc.eval('1e3')).toBe(1000);
      expect(soc.eval('1e-3')).toBe(0.001);
      expect(soc.eval('2.5e2')).toBe(250);
    });
  });

  describe('Complex Expressions', () => {
    it('should evaluate mixed operations', () => {
      expect(soc.eval('2 + 3 * 4 - 5')).toBe(9);
      expect(soc.eval('10 / 2 + 3 * 4')).toBe(17);
    });

    it('should handle combinations with parentheses and power', () => {
      expect(soc.eval('(2 + 3) ^ 2')).toBe(25);
      expect(soc.eval('2 ^ (3 + 1)')).toBe(16);
    });
  });

  describe('Initialization', () => {
    it('should throw if not initialized', () => {
      const uninitializedSOC = new SOC();
      expect(() => uninitializedSOC.eval('2 + 3')).toThrow();
    });

    it('should report initialization status', () => {
      expect(soc.isInitialized()).toBe(true);

      const uninitializedSOC = new SOC();
      expect(uninitializedSOC.isInitialized()).toBe(false);
    });
  });
});
