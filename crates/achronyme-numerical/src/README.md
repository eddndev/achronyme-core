# Numerical Implementation

**Documentación técnica interna de la implementación de métodos numéricos.**

## 🏛️ Arquitectura del sistema

### Flujo de datos

```
Usuario SOC:  diff(f, 2.0)
      ↓
achronyme-eval: handlers::numerical::handle_diff()
      ↓
achronyme-numerical: diff_central(&mut evaluator, &func, x, h)
      ↓
LambdaEvaluator trait: evaluator.eval_at(func, x + h)
      ↓
Evaluator: Evalúa func(x + h) → f64
      ↓
Resultado: f'(x) ≈ (f(x+h) - f(x-h)) / 2h
```

### Separación de responsabilidades

```
┌─────────────────────────────────────────────────────────┐
│  achronyme-numerical (ESTE CRATE)                       │
│  ├── differentiation.rs                                 │
│  │   └── Algoritmos de diferencias finitas             │
│  ├── integration.rs                                     │
│  │   └── Algoritmos de cuadratura                      │
│  └── solvers.rs                                         │
│      └── Algoritmos de búsqueda de raíces              │
└─────────────────────────────────────────────────────────┘
                         ↑
                         │ usa
                         │
┌─────────────────────────────────────────────────────────┐
│  achronyme-types                                        │
│  ├── Function (representación de funciones)            │
│  └── LambdaEvaluator trait (evaluar funciones)         │
└─────────────────────────────────────────────────────────┘
```

## 📁 Estructura de módulos

```
src/
├── lib.rs                    # Re-exports públicos
├── differentiation.rs        # 218 LOC - Diferencias finitas
├── integration.rs            # 349 LOC - Cuadratura numérica
└── solvers.rs                # 326 LOC - Root finding

Total: ~900 LOC
```

### lib.rs - API pública

```rust
pub mod differentiation;
pub mod integration;
pub mod solvers;

// Re-exports para conveniencia
pub use differentiation::*;
pub use integration::*;
pub use solvers::*;
```

**Responsabilidades**:
- Organizar módulos
- Re-exportar funciones públicas
- Documentación a nivel de crate

## 🔍 Módulo: differentiation.rs

### Funciones implementadas

| Función | Firma | Complejidad | Precisión |
|---------|-------|-------------|-----------|
| `diff_forward` | `<F: FnMut(f64) -> f64>` | O(1) | O(h) |
| `diff_backward` | `<F: FnMut(f64) -> f64>` | O(1) | O(h) |
| `diff_central` | `<E: LambdaEvaluator>` | O(1) | O(h²) |
| `diff2_central` | `<E: LambdaEvaluator>` | O(1) | O(h²) |
| `diff3_central` | `<E: LambdaEvaluator>` | O(1) | O(h²) |
| `gradient` | `<E: LambdaEvaluator>` | O(n) | O(h²) |

### Implementación de diff_central

```rust
pub fn diff_central<E>(
    evaluator: &mut E,
    func: &Function,
    x: f64,
    h: f64,
) -> Result<f64, String>
where
    E: LambdaEvaluator,
{
    // Evaluar f(x + h)
    let f_plus = evaluator.eval_at(func, x + h)?;

    // Evaluar f(x - h)
    let f_minus = evaluator.eval_at(func, x - h)?;

    // f'(x) ≈ (f(x+h) - f(x-h)) / 2h
    Ok((f_plus - f_minus) / (2.0 * h))
}
```

**Detalles técnicos**:
1. **Separación de evaluaciones**: No evalúa f(x) directamente (no se necesita)
2. **Propagación de errores**: Usa `?` para propagar errores de evaluación
3. **No assumptions**: No asume nada sobre la función (puede ser no lineal)
4. **Step size h**: Caller decide h (típicamente 1e-5)

### Implementación de gradient

