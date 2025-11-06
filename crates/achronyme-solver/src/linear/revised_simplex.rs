use achronyme_types::matrix::Matrix;

/// Resolver un problema de programación lineal usando el método Revised Simplex
///
/// El Revised Simplex es una versión más eficiente del Simplex estándar que:
/// - NO mantiene el tableau completo en memoria
/// - Solo almacena la base inversa B⁻¹
/// - Recalcula columnas del tableau on-demand
///
/// **Ventajas sobre Primal Simplex**:
/// - Memoria: O(m²) vs O(m*n) donde m << n (muchas variables, pocas restricciones)
/// - Más eficiente para matrices dispersas (sparse)
/// - Mejor para problemas grandes (n > 1000)
///
/// **Desventajas**:
/// - Más complejo de implementar
/// - Overhead de recalcular columnas
/// - No es mejor si n ≈ m
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
/// # Nota sobre la Implementación
///
/// Esta es una implementación simplificada del Revised Simplex.
/// Una implementación productiva requeriría:
/// - Factorización LU de la base para eficiencia
/// - Actualización incremental de B⁻¹ (product form)
/// - Manejo de degeneración (reglas de Bland, etc.)
/// - Reoptimización periódica para estabilidad numérica
pub fn solve(c: &[f64], a: &Matrix, b: &[f64], sense: f64) -> Result<Vec<f64>, String> {
    // Validar sense
    if sense != 1.0 && sense != -1.0 {
        return Err("sense must be 1.0 (maximize) or -1.0 (minimize)".to_string());
    }

    let n = c.len(); // Variables de decisión
    let m = a.rows;  // Restricciones

    // Validaciones
    if a.cols != n {
        return Err(format!(
            "Matrix A has {} columns but c has {} elements",
            a.cols, n
        ));
    }
    if b.len() != m {
        return Err(format!(
            "Matrix A has {} rows but b has {} elements",
            m, b.len()
        ));
    }

    // Verificar que todos los b[i] sean no negativos
    for (i, &bi) in b.iter().enumerate() {
        if bi < 0.0 {
            return Err(format!(
                "RHS b[{}] = {} is negative. Use two_phase_simplex for this case.",
                i, bi
            ));
        }
    }

    // Construir matriz aumentada [A | I] para incluir variables de holgura
    let total_vars = n + m;
    let mut aug_a = vec![0.0; m * total_vars];

    // Copiar A
    for i in 0..m {
        for j in 0..n {
            aug_a[i * total_vars + j] = a.data[i * n + j];
        }
    }

    // Agregar matriz identidad (holguras)
    for i in 0..m {
        aug_a[i * total_vars + (n + i)] = 1.0;
    }

    // Vector de costos aumentado [c | 0]
    let mut c_aug = vec![0.0; total_vars];
    for j in 0..n {
        c_aug[j] = -sense * c[j]; // Negativo porque maximizamos
    }

    // Base inicial: variables de holgura [n, n+1, ..., n+m-1]
    let mut basis: Vec<usize> = (n..n + m).collect();

    // Solución básica inicial
    let mut x_b = b.to_vec();

    // Configuración
    let max_iterations = 10000;

    // Algoritmo Revised Simplex
    for iteration in 0..max_iterations {
        // Paso 1: Calcular multiplicadores simplex (precios sombra duales)
        // π = c_B^T * B⁻¹
        let b_inv = compute_basis_inverse(&aug_a, &basis, m, total_vars)?;
        let c_b = get_basis_costs(&c_aug, &basis);
        let pi = multiply_vector_matrix(&c_b, &b_inv, m);

        // Paso 2: Calcular costos reducidos para variables no básicas
        // r_j = c_j - π^T * A_j
        let mut entering_col = None;
        let mut min_reduced_cost = 0.0;

        for j in 0..total_vars {
            // Si j está en la base, su costo reducido es 0
            if basis.contains(&j) {
                continue;
            }

            // Extraer columna A_j
            let a_j = get_column(&aug_a, j, m, total_vars);

            // Calcular r_j = c_j - π^T * A_j
            let pi_a_j = dot_product(&pi, &a_j);
            let r_j = c_aug[j] - pi_a_j;

            // Para maximización, buscamos r_j < 0
            if r_j < min_reduced_cost - 1e-10 {
                min_reduced_cost = r_j;
                entering_col = Some(j);
            }
        }

        // Paso 3: Verificar optimalidad
        if entering_col.is_none() {
            // Todos los costos reducidos son >= 0, solución óptima
            return extract_solution(&basis, &x_b, n, m);
        }

        let entering = entering_col.unwrap();

        // Paso 4: Calcular dirección: d = B⁻¹ * A_entering
        let a_entering = get_column(&aug_a, entering, m, total_vars);
        let d = multiply_matrix_vector(&b_inv, &a_entering, m);

        // Paso 5: Minimum ratio test para encontrar leaving variable
        let mut min_ratio = f64::INFINITY;
        let mut leaving_idx = None;

        for i in 0..m {
            if d[i] > 1e-10 {
                let ratio = x_b[i] / d[i];
                if ratio < min_ratio {
                    min_ratio = ratio;
                    leaving_idx = Some(i);
                }
            }
        }

        if leaving_idx.is_none() {
            return Err(format!(
                "Unbounded problem at iteration {} (no valid leaving variable)",
                iteration
            ));
        }

        let leaving_row = leaving_idx.unwrap();

        // Paso 6: Actualizar base y solución
        let theta = min_ratio;

        // Actualizar x_b
        for i in 0..m {
            x_b[i] -= theta * d[i];
        }
        x_b[leaving_row] = theta;

        // Actualizar base
        basis[leaving_row] = entering;
    }

    Err(format!(
        "Maximum iterations ({}) reached",
        max_iterations
    ))
}

