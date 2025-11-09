use achronyme_types::tensor::RealTensor;
use super::simplex;

/// Resolver un problema de programación lineal con auto-selección de método
///
/// Esta función selecciona automáticamente el mejor algoritmo basándose en
/// las características del problema:
///   - Problemas grandes (n > 1000 o m > 1000): Interior Point (futuro)
///   - Muchas variables, pocas restricciones (m < n/2): Dual Simplex (futuro)
///   - Matriz dispersa: Revised Simplex (futuro)
///   - Caso general: Primal Simplex
///
/// Por ahora, solo usa Primal Simplex.
///
/// Args:
///   - c: vector de coeficientes objetivo (n elementos)
///   - a: matriz de restricciones (m × n)
///   - b: vector de lado derecho (m elementos)
///   - sense: 1.0 para maximizar, -1.0 para minimizar
///
/// Returns:
///   - Ok(x): vector solución óptima (n elementos)
///   - Err: mensaje de error
pub fn solve(c: &[f64], a: &RealTensor, b: &[f64], sense: f64) -> Result<Vec<f64>, String> {
    let _n = c.len();
    let _m = a.rows();

    // Heurística de selección de método (por ahora, solo Simplex)
    // TODO: Implementar Dual Simplex, Revised Simplex, Interior Point

    // if n > 5000 || m > 5000 {
    //     // Problemas grandes: Interior Point
    //     return interior_point::solve(c, a, b, sense);
    // } else if m < n / 2 {
    //     // Muchas variables, pocas restricciones: Dual Simplex
    //     return dual_simplex::solve(c, a, b, sense);
    // } else if is_sparse(a) {
    //     // Matriz dispersa: Revised Simplex
    //     return revised_simplex::solve(c, a, b, sense);
    // }

    // Caso general: Primal Simplex
    simplex::solve(c, a, b, sense)
}

/// Verificar si una matriz es dispersa (sparse)
///
/// Una matriz se considera dispersa si más del 50% de sus elementos son cero
#[allow(dead_code)]
fn is_sparse(tensor: &RealTensor) -> bool {
    let total = tensor.size();
    let zeros = tensor.data().iter().filter(|&&x| x.abs() < 1e-10).count();
    zeros > total / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linprog_auto_selects_simplex() {
        // maximize z = 3x₁ + 5x₂
        // subject to:
        //   x₁ ≤ 4
        //   2x₂ ≤ 12
        //   3x₁ + 2x₂ ≤ 18

        let c = vec![3.0, 5.0];
        let a = RealTensor::matrix(3, 2, vec![1.0, 0.0, 0.0, 2.0, 3.0, 2.0]).unwrap();
        let b = vec![4.0, 12.0, 18.0];

        let solution = solve(&c, &a, &b, 1.0).unwrap();

        assert!((solution[0] - 2.0).abs() < 1e-6);
        assert!((solution[1] - 6.0).abs() < 1e-6);
    }

    #[test]
    fn test_is_sparse() {
        // Matriz dispersa: 6 ceros de 9 elementos (66%)
        let sparse = RealTensor::matrix(3, 3, vec![
            1.0, 0.0, 0.0,
            0.0, 2.0, 0.0,
            0.0, 0.0, 3.0,
        ]).unwrap();

        assert!(is_sparse(&sparse));

        // Matriz densa: 0 ceros de 4 elementos (0%)
        let dense = RealTensor::matrix(2, 2, vec![
            1.0, 2.0,
            3.0, 4.0,
        ]).unwrap();

        assert!(!is_sparse(&dense));
    }
}