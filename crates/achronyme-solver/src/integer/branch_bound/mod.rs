//! Branch & Bound algorithm for Integer Linear Programming
//!
//! This module provides solvers for:
//! - Integer Linear Programming (ILP)
//! - Binary Linear Programming (0-1 ILP)
//!
//! The implementation uses the Branch & Bound algorithm with LP relaxation.

mod node;
mod helpers;
mod bounded_lp;
mod solvers;

// Re-export public API
pub use solvers::{intlinprog, binary_linprog};

#[cfg(test)]
mod tests {
    use super::*;
    use achronyme_types::tensor::RealTensor;
    use crate::linear::simplex;

    #[test]
    fn test_intlinprog_simple() {
        // maximize z = 3x₁ + 2x₂
        // subject to: x₁ + x₂ ≤ 4
        // x₁, x₂ ∈ ℤ₊

        let c = vec![3.0, 2.0];
        let a = RealTensor::matrix(1, 2, vec![1.0, 1.0]).unwrap();
        let b = vec![4.0];
        let integer_vars = vec![0, 1];

        let solution = intlinprog(&c, &a, &b, 1.0, &integer_vars).unwrap();

        // Optimal integer solution: x = [4, 0] with z = 12
        assert!((solution[0].round() - solution[0]).abs() < 1e-6, "x[0] should be integer");
        assert!((solution[1].round() - solution[1]).abs() < 1e-6, "x[1] should be integer");

        let z = simplex::objective_value(&c, &solution).unwrap();
        assert!((z - 12.0).abs() < 1e-3, "Objective should be 12");
    }

    #[test]
    fn test_binary_linprog_knapsack() {
        // Knapsack: maximize z = 60x₁ + 100x₂ + 120x₃
        // subject to: 10x₁ + 20x₂ + 30x₃ ≤ 50
        // xᵢ ∈ {0, 1}

        let c = vec![60.0, 100.0, 120.0];
        let a = RealTensor::matrix(1, 3, vec![10.0, 20.0, 30.0]).unwrap();
        let b = vec![50.0];
        let binary_vars = vec![0, 1, 2];

        let solution = binary_linprog(&c, &a, &b, 1.0, &binary_vars).unwrap();

        // Check binary
        for &idx in &binary_vars {
            let val = solution[idx];
            assert!((val - val.round()).abs() < 1e-6, "Variable {} = {} is not binary", idx, val);
        }

        // Optimal: take items 2 and 3 (x = [0, 1, 1]), z = 220
        let z = simplex::objective_value(&c, &solution).unwrap();
        assert!((z - 220.0).abs() < 1e-3, "Objective should be 220, got z = {}", z);

        // Verify optimal solution
        assert!((solution[0]).abs() < 1e-6, "x₁ should be 0");
        assert!((solution[1] - 1.0).abs() < 1e-6, "x₂ should be 1");
        assert!((solution[2] - 1.0).abs() < 1e-6, "x₃ should be 1");
    }

    #[test]
    fn test_knapsack_small_capacity() {
        // Small capacity knapsack
        // Items: (value, weight) = [(4,2), (3,1), (2,1)]
        // Capacity: 2
        // Optimal: take items 2 and 3 → x = [0, 1, 1], z = 5

        let c = vec![4.0, 3.0, 2.0];
        let a = RealTensor::matrix(1, 3, vec![2.0, 1.0, 1.0]).unwrap();
        let b = vec![2.0];
        let binary_vars = vec![0, 1, 2];

        let solution = binary_linprog(&c, &a, &b, 1.0, &binary_vars).unwrap();

        let z = simplex::objective_value(&c, &solution).unwrap();
        assert!((z - 5.0).abs() < 1e-3, "Expected z = 5, got z = {}", z);

        // Verify solution
        assert!((solution[0]).abs() < 1e-6, "x₁ should be 0");
        assert!((solution[1] - 1.0).abs() < 1e-6, "x₂ should be 1");
        assert!((solution[2] - 1.0).abs() < 1e-6, "x₃ should be 1");
    }

    #[test]
    fn test_knapsack_large_instance() {
        // Classic 5-item knapsack
        // Items: values = [10, 20, 30, 40, 50], weights = [5, 10, 15, 20, 25]
        // Capacity: 50
        // Optimal: take items 2, 3, 4 → x = [0, 1, 1, 1, 0], z = 90

        let c = vec![10.0, 20.0, 30.0, 40.0, 50.0];
        let a = RealTensor::matrix(1, 5, vec![5.0, 10.0, 15.0, 20.0, 25.0]).unwrap();
        let b = vec![50.0];
        let binary_vars = vec![0, 1, 2, 3, 4];

        let solution = binary_linprog(&c, &a, &b, 1.0, &binary_vars).unwrap();

        // Check all variables are binary
        for &idx in &binary_vars {
            let val = solution[idx];
            assert!((val - val.round()).abs() < 1e-6, "Variable {} should be binary", idx);
        }

        let z = simplex::objective_value(&c, &solution).unwrap();
        // Optimal: x = [1, 1, 1, 1, 0] → weight = 5+10+15+20 = 50, value = 10+20+30+40 = 100
        assert!((z - 100.0).abs() < 1e-3, "Expected z = 100, got z = {}", z);

        // Verify constraint satisfaction
        let weight: f64 = solution.iter()
            .zip(vec![5.0, 10.0, 15.0, 20.0, 25.0].iter())
            .map(|(x, w)| x * w)
            .sum();
        assert!(weight <= 50.0 + 1e-6, "Weight constraint violated: {} > 50", weight);
    }

