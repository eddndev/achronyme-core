# Performance Guide - Achronyme SDK v2.0

The Achronyme SDK is designed for high performance by minimizing the overhead between the JavaScript environment and the high-speed WebAssembly (WASM) core. This guide covers the key patterns for writing fast and efficient code.

## 1. The Golden Rule: Minimize JS-WASM Communication

Every call from JavaScript to WASM (and back) has a small but non-zero overhead. High-performance code minimizes these round-trips.

-   **Bad (Chatty)**: Many small operations, each crossing the JS-WASM boundary.
-   **Good (Efficient)**: Fewer, larger operations that do more work within WASM.

---

## 2. Zero-Copy Data Access with `.data`

This is the **single most important performance feature** of the SDK.

When you access the `.data` property of a `Vector` or `Matrix`, you get a `Float64Array` that is a **direct view** into the WASM memory. Access is instantaneous because no data is copied.

In contrast, `.toArray()` creates a **full copy** of the data, which is slow and memory-intensive for large datasets.

### Example:

```typescript
await ach.use(async () => {
    const v = ach.vector([...Array(10_000_000).keys()]); // 10 million elements

    // ⚡ INSTANT (Zero-Copy)
    console.time('data view');
    const view = v.data;
    console.timeEnd('data view'); // ~ <1ms

    // 🐢 SLOW (Full Copy)
    console.time('toArray copy');
    const copy = v.toArray();
    console.timeEnd('toArray copy'); // ~ 80ms+
});
```

**Best Practice:**
-   Use `.data` for reading, iterating, or passing data to other libraries (like plotting tools).
-   Use `.toArray()` only when you absolutely need a separate, mutable JavaScript copy of the data.

---

## 3. Prefer WASM-Native Operations

The SDK provides two ways to perform operations like `map`:

1.  **JS Callback**: `vector.map(x => Math.sin(x))`
2.  **WASM-Native Function**: `ach.math.sin(vector)`

The WASM-native function is significantly faster because the entire loop runs inside WASM, avoiding the need to call a JavaScript function for every single element.

### Example:

```typescript
await ach.use(async () => {
    const v = ach.linspace(0, 10, 1_000_000); // 1 million elements

    // 🐢 SLOW: Calls a JS function 1,000,000 times
    console.time('map with JS callback');
    const result1 = v.map(x => Math.sin(x));
    console.timeEnd('map with JS callback');

    // ⚡ FAST: Entire operation runs inside WASM
    console.time('WASM-native sin');
    const result2 = ach.math.sin(v);
    console.timeEnd('WASM-native sin');
});
```

**Best Practice:** Always use the built-in `ach.math`, `ach.dsp`, etc. functions instead of `map` with a simple JS callback. Use `map` for custom logic that doesn't have a built-in equivalent.

---

## 4. Batch Operations with the `eval()` Engine

For complex sequences of calculations, the `eval()` engine is your most powerful tool. It can parse and execute an entire expression in a single call to WASM, completely eliminating the overhead of intermediate operations.

### Example: `map(x => sqrt(abs(sin(x*2))), v)`

**The Slow Way (step-by-step):**
```typescript
// 4 separate operations, 4 JS-WASM round-trips
const v = ach.linspace(0, 10, 1000);
const v2 = ach.vecOps.vscale(v, 2);
const v3 = ach.math.sin(v2);
const v4 = ach.math.abs(v3);
const result = ach.math.sqrt(v4);
```

**The Fast Way (`eval`):**
```typescript
// 1 operation, 1 JS-WASM round-trip
ach.eval("let v = linspace(0, 10, 1000)");
const result = ach.eval("map(x => sqrt(abs(sin(x*2))), v)");
```

**Best Practice:** For multi-step formulas or data processing pipelines, compose them into a single `eval` string to achieve maximum performance.

---

## 5. Use Built-in Utility Functions

Functions like `ach.linspace`, `ach.identity`, `ach.zeros`, and `ach.ones` are implemented in Rust/WASM. They are much faster at creating large datasets than generating them in JavaScript and then passing them to `ach.vector`.

-   **Bad (Slow)**: `ach.vector(new Array(1_000_000).fill(0))`
-   **Good (Fast)**: `ach.zeros(1_000_000)`

## Summary of Performance Patterns

| Priority | Pattern | Why it's Fast |
| :--- | :--- | :--- |
| **High** | **Use `.data` for reads** | Avoids slow, memory-heavy data copies. |
| **High** | **Use `eval()` for pipelines** | Executes many steps in one WASM call, minimizing overhead. |
| **Medium** | **Prefer `ach.math.*` over `map()`** | Keeps loops inside the fast WASM environment. |
| **Medium** | **Use `ach.linspace`, `ach.zeros`** | Creates data directly in WASM memory. |
| **Low** | **Use `ach.use()`** | While primarily for safety, it's also efficient at batch-releasing memory. |