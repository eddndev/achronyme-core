// tests/soc.js
export const tests = {
  'soc-simple-expr': async (ach) => {
    ach.resetEvaluator();
    const result = ach.eval("2 + 3 * 4");
    return `2 + 3 * 4 = ${result}`;
  },
  'soc-lambda-create': async (ach) => {
    ach.resetEvaluator();
    const result = ach.eval("x => x * 2");
    return `Created lambda: x => x * 2 (result: ${result})`
  },
  'soc-lambda-call': async (ach) => {
    ach.resetEvaluator();
    ach.eval("let double = x => x * 2");
    const result = ach.eval("double(5)");
    return `double(5) = ${result}`;
  },
  'soc-lambda-closure': async (ach) => {
    ach.resetEvaluator();
    ach.eval("let multiplier = 3");
    ach.eval("let mult = x => x * multiplier");
    const result = ach.eval("mult(4)");
    return `mult(4) with closure (multiplier=3) = ${result}`;
  },
  'soc-map': async (ach) => {
    ach.resetEvaluator();
    const result = ach.eval("map(x => x * 2, [1, 2, 3, 4])");
    return `map(x => x * 2, [1,2,3,4]) = ${result}`;
  },
  'soc-filter': async (ach) => {
    ach.resetEvaluator();
    const result = ach.eval("filter(x => x > 3, [1, 2, 3, 4, 5, 6])");
    return `filter(x => x > 3, [1,2,3,4,5,6]) = ${result}`;
  },
  'soc-reduce': async (ach) => {
    ach.resetEvaluator();
    const result = ach.eval("reduce((acc, x) => acc + x, 0, [1, 2, 3, 4, 5])");
    return `reduce sum [1,2,3,4,5] = ${result}`;
  },
  'soc-pipe': async (ach) => {
    ach.resetEvaluator();
    ach.eval("let addTwo = x => x + 2");
    ach.eval("let double2 = x => x * 2");
    const result = ach.eval("pipe(5, addTwo, double2)");
    return `pipe(5, addTwo, double2) = ${result} (5 + 2 = 7, 7 * 2 = 14)`;
  },
};
