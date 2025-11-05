/**
 * Numerical Calculus Demo
 *
 * Demonstrates numerical differentiation, integration, and root finding
 * using the Achronyme SDK
 */

import { Achronyme } from '../../../src/sdk/Achronyme';

async function main() {
    const ach = new Achronyme();
    await ach.init();

    console.log('=== Achronyme Numerical Calculus Demo ===\n');

    // ========================================================================
    // Numerical Differentiation
    // ========================================================================

    console.log('--- Numerical Differentiation ---');

    await ach.use(async () => {
        // First derivative: f(x) = x^2, f'(2) = 4
        const f1 = ach.eval('x => x^2');
        const df1 = ach.numerical.diff(f1, 2, 1e-5);
        console.log(`diff(x^2, 2) = ${df1.toFixed(6)} (expected: 4.0)`);

        // Second derivative: f(x) = x^3, f''(2) = 12
        const f2 = ach.eval('x => x^3');
        const d2f2 = ach.numerical.diff2(f2, 2, 1e-3);
        console.log(`diff2(x^3, 2) = ${d2f2.toFixed(6)} (expected: 12.0)`);

        // Third derivative: f(x) = x^4, f'''(2) = 48
        const f3 = ach.eval('x => x^4');
        const d3f3 = ach.numerical.diff3(f3, 2, 1e-2);
        console.log(`diff3(x^4, 2) = ${d3f3.toFixed(6)} (expected: 48.0)`);
    });

    // ========================================================================
    // Numerical Integration
    // ========================================================================

    console.log('\n--- Numerical Integration ---');

    await ach.use(async () => {
        // Trapezoidal rule: ∫x dx from 0 to 1 = 0.5
        const f1 = ach.eval('x => x');
        const int1 = ach.numerical.integral(f1, 0, 1, 1000);
        console.log(`integral(x, 0, 1) = ${int1.toFixed(6)} (expected: 0.5)`);

        // Simpson's rule: ∫x^2 dx from 0 to 1 = 1/3
        const f2 = ach.eval('x => x^2');
        const int2 = ach.numerical.simpson(f2, 0, 1, 100);
        console.log(`simpson(x^2, 0, 1) = ${int2.toFixed(6)} (expected: 0.333...)`);

        // Romberg: ∫sin(x) dx from 0 to π = 2
        const f3 = ach.eval('x => sin(x)');
        const int3 = ach.numerical.romberg(f3, 0, Math.PI, 1e-10);
        console.log(`romberg(sin, 0, π) = ${int3.toFixed(6)} (expected: 2.0)`);

        // Adaptive quadrature: ∫e^x dx from 0 to 1 = e - 1
        const f4 = ach.eval('x => exp(x)');
        const int4 = ach.numerical.quad(f4, 0, 1);
        const expected = Math.exp(1) - 1;
        console.log(`quad(exp, 0, 1) = ${int4.toFixed(6)} (expected: ${expected.toFixed(6)})`);
    });

    // ========================================================================
    // Root Finding
    // ========================================================================

    console.log('\n--- Root Finding ---');

    await ach.use(async () => {
        // Bisection: x^2 - 4 = 0, root = 2
        const f1 = ach.eval('x => x^2 - 4');
        const root1 = ach.numerical.solve(f1, 0, 5, 1e-6);
        console.log(`solve(x^2 - 4, [0,5]) = ${root1.toFixed(6)} (expected: 2.0)`);

        // Newton's method: x^2 - 4 = 0, root = 2
        const f2 = ach.eval('x => x^2 - 4');
        const df2 = ach.eval('x => 2*x');
        const root2 = ach.numerical.newton(f2, df2, 1, 1e-10, 100);
        console.log(`newton(x^2 - 4, x0=1) = ${root2.toFixed(6)} (expected: 2.0)`);

        // Secant method: x^3 - x - 2 = 0, root ≈ 1.521
        const f3 = ach.eval('x => x^3 - x - 2');
        const root3 = ach.numerical.secant(f3, 1, 2, 1e-10, 100);
        console.log(`secant(x^3 - x - 2, [1,2]) = ${root3.toFixed(6)} (expected: 1.521...)`);
    });

    console.log('\n✅ All numerical calculus tests completed!');
}

main().catch(console.error);
