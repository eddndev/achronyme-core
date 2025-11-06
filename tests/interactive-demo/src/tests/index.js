// tests/index.js
import { tests as basicTests } from './basic.js';
import { tests as vectorTests } from './vector.js';
import { tests as complexTests } from './complex.js';
import { tests as dspTests } from './dsp.js';
import { tests as matrixTests } from './matrix.js';
import { tests as hofTests } from './hof.js';
import { tests as socTests } from './soc.js';
import { tests as numericalTests } from './numerical.js';
import { tests as optimizationTests } from './optimization.js';
import { tests as stressTests } from './stress.js';
import { tests as benchmarkTests } from './benchmarks.js';

export const allTests = {
  ...basicTests,
  ...vectorTests,
  ...complexTests,
  ...dspTests,
  ...matrixTests,
  ...hofTests,
  ...socTests,
  ...numericalTests,
  ...optimizationTests,
  ...stressTests,
  ...benchmarkTests,
};
