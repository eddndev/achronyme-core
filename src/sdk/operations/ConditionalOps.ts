/**
 * ConditionalOps.ts
 *
 * Conditional expressions and piecewise functions module
 * All operations use the Rust/WASM engine through eval - no JavaScript math
 */

import type { AchronymeSession } from '../core/Session';
import { Vector } from '../values/Vector';

/**
 * Conditional operations
 *
 * All methods use the Rust/WASM piecewise engine internally.
 * No JavaScript-based mathematical computation.
 *
 * @example
 * ```typescript
 * await ach.use(async () => {
 *   // Scalar operations (use Rust piecewise internally)
 *   const absVal = ach.conditional.absValue(-5); // 5
 *   const reluVal = ach.conditional.reluValue(-3); // 0
 *
 *   // Vector operations (use Rust map + piecewise)
 *   const v = ach.vector([-5, -2, 0, 2, 5]);
 *   const absVec = ach.conditional.absVector(v); // [5, 2, 0, 2, 5]
 *   const reluVec = ach.conditional.reluVector(v); // [0, 0, 0, 2, 5]
 * });
 * ```
 */
export class ConditionalOps {
    constructor(private session: AchronymeSession) {}

    // ========================================================================
    // Scalar Operations (use Rust engine internally)
    // ========================================================================

    /**
     * Absolute value (scalar) - uses Rust piecewise
     */
    absValue(x: number): number {
        const result = this.session.wasm._eval(`(x => piecewise([x < 0, -x], x))(${x})`);
        return parseFloat(result);
    }

    /**
     * Absolute value (vector) - uses Rust map + piecewise
     */
    absVector(v: Vector): Vector {
        const handle = this.session.wasm.evalToHandle(`map(x => piecewise([x < 0, -x], x), ${this.vectorToSOC(v)})`);
        return new Vector(this.session, handle);
    }

    /**
     * ReLU activation (scalar): max(0, x) - uses Rust piecewise
     */
    reluValue(x: number): number {
        const result = this.session.wasm._eval(`(x => piecewise([x > 0, x], 0))(${x})`);
        return parseFloat(result);
    }

    /**
     * ReLU activation (vector) - uses Rust map + piecewise
     */
    reluVector(v: Vector): Vector {
        const handle = this.session.wasm.evalToHandle(`map(x => piecewise([x > 0, x], 0), ${this.vectorToSOC(v)})`);
        return new Vector(this.session, handle);
    }

    /**
     * Leaky ReLU activation (scalar) - uses Rust piecewise
     */
    leakyReluValue(x: number, alpha: number = 0.01): number {
        const result = this.session.wasm._eval(`(x => piecewise([x > 0, x], ${alpha} * x))(${x})`);
        return parseFloat(result);
    }

    /**
     * Leaky ReLU activation (vector) - uses Rust map + piecewise
     */
    leakyReluVector(v: Vector, alpha: number = 0.01): Vector {
        const handle = this.session.wasm.evalToHandle(`map(x => piecewise([x > 0, x], ${alpha} * x), ${this.vectorToSOC(v)})`);
        return new Vector(this.session, handle);
    }

    /**
     * Sign function (scalar): -1, 0, or 1 - uses Rust piecewise
     */
    signValue(x: number): number {
        const result = this.session.wasm._eval(`(x => piecewise([x < 0, -1], [x > 0, 1], 0))(${x})`);
        return parseFloat(result);
    }

    /**
     * Sign function (vector) - uses Rust map + piecewise
     */
    signVector(v: Vector): Vector {
        const handle = this.session.wasm.evalToHandle(`map(x => piecewise([x < 0, -1], [x > 0, 1], 0), ${this.vectorToSOC(v)})`);
        return new Vector(this.session, handle);
    }

    /**
     * Heaviside step function (scalar): 0 if x < 0, 1 if x >= 0 - uses Rust piecewise
     */
    heavisideValue(x: number): number {
        const result = this.session.wasm._eval(`(x => piecewise([x < 0, 0], 1))(${x})`);
        return parseFloat(result);
    }

