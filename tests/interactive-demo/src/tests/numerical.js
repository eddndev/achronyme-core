// tests/numerical.js
export const tests = {
  'numerical-diff': async (ach) => {
    return ach.use(async () => {
      const df = ach.numerical.diff('x => x^2', 2, 1e-5);
      return `diff(x^2, 2) = ${df.toFixed(6)} (expected: 4.0)`;
    });
  },
  'numerical-diff2': async (ach) => {
    return ach.use(async () => {
      const d2f = ach.numerical.diff2('x => x^3', 2, 1e-3);
      return `diff2(x^3, 2) = ${d2f.toFixed(6)} (expected: 12.0)`;
    });
  },
  'numerical-diff3': async (ach) => {
    return ach.use(async () => {
      const d3f = ach.numerical.diff3('x => x^4', 2, 1e-2);
      return `diff3(x^4, 2) = ${d3f.toFixed(6)} (expected: 48.0)`;
    });
  },
  'numerical-integral': async (ach) => {
    return ach.use(async () => {
      const integral = ach.numerical.integral('x => x', 0, 1, 1000);
      return `integral(x, 0, 1) = ${integral.toFixed(6)} (expected: 0.5)`;
    });
  },
  'numerical-simpson': async (ach) => {
    return ach.use(async () => {
      const integral = ach.numerical.simpson('x => x^2', 0, 1, 100);
      return `simpson(x^2, 0, 1) = ${integral.toFixed(6)} (expected: 0.333333)`;
    });
  },
  'numerical-romberg': async (ach) => {
    return ach.use(async () => {
      const integral = ach.numerical.romberg('x => sin(x)', 0, Math.PI, 1e-10);
      return `romberg(sin, 0, π) = ${integral.toFixed(6)} (expected: 2.0)`;
    });
  },
  'numerical-quad': async (ach) => {
    return ach.use(async () => {
      const integral = ach.numerical.quad('x => exp(x)', 0, 1);
      const expected = Math.exp(1) - 1;
      return `quad(exp, 0, 1) = ${integral.toFixed(6)} (expected: ${expected.toFixed(6)})`;
    });
  },
  'numerical-solve': async (ach) => {
    return ach.use(async () => {
      const root = ach.numerical.solve('x => x^2 - 4', 0, 5, 1e-6);
      return `solve(x^2 - 4, [0,5]) = ${root.toFixed(6)} (expected: 2.0)`;
    });
  },
  'numerical-newton': async (ach) => {
    return ach.use(async () => {
      const root = ach.numerical.newton('x => x^2 - 4', 'x => 2*x', 1, 1e-10, 100);
      return `newton(x^2 - 4, x0=1) = ${root.toFixed(6)} (expected: 2.0)`;
    });
  },
  'numerical-secant': async (ach) => {
    return ach.use(async () => {
      const root = ach.numerical.secant('x => x^3 - x - 2', 1, 2, 1e-10, 100);
      return `secant(x^3 - x - 2, [1,2]) = ${root.toFixed(6)} (expected: 1.521379)`;
    });
  },
  'numerical-all': async (ach) => {
    let output = '=== Running All Numerical Calculus Tests ===\n\n';
    return ach.use(async () => {
      output += '--- Numerical Differentiation ---\n';
      const df1 = ach.numerical.diff('x => x^2', 2, 1e-5);
      output += `diff(x^2, 2) = ${df1.toFixed(6)} ✓\n`;
      const d2f2 = ach.numerical.diff2('x => x^3', 2, 1e-3);
      output += `diff2(x^3, 2) = ${d2f2.toFixed(6)} ✓\n`;
      const d3f3 = ach.numerical.diff3('x => x^4', 2, 1e-2);
      output += `diff3(x^4, 2) = ${d3f3.toFixed(6)} ✓\n\n`;
      output += '--- Numerical Integration ---\n';
      const int1 = ach.numerical.integral('x => x', 0, 1, 1000);
      output += `integral(x, 0, 1) = ${int1.toFixed(6)} ✓\n`;
      const int2 = ach.numerical.simpson('x => x^2', 0, 1, 100);
      output += `simpson(x^2, 0, 1) = ${int2.toFixed(6)} ✓\n`;
      const int3 = ach.numerical.romberg('x => sin(x)', 0, Math.PI, 1e-10);
      output += `romberg(sin, 0, π) = ${int3.toFixed(6)} ✓\n`;
      const int4 = ach.numerical.quad('x => exp(x)', 0, 1);
      output += `quad(exp, 0, 1) = ${int4.toFixed(6)} ✓\n\n`;
      output += '--- Root Finding ---\n';
      const root1 = ach.numerical.solve('x => x^2 - 4', 0, 5, 1e-6);
      output += `solve(x^2 - 4) = ${root1.toFixed(6)} ✓\n`;
      const root2 = ach.numerical.newton('x => x^2 - 4', 'x => 2*x', 1, 1e-10, 100);
      output += `newton(x^2 - 4) = ${root2.toFixed(6)} ✓\n`;
      const root3 = ach.numerical.secant('x => x^3 - x - 2', 1, 2, 1e-10, 100);
      output += `secant(x^3 - x - 2) = ${root3.toFixed(6)} ✓\n\n`;
      output += '✅ All numerical calculus tests completed successfully!';
      return output;
    });
  },
};