```rust
pub fn gradient<E>(
    evaluator: &mut E,
    func: &Function,
    point: &[f64],
    h: f64,
) -> Result<Vec<f64>, String>
where
    E: LambdaEvaluator,
{
    let n = point.len();
    let mut grad = vec![0.0; n];

    for i in 0..n {
        // Crear point_plus y point_minus
        let mut point_plus = point.to_vec();
        let mut point_minus = point.to_vec();

        point_plus[i] += h;
        point_minus[i] -= h;

        // Evaluar en ambos puntos
        let f_plus = evaluator.eval_vec_at(func, &point_plus)?;
        let f_minus = evaluator.eval_vec_at(func, &point_minus)?;

        // ∂f/∂x_i ≈ (f(..., x_i + h, ...) - f(..., x_i - h, ...)) / 2h
        grad[i] = (f_plus - f_minus) / (2.0 * h);
    }

    Ok(grad)
}
```

**Características**:
- **Costo**: 2n evaluaciones de función (eficiente)
- **Uso de memoria**: O(n) para almacenar gradiente
- **Paralelizable**: Cada componente es independiente (no implementado aún)

### API antigua vs nueva

**Antigua** (closures):
```rust
pub fn diff_forward<F>(mut f: F, x: f64, h: f64) -> f64
where
    F: FnMut(f64) -> f64,
{
    (f(x + h) - f(x)) / h
}
```

**Nueva** (LambdaEvaluator):
```rust
pub fn diff_central<E>(
    evaluator: &mut E,
    func: &Function,
    x: f64,
    h: f64,
) -> Result<f64, String>
where
    E: LambdaEvaluator,
{
    let f_plus = evaluator.eval_at(func, x + h)?;
    let f_minus = evaluator.eval_at(func, x - h)?;
    Ok((f_plus - f_minus) / (2.0 * h))
}
```

**Ventajas de la nueva API**:
- ✅ Integración con sistema de tipos de Achronyme
- ✅ Manejo de errores robusto (`Result<f64, String>`)
- ✅ Soporta funciones SOC user-defined
- ✅ Permite evaluación en entorno con variables

## 🔍 Módulo: integration.rs

### Funciones implementadas

| Función | Subdivisions | Precisión | Adaptativo | Uso |
|---------|--------------|-----------|------------|-----|
| `trapz` | n fijo | O(n⁻²) | No | General, rápido |
| `simpson` | n fijo (par) | O(n⁻⁴) | No | Más preciso |
| `simpson38` | n fijo (×3) | O(n⁻⁴) | No | Alternativa |
| `romberg` | 2^k | Exponencial | Sí | Alta precisión |
| `quad` | Adaptativo | Según tol | Sí | Balance |
| `trapz_discrete` | - | - | No | Datos tabulados |

### Implementación de trapz

```rust
pub fn trapz<E>(
    evaluator: &mut E,
    func: &Function,
    a: f64,
    b: f64,
    n: usize,
) -> Result<f64, String>
where
    E: LambdaEvaluator,
{
    if n == 0 {
        return Ok(0.0);
    }

    let h = (b - a) / n as f64;

    // Evaluar extremos
    let f_a = evaluator.eval_at(func, a)?;
    let f_b = evaluator.eval_at(func, b)?;

    // Suma ponderada: 0.5 * (f(a) + f(b))
    let mut sum = 0.5 * (f_a + f_b);

    // Sumar puntos interiores con peso 1.0
    for i in 1..n {
        sum += evaluator.eval_at(func, a + i as f64 * h)?;
    }

    Ok(h * sum)
}
```

**Características**:
- **Evaluaciones**: n + 1 evaluaciones de función
- **Estabilidad**: Acumula suma (puede tener error de redondeo)
- **Mejora**: Compensated summation (Kahan) para mayor precisión (no implementado)

### Implementación de simpson

```rust
pub fn simpson<E>(
    evaluator: &mut E,
    func: &Function,
    a: f64,
    b: f64,
    n: usize,
) -> Result<f64, String>
where
    E: LambdaEvaluator,
{
    // Asegurar n par
    let n = if n % 2 == 0 { n } else { n + 1 };

    if n == 0 {
        return Ok(0.0);
    }

    let h = (b - a) / n as f64;

    let f_a = evaluator.eval_at(func, a)?;
    let f_b = evaluator.eval_at(func, b)?;
    let mut sum = f_a + f_b;

    // Patrón 4-2-4-2-... (alternar pesos)
    for i in 1..n {
        let x = a + i as f64 * h;
        let coefficient = if i % 2 == 0 { 2.0 } else { 4.0 };
        sum += coefficient * evaluator.eval_at(func, x)?;
    }

    Ok((h / 3.0) * sum)
}
```

