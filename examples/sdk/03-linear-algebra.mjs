import { Achronyme } from '../../src/sdk/index.js';

async function main() {
  console.log('--- SDK Example: Linear Algebra ---');

  const ach = new Achronyme();
  await ach.init();

  await ach.use(async () => {
    // Create a 2x2 matrix
    const A = ach.matrix([
      [4, 7],
      [2, 6]
    ]);
    console.log('Matrix A:\n', A.toArray());

    // --- Basic Operations ---
    const A_t = ach.linalg.transpose(A);
    console.log('Transpose of A:\n', A_t.toArray());

    const detA = ach.linalg.det(A);
    console.log(`Determinant of A: ${detA.toFixed(2)}`); // 4*6 - 7*2 = 24 - 14 = 10

    const invA = ach.linalg.inverse(A);
    console.log('Inverse of A:\n', invA.toArray());

    // --- Decompositions ---
    console.log('\n--- Decompositions ---');
    const B = ach.matrix([
      [1, 4, 8, 4],
      [4, 2, 3, 7],
      [8, 3, 6, 9],
      [4, 7, 9, 2],
    ]);

    // LU Decomposition
    const { L, U, P } = ach.linalg.lu(B);
    console.log('LU Decomposition of B:');
    console.log('  - L Matrix (first 2x2):\n', L.toArray().slice(0,2).map(r => r.slice(0,2)));
    console.log('  - U Matrix (first 2x2):\n', U.toArray().slice(0,2).map(r => r.slice(0,2)));

    // All created matrices (A, A_t, invA, B, L, U, P) will be auto-disposed.
  });

  console.log('--- End of Linear Algebra Example ---\n');
}

main().catch(console.error);
