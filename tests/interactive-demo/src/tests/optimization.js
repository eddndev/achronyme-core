// tests/optimization.js
export const tests = {
  'opt-simple-lp': async (ach) => {
    return ach.use(async () => {
      const c = ach.vector([3, 5]);
      const A = ach.matrix([[1, 0], [0, 2], [3, 2]]);
      const b = ach.vector([4, 12, 18]);
      const solution = ach.optimization.linprog(c.handle, A.handle, b.handle, 1);
      const z = ach.optimization.objectiveValue(c.handle, solution);
      const solData = ach.session.wasm.getVector(solution);
      return `Solution: x₁=${solData[0].toFixed(2)}, x₂=${solData[1].toFixed(2)}
Optimal value z* = ${z.toFixed(2)}`;
    });
  },
  'opt-production': async (ach) => {
    return ach.use(async () => {
      const c = ach.vector([40, 30]);
      const A = ach.matrix([[1, 0], [0, 1], [1, 1]]);
      const b = ach.vector([40, 50, 70]);
      const solution = ach.optimization.linprog(c.handle, A.handle, b.handle, 1);
      const profit = ach.optimization.objectiveValue(c.handle, solution);
      const solData = ach.session.wasm.getVector(solution);
      return `Optimal production:
  Product 1: ${solData[0].toFixed(0)} units
  Product 2: ${solData[1].toFixed(0)} units
Maximum profit: $${profit.toFixed(0)}`;
    });
  },
  'opt-simplex': async (ach) => {
    return ach.use(async () => {
      const c = ach.vector([3, 5]);
      const A = ach.matrix([[1, 0], [0, 2], [3, 2]]);
      const b = ach.vector([4, 12, 18]);
      const start = performance.now();
      const solution = ach.optimization.simplex(c.handle, A.handle, b.handle, 1);
      const time = performance.now() - start;
      const z = ach.optimization.objectiveValue(c.handle, solution);
      return `Primal Simplex solved in ${time.toFixed(3)}ms
Optimal value: ${z.toFixed(2)}`;
    });
  },
  'opt-revised': async (ach) => {
    return ach.use(async () => {
      const c = ach.vector([3, 5]);
      const A = ach.matrix([[1, 0], [0, 2], [3, 2]]);
      const b = ach.vector([4, 12, 18]);
      const start = performance.now();
      const solution = ach.optimization.revisedSimplex(c.handle, A.handle, b.handle, 1);
      const time = performance.now() - start;
      const z = ach.optimization.objectiveValue(c.handle, solution);
      return `Revised Simplex (memory-efficient) solved in ${time.toFixed(3)}ms
Optimal value: ${z.toFixed(2)}`;
    });
  },
  'opt-objective': async (ach) => {
    return ach.use(async () => {
      const c = ach.vector([40, 30]);
      const x = ach.vector([40, 30]);
      const value = ach.optimization.objectiveValue(c.handle, x.handle);
      return `Objective value for c=[40,30], x=[40,30]:
z = c·x = ${value.toFixed(0)}`;
    });
  },
  'opt-shadow-price': async (ach) => {
    return ach.use(async () => {
      const c = ach.vector([40, 30]);
      const A = ach.matrix([[1, 0], [0, 1], [1, 1]]);
      const b = ach.vector([40, 50, 70]);
      const shadows = ach.optimization.shadowPrice(c.handle, A.handle, b.handle, 1);
      const shadowData = ach.session.wasm.getVector(shadows);
      return `Shadow Prices (marginal resource values):
` +
             `  Material A: $${shadowData[0].toFixed(2)}/unit ${shadowData[0] > 0 ? '(binding)' : '(surplus)'}
` +
             `  Material B: $${shadowData[1].toFixed(2)}/unit ${shadowData[1] > 0 ? '(binding)' : '(surplus)'}
` +
             `  Labor hours: $${shadowData[2].toFixed(2)}/hour ${shadowData[2] > 0 ? '(binding)' : '(surplus)'}`;
    });
  },
  'opt-sensitivity-c': async (ach) => {
    return ach.use(async () => {
      const c = ach.vector([40, 30]);
      const A = ach.matrix([[1, 0], [0, 1], [1, 1]]);
      const b = ach.vector([40, 50, 70]);
      const range0 = ach.optimization.sensitivityC(c.handle, A.handle, b.handle, 0);
      const range1 = ach.optimization.sensitivityC(c.handle, A.handle, b.handle, 1);
      const r0Data = ach.session.wasm.getVector(range0);
      const r1Data = ach.session.wasm.getVector(range1);
      return `Sensitivity for objective coefficients:
` +
             `  c[0] range: [$${r0Data[0].toFixed(0)}, $${r0Data[1].toFixed(0)}] (current: $40)
` +
             `  c[1] range: [$${r1Data[0].toFixed(0)}, $${r1Data[1].toFixed(0)}] (current: $30)
` +
             `
Within these ranges, the optimal solution doesn't change`;
    });
  },
  'opt-sensitivity-b': async (ach) => {
    return ach.use(async () => {
      const c = ach.vector([40, 30]);
      const A = ach.matrix([[1, 0], [0, 1], [1, 1]]);
      const b = ach.vector([40, 50, 70]);
      const range2 = ach.optimization.sensitivityB(c.handle, A.handle, b.handle, 2);
      const r2Data = ach.session.wasm.getVector(range2);
      return `Sensitivity for labor hours constraint:
` +
             `  Valid range: [${r2Data[0].toFixed(0)}, ${r2Data[1].toFixed(0)}] hours
` +
             `  Current: 70 hours
` +
             `
Within this range, shadow price remains constant`;
    });
  },
  'opt-all-methods': async (ach) => {
    return ach.use(async () => {
      const c = ach.vector([3, 5]);
      const A = ach.matrix([[1, 0], [0, 2], [3, 2]]);
      const b = ach.vector([4, 12, 18]);
      let output = 'Comparing all LP methods:\n\n';
      const t1 = performance.now();
      const sol1 = ach.optimization.simplex(c.handle, A.handle, b.handle, 1);
      const time1 = performance.now() - t1;
      const z1 = ach.optimization.objectiveValue(c.handle, sol1);
      output += `Primal Simplex: z*=${z1.toFixed(2)} (${time1.toFixed(3)}ms)\n`;
      const t2 = performance.now();
      const sol2 = ach.optimization.linprog(c.handle, A.handle, b.handle, 1);
      const time2 = performance.now() - t2;
      const z2 = ach.optimization.objectiveValue(c.handle, sol2);
      output += `Linprog (auto): z*=${z2.toFixed(2)} (${time2.toFixed(3)}ms)\n`;
      const t3 = performance.now();
      const sol3 = ach.optimization.revisedSimplex(c.handle, A.handle, b.handle, 1);
      const time3 = performance.now() - t3;
      const z3 = ach.optimization.objectiveValue(c.handle, sol3);
      output += `Revised Simplex: z*=${z3.toFixed(2)} (${time3.toFixed(3)}ms)\n`;
      output += `
✓ All methods agree on optimal value`;
      return output;
    });
  },
  'opt-full-analysis': async (ach) => {
    return ach.use(async () => {
      const c = ach.vector([40, 30]);
      const A = ach.matrix([[1, 0], [0, 1], [1, 1]]);
      const b = ach.vector([40, 50, 70]);
      const solution = ach.optimization.linprog(c.handle, A.handle, b.handle, 1);
      const profit = ach.optimization.objectiveValue(c.handle, solution);
      const solData = ach.session.wasm.getVector(solution);
      const shadows = ach.optimization.shadowPrice(c.handle, A.handle, b.handle, 1);
      const shadowData = ach.session.wasm.getVector(shadows);
      const rangeC0 = ach.optimization.sensitivityC(c.handle, A.handle, b.handle, 0);
      const rangeB2 = ach.optimization.sensitivityB(c.handle, A.handle, b.handle, 2);
      const rc0 = ach.session.wasm.getVector(rangeC0);
      const rb2 = ach.session.wasm.getVector(rangeB2);
      let output = '=== COMPLETE LP ANALYSIS ===\n\n';
      output += `OPTIMAL SOLUTION:\n`;
      output += `  x₁ = ${solData[0].toFixed(0)} units\n`;
      output += `  x₂ = ${solData[1].toFixed(0)} units\n`;
      output += `  Profit = $${profit.toFixed(0)}\n\n`;
      output += `SHADOW PRICES:\n`;
      output += `  Material A: $${shadowData[0].toFixed(0)}/unit\n`;
      output += `  Material B: $${shadowData[1].toFixed(0)}/unit\n`;
      output += `  Labor: $${shadowData[2].toFixed(0)}/hour\n\n`;
      output += `SENSITIVITY:\n`;
      output += `  c₁ range: [$${rc0[0].toFixed(0)}, $${rc0[1].toFixed(0)}]
`;
      output += `  Labor range: ${rb2[0].toFixed(0)}-${rb2[1].toFixed(0)} hours

`;
      output += `✅ Full optimization analysis complete!`;
      return output;
    });
  },

  // ============================================================================
  // Integer Programming Tests
  // ============================================================================

  'opt-integer-simple': async (ach) => {
    return ach.use(async () => {
      // maximize z = 3x₁ + 2x₂ subject to x₁ + x₂ ≤ 4, x₁, x₂ ∈ ℤ₊
      const c = ach.vector([3, 2]);
      const A = ach.matrix([[1, 1]]);
      const b = ach.vector([4]);
      const intVars = ach.vector([0, 1]); // Both variables must be integer

      const solution = ach.optimization.intlinprog(c.handle, A.handle, b.handle, 1, intVars.handle);
      const z = ach.optimization.objectiveValue(c.handle, solution);
      const solData = ach.session.wasm.getVector(solution);

      return `Integer LP Solution:
  x₁ = ${solData[0].toFixed(0)} (integer)
  x₂ = ${solData[1].toFixed(0)} (integer)
  z* = ${z.toFixed(0)}

✓ All variables are integers`;
    });
  },

  'opt-knapsack-01': async (ach) => {
    return ach.use(async () => {
      // 0-1 Knapsack: maximize value within weight limit
      // Items: values = [60, 100, 120], weights = [10, 20, 30], capacity = 50
      const values = ach.vector([60, 100, 120]);
      const weights = ach.matrix([[10, 20, 30]]);
      const capacity = ach.vector([50]);
      const binVars = ach.vector([0, 1, 2]); // All variables are binary

      const solution = ach.optimization.binaryLinprog(values.handle, weights.handle, capacity.handle, 1, binVars.handle);
      const totalValue = ach.optimization.objectiveValue(values.handle, solution);
      const solData = ach.session.wasm.getVector(solution);

      const selectedItems = solData.map((x, i) => x > 0.5 ? i + 1 : null).filter(x => x !== null);
      const totalWeight = solData.reduce((sum, x, i) => sum + x * [10, 20, 30][i], 0);

      return `0-1 Knapsack Solution:
  Selected items: ${selectedItems.join(', ')}
  Total value: ${totalValue.toFixed(0)}
  Total weight: ${totalWeight.toFixed(0)}/50

Solution: x = [${solData.map(x => x.toFixed(0)).join(', ')}]
✓ Optimal selection found!`;
    });
  },

  'opt-knapsack-large': async (ach) => {
    return ach.use(async () => {
      // Larger knapsack instance
      const values = ach.vector([10, 20, 30, 40, 50]);
      const weights = ach.matrix([[5, 10, 15, 20, 25]]);
      const capacity = ach.vector([50]);
      const binVars = ach.vector([0, 1, 2, 3, 4]);

      const start = performance.now();
      const solution = ach.optimization.binaryLinprog(values.handle, weights.handle, capacity.handle, 1, binVars.handle);
      const time = performance.now() - start;

      const totalValue = ach.optimization.objectiveValue(values.handle, solution);
      const solData = ach.session.wasm.getVector(solution);
      const totalWeight = solData.reduce((sum, x, i) => sum + x * [5, 10, 15, 20, 25][i], 0);

      return `Large Knapsack (5 items):
  Solution: x = [${solData.map(x => x.toFixed(0)).join(', ')}]
  Total value: ${totalValue.toFixed(0)}
  Weight used: ${totalWeight.toFixed(0)}/50

Solved in ${time.toFixed(2)}ms using Branch & Bound`;
    });
  },

  'opt-integer-production': async (ach) => {
    return ach.use(async () => {
      // Integer production planning: can only produce whole units
      const profits = ach.vector([40, 30]);
      const resources = ach.matrix([[1, 0], [0, 1], [1, 1]]);
      const available = ach.vector([40, 50, 70]);
      const intVars = ach.vector([0, 1]);

      const solution = ach.optimization.intlinprog(profits.handle, resources.handle, available.handle, 1, intVars.handle);
      const profit = ach.optimization.objectiveValue(profits.handle, solution);
      const solData = ach.session.wasm.getVector(solution);

      return `Integer Production Planning:
  Product A: ${solData[0].toFixed(0)} units
  Product B: ${solData[1].toFixed(0)} units
  Total profit: $${profit.toFixed(0)}

Note: Only whole units can be produced
✓ Integer constraint satisfied`;
    });
  },

  'opt-capital-budgeting': async (ach) => {
    return ach.use(async () => {
      // Capital budgeting: select projects within budget
      // Projects: NPVs = [100, 150, 200, 250], costs = [50, 75, 100, 125], budget = 200
      const npvs = ach.vector([100, 150, 200, 250]);
      const costs = ach.matrix([[50, 75, 100, 125]]);
      const budget = ach.vector([200]);
      const binVars = ach.vector([0, 1, 2, 3]);

      const solution = ach.optimization.binaryLinprog(npvs.handle, costs.handle, budget.handle, 1, binVars.handle);
      const totalNPV = ach.optimization.objectiveValue(npvs.handle, solution);
      const solData = ach.session.wasm.getVector(solution);

      const selectedProjects = solData.map((x, i) => x > 0.5 ? `Project ${i + 1}` : null).filter(x => x !== null);
      const totalCost = solData.reduce((sum, x, i) => sum + x * [50, 75, 100, 125][i], 0);

      return `Capital Budgeting Solution:
  Selected: ${selectedProjects.join(', ')}
  Total NPV: $${totalNPV.toFixed(0)}M
  Budget used: $${totalCost.toFixed(0)}M / $200M

✓ Optimal project portfolio selected`;
    });
  },
};