// ============================================================================
// Funciones Auxiliares de Álgebra Lineal
// ============================================================================

/// Calcular la inversa de la submatriz base B
///
/// Usa eliminación gaussiana con pivoteo parcial
fn compute_basis_inverse(
    aug_a: &[f64],
    basis: &[usize],
    m: usize,
    total_vars: usize,
) -> Result<Vec<f64>, String> {
    // Extraer matriz base B (m × m)
    let mut b = vec![0.0; m * m];
    for (i, &col) in basis.iter().enumerate() {
        for row in 0..m {
            b[row * m + i] = aug_a[row * total_vars + col];
        }
    }

    // Invertir usando eliminación gaussiana
    invert_matrix(&b, m)
}

/// Invertir una matriz usando eliminación gaussiana (Gauss-Jordan)
fn invert_matrix(mat: &[f64], n: usize) -> Result<Vec<f64>, String> {
    // Crear matriz aumentada [A | I]
    let mut aug = vec![0.0; n * (2 * n)];

    for i in 0..n {
        for j in 0..n {
            aug[i * (2 * n) + j] = mat[i * n + j];
        }
        aug[i * (2 * n) + (n + i)] = 1.0; // Identidad
    }

    // Eliminación hacia adelante
    for k in 0..n {
        // Encontrar pivote
        let mut max_row = k;
        let mut max_val = aug[k * (2 * n) + k].abs();

        for i in (k + 1)..n {
            let val = aug[i * (2 * n) + k].abs();
            if val > max_val {
                max_val = val;
                max_row = i;
            }
        }

        if max_val < 1e-10 {
            return Err("Matrix is singular (not invertible)".to_string());
        }

        // Intercambiar filas
        if max_row != k {
            for j in 0..(2 * n) {
                let temp = aug[k * (2 * n) + j];
                aug[k * (2 * n) + j] = aug[max_row * (2 * n) + j];
                aug[max_row * (2 * n) + j] = temp;
            }
        }

        // Normalizar fila pivote
        let pivot = aug[k * (2 * n) + k];
        for j in 0..(2 * n) {
            aug[k * (2 * n) + j] /= pivot;
        }

        // Eliminar columna k en otras filas
        for i in 0..n {
            if i != k {
                let factor = aug[i * (2 * n) + k];
                for j in 0..(2 * n) {
                    aug[i * (2 * n) + j] -= factor * aug[k * (2 * n) + j];
                }
            }
        }
    }

    // Extraer matriz inversa (mitad derecha)
    let mut inv = vec![0.0; n * n];
    for i in 0..n {
        for j in 0..n {
            inv[i * n + j] = aug[i * (2 * n) + (n + j)];
        }
    }

    Ok(inv)
}

/// Obtener costos de variables en la base
fn get_basis_costs(c: &[f64], basis: &[usize]) -> Vec<f64> {
    basis.iter().map(|&j| c[j]).collect()
}

/// Multiplicar vector por matriz: v^T * M
fn multiply_vector_matrix(v: &[f64], m: &[f64], n: usize) -> Vec<f64> {
    let mut result = vec![0.0; n];
    for j in 0..n {
        for i in 0..n {
            result[j] += v[i] * m[i * n + j];
        }
    }
    result
}

/// Multiplicar matriz por vector: M * v
fn multiply_matrix_vector(m: &[f64], v: &[f64], n: usize) -> Vec<f64> {
    let mut result = vec![0.0; n];
    for i in 0..n {
        for j in 0..n {
            result[i] += m[i * n + j] * v[j];
        }
    }
    result
}

/// Producto punto de dos vectores
fn dot_product(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

/// Obtener una columna de la matriz
fn get_column(mat: &[f64], col: usize, rows: usize, cols: usize) -> Vec<f64> {
    (0..rows).map(|i| mat[i * cols + col]).collect()
}

/// Extraer solución final
fn extract_solution(basis: &[usize], x_b: &[f64], n: usize, _m: usize) -> Result<Vec<f64>, String> {
    let mut solution = vec![0.0; n];

    for (i, &var) in basis.iter().enumerate() {
        if var < n {
            // Variable de decisión (no holgura)
            solution[var] = x_b[i];
        }
    }

    Ok(solution)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_revised_simplex_basic() {
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

    #[test]
    fn test_matrix_inversion() {
        // Test 2×2
        let mat = vec![4.0, 7.0, 2.0, 6.0];
        let inv = invert_matrix(&mat, 2).unwrap();

        // Verificar A * A⁻¹ = I
        let result = vec![
            mat[0] * inv[0] + mat[1] * inv[2],
            mat[0] * inv[1] + mat[1] * inv[3],
            mat[2] * inv[0] + mat[3] * inv[2],
            mat[2] * inv[1] + mat[3] * inv[3],
        ];

        assert!((result[0] - 1.0).abs() < 1e-10);
        assert!(result[1].abs() < 1e-10);
        assert!(result[2].abs() < 1e-10);
        assert!((result[3] - 1.0).abs() < 1e-10);
    }
}