    #[test]
    fn test_knapsack_all_items_fit() {
        // All items fit in knapsack
        // Items: values = [5, 10, 15], weights = [1, 2, 3]
        // Capacity: 10 (all items total = 6)
        // Optimal: take all items → x = [1, 1, 1], z = 30

        let c = vec![5.0, 10.0, 15.0];
        let a = RealTensor::matrix(1, 3, vec![1.0, 2.0, 3.0]).unwrap();
        let b = vec![10.0];
        let binary_vars = vec![0, 1, 2];

        let solution = binary_linprog(&c, &a, &b, 1.0, &binary_vars).unwrap();

        let z = simplex::objective_value(&c, &solution).unwrap();
        assert!((z - 30.0).abs() < 1e-3, "Expected z = 30, got z = {}", z);

        // All items should be selected
        assert!((solution[0] - 1.0).abs() < 1e-6, "x₁ should be 1");
        assert!((solution[1] - 1.0).abs() < 1e-6, "x₂ should be 1");
        assert!((solution[2] - 1.0).abs() < 1e-6, "x₃ should be 1");
    }

    #[test]
    fn test_knapsack_one_item_only() {
        // Only one item fits
        // Items: values = [100, 200, 300], weights = [50, 50, 50]
        // Capacity: 50
        // Optimal: take item 3 → x = [0, 0, 1], z = 300

        let c = vec![100.0, 200.0, 300.0];
        let a = RealTensor::matrix(1, 3, vec![50.0, 50.0, 50.0]).unwrap();
        let b = vec![50.0];
        let binary_vars = vec![0, 1, 2];

        let solution = binary_linprog(&c, &a, &b, 1.0, &binary_vars).unwrap();

        let z = simplex::objective_value(&c, &solution).unwrap();
        assert!((z - 300.0).abs() < 1e-3, "Expected z = 300, got z = {}", z);

        // Only best item should be selected
        assert!((solution[2] - 1.0).abs() < 1e-6, "x₃ should be 1 (best item)");

        // Total selected should be 1
        let total_selected: f64 = solution.iter().sum();
        assert!((total_selected - 1.0).abs() < 1e-6, "Only one item should be selected");
    }

    #[test]
    fn test_knapsack_tight_capacity() {
        // Tight capacity constraint
        // Items: values = [16, 19, 23, 28], weights = [2, 3, 4, 5]
        // Capacity: 7
        // Optimal: items 1, 2 (weight = 2+3 = 5) or items 2, 3 (weight = 3+4 = 7)
        // Best is items 2, 3 → x = [0, 1, 1, 0], z = 42

        let c = vec![16.0, 19.0, 23.0, 28.0];
        let a = RealTensor::matrix(1, 4, vec![2.0, 3.0, 4.0, 5.0]).unwrap();
        let b = vec![7.0];
        let binary_vars = vec![0, 1, 2, 3];

        let solution = binary_linprog(&c, &a, &b, 1.0, &binary_vars).unwrap();

        let z = simplex::objective_value(&c, &solution).unwrap();
        assert!(z >= 35.0, "Objective should be at least 35, got z = {}", z);

        // Check constraint satisfaction
        let weight: f64 = solution.iter()
            .zip(vec![2.0, 3.0, 4.0, 5.0].iter())
            .map(|(x, w)| x * w)
            .sum();
        assert!(weight <= 7.0 + 1e-6, "Weight exceeds capacity: {} > 7", weight);
    }

    #[test]
    fn test_intlinprog_multiple_constraints() {
        // Integer LP with multiple constraints
        // maximize z = 3x₁ + 4x₂
        // subject to: 2x₁ + 3x₂ ≤ 12, x₁ + x₂ ≤ 5
        // x₁, x₂ ∈ ℤ₊

        let c = vec![3.0, 4.0];
        let a = RealTensor::matrix(2, 2, vec![2.0, 3.0, 1.0, 1.0]).unwrap();
        let b = vec![12.0, 5.0];
        let integer_vars = vec![0, 1];

        let solution = intlinprog(&c, &a, &b, 1.0, &integer_vars).unwrap();

        // Check integer
        for &idx in &integer_vars {
            let val = solution[idx];
            assert!((val - val.round()).abs() < 1e-6, "Variable {} should be integer", idx);
        }

        let z = simplex::objective_value(&c, &solution).unwrap();
        // Optimal: x = [3, 2] with z = 17 (constraints: 2*3+3*2=12≤12, 3+2=5≤5)
        assert!((z - 17.0).abs() < 1e-3, "Expected z = 17, got z = {}", z);
    }
}
