/**
 * Achronyme Core - High-performance mathematical computation engine
 *
 * Phase 1: Basic arithmetic evaluator
 *
 * @example
 * ```typescript
 * import { SOC } from '@achronyme/core';
 *
 * const soc = new SOC();
 * await soc.init();
 *
 * const result = soc.eval('2 + 3 * 4');
 * console.log(result); // 14
 * ```
 */

import { loadWASM, isLoaded, getModule, AchronymeModule } from './loader';

/**
 * Superior Order Calculator (SOC)
 *
 * Main interface for evaluating mathematical expressions.
 */
export class SOC {
  private module: AchronymeModule | null = null;

  /**
   * Initialize the SOC (load WASM module)
   *
   * Must be called before using eval()
   */
  async init(): Promise<void> {
    if (!this.module) {
      this.module = await loadWASM();
    }
  }

  /**
   * Evaluate a mathematical expression
   *
   * @param expression - String expression to evaluate
   * @returns Evaluated result
   *
   * @example
   * ```typescript
   * soc.eval('2 + 3 * 4')        // 14
   * soc.eval('(2 + 3) * 4')      // 20
   * soc.eval('2 ^ 3 ^ 2')        // 512 (right-associative)
   * soc.eval('-5 + 3')           // -2
   * soc.eval('3.14 * 2')         // 6.28
   * soc.eval('1e-3 + 2e10')      // 20000000000.001
   * ```
   */
  eval(expression: string): number {
    if (!this.module) {
      throw new Error('SOC not initialized. Call await soc.init() first.');
    }

    const result = this.module.eval(expression);

    if (isNaN(result)) {
      throw new Error(`Failed to evaluate expression: ${expression}`);
    }

    return result;
  }

  /**
   * Check if the module is initialized
   */
  isInitialized(): boolean {
    return this.module !== null;
  }
}

/**
 * Singleton instance for convenience
 *
 * @example
 * ```typescript
 * import { soc } from '@achronyme/core';
 *
 * await soc.init();
 * console.log(soc.eval('2 + 3 * 4')); // 14
 * ```
 */
export const soc = new SOC();

// Re-export types
export type { AchronymeModule } from './loader';