    /**
     * Heaviside step function (vector) - uses Rust map + piecewise
     */
    heavisideVector(v: Vector): Vector {
        const handle = this.session.wasm.evalToHandle(`map(x => piecewise([x < 0, 0], 1), ${this.vectorToSOC(v)})`);
        return new Vector(this.session, handle);
    }

    /**
     * Clamp function (scalar): saturate between min and max - uses Rust piecewise
     */
    clampValue(x: number, min: number, max: number): number {
        const result = this.session.wasm._eval(`(x => piecewise([x < ${min}, ${min}], [x > ${max}, ${max}], x))(${x})`);
        return parseFloat(result);
    }

    /**
     * Clamp function (vector) - uses Rust map + piecewise
     */
    clampVector(v: Vector, min: number, max: number): Vector {
        const handle = this.session.wasm.evalToHandle(`map(x => piecewise([x < ${min}, ${min}], [x > ${max}, ${max}], x), ${this.vectorToSOC(v)})`);
        return new Vector(this.session, handle);
    }

    /**
     * Rectangular pulse (scalar) - uses Rust piecewise with abs
     */
    rectValue(t: number, width: number = 1): number {
        const halfWidth = width / 2;
        const result = this.session.wasm._eval(`(t => piecewise([piecewise([t < 0, -t], t) <= ${halfWidth}, 1], 0))(${t})`);
        return parseFloat(result);
    }

    /**
     * Rectangular pulse (vector) - uses Rust map + piecewise
     */
    rectVector(v: Vector, width: number = 1): Vector {
        const halfWidth = width / 2;
        const handle = this.session.wasm.evalToHandle(`map(t => piecewise([piecewise([t < 0, -t], t) <= ${halfWidth}, 1], 0), ${this.vectorToSOC(v)})`);
        return new Vector(this.session, handle);
    }

    /**
     * Square wave (scalar): period 2π - uses Rust piecewise
     */
    squareWaveValue(t: number): number {
        const result = this.session.wasm._eval(`(t => piecewise([(t % (2*PI)) < PI, 1], -1))(${t})`);
        return parseFloat(result);
    }

    /**
     * Square wave (vector) - uses Rust map + piecewise
     */
    squareWaveVector(v: Vector): Vector {
        const handle = this.session.wasm.evalToHandle(`map(t => piecewise([(t % (2*PI)) < PI, 1], -1), ${this.vectorToSOC(v)})`);
        return new Vector(this.session, handle);
    }

    /**
     * Triangle wave (scalar): period 2 - uses Rust piecewise
     */
    triangleWaveValue(t: number): number {
        const result = this.session.wasm._eval(`(t => piecewise([(t % 2) < 1, t % 2], 2 - (t % 2)))(${t})`);
        return parseFloat(result);
    }

    /**
     * Triangle wave (vector) - uses Rust map + piecewise
     */
    triangleWaveVector(v: Vector): Vector {
        const handle = this.session.wasm.evalToHandle(`map(t => piecewise([(t % 2) < 1, t % 2], 2 - (t % 2)), ${this.vectorToSOC(v)})`);
        return new Vector(this.session, handle);
    }

    // ========================================================================
    // Helper Methods
    // ========================================================================

    /**
     * Convert Vector to SOC array literal
     */
    private vectorToSOC(v: Vector): string {
        return `[${Array.from(v.data).join(', ')}]`;
    }

    // ========================================================================
    // SOC Expression Builders (for advanced usage)
    // ========================================================================

    // ========================================================================
    // Common Piecewise Function Builders
    // ========================================================================

    /**
     * Absolute value function builder
     *
     * Returns: `x => piecewise([x < 0, -x], x)`
     *
     * @param varName Variable name (default: 'x')
     * @returns SOC lambda expression string
     *
     * @example
     * ```typescript
     * ach.eval(`let abs = ${ach.conditional.abs()}`);
     * ach.eval('abs(-5)'); // "5"
     * ach.eval('abs(3)');  // "3"
     * ```
     */
    abs(varName: string = 'x'): string {
        return `${varName} => piecewise([${varName} < 0, -${varName}], ${varName})`;
    }

