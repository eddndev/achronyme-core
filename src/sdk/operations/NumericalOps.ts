/**
 * Numerical Calculus Operations
 *
 * Provides numerical methods for:
 * - Differentiation (derivatives)
 * - Integration (quadrature)
 * - Root finding (solving equations)
 */

import type { RustWASM } from '../core/RustBindings';
import type { Handle } from '../types';

export class NumericalOps {
  constructor(private bindings: RustWASM) {}

  // ============================================================================
  // Numerical Differentiation
  // ============================================================================

  /**
   * Compute first derivative using central difference method
   * f'(x) ≈ (f(x+h) - f(x-h)) / (2h)
   *
   * @param funcHandle - Handle to the function to differentiate
   * @param x - Point at which to compute the derivative
   * @param h - Step size (default: 1e-5)
   * @returns The numerical derivative f'(x)
   *
   * @example
   * ```typescript
   * // f(x) = x^2, f'(2) = 4
   * const f = session.use(ctx => ctx.eval('x => x^2'));
   * const derivative = numerical.diff(f, 2, 1e-5);
   * console.log(derivative); // ~4.0
   * ```
   */
  diff(funcHandle: Handle, x: number, h: number = 1e-5): number {
    return this.bindings.numDiff(funcHandle, x, h);
  }

  /**
   * Compute second derivative using central difference method
   * f''(x) ≈ (f(x+h) - 2f(x) + f(x-h)) / h²
   *
   * @param funcHandle - Handle to the function
   * @param x - Point at which to compute the second derivative
   * @param h - Step size (default: 1e-3)
   * @returns The numerical second derivative f''(x)
   *
   * @example
   * ```typescript
   * // f(x) = x^3, f''(2) = 12
   * const f = session.use(ctx => ctx.eval('x => x^3'));
   * const d2 = numerical.diff2(f, 2, 1e-3);
   * console.log(d2); // ~12.0
   * ```
   */
  diff2(funcHandle: Handle, x: number, h: number = 1e-3): number {
    return this.bindings.numDiff2(funcHandle, x, h);
  }

  /**
   * Compute third derivative using central difference method
   *
   * @param funcHandle - Handle to the function
   * @param x - Point at which to compute the third derivative
   * @param h - Step size (default: 1e-2)
   * @returns The numerical third derivative f'''(x)
   *
   * @example
   * ```typescript
   * // f(x) = x^4, f'''(2) = 48
   * const f = session.use(ctx => ctx.eval('x => x^4'));
   * const d3 = numerical.diff3(f, 2, 1e-2);
   * console.log(d3); // ~48.0
   * ```
   */
  diff3(funcHandle: Handle, x: number, h: number = 1e-2): number {
    return this.bindings.numDiff3(funcHandle, x, h);
  }

  // ============================================================================
  // Numerical Integration
  // ============================================================================

  /**
   * Numerical integration using trapezoidal rule
   * ∫f(x)dx ≈ h/2 * (f(x₀) + 2f(x₁) + ... + 2f(xₙ₋₁) + f(xₙ))
   *
   * @param funcHandle - Handle to the function to integrate
   * @param a - Lower limit of integration
   * @param b - Upper limit of integration
   * @param n - Number of subdivisions (higher = more accurate)
   * @returns The numerical integral
   *
   * @example
   * ```typescript
   * // ∫x dx from 0 to 1 = 0.5
   * const f = session.use(ctx => ctx.eval('x => x'));
   * const integral = numerical.integral(f, 0, 1, 1000);
   * console.log(integral); // ~0.5
   * ```
   */
  integral(funcHandle: Handle, a: number, b: number, n: number): number {
    return this.bindings.numIntegral(funcHandle, a, b, n);
  }

  /**
   * Numerical integration using Simpson's 1/3 rule
   * More accurate than trapezoidal rule for smooth functions
   *
   * @param funcHandle - Handle to the function to integrate
   * @param a - Lower limit of integration
   * @param b - Upper limit of integration
   * @param n - Number of subdivisions (will be rounded to even)
   * @returns The numerical integral
   *
   * @example
   * ```typescript
   * // ∫x² dx from 0 to 1 = 1/3
   * const f = session.use(ctx => ctx.eval('x => x^2'));
   * const integral = numerical.simpson(f, 0, 1, 100);
   * console.log(integral); // ~0.333...
   * ```
   */
  simpson(funcHandle: Handle, a: number, b: number, n: number): number {
    return this.bindings.numSimpson(funcHandle, a, b, n);
  }