**Patrón de pesos**:
```
i:     0   1   2   3   4   5   ...  n
peso:  1   4   2   4   2   4   ...  1
```

**Por qué es más preciso**:
- Trapz usa interpolación lineal (rectas)
- Simpson usa interpolación cuadrática (parábolas)
- Exacto para polinomios de grado ≤ 3

### Implementación de Romberg

```rust
pub fn romberg<E>(
    evaluator: &mut E,
    func: &Function,
    a: f64,
    b: f64,
    tol: f64,
    max_iter: usize,
) -> Result<f64, String>
where
    E: LambdaEvaluator,
{
    let mut r = vec![vec![0.0; max_iter]; max_iter];

    // Columna 0: Regla del trapecio con 1, 2, 4, 8, ... subdivisiones
    let f_a = evaluator.eval_at(func, a)?;
    let f_b = evaluator.eval_at(func, b)?;
    r[0][0] = (b - a) * (f_a + f_b) / 2.0;

    for i in 1..max_iter {
        let n = 1 << i; // 2^i
        let h = (b - a) / n as f64;

        // Trapecio compuesto (solo evaluar nuevos puntos)
        let mut sum = 0.0;
        for j in 1..n {
            if j % 2 == 1 {
                sum += evaluator.eval_at(func, a + j as f64 * h)?;
            }
        }

        r[i][0] = 0.5 * r[i - 1][0] + h * sum;

        // Extrapolación de Richardson
        for j in 1..=i {
            let power = 4_f64.powi(j as i32);
            r[i][j] = (power * r[i][j - 1] - r[i - 1][j - 1]) / (power - 1.0);
        }

        // Verificar convergencia
        if i > 0 && (r[i][i] - r[i - 1][i - 1]).abs() < tol {
            return Ok(r[i][i]);
        }
    }

    Ok(r[max_iter - 1][max_iter - 1])
}
```

**Tabla de Romberg** (ejemplo):
```
       j=0         j=1         j=2         j=3
i=0   R[0,0]
i=1   R[1,0]     R[1,1]
i=2   R[2,0]     R[2,1]     R[2,2]
i=3   R[3,0]     R[3,1]     R[3,2]     R[3,3]
```

**Fórmula de Richardson**:
```
R[i,j] = (4^j * R[i,j-1] - R[i-1,j-1]) / (4^j - 1)
```

**Ventaja**: Cada columna a la derecha duplica el orden de precisión.

### Implementación de trapz_discrete

```rust
pub fn trapz_discrete(x: &[f64], y: &[f64]) -> f64 {
    if x.len() != y.len() || x.len() < 2 {
        return 0.0;
    }

    let mut sum = 0.0;

    for i in 0..x.len() - 1 {
        let h = x[i + 1] - x[i];
        sum += 0.5 * h * (y[i] + y[i + 1]);
    }

    sum
}
```

**Uso**: Cuando tienes datos experimentales en lugar de una función analítica.

**Ejemplo**:
```rust
let x = vec![0.0, 0.5, 1.0, 1.5, 2.0];
let y = vec![0.0, 0.25, 1.0, 2.25, 4.0]; // y = x²
let area = trapz_discrete(&x, &y); // ≈ 2.67 (exacto: 8/3)
```

## 🔍 Módulo: solvers.rs

### Funciones implementadas

| Función | Convergencia | Requisitos | Complejidad | Uso |
|---------|--------------|------------|-------------|-----|
| `bisect` | Lineal | f(a)·f(b) < 0 | O(log ε⁻¹) | Robusto |
| `newton` | Cuadrática | f, f' | O(log log ε⁻¹) | Rápido |
| `secant` | Superlineal | f | O(k log ε⁻¹) | Balance |
| `fixed_point_iteration` | Lineal | \|g'(x)\| < 1 | O(ε⁻¹) | Simple |
| `newton_system_2d` | Cuadrática | f1, f2 | O(k) | Sistemas |

### Implementación de bisect