    /**
     * ReLU activation function builder
     *
     * ReLU(x) = max(0, x) = { 0 if x ≤ 0, x if x > 0 }
     *
     * Returns: `x => piecewise([x > 0, x], 0)`
     *
     * @param varName Variable name (default: 'x')
     * @returns SOC lambda expression string
     *
     * @example
     * ```typescript
     * ach.eval(`let relu = ${ach.conditional.relu()}`);
     * ach.eval('relu(5)');  // "5"
     * ach.eval('relu(-3)'); // "0"
     * ```
     */
    relu(varName: string = 'x'): string {
        return `${varName} => piecewise([${varName} > 0, ${varName}], 0)`;
    }

    /**
     * Leaky ReLU activation function builder
     *
     * LeakyReLU(x) = { αx if x ≤ 0, x if x > 0 }
     *
     * Returns: `x => piecewise([x > 0, x], α * x)`
     *
     * @param alpha Slope for negative values (default: 0.01)
     * @param varName Variable name (default: 'x')
     * @returns SOC lambda expression string
     *
     * @example
     * ```typescript
     * ach.eval(`let leaky_relu = ${ach.conditional.leakyRelu(0.01)}`);
     * ach.eval('leaky_relu(10)');  // "10"
     * ach.eval('leaky_relu(-10)'); // "-0.1"
     * ```
     */
    leakyRelu(alpha: number = 0.01, varName: string = 'x'): string {
        return `${varName} => piecewise([${varName} > 0, ${varName}], ${alpha} * ${varName})`;
    }

    /**
     * Sign function builder
     *
     * sign(x) = { -1 if x < 0, 1 if x > 0, 0 if x == 0 }
     *
     * Returns: `x => piecewise([x < 0, -1], [x > 0, 1], 0)`
     *
     * @param varName Variable name (default: 'x')
     * @returns SOC lambda expression string
     *
     * @example
     * ```typescript
     * ach.eval(`let sign = ${ach.conditional.sign()}`);
     * ach.eval('sign(-10)'); // "-1"
     * ach.eval('sign(0)');   // "0"
     * ach.eval('sign(7)');   // "1"
     * ```
     */
    sign(varName: string = 'x'): string {
        return `${varName} => piecewise([${varName} < 0, -1], [${varName} > 0, 1], 0)`;
    }

    /**
     * Heaviside step function builder
     *
     * H(x) = { 0 if x < 0, 1 if x ≥ 0 }
     *
     * Returns: `x => piecewise([x < 0, 0], 1)`
     *
     * @param varName Variable name (default: 'x')
     * @returns SOC lambda expression string
     *
     * @example
     * ```typescript
     * ach.eval(`let H = ${ach.conditional.heaviside()}`);
     * ach.eval('H(-2)'); // "0"
     * ach.eval('H(0)');  // "1"
     * ach.eval('H(5)');  // "1"
     * ```
     */
    heaviside(varName: string = 'x'): string {
        return `${varName} => piecewise([${varName} < 0, 0], 1)`;
    }

    /**
     * Clamp function builder (saturation)
     *
     * clamp(x, min, max) = { min if x < min, max if x > max, x otherwise }
     *
     * Returns: `x => piecewise([x < min, min], [x > max, max], x)`
     *
     * @param min Minimum value
     * @param max Maximum value
     * @param varName Variable name (default: 'x')
     * @returns SOC lambda expression string
     *
     * @example
     * ```typescript
     * ach.eval(`let clamp = ${ach.conditional.clamp(-1, 1)}`);
     * ach.eval('clamp(-5)');  // "-1"
     * ach.eval('clamp(0.5)'); // "0.5"
     * ach.eval('clamp(10)');  // "1"
     * ```
     */
    clamp(min: number, max: number, varName: string = 'x'): string {
        return `${varName} => piecewise([${varName} < ${min}, ${min}], [${varName} > ${max}, ${max}], ${varName})`;
    }

