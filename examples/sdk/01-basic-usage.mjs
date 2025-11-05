import { Achronyme } from '../../src/sdk/index.js';

async function main() {
  console.log('--- SDK Example: Basic Usage ---');

  const ach = new Achronyme();
  await ach.init();

  // The `ach.use()` block is the recommended way to work,
  // as it automatically manages memory and cleans up all created
  // values (vectors, matrices, etc.) when the block is exited.
  await ach.use(async () => {
    console.log('Inside ach.use() session...');

    // Create a scalar value
    const s = ach.scalar(10);
    console.log(`Scalar value: ${s.value}`);

    // Create a vector
    const v = ach.vector([1, 2, 3, 4]);
    console.log(`Vector: ${v.toString()}`);

    // Perform operations using the modular API
    // All operations create new values
    const v_plus_10 = v.map(val => val + s.value);
    const sin_v = ach.math.sin(v);

    console.log(`Vector + 10: ${v_plus_10.toString()}`);
    console.log(`sin(Vector): ${sin_v.toString()}`);

    // Access data with a zero-copy view (very fast)
    const sin_v_data = sin_v.data;
    console.log('Zero-copy view of sin(v):', sin_v_data);

    // Get a standard JS array copy (slower, but useful for external libraries)
    const sin_v_array = sin_v.toArray();
    console.log('Array copy of sin(v):', sin_v_array);

    console.log(`Active values before exiting session: ${ach.getActiveValuesCount()}`);
  });

  console.log(`Active values after session: ${ach.getActiveValuesCount()}`);
  console.log('--- End of Basic Usage Example ---\n');
}

main().catch(console.error);
