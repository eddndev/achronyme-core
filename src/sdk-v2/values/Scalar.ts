/**
 * Scalar.ts
 *
 * Scalar value (single number)
 * Simple wrapper for consistency with other value types
 */

import { Value } from './Value';
import type { Handle } from '../types';
import type { AchronymeSession } from '../core/Session';

/**
 * Scalar value (single number)
 *
 * Represents a single floating-point number
 * Primarily used for consistency with the value system
 *
 * Usage:
 * ```typescript
 * const s = session.scalar(42);
 * const value = s.value; // 42
 * const num = +s; // 42 (numeric coercion)
 * ```
 */
export class Scalar extends Value {
    private _value?: number;

    constructor(session: AchronymeSession, handle: Handle, value?: number) {
        super(session, handle, 'scalar');
        this._value = value;
    }

    /**
     * Get the scalar value
     */
    get value(): number {
        this.checkDisposed();

        if (this._value === undefined) {
            // Extract from WASM if not cached
            const data = this.wasm.getVector(this.handle);
            this._value = data[0];
        }

        return this._value;
    }

    /**
     * Convert to JavaScript number (for compatibility)
     */
    toNumber(): number {
        return this.value;
    }

    /**
     * Convert to array (single-element array)
     */
    toArray(): number[] {
        return [this.value];
    }

    /**
     * Numeric coercion support
     *
     * Allows: const x = +scalar;
     */
    valueOf(): number {
        return this.value;
    }

    /**
     * String representation
     */
    toString(): string {
        if (this.isDisposed) {
            return '[Scalar (disposed)]';
        }
        return `[Scalar: ${this.value}]`;
    }
}