```rust
pub fn bisect<E>(
    evaluator: &mut E,
    func: &Function,
    mut a: f64,
    mut b: f64,
    tol: f64,
) -> Result<f64, String>
where
    E: LambdaEvaluator,
{
    let fa = evaluator.eval_at(func, a)?;
    let fb = evaluator.eval_at(func, b)?;

    // Verificar teorema del valor intermedio
    if fa * fb > 0.0 {
        return Err("bisect: f(a) and f(b) must have opposite signs".to_string());
    }

    // Iterar hasta convergencia
    while (b - a).abs() > tol {
        let c = (a + b) / 2.0;
        let fc = evaluator.eval_at(func, c)?;

        // Si f(c) ≈ 0, hemos encontrado la raíz
        if fc.abs() < tol {
            return Ok(c);
        }

        // Actualizar intervalo
        if fa * fc < 0.0 {
            b = c;
        } else {
            a = c;
        }
    }

    Ok((a + b) / 2.0)
}
```

**Invariante**: f(a) · f(b) < 0 (signos opuestos) en cada iteración.

**Convergencia**:
```
Iteración k: error ≤ (b₀ - a₀) / 2^k
```

Para ε = 1e-6, b₀ - a₀ = 10:
```
k = log₂(10 / 1e-6) = log₂(10^7) ≈ 23 iteraciones
```

### Implementación de newton

```rust
pub fn newton<E>(
    evaluator: &mut E,
    func: &Function,
    dfunc: &Function,
    mut x: f64,
    tol: f64,
    max_iter: usize,
) -> Result<f64, String>
where
    E: LambdaEvaluator,
{
    for _ in 0..max_iter {
        let fx = evaluator.eval_at(func, x)?;

        // Verificar convergencia
        if fx.abs() < tol {
            return Ok(x);
        }

        let dfx = evaluator.eval_at(dfunc, x)?;

        // Evitar división por cero
        if dfx.abs() < 1e-12 {
            return Err("Newton: derivative too small, cannot continue".to_string());
        }

        // x_{n+1} = x_n - f(x_n) / f'(x_n)
        x = x - fx / dfx;
    }

    Ok(x)
}
```

**Convergencia cuadrática**:
```
|e_{n+1}| ≤ C|e_n|²

Ejemplo: error = 0.1
Iter 1: 0.01
Iter 2: 0.0001
Iter 3: 0.00000001  (8 dígitos de precisión en 3 iteraciones!)
```

**Problema**: Puede divergir si el punto inicial está lejos de la raíz.

### Implementación de secant

```rust
pub fn secant<E>(
    evaluator: &mut E,
    func: &Function,
    mut x0: f64,
    mut x1: f64,
    tol: f64,
    max_iter: usize,
) -> Result<f64, String>
where
    E: LambdaEvaluator,
{
    let mut fx0 = evaluator.eval_at(func, x0)?;

    for _ in 0..max_iter {
        let fx1 = evaluator.eval_at(func, x1)?;

        if fx1.abs() < tol {
            return Ok(x1);
        }

        // Evitar división por cero
        if (fx1 - fx0).abs() < 1e-12 {
            return Err("Secant: denominator too small, cannot continue".to_string());
        }

        // x_{n+1} = x_n - f(x_n) * (x_n - x_{n-1}) / (f(x_n) - f(x_{n-1}))
        let x2 = x1 - fx1 * (x1 - x0) / (fx1 - fx0);

        // Actualizar para próxima iteración
        x0 = x1;
        fx0 = fx1;
        x1 = x2;
    }

    Ok(x1)
}
```

**Ventaja sobre Newton**:
- No requiere calcular f'(x) (ahorro de 1 evaluación por iteración)
- Aproxima f'(x) con diferencias finitas usando dos puntos

**Convergencia**:
```
|e_{n+1}| ≈ |e_n|^φ  donde φ = (1 + √5) / 2 ≈ 1.618 (golden ratio)
```

Más lento que Newton pero más rápido que bisección.

### Implementación de newton_system_2d