  /**
   * Romberg integration using Richardson extrapolation
   * High-accuracy adaptive integration method
   *
   * @param funcHandle - Handle to the function to integrate
   * @param a - Lower limit of integration
   * @param b - Upper limit of integration
   * @param tol - Desired tolerance (default: 1e-10)
   * @returns The numerical integral
   *
   * @example
   * ```typescript
   * // ∫sin(x) dx from 0 to π = 2
   * const f = session.use(ctx => ctx.eval('x => sin(x)'));
   * const integral = numerical.romberg(f, 0, Math.PI, 1e-10);
   * console.log(integral); // ~2.0
   * ```
   */
  romberg(funcHandle: Handle, a: number, b: number, tol: number = 1e-10): number {
    return this.bindings.numRomberg(funcHandle, a, b, tol);
  }

  /**
   * Adaptive quadrature integration
   * Automatically adapts step size to achieve high accuracy
   *
   * @param funcHandle - Handle to the function to integrate
   * @param a - Lower limit of integration
   * @param b - Upper limit of integration
   * @returns The numerical integral
   *
   * @example
   * ```typescript
   * // ∫e^x dx from 0 to 1 = e - 1 ≈ 1.718
   * const f = session.use(ctx => ctx.eval('x => exp(x)'));
   * const integral = numerical.quad(f, 0, 1);
   * console.log(integral); // ~1.718...
   * ```
   */
  quad(funcHandle: Handle, a: number, b: number): number {
    return this.bindings.numQuad(funcHandle, a, b);
  }

  // ============================================================================
  // Root Finding
  // ============================================================================

  /**
   * Find root using bisection method
   * Finds x such that f(x) = 0 in interval [a, b]
   * Requires f(a) and f(b) to have opposite signs
   *
   * @param funcHandle - Handle to the function
   * @param a - Left endpoint of interval
   * @param b - Right endpoint of interval
   * @param tol - Tolerance (stop when |b - a| < tol)
   * @returns The root x where f(x) ≈ 0
   *
   * @example
   * ```typescript
   * // Find root of x² - 4 = 0 in [0, 5] (root = 2)
   * const f = session.use(ctx => ctx.eval('x => x^2 - 4'));
   * const root = numerical.solve(f, 0, 5, 1e-6);
   * console.log(root); // ~2.0
   * ```
   */
  solve(funcHandle: Handle, a: number, b: number, tol: number): number {
    return this.bindings.numSolve(funcHandle, a, b, tol);
  }

  /**
   * Find root using Newton's method
   * Faster convergence than bisection, but requires derivative
   * x_{n+1} = x_n - f(x_n) / f'(x_n)
   *
   * @param funcHandle - Handle to the function
   * @param dfuncHandle - Handle to the derivative function
   * @param x0 - Initial guess
   * @param tol - Tolerance (stop when |f(x)| < tol)
   * @param maxIter - Maximum number of iterations
   * @returns The root x where f(x) ≈ 0
   *
   * @example
   * ```typescript
   * // Find root of x² - 4 = 0 (root = 2)
   * const f = session.use(ctx => ctx.eval('x => x^2 - 4'));
   * const df = session.use(ctx => ctx.eval('x => 2*x'));
   * const root = numerical.newton(f, df, 1, 1e-10, 100);
   * console.log(root); // ~2.0
   * ```
   */
  newton(
    funcHandle: Handle,
    dfuncHandle: Handle,
    x0: number,
    tol: number,
    maxIter: number
  ): number {
    return this.bindings.numNewton(funcHandle, dfuncHandle, x0, tol, maxIter);
  }

  /**
   * Find root using secant method
   * Similar to Newton but doesn't require derivative
   * Uses two initial points instead
   *
   * @param funcHandle - Handle to the function
   * @param x0 - First initial guess
   * @param x1 - Second initial guess
   * @param tol - Tolerance (stop when |f(x)| < tol)
   * @param maxIter - Maximum number of iterations
   * @returns The root x where f(x) ≈ 0
   *
   * @example
   * ```typescript
   * // Find root of x³ - x - 2 = 0 (root ≈ 1.521)
   * const f = session.use(ctx => ctx.eval('x => x^3 - x - 2'));
   * const root = numerical.secant(f, 1, 2, 1e-10, 100);
   * console.log(root); // ~1.521...
   * ```
   */
  secant(
    funcHandle: Handle,
    x0: number,
    x1: number,
    tol: number,
    maxIter: number
  ): number {
    return this.bindings.numSecant(funcHandle, x0, x1, tol, maxIter);
  }
}
