use achronyme_types::matrix::Matrix;
use super::tableau::Tableau;

/// Resolver un problema de programación lineal usando el método Simplex
///
/// Forma estándar:
///   maximize/minimize z = c^T * x
///   subject to:
///     A * x ≤ b
///     x ≥ 0
///
/// Args:
///   - c: vector de coeficientes objetivo (n elementos)
///   - a: matriz de restricciones (m × n)
///   - b: vector de lado derecho (m elementos)
///   - sense: 1.0 para maximizar, -1.0 para minimizar
///
/// Returns:
///   - Ok(x): vector solución óptima (n elementos)
///   - Err: mensaje de error (infactible, no acotado, etc.)
///
/// # Ejemplo
///
/// ```
/// use achronyme_types::matrix::Matrix;
/// use achronyme_solver::linear::simplex::solve;
///
/// // maximize z = 3x₁ + 5x₂
/// // subject to:
/// //   x₁ ≤ 4
/// //   2x₂ ≤ 12
/// //   3x₁ + 2x₂ ≤ 18
/// //   x₁, x₂ ≥ 0
///
/// let c = vec![3.0, 5.0];
/// let a = Matrix::new(3, 2, vec![
///     1.0, 0.0,
///     0.0, 2.0,
///     3.0, 2.0,
/// ]).unwrap();
/// let b = vec![4.0, 12.0, 18.0];
///
/// let solution = solve(&c, &a, &b, 1.0).unwrap();
/// // solution ≈ [2.0, 6.0]
/// // z* = 3*2 + 5*6 = 36
/// ```
pub fn solve(c: &[f64], a: &Matrix, b: &[f64], sense: f64) -> Result<Vec<f64>, String> {
    // Validar sense
    if sense != 1.0 && sense != -1.0 {
        return Err("sense must be 1.0 (maximize) or -1.0 (minimize)".to_string());
    }

    // Crear tableau inicial
    let mut tableau = Tableau::new(c, a, b, sense)?;

    // Configuración
    let max_iterations = 10000;
    let mut iteration = 0;

    // Algoritmo Simplex: iterar hasta encontrar solución óptima
    loop {
        iteration += 1;

        if iteration > max_iterations {
            return Err(format!(
                "Maximum iterations ({}) reached. Problem may be degenerate or cycling.",
                max_iterations
            ));
        }

        // Paso 1: Verificar optimalidad
        if tableau.is_optimal() {
            // Solución óptima encontrada
            return Ok(tableau.extract_solution());
        }

        // Paso 2: Encontrar variable entrante (entering variable)
        let entering = match tableau.find_entering_variable() {
            Some(col) => col,
            None => {
                // No hay columnas con coeficiente negativo → óptimo
                return Ok(tableau.extract_solution());
            }
        };

        // Paso 3: Encontrar variable saliente (leaving variable)
        let leaving = match tableau.find_leaving_variable(entering) {
            Ok(row) => row,
            Err(e) => {
                // Problema no acotado (unbounded)
                return Err(format!("Unbounded problem at iteration {}: {}", iteration, e));
            }
        };

        // Paso 4: Realizar pivoteo
        tableau.pivot(entering, leaving);
    }
}

