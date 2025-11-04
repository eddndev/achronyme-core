# Memory Management Guide - Achronyme SDK v2.0

This guide explains how to manage memory effectively in the Achronyme SDK to prevent leaks and ensure optimal performance.

## The Core Concept: WASM Memory

Values created with Achronyme (like `Vector` and `Matrix`) reside in a special memory space managed by WebAssembly (WASM). The JavaScript garbage collector **cannot** see or clean up this memory automatically.

If you create values and don't release them, you will have a **memory leak**.

The SDK provides two patterns to manage this: a safe, automatic session-based approach, and a manual approach for advanced control.

---

## Pattern 1: Session-Based Management with `use()` (Recommended)

The safest and easiest way to manage memory is with the `ach.use()` method. It creates a temporary "session" or scope. Any Achronyme `Value` created inside this scope is automatically tracked and disposed of when the scope exits.

### How It Works

```typescript
import { Achronyme } from '@achronyme/core';

const ach = new Achronyme();
await ach.init();

console.log('Before use():', ach.getActiveValuesCount()); // 0

// Start a managed session
await ach.use(async () => {
    console.log('Inside use():', ach.getActiveValuesCount()); // 0

    const v1 = ach.vector([1, 2, 3]);
    const v2 = ach.vector([4, 5, 6]);
    const result = ach.vecOps.vadd(v1, v2);

    console.log('Values created:', ach.getActiveValuesCount()); // 3

    const data = result.toArray(); // Copy data out if needed
    console.log('Result:', data);

    // No need to call dispose()!
    // v1, v2, and result will be cleaned up automatically.
});

console.log('After use():', ach.getActiveValuesCount()); // 0. All cleaned up!
```

### Advantages of `use()`

-   **Leak-Proof**: Guarantees cleanup, even if errors are thrown inside the block.
-   **Simple**: No need to manually track and call `.dispose()` on every value.
-   **Clean Code**: Keeps resource management logic contained and implicit.

**Rule of Thumb: Always wrap your Achronyme operations in `ach.use()` unless you have a specific reason not to.**

---

## Pattern 2: Manual Cleanup

For long-running applications or complex scenarios where you need fine-grained control over a value's lifetime, you can manage memory manually.

### How It Works

When you create a value outside of a `use()` block, you are responsible for calling its `.dispose()` method.

```typescript
const ach = new Achronyme();
await ach.init();

// 1. Create values
const v1 = ach.vector([1, 2, 3]);
const v2 = ach.math.sin(v1);

console.log('Active values:', ach.getActiveValuesCount()); // 2

// 2. Use them
const data = v2.toArray();
console.log(data);

// 3. Manually dispose of them
v1.dispose();
v2.dispose();

console.log('Active values after dispose:', ach.getActiveValuesCount()); // 0
```

### `try...finally` for Safety

When managing memory manually, always use a `try...finally` block to ensure cleanup happens even if an error occurs.

```typescript
const v = ach.vector([1, 2, 3]);
try {
    // ... perform operations that might fail ...
    const result = ach.math.sqrt(v);
    // ...
    result.dispose();
} finally {
    // This block always runs
    v.dispose();
}
```

### `ach.cleanup()`

If you have many manually created values, you can dispose of all of them at once using `ach.cleanup()`.

```typescript
const v1 = ach.vector([1, 2, 3]);
const m1 = ach.matrix([[1,2],[3,4]]);
// ... many more values

// Clean up everything associated with this `ach` instance
ach.cleanup();
console.log('Active values:', ach.getActiveValuesCount()); // 0
```

---

## Debugging Memory Leaks

The SDK provides tools to monitor memory usage.

### `getMemoryStats()`

This method gives you a snapshot of the memory state.

```typescript
const stats = ach.getMemoryStats();
console.log(stats);
// {
//   allocated: 10,  // Total handles ever created
//   freed: 5,       // Total handles ever freed
//   active: 5,      // Currently active handles
//   leaked: 0       // Should always be 0 with this memory model
// }
```

A common debugging pattern is to log the `active` count before and after an operation. If it increases and doesn't decrease back to the baseline, you have a leak.

### `getActiveValuesCount()`

A simpler method to just get the number of active values.

```typescript
function doWork() {
    ach.use(() => {
        const v = ach.vector([1,2,3]);
        // ...
    });
}

console.log(ach.getActiveValuesCount()); // 0
doWork();
console.log(ach.getActiveValuesCount()); // Should still be 0
```

---

## Best Practices

-   **Prefer `ach.use()`**: It's the safest and primary pattern.
-   **Extract Data**: If you need to store results long-term, copy them out to a standard JS array (`.toArray()`) and then let the Achronyme `Value` be disposed.
-   **Short-Lived Values**: Keep the lifetime of `Value` objects as short as possible.
-   **`try...finally`**: When using manual cleanup, always wrap it in `try...finally`.
-   **Monitor `active` count**: During development, frequently check `ach.getActiveValuesCount()` to catch leaks early.