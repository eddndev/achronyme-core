use achronyme_types::matrix::Matrix;

/// Simplex Tableau
///
/// Representa el tableau del método Simplex en forma estándar:
///
/// ```text
/// [ A | I | b ]  Variables de decisión | Variables de holgura | RHS
/// [---+---+---]
/// [-c | 0 | 0 ]  Fila objetivo (z)
/// ```
///
/// Ejemplo para:
///   maximize z = 3x₁ + 5x₂
///   subject to:
///     x₁ ≤ 4
///     2x₂ ≤ 12
///     3x₁ + 2x₂ ≤ 18
///     x₁, x₂ ≥ 0
///
/// Tableau inicial:
/// ```text
///     x₁  x₂  s₁  s₂  s₃ | RHS
/// s₁ [ 1   0   1   0   0 |  4  ]
/// s₂ [ 0   2   0   1   0 | 12  ]
/// s₃ [ 3   2   0   0   1 | 18  ]
///    [--------------------+-----]
/// z  [-3  -5   0   0   0 |  0  ]
/// ```
#[derive(Debug, Clone)]
pub struct Tableau {
    /// Datos del tableau (m+1 filas × n+m+1 columnas)
    /// Últimas fila = fila objetivo
    /// Última columna = RHS (lado derecho)
    pub data: Vec<Vec<f64>>,

    /// Número de variables de decisión (originales)
    pub num_vars: usize,

    /// Número de restricciones (= número de variables de holgura)
    pub num_constraints: usize,

    /// Variables básicas actuales (índices de columnas)
    /// basis[i] = índice de la variable básica en la fila i
    pub basis: Vec<usize>,
}

impl Tableau {
    /// Crear tableau inicial desde el problema LP
    ///
    /// Args:
    ///   - c: coeficientes objetivo (n elementos)
    ///   - a: matriz de restricciones (m × n)
    ///   - b: lado derecho (m elementos)
    ///   - sense: 1.0 para maximizar, -1.0 para minimizar
    ///
    /// Returns:
    ///   Tableau inicial en forma estándar
    pub fn new(c: &[f64], a: &Matrix, b: &[f64], sense: f64) -> Result<Self, String> {
        let n = c.len(); // Número de variables
        let m = a.rows; // Número de restricciones

        // Validaciones
        if a.cols != n {
            return Err(format!(
                "Matrix A has {} columns but c has {} elements",
                a.cols,
                n
            ));
        }
        if b.len() != m {
            return Err(format!(
                "Matrix A has {} rows but b has {} elements",
                m,
                b.len()
            ));
        }

        // Verificar que todos los b[i] sean no negativos
        for (i, &bi) in b.iter().enumerate() {
            if bi < 0.0 {
                return Err(format!(
                    "RHS b[{}] = {} is negative. All constraints must have non-negative RHS.",
                    i, bi
                ));
            }
        }

        // Construir tableau: (m+1) filas × (n+m+1) columnas
        // Filas 0..m-1: restricciones con variables de holgura
        // Fila m: fila objetivo
        let mut data = vec![vec![0.0; n + m + 1]; m + 1];

        // Llenar las restricciones: [A | I | b]
        for i in 0..m {
            // Copiar fila de A
            for j in 0..n {
                data[i][j] = a.data[i * n + j];
            }

            // Agregar variable de holgura (matriz identidad)
            data[i][n + i] = 1.0;

            // Copiar RHS
            data[i][n + m] = b[i];
        }

        // Llenar fila objetivo: [-c | 0 | 0]
        for j in 0..n {
            // Si maximizamos, usamos -c (porque queremos maximizar z = c^T x)
            // Si minimizamos, usamos c (porque queremos minimizar z = c^T x)
            data[m][j] = -sense * c[j];
        }
        // Variables de holgura tienen coeficiente 0 en la función objetivo
        for j in n..n + m {
            data[m][j] = 0.0;
        }
        // RHS inicial de z es 0
        data[m][n + m] = 0.0;

        // Variables básicas iniciales: variables de holgura (columnas n..n+m-1)
        let basis: Vec<usize> = (n..n + m).collect();

        Ok(Tableau {
            data,
            num_vars: n,
            num_constraints: m,
            basis,
        })
    }

