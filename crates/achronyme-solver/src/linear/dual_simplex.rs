use achronyme_types::matrix::Matrix;
use super::tableau::Tableau;

/// Resolver un problema de programación lineal usando el método Dual Simplex
///
/// El Dual Simplex es útil cuando:
/// - Tienes una solución dual factible pero no primal factible
/// - Agregaste nuevas restricciones a un problema ya resuelto
/// - Problema tiene muchas variables pero pocas restricciones
///
/// Diferencia con Primal Simplex:
/// - Primal: Parte de solución primal factible, busca optimalidad
/// - Dual: Parte de solución óptima (dual factible), busca factibilidad primal
///
/// Args:
///   - c: vector de coeficientes objetivo (n elementos)
///   - a: matriz de restricciones (m × n)
///   - b: vector de lado derecho (m elementos)
///   - sense: 1.0 para maximizar, -1.0 para minimizar
///
/// Returns:
///   - Ok(x): vector solución óptima (n elementos)
///   - Err: mensaje de error (infactible, etc.)
pub fn solve(c: &[f64], a: &Matrix, b: &[f64], sense: f64) -> Result<Vec<f64>, String> {
    // Validar sense
    if sense != 1.0 && sense != -1.0 {
        return Err("sense must be 1.0 (maximize) or -1.0 (minimize)".to_string());
    }

    // Para el método dual, primero transformamos el problema
    // Si estamos maximizando z = c^T x sujeto a Ax <= b
    // El dual es: minimizar w = b^T y sujeto a A^T y >= c

    // Por ahora, implementamos el dual simplex estándar
    // que opera sobre el mismo tableau que el primal
    // pero con reglas diferentes de selección

    // Crear tableau inicial
    let mut tableau = Tableau::new(c, a, b, sense)?;

    // Configuración
    let max_iterations = 10000;
    let mut iteration = 0;

    // Algoritmo Dual Simplex: iterar hasta encontrar solución primal factible y óptima
    loop {
        iteration += 1;

        if iteration > max_iterations {
            return Err(format!(
                "Maximum iterations ({}) reached in dual simplex.",
                max_iterations
            ));
        }

        // Paso 1: Verificar si la solución es primal factible Y óptima
        if tableau.is_optimal() && is_primal_feasible(&tableau) {
            return Ok(tableau.extract_solution());
        }

        // Paso 2: Encontrar fila con RHS negativo (variable básica infactible)
        let leaving = match find_leaving_variable_dual(&tableau) {
            Some(row) => row,
            None => {
                // No hay filas con RHS negativo
                if tableau.is_optimal() {
                    // Ya es óptimo y factible
                    return Ok(tableau.extract_solution());
                } else {
                    // Factible pero no óptimo, cambiar a primal simplex
                    return Err("Dual simplex found feasible solution but not optimal. Use primal simplex.".to_string());
                }
            }
        };

        // Paso 3: Encontrar columna entrante (dual ratio test)
        let entering = match find_entering_variable_dual(&tableau, leaving) {
            Ok(col) => col,
            Err(e) => {
                return Err(format!("Dual infeasible at iteration {}: {}", iteration, e));
            }
        };

        // Paso 4: Realizar pivoteo
        tableau.pivot(entering, leaving);
    }
}

/// Verificar si la solución actual es primal factible
/// (todas las variables básicas tienen valor no negativo)
fn is_primal_feasible(tableau: &Tableau) -> bool {
    let m = tableau.num_constraints;
    let n = tableau.num_vars;
    let rhs_col = n + m;

    for i in 0..m {
        if tableau.data[i][rhs_col] < -1e-10 {
            return false;
        }
    }

    true
}

/// Encontrar la fila que sale (leaving row) para Dual Simplex
/// Regla: fila con el RHS más negativo
fn find_leaving_variable_dual(tableau: &Tableau) -> Option<usize> {
    let m = tableau.num_constraints;
    let n = tableau.num_vars;
    let rhs_col = n + m;

    let mut min_rhs = 0.0;
    let mut leaving_row = None;

    for i in 0..m {
        let rhs = tableau.data[i][rhs_col];
        if rhs < min_rhs - 1e-10 {
            min_rhs = rhs;
            leaving_row = Some(i);
        }
    }

    leaving_row
}

/// Encontrar la columna entrante (entering column) para Dual Simplex
/// Dual ratio test: minimizar |c_j / a_ij| donde a_ij < 0
fn find_entering_variable_dual(tableau: &Tableau, leaving_row: usize) -> Result<usize, String> {
    let m = tableau.num_constraints;
    let n = tableau.num_vars;
    let total_cols = n + m;
    let obj_row = m;

    let mut min_ratio = f64::INFINITY;
    let mut entering_col = None;

    for j in 0..total_cols {
        let a_ij = tableau.data[leaving_row][j];

        // Solo considerar coeficientes negativos
        if a_ij < -1e-10 {
            let c_j = tableau.data[obj_row][j];
            let ratio = c_j.abs() / a_ij.abs();

            if ratio < min_ratio {
                min_ratio = ratio;
                entering_col = Some(j);
            }
        }
    }

    entering_col.ok_or_else(|| {
        "Dual infeasible: no valid entering variable (all coefficients in leaving row are non-negative)".to_string()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dual_simplex_basic() {
        // Problema simple que el dual simplex puede resolver
        // maximize z = 3x₁ + 5x₂
        // subject to:
        //   x₁ ≤ 4
        //   2x₂ ≤ 12
        //   3x₁ + 2x₂ ≤ 18

        let c = vec![3.0, 5.0];
        let a = Matrix::new(3, 2, vec![1.0, 0.0, 0.0, 2.0, 3.0, 2.0]).unwrap();
        let b = vec![4.0, 12.0, 18.0];

        let solution = solve(&c, &a, &b, 1.0).unwrap();

        // Verificar solución
        assert!((solution[0] - 2.0).abs() < 1e-6, "x₁ should be 2.0");
        assert!((solution[1] - 6.0).abs() < 1e-6, "x₂ should be 6.0");
    }
}