```rust
pub fn newton_system_2d<F1, F2>(
    mut f1: F1,
    mut f2: F2,
    mut x: f64,
    mut y: f64,
    tol: f64,
    max_iter: usize,
) -> (f64, f64)
where
    F1: FnMut(f64, f64) -> f64,
    F2: FnMut(f64, f64) -> f64,
{
    let h = 1e-8;

    for _ in 0..max_iter {
        let f1_val = f1(x, y);
        let f2_val = f2(x, y);

        // Verificar convergencia
        if f1_val.abs() < tol && f2_val.abs() < tol {
            return (x, y);
        }

        // Jacobiano (diferencias finitas)
        let df1_dx = (f1(x + h, y) - f1_val) / h;
        let df1_dy = (f1(x, y + h) - f1_val) / h;
        let df2_dx = (f2(x + h, y) - f2_val) / h;
        let df2_dy = (f2(x, y + h) - f2_val) / h;

        // Determinante del Jacobiano
        let det = df1_dx * df2_dy - df1_dy * df2_dx;

        if det.abs() < 1e-12 {
            break; // Jacobiano singular
        }

        // Resolver J · [dx, dy]^T = -[f1, f2]^T usando regla de Cramer
        let dx = (-f1_val * df2_dy + f2_val * df1_dy) / det;
        let dy = (f1_val * df2_dx - f2_val * df1_dx) / det;

        // Actualizar
        x += dx;
        y += dy;
    }

    (x, y)
}
```

**Jacobiano** para sistema 2D:
```
J = | ∂f1/∂x  ∂f1/∂y |
    | ∂f2/∂x  ∂f2/∂y |
```

**Iteración de Newton**:
```
[x]     [x]       -1  [f1(x,y)]
[y]   = [y]  - J      [f2(x,y)]
 n+1     n
```

**Ejemplo**: Encontrar intersección de círculo y recta:
```
f1(x, y) = x² + y² - 25  (círculo de radio 5)
f2(x, y) = x - y - 1      (recta y = x - 1)

Soluciones: (4, 3) y (-3, -4)
```

## 🧪 Testing

### Estado actual de los tests

**Problema**: Tests marcados como `#[ignore]` porque:
```rust
// API antigua (closures)
let f = |x: f64| x * x;
let derivative = diff_forward(f, 2.0, 1e-5);
```

**Necesitan refactoring a**:
```rust
// API nueva (LambdaEvaluator)
let mut evaluator = Evaluator::new();
let func = parse_to_function("x => x * x")?;
let derivative = diff_central(&mut evaluator, &func, 2.0, 1e-5)?;
```

### Tests pendientes

```rust
// differentiation.rs
#[test]
#[ignore]
fn test_forward_difference() { /* TODO */ }

#[test]
#[ignore]
fn test_central_difference() { /* TODO */ }

#[test]
#[ignore]
fn test_gradient() { /* TODO */ }

// integration.rs
#[test]
#[ignore]
fn test_trapz_linear() { /* TODO */ }

#[test]
#[ignore]
fn test_simpson_quadratic() { /* TODO */ }

#[test]
#[ignore]
fn test_romberg() { /* TODO */ }

// solvers.rs
#[test]
#[ignore]
fn test_bisect_quadratic() { /* TODO */ }

#[test]
#[ignore]
fn test_newton_quadratic() { /* TODO */ }

#[test]
#[ignore]
fn test_secant() { /* TODO */ }
```

### Test de trapz_discrete (único que funciona)

```rust
#[test]
fn test_trapz_discrete() {
    let x = vec![0.0, 0.5, 1.0];
    let y = vec![0.0, 0.25, 1.0];  // y = x²
    let result = trapz_discrete(&x, &y);
    // ∫₀¹ x² dx = 1/3 ≈ 0.333
    assert!((result - 1.0 / 3.0).abs() < 0.05);
}
```

## 📐 Patrones de diseño

### 1. Trait-based API

Todas las funciones usan genéricos con trait bound:
```rust
pub fn diff_central<E>(
    evaluator: &mut E,
    func: &Function,
    x: f64,
    h: f64,
) -> Result<f64, String>
where
    E: LambdaEvaluator,
{
    // ...
}
```

**Ventajas**:
- ✅ Flexible: Cualquier tipo que implemente `LambdaEvaluator` funciona
- ✅ Testeable: Puede usar mock evaluator en tests
- ✅ Sin overhead: Monomorphization elimina costo de abstracción