    /// Verificar si la solución actual es óptima
    ///
    /// Para maximización: todos los coeficientes de la fila objetivo son ≥ 0
    /// Para minimización: todos los coeficientes de la fila objetivo son ≤ 0
    pub fn is_optimal(&self) -> bool {
        let m = self.num_constraints;
        let n = self.num_vars;
        let total_cols = n + m;

        // Revisar todos los coeficientes de la fila objetivo (excepto RHS)
        for j in 0..total_cols {
            if self.data[m][j] < -1e-10 {
                // Hay un coeficiente negativo → no es óptimo
                return false;
            }
        }

        true
    }

    /// Encontrar la variable que entra a la base (entering variable)
    ///
    /// Regla: columna con el coeficiente más negativo en la fila objetivo
    ///
    /// Returns:
    ///   - Some(j): índice de la columna entrante
    ///   - None: si no hay columnas con coeficiente negativo (problema óptimo)
    pub fn find_entering_variable(&self) -> Option<usize> {
        let m = self.num_constraints;
        let n = self.num_vars;
        let total_cols = n + m;

        let mut min_coeff = 0.0;
        let mut entering_col = None;

        for j in 0..total_cols {
            let coeff = self.data[m][j];
            if coeff < min_coeff - 1e-10 {
                min_coeff = coeff;
                entering_col = Some(j);
            }
        }

        entering_col
    }

    /// Encontrar la variable que sale de la base (leaving variable)
    ///
    /// Regla del cociente mínimo (minimum ratio test):
    ///   - Para cada fila i: ratio[i] = RHS[i] / coeff[i][entering]
    ///   - Elegir fila con ratio mínimo positivo
    ///
    /// Args:
    ///   - entering: índice de la columna entrante
    ///
    /// Returns:
    ///   - Ok(i): índice de la fila saliente
    ///   - Err: si el problema es no acotado (unbounded)
    pub fn find_leaving_variable(&self, entering: usize) -> Result<usize, String> {
        let m = self.num_constraints;
        let n = self.num_vars;
        let rhs_col = n + m;

        let mut min_ratio = f64::INFINITY;
        let mut leaving_row = None;

        for i in 0..m {
            let coeff = self.data[i][entering];
            let rhs = self.data[i][rhs_col];

            // Solo considerar coeficientes positivos (dirección de mejora)
            if coeff > 1e-10 {
                let ratio = rhs / coeff;

                if ratio < min_ratio {
                    min_ratio = ratio;
                    leaving_row = Some(i);
                }
            }
        }

        leaving_row.ok_or_else(|| {
            "Unbounded problem: no valid leaving variable (all coefficients are non-positive)"
                .to_string()
        })
    }

    /// Realizar operación de pivoteo
    ///
    /// Transforma el tableau para que:
    ///   1. La variable entering entre a la base
    ///   2. La variable leaving salga de la base
    ///   3. La columna entering tenga un 1 en la fila leaving y 0s en las demás
    ///
    /// Args:
    ///   - entering: índice de la columna entrante
    ///   - leaving: índice de la fila saliente
    pub fn pivot(&mut self, entering: usize, leaving: usize) {
        let m = self.num_constraints;
        let n = self.num_vars;
        let total_cols = n + m + 1; // Incluye RHS

        // 1. Normalizar la fila pivote (hacer que el elemento pivote sea 1)
        let pivot_element = self.data[leaving][entering];
        for j in 0..total_cols {
            self.data[leaving][j] /= pivot_element;
        }

        // 2. Eliminar la columna entering en todas las demás filas (hacer 0s)
        for i in 0..=m {
            // Incluye la fila objetivo
            if i != leaving {
                let factor = self.data[i][entering];
                for j in 0..total_cols {
                    self.data[i][j] -= factor * self.data[leaving][j];
                }
            }
        }

        // 3. Actualizar la base
        self.basis[leaving] = entering;
    }

    /// Extraer la solución del tableau
    ///
    /// Returns:
    ///   - Vector de n elementos con los valores de las variables de decisión
    pub fn extract_solution(&self) -> Vec<f64> {
        let n = self.num_vars;
        let m = self.num_constraints;
        let rhs_col = n + m;

        let mut solution = vec![0.0; n];

        // Las variables básicas tienen valor = RHS de su fila
        // Las variables no básicas tienen valor = 0
        for (i, &basic_var) in self.basis.iter().enumerate() {
            if basic_var < n {
                // Es una variable de decisión (no una variable de holgura)
                solution[basic_var] = self.data[i][rhs_col];
            }
        }

        solution
    }