    /**
     * Progressive tax function builder
     *
     * Creates a piecewise tax function with multiple brackets.
     *
     * @param brackets Array of income thresholds (e.g., [10000, 50000])
     * @param rates Array of tax rates (e.g., [0.1, 0.2, 0.3])
     * @param varName Variable name (default: 'income')
     * @returns SOC lambda expression string
     *
     * @example
     * ```typescript
     * // Tax brackets: 0-10K=10%, 10K-50K=20%, 50K+=30%
     * ach.eval(`let tax = ${ach.conditional.tax([10000, 50000], [0.1, 0.2, 0.3])}`);
     * ach.eval('tax(5000)');   // "500"
     * ach.eval('tax(30000)');  // "6000"
     * ach.eval('tax(100000)'); // "30000"
     * ```
     */
    tax(brackets: number[], rates: number[], varName: string = 'income'): string {
        if (rates.length !== brackets.length + 1) {
            throw new Error('rates array must have one more element than brackets');
        }

        const cases: string[] = [];
        for (let i = 0; i < brackets.length; i++) {
            cases.push(`[${varName} <= ${brackets[i]}, ${varName} * ${rates[i]}]`);
        }

        const defaultRate = rates[rates.length - 1];
        return `${varName} => piecewise(${cases.join(', ')}, ${varName} * ${defaultRate})`;
    }

    /**
     * Square wave function builder
     *
     * Square wave with period 2π: { 1 if (t % 2π) < π, -1 otherwise }
     *
     * Returns: `t => piecewise([(t % (2*PI)) < PI, 1], -1)`
     *
     * @param varName Variable name (default: 't')
     * @returns SOC lambda expression string
     *
     * @example
     * ```typescript
     * ach.eval(`let square = ${ach.conditional.squareWave()}`);
     * // Use with map for signal generation
     * ach.eval('let times = linspace(0, 2*PI, 32)');
     * ach.eval('let signal = map(square, times)');
     * ```
     */
    squareWave(varName: string = 't'): string {
        return `${varName} => piecewise([(${varName} % (2*PI)) < PI, 1], -1)`;
    }

    /**
     * Rectangular pulse function builder
     *
     * Rect(t) = { 1 if |t| ≤ width/2, 0 otherwise }
     *
     * Returns: `t => piecewise([abs(t) <= width/2, 1], 0)`
     *
     * @param width Pulse width (default: 1)
     * @param varName Variable name (default: 't')
     * @returns SOC lambda expression string
     *
     * @example
     * ```typescript
     * ach.eval(`let rect = ${ach.conditional.rect(1)}`);
     * ach.eval('rect(0)');    // "1"
     * ach.eval('rect(0.5)');  // "1"
     * ach.eval('rect(1)');    // "0"
     * ```
     */
    rect(width: number = 1, varName: string = 't'): string {
        const halfWidth = width / 2;
        return `${varName} => piecewise([abs(${varName}) <= ${halfWidth}, 1], 0)`;
    }

    /**
     * Triangle wave function builder
     *
     * Triangle wave with period 2: rising from 0 to 1, then falling back to 0
     *
     * Returns: `t => piecewise([(t % 2) < 1, t % 2], 2 - (t % 2))`
     *
     * @param varName Variable name (default: 't')
     * @returns SOC lambda expression string
     *
     * @example
     * ```typescript
     * ach.eval(`let triangle = ${ach.conditional.triangleWave()}`);
     * ach.eval('let times = linspace(0, 4, 32)');
     * ach.eval('let signal = map(triangle, times)');
     * ```
     */
    triangleWave(varName: string = 't'): string {
        return `${varName} => piecewise([(${varName} % 2) < 1, ${varName} % 2], 2 - (${varName} % 2))`;
    }
}