### 2. Error propagation with Result

```rust
let f_plus = evaluator.eval_at(func, x + h)?;
```

Propaga errores de evaluación al caller:
- División por cero
- Dominio inválido (ej: sqrt(-1))
- Stack overflow en recursión

### 3. Numerical stability checks

```rust
if dfx.abs() < 1e-12 {
    return Err("Newton: derivative too small, cannot continue".to_string());
}
```

Evita:
- División por cero
- Overflow/underflow
- Cancelación catastrófica

### 4. Early return on convergence

```rust
if fx.abs() < tol {
    return Ok(x);
}
```

Ahorra iteraciones innecesarias cuando ya convergió.

## 🔧 Extensión y mantenimiento

### Agregar nuevo método de integración

**Ejemplo**: Gauss-Legendre quadrature

1. **Implementar en `integration.rs`**:
```rust
/// Gauss-Legendre quadrature (n-point)
///
/// Exacto para polinomios de grado ≤ 2n-1
pub fn gauss_legendre<E>(
    evaluator: &mut E,
    func: &Function,
    a: f64,
    b: f64,
    n: usize,
) -> Result<f64, String>
where
    E: LambdaEvaluator,
{
    // Tabla de nodos y pesos pre-calculados
    let (nodes, weights) = match n {
        2 => (vec![-0.5773502691896257, 0.5773502691896257],
              vec![1.0, 1.0]),
        3 => (vec![-0.7745966692414834, 0.0, 0.7745966692414834],
              vec![0.5555555555555556, 0.8888888888888888, 0.5555555555555556]),
        // ... más puntos
        _ => return Err("Gauss-Legendre: unsupported number of points".to_string()),
    };

    // Transformar de [-1, 1] a [a, b]
    let mid = (a + b) / 2.0;
    let half = (b - a) / 2.0;

    let mut sum = 0.0;
    for i in 0..n {
        let x = mid + half * nodes[i];
        sum += weights[i] * evaluator.eval_at(func, x)?;
    }

    Ok(half * sum)
}
```

2. **Re-exportar en `lib.rs`**:
```rust
pub use integration::gauss_legendre;
```

3. **Agregar test**:
```rust
#[test]
fn test_gauss_legendre_polynomial() {
    // Exacto para polinomios grado ≤ 2n-1
    // ∫₀¹ x⁴ dx = 1/5 con n=3 puntos (grado 5)
}
```

### Agregar método de raíces (Brent)

**Brent's method**: Combina bisección, secant e interpolación cuadrática

```rust
pub fn brent<E>(
    evaluator: &mut E,
    func: &Function,
    mut a: f64,
    mut b: f64,
    tol: f64,
) -> Result<f64, String>
where
    E: LambdaEvaluator,
{
    // 1. Verificar f(a) * f(b) < 0
    // 2. Iterar:
    //    - Intentar interpolación cuadrática inversa
    //    - Si falla, usar secant
    //    - Si secant diverge, usar bisección
    // 3. Retornar cuando |b - a| < tol
}
```

## 📊 Complejidad y rendimiento

### Evaluaciones de función por método

| Operación | Evaluaciones | Notas |
|-----------|--------------|-------|
| diff_central(f, x) | 2 | f(x+h), f(x-h) |
| diff2_central(f, x) | 3 | f(x+h), f(x), f(x-h) |
| gradient(f, p) | 2n | n dimensiones |
| trapz(f, a, b, n) | n+1 | Todos los puntos |
| simpson(f, a, b, n) | n+1 | Todos los puntos |
| romberg(f, a, b) | ~2^k | Adaptativo, k iteraciones |
| bisect(f, a, b) | log₂(L/ε) | L = b-a, ε = tolerancia |
| newton(f, df, x₀) | 2k | k iteraciones, f y f' cada vez |
| secant(f, x₀, x₁) | k+1 | k iteraciones, solo f |

### Optimizaciones posibles

1. **Memoization**: Cache evaluaciones repetidas
   ```rust
   let mut cache = HashMap::new();
   if let Some(&val) = cache.get(&x) {
       return val;
   }
   ```