    /// Obtener el valor objetivo actual
    pub fn objective_value(&self) -> f64 {
        let m = self.num_constraints;
        let n = self.num_vars;
        let rhs_col = n + m;

        self.data[m][rhs_col]
    }

    /// Obtener las variables básicas actuales (para debugging)
    pub fn basis(&self) -> &[usize] {
        &self.basis
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tableau_creation() {
        // maximize z = 3x₁ + 5x₂
        // subject to:
        //   x₁ ≤ 4
        //   2x₂ ≤ 12
        //   3x₁ + 2x₂ ≤ 18
        let c = vec![3.0, 5.0];
        let a = Matrix::new(3, 2, vec![1.0, 0.0, 0.0, 2.0, 3.0, 2.0]).unwrap();
        let b = vec![4.0, 12.0, 18.0];

        let tableau = Tableau::new(&c, &a, &b, 1.0).unwrap();

        // Verificar dimensiones
        assert_eq!(tableau.num_vars, 2);
        assert_eq!(tableau.num_constraints, 3);

        // Verificar tableau inicial
        // Fila 0: [1, 0, 1, 0, 0 | 4]
        assert_eq!(tableau.data[0][0], 1.0);
        assert_eq!(tableau.data[0][1], 0.0);
        assert_eq!(tableau.data[0][2], 1.0); // s₁
        assert_eq!(tableau.data[0][5], 4.0); // RHS

        // Fila objetivo: [-3, -5, 0, 0, 0 | 0]
        assert_eq!(tableau.data[3][0], -3.0);
        assert_eq!(tableau.data[3][1], -5.0);
        assert_eq!(tableau.data[3][5], 0.0);
    }

    #[test]
    fn test_is_optimal() {
        let c = vec![3.0, 5.0];
        let a = Matrix::new(3, 2, vec![1.0, 0.0, 0.0, 2.0, 3.0, 2.0]).unwrap();
        let b = vec![4.0, 12.0, 18.0];

        let tableau = Tableau::new(&c, &a, &b, 1.0).unwrap();

        // Tableau inicial no es óptimo (tiene coeficientes negativos)
        assert!(!tableau.is_optimal());
    }

    #[test]
    fn test_find_entering_variable() {
        let c = vec![3.0, 5.0];
        let a = Matrix::new(3, 2, vec![1.0, 0.0, 0.0, 2.0, 3.0, 2.0]).unwrap();
        let b = vec![4.0, 12.0, 18.0];

        let tableau = Tableau::new(&c, &a, &b, 1.0).unwrap();

        // Columna con coeficiente más negativo es x₂ (columna 1, coeficiente -5)
        let entering = tableau.find_entering_variable();
        assert_eq!(entering, Some(1));
    }

    #[test]
    fn test_find_leaving_variable() {
        let c = vec![3.0, 5.0];
        let a = Matrix::new(3, 2, vec![1.0, 0.0, 0.0, 2.0, 3.0, 2.0]).unwrap();
        let b = vec![4.0, 12.0, 18.0];

        let tableau = Tableau::new(&c, &a, &b, 1.0).unwrap();

        // Entering variable es x₂ (columna 1)
        // Ratios: fila 0: 4/0 = ∞, fila 1: 12/2 = 6, fila 2: 18/2 = 9
        // Mínimo ratio es fila 1
        let leaving = tableau.find_leaving_variable(1).unwrap();
        assert_eq!(leaving, 1);
    }

    #[test]
    fn test_pivot() {
        let c = vec![3.0, 5.0];
        let a = Matrix::new(3, 2, vec![1.0, 0.0, 0.0, 2.0, 3.0, 2.0]).unwrap();
        let b = vec![4.0, 12.0, 18.0];

        let mut tableau = Tableau::new(&c, &a, &b, 1.0).unwrap();

        // Pivotear: entering = 1 (x₂), leaving = 1 (s₂)
        tableau.pivot(1, 1);

        // Después del pivoteo:
        // - Columna 1 debe tener un 1 en fila 1 y 0s en las demás
        assert!((tableau.data[1][1] - 1.0).abs() < 1e-10);
        assert!(tableau.data[0][1].abs() < 1e-10);
        assert!(tableau.data[2][1].abs() < 1e-10);
        assert!(tableau.data[3][1].abs() < 1e-10);

        // Base debe actualizarse
        assert_eq!(tableau.basis[1], 1); // x₂ entra a la base
    }
}