/// Calcular el valor objetivo para una solución dada
///
/// z = c^T * x
///
/// Args:
///   - c: vector de coeficientes objetivo
///   - x: vector solución
///
/// Returns:
///   - z: valor objetivo
pub fn objective_value(c: &[f64], x: &[f64]) -> Result<f64, String> {
    if c.len() != x.len() {
        return Err(format!(
            "Dimension mismatch: c has {} elements but x has {}",
            c.len(),
            x.len()
        ));
    }

    let mut z = 0.0;
    for i in 0..c.len() {
        z += c[i] * x[i];
    }

    Ok(z)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_lp_maximize() {
        // maximize z = 3x₁ + 5x₂
        // subject to:
        //   x₁ ≤ 4
        //   2x₂ ≤ 12
        //   3x₁ + 2x₂ ≤ 18
        //   x₁, x₂ ≥ 0
        //
        // Solución óptima: x₁ = 2, x₂ = 6, z = 36

        let c = vec![3.0, 5.0];
        let a = Matrix::new(3, 2, vec![1.0, 0.0, 0.0, 2.0, 3.0, 2.0]).unwrap();
        let b = vec![4.0, 12.0, 18.0];

        let solution = solve(&c, &a, &b, 1.0).unwrap();

        // Verificar solución
        assert!((solution[0] - 2.0).abs() < 1e-6, "x₁ should be 2.0");
        assert!((solution[1] - 6.0).abs() < 1e-6, "x₂ should be 6.0");

        // Verificar valor objetivo
        let z = objective_value(&c, &solution).unwrap();
        assert!((z - 36.0).abs() < 1e-6, "z should be 36.0");
    }

    #[test]
    fn test_simple_lp_minimize() {
        // minimize z = 2x₁ + 3x₂
        // subject to:
        //   x₁ + x₂ ≥ 4  → -x₁ - x₂ ≤ -4 (NO, necesitamos b ≥ 0)
        //   2x₁ + x₂ ≥ 5  → -2x₁ - x₂ ≤ -5 (NO)
        //
        // Para este test, usaremos restricciones ≤ con b ≥ 0
        // minimize z = x₁ + x₂
        // subject to:
        //   -x₁ ≤ 0  (x₁ ≥ 0, redundante)
        //   -x₂ ≤ 0  (x₂ ≥ 0, redundante)
        //   x₁ + x₂ ≤ 10
        //
        // Solución óptima: x₁ = 0, x₂ = 0, z = 0

        let c = vec![1.0, 1.0];
        let a = Matrix::new(1, 2, vec![1.0, 1.0]).unwrap();
        let b = vec![10.0];

        let solution = solve(&c, &a, &b, -1.0).unwrap();

        // Solución trivial: (0, 0)
        assert!(solution[0].abs() < 1e-6, "x₁ should be 0.0");
        assert!(solution[1].abs() < 1e-6, "x₂ should be 0.0");

        let z = objective_value(&c, &solution).unwrap();
        assert!(z.abs() < 1e-6, "z should be 0.0");
    }

    #[test]
    fn test_production_problem() {
        // Problema de producción (ejemplo clásico)
        // maximize z = 40x₁ + 30x₂
        // subject to:
        //   x₁ ≤ 40        (materia prima A)
        //   x₂ ≤ 50        (materia prima B)
        //   x₁ + x₂ ≤ 70   (horas de trabajo)
        //   x₁, x₂ ≥ 0
        //
        // Solución óptima: x₁ = 40, x₂ = 30, z = 2500

        let c = vec![40.0, 30.0];
        let a = Matrix::new(3, 2, vec![1.0, 0.0, 0.0, 1.0, 1.0, 1.0]).unwrap();
        let b = vec![40.0, 50.0, 70.0];

        let solution = solve(&c, &a, &b, 1.0).unwrap();

        assert!((solution[0] - 40.0).abs() < 1e-6, "x₁ should be 40.0");
        assert!((solution[1] - 30.0).abs() < 1e-6, "x₂ should be 30.0");

        let z = objective_value(&c, &solution).unwrap();
        assert!((z - 2500.0).abs() < 1e-6, "z should be 2500.0");
    }

    #[test]
    fn test_unbounded_problem() {
        // maximize z = x₁ + x₂
        // subject to:
        //   -x₁ - x₂ ≤ -1  (necesitaríamos b ≥ 0, entonces esto no funciona)
        //
        // Usemos un ejemplo más simple:
        // maximize z = x₁
        // subject to:
        //   -x₁ ≤ 0  (x₁ ≥ 0, pero sin cota superior)
        //
        // Este problema es no acotado (unbounded)

        let c = vec![1.0];
        let a = Matrix::new(1, 1, vec![-1.0]).unwrap();
        let b = vec![0.0];

        let result = solve(&c, &a, &b, 1.0);

        // Debería fallar con "Unbounded"
        assert!(result.is_err());
        if let Err(msg) = result {
            assert!(msg.contains("Unbounded"), "Error should mention unbounded: {}", msg);
        }
    }

    #[test]
    fn test_negative_rhs() {
        // Test que verifica que rechazamos b[i] < 0
        let c = vec![1.0, 1.0];
        let a = Matrix::new(2, 2, vec![1.0, 0.0, 0.0, 1.0]).unwrap();
        let b = vec![5.0, -3.0]; // b[1] es negativo

        let result = solve(&c, &a, &b, 1.0);

        assert!(result.is_err());
        if let Err(msg) = result {
            assert!(msg.contains("negative"), "Error should mention negative RHS: {}", msg);
        }
    }

    #[test]
    fn test_objective_value_function() {
        let c = vec![3.0, 5.0];
        let x = vec![2.0, 6.0];

        let z = objective_value(&c, &x).unwrap();
        assert!((z - 36.0).abs() < 1e-10);
    }

    #[test]
    fn test_objective_value_dimension_mismatch() {
        let c = vec![1.0, 2.0];
        let x = vec![1.0, 2.0, 3.0]; // Dimensión incorrecta

        let result = objective_value(&c, &x);
        assert!(result.is_err());
    }
}
