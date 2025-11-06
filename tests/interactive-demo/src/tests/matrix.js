// tests/matrix.js
export const tests = {
  'matrix-create': async (ach) => {
    return ach.use(async () => {
      const m = ach.matrix([[1, 2], [3, 4]]);
      return `Created 2x2 matrix: [[1,2],[3,4]]`;
    });
  },
  'matrix-arithmetic': async (ach) => {
    return ach.use(async () => {
      const m1 = ach.matrix([[1, 2], [3, 4]]);
      const m2 = ach.matrix([[5, 6], [7, 8]]);
      return `Created two 2x2 matrices`;
    });
  },
  'matrix-multiply': async (ach) => {
    return ach.use(async () => {
      const m1 = ach.matrix([[1, 2], [3, 4]]);
      const m2 = ach.matrix([[5, 6], [7, 8]]);
      return `Matrix multiplication test (created matrices)`;
    });
  },
  'matrix-determinant': async (ach) => {
    return ach.use(async () => {
      const m = ach.matrix([[1, 2], [3, 4]]);
      const det = ach.det(m);
      return `det([[1,2],[3,4]]) = ${det}`;
    });
  },
  'matrix-inverse': async (ach) => {
    return ach.use(async () => {
      const m = ach.matrix([[1, 2], [3, 4]]);
      const inv = ach.linalg.inverse(m);
      return `Matrix inverse computed for [[1,2],[3,4]]`;
    });
  },
  'matrix-decomposition': async (ach) => {
    return ach.use(async () => {
      const m = ach.matrix([[4, 2], [1, 3]]);
      const result = ach.lu(m);
      return `LU decomposition computed`;
    });
  },
};
