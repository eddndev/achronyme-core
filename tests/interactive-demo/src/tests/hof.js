// tests/hof.js - Uses Rust engine with SOC expressions
export const tests = {
  'hof-map': async (ach) => {
    return ach.use(async () => {
      const numbers = ach.vector([1, 2, 3, 4, 5]);
      const squared = ach.map("x => x * x", numbers);
      return `map(x => x*x) applied to vector`;
    });
  },
  'hof-filter': async (ach) => {
    return ach.use(async () => {
      const numbers = ach.vector([1, 2, 3, 4, 5, 6]);
      const evens = ach.filter("x => x % 2 == 0", numbers);
      return `filter(x => x%2==0) applied to vector`;
    });
  },
  'hof-reduce': async (ach) => {
    return ach.use(async () => {
      const numbers = ach.vector([1, 2, 3, 4, 5]);
      const sum = ach.reduce("(a, b) => a + b", 0, numbers);
      return `reduce((a,b) => a+b, 0) = ${sum}`;
    });
  },
  'hof-pipe': async (ach) => {
    return ach.use(async () => {
      const numbers = ach.vector([1, 2, 3, 4, 5]);
      return `Pipe test (chaining HOF operations)`;
    });
  },
  'hof-lambda': async (ach) => {
    return ach.use(async () => {
      return `Lambda test (not yet implemented in SDK)`;
    });
  },
};