2. **Parallel gradient**: Evaluar ∂f/∂xᵢ en paralelo
   ```rust
   use rayon::prelude::*;
   let grad: Vec<f64> = (0..n).into_par_iter()
       .map(|i| compute_partial_derivative(i))
       .collect();
   ```

3. **Adaptive step size**: Ajustar h según magnitud de f
   ```rust
   let h = h_base * (1.0 + f_val.abs()).sqrt();
   ```

4. **Richardson extrapolation**: Mejorar precisión usando múltiples h
   ```rust
   let d1 = diff_central(f, x, h);
   let d2 = diff_central(f, x, h/2);
   let improved = (4.0 * d2 - d1) / 3.0; // O(h⁴) precisión
   ```

## 🎯 Casos especiales y limitaciones

### Diferenciación

**Problemas**:
- **h muy pequeño**: Error de redondeo domina
- **h muy grande**: Error de truncamiento domina
- **Funciones discontinuas**: No funciona

**Solución**:
```rust
// Elegir h adaptativo
let h = if x.abs() > 1.0 {
    1e-5 * x.abs() // Proporcional a magnitud de x
} else {
    1e-5
};
```

### Integración

**Problemas**:
- **Funciones oscilatorias**: Requieren muchas subdivisiones
- **Singularidades**: Integral diverge o requiere tratamiento especial
- **Intervalos infinitos**: Requieren cambio de variable

**Ejemplo**: ∫₀^∞ e⁻ˣ dx
```rust
// Cambio de variable: x = t/(1-t), dx = dt/(1-t)²
// ∫₀^∞ e⁻ˣ dx = ∫₀¹ e⁻ᵗ⁄⁽¹⁻ᵗ⁾ / (1-t)² dt
```

### Resolución de raíces

**Problemas**:
- **Raíces múltiples**: Convergencia lenta
- **Múltiples raíces**: Solo encuentra una
- **Sin raíz**: Bisección falla si f(a)·f(b) > 0

**Estrategias**:
- Usar deflación para encontrar múltiples raíces
- Dividir dominio y buscar en cada intervalo
- Combinar métodos (Brent = bisección + secant + inversa cuadrática)

## 🔗 Integración con achronyme-eval

### Flujo completo: Usuario → Resultado

1. **Usuario escribe SOC**:
   ```javascript
   let f = x => x^2 - 4
   solve(f, 0, 5)
   ```

2. **Parser genera AST**:
   ```rust
   AstNode::FunctionCall {
       name: "solve",
       args: vec![
           AstNode::Lambda { ... },
           AstNode::Number(0.0),
           AstNode::Number(5.0)
       ]
   }
   ```

3. **Evaluador despacha a handler**:
   ```rust
   // achronyme-eval/src/handlers/numerical.rs
   "solve" => {
       let func = eval_to_function(args[0])?;
       let a = eval_to_f64(args[1])?;
       let b = eval_to_f64(args[2])?;

       let root = bisect(evaluator, &func, a, b, 1e-6)?;
       Ok(Value::Number(root))
   }
   ```

4. **Numerical crate ejecuta algoritmo**:
   ```rust
   // achronyme-numerical/src/solvers.rs
   pub fn bisect<E>(evaluator, func, a, b, tol) {
       // ... algoritmo de bisección
       Ok((a + b) / 2.0)
   }
   ```

5. **Resultado retorna al usuario**:
   ```
   2.0
   ```

## 📚 Referencias

### Libros
- **Burden & Faires** (2010). *Numerical Analysis*. Brooks/Cole.
- **Press et al.** (2007). *Numerical Recipes: The Art of Scientific Computing*. Cambridge.
- **Heath** (2018). *Scientific Computing: An Introductory Survey*. SIAM.

### Papers
- **Richardson** (1911). "The Approximate Arithmetical Solution by Finite Differences of Physical Problems Involving Differential Equations."
- **Romberg** (1955). "Vereinfachte numerische Integration."
- **Brent** (1973). "An algorithm with guaranteed convergence for finding a zero of a function."

### Online
- [Numerical Methods - MIT OpenCourseWare](https://ocw.mit.edu)
- [GSL Manual](https://www.gnu.org/software/gsl/doc/html/index.html)
- [SciPy Documentation](https://docs.scipy.org/doc/scipy/reference/integrate.html)
