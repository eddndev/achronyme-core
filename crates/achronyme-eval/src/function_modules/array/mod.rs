//! Array utility functions
//!
//! This module provides essential array operations that complement
//! the existing higher-order functions (map, filter, reduce).
//!
//! Tier 1 Functions:
//! - product(array) - Multiply all elements
//! - range(start, end, step?) - Generate integer sequence
//! - len(array) - Get array/vector length
//! - reverse(array) - Reverse array order
//!
//! Tier 2 Functions (Predicates & Search):
//! - any(array, predicate) - Check if any element matches
//! - all(array, predicate) - Check if all elements match
//! - find(array, predicate) - Find first matching element
//! - findIndex(array, predicate) - Find index of first match
//! - count(array, predicate) - Count matching elements
//! - contains(array, value) - Check if value exists
//!
//! Tier 3 Functions (Array Transformations):
//! - zip(array1, array2) - Combine two arrays into pairs
//! - flatten(nestedArray, depth?) - Flatten nested arrays
//! - take(array, n) - Take first n elements
//! - drop(array, n) - Skip first n elements
//! - slice(array, start, end?) - Extract subarray
//! - unique(array) - Remove duplicates
//! - chunk(array, size) - Split into chunks

pub mod core;
pub mod search;
pub mod transform;

#[cfg(test)]
mod tests;

use crate::functions::FunctionRegistry;

/// Register all array utility functions
pub fn register_functions(registry: &mut FunctionRegistry) {
    // Tier 1: Essential operations
    registry.register("product", core::product, 1);
    registry.register("range", core::range, -1); // Variadic: 2 or 3 args
    registry.register("len", core::len, 1);
    registry.register("reverse", core::reverse, 1);

    // Tier 2: Predicates and search
    // Note: any, all, find, findIndex, count are handled as HOFs in handlers/function_call.rs
    // They need evaluator access to apply lambda predicates
    registry.register("contains", search::contains, 2);

    // Tier 3: Array transformations
    registry.register("zip", transform::zip, 2);
    registry.register("flatten", transform::flatten, -1); // Variadic: 1 or 2 args
    registry.register("take", transform::take, 2);
    registry.register("drop", transform::drop, 2);
    registry.register("slice", transform::slice, -1); // Variadic: 2 or 3 args
    registry.register("unique", transform::unique, 1);
    registry.register("chunk", transform::chunk, 2);
}
