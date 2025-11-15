# Achronyme Numerical

**Métodos numéricos para cálculo diferencial, integral y resolución de ecuaciones.**

## 🎯 Responsabilidad

El crate `achronyme-numerical` proporciona algoritmos de **análisis numérico** para operaciones que no tienen solución analítica cerrada o requieren aproximación numérica:

```
Usuario SOC
      ↓
achronyme-parser  →  AST
      ↓
achronyme-eval    →  Llama funciones numéricas (diff, integral, bisect, newton)
      ↓
achronyme-numerical  →  Algoritmos numéricos (ESTE CRATE)
      ↓
achronyme-types   →  Function, LambdaEvaluator
```

### Capacidades principales:
- **Diferenciación numérica** - Calcular derivadas usando diferencias finitas
- **Integración numérica** - Calcular integrales usando cuadratura
- **Resolución de ecuaciones** - Encontrar raíces de funciones no lineales

## 📦 Dependencias

### Internas:
- **`achronyme-types`** - Para `Function`, `LambdaEvaluator` trait

### Externas:
- **Ninguna** - Implementación pura sin dependencias externas

## 🔌 Usado por

- **`achronyme-eval`** - Evaluador que llama los métodos numéricos desde código SOC:
  - `diff(f, x, h)` → `diff_central()`
  - `integral(f, a, b, n)` → `trapz()`, `simpson()`, `romberg()`
  - `solve(f, a, b)` → `bisect()`, `newton()`, `secant()`

## 🏗️ Arquitectura de alto nivel

```
achronyme-numerical/
├── src/
│   ├── lib.rs                # Re-exports públicos
│   ├── differentiation.rs    # Diferencias finitas (forward, backward, central)
│   ├── integration.rs        # Cuadratura (trapz, Simpson, Romberg)
│   └── solvers.rs            # Root finding (bisect, Newton, secant)
└── tests/
    └── (no hay tests actualmente - están marcados como #[ignore])
```

### Módulos:
- **`differentiation`** - Derivadas de orden 1, 2, 3 y gradientes
- **`integration`** - Métodos de cuadratura adaptativa y no adaptativa
- **`solvers`** - Métodos de búsqueda de raíces y sistemas de ecuaciones

## 📊 Algoritmos clave

### 1. Diferenciación numérica

| Método | Fórmula | Error | Uso |
|--------|---------|-------|-----|
| **Forward difference** | f'(x) ≈ (f(x+h) - f(x)) / h | O(h) | Rápido, menos preciso |
| **Backward difference** | f'(x) ≈ (f(x) - f(x-h)) / h | O(h) | Similar a forward |
| **Central difference** | f'(x) ≈ (f(x+h) - f(x-h)) / 2h | O(h²) | **Recomendado**: más preciso |
| **Second derivative** | f''(x) ≈ (f(x+h) - 2f(x) + f(x-h)) / h² | O(h²) | Derivadas de orden superior |
| **Gradient** | ∇f = [∂f/∂x₁, ..., ∂f/∂xₙ] | O(h²) | Funciones multivariables |

**Complejidad**: O(k) evaluaciones de función para derivada de orden k

### 2. Integración numérica

| Método | Complejidad | Error | Uso |
|--------|-------------|-------|-----|
| **Trapezoidal** | O(n) | O(n⁻²) | General, simple |
| **Simpson 1/3** | O(n) | O(n⁻⁴) | Más preciso que trapz |
| **Simpson 3/8** | O(n) | O(n⁻⁴) | Alternativa a Simpson 1/3 |
| **Romberg** | O(n log n) | O(e⁻ᶜⁿ) | **Adaptativo**: alta precisión |
| **Quad** | O(n) adaptativo | Según tolerancia | Adaptativo con refinamiento |

**Complejidad**: O(n) evaluaciones de función para n subdivisiones

### 3. Resolución de ecuaciones

| Método | Convergencia | Complejidad | Requisitos | Uso |
|--------|--------------|-------------|------------|-----|
| **Bisection** | Lineal | O(log ε⁻¹) | f(a)·f(b) < 0 | **Robusto**: siempre converge |
| **Newton** | Cuadrática | O(log log ε⁻¹) | f'(x) conocida | **Rápido**: si buen punto inicial |
| **Secant** | Superlineal (1.618) | O(log ε⁻¹) | No requiere f' | Balance velocidad/simplicidad |
| **Fixed-point** | Lineal | O(ε⁻¹) | \|g'(x)\| < 1 | Iteración simple |

**Convergencia**:
- Bisección: ε_n = (b-a)/2ⁿ
- Newton: ε_{n+1} ≈ ε_n²
- Secant: ε_{n+1} ≈ ε_n^φ (φ = 1.618, golden ratio)

## 🚀 Ejemplos de uso

### Diferenciación

```rust
use achronyme_numerical::diff_central;
use achronyme_types::function::Function;

// Crear función f(x) = x²
let func = Function::UserDefined { /* ... */ };

// Calcular f'(x) en x = 2.0
let derivative = diff_central(&mut evaluator, &func, 2.0, 1e-5)?;
// f'(2) = 4.0 (exacto: 2x = 4)

// Gradiente de f(x, y) = x² + y²
let gradient = gradient(&mut evaluator, &func, &[1.0, 2.0], 1e-5)?;
// ∇f(1,2) = [2.0, 4.0] (exacto: [2x, 2y])
```

**Desde SOC**:
```javascript
let f = x => x^2
diff(f, 2.0)        // → 4.0

let g = (x, y) => x^2 + y^2
gradient(g, [1, 2]) // → [2, 4]
```

### Integración

```rust
use achronyme_numerical::{trapz, simpson, romberg};

// ∫x dx de 0 a 1 = 0.5
let result = trapz(&mut evaluator, &func, 0.0, 1.0, 100)?;
// result ≈ 0.5 (con n=100 subdivisiones)

// ∫sin(x) dx de 0 a π = 2
let result = simpson(&mut evaluator, &sin_func, 0.0, PI, 100)?;
// result ≈ 2.0 (más preciso que trapz)

// Integración adaptativa de alta precisión
let result = romberg(&mut evaluator, &func, 0.0, PI, 1e-10, 20)?;
// result ≈ 2.0 (precisión 1e-10)
```

**Desde SOC**:
```javascript
let f = x => x
integral(f, 0, 1)          // → 0.5

let g = x => sin(x)
integral(g, 0, pi)         // → 2.0

// Integración adaptativa
quad(g, 0, pi, 1e-10)      // → 2.0 (alta precisión)
```

### Resolución de ecuaciones

```rust
use achronyme_numerical::{bisect, newton, secant};

// Resolver x² - 4 = 0 (raíz en x = 2)
let root = bisect(&mut evaluator, &func, 0.0, 5.0, 1e-6)?;
// root ≈ 2.0

// Newton (requiere derivada)
let root = newton(&mut evaluator, &func, &dfunc, 1.0, 1e-10, 100)?;
// Convergencia cuadrática → muy rápido

// Secant (no requiere derivada)
let root = secant(&mut evaluator, &func, 1.0, 3.0, 1e-10, 100)?;
// Más rápido que bisección, no requiere f'
```

**Desde SOC**:
```javascript
let f = x => x^2 - 4
solve(f, 0, 5)             // → 2.0 (usa bisección)

// Newton con derivada
let df = x => 2*x
newton(f, df, 1.0)         // → 2.0 (convergencia rápida)

// Sistema 2D: x² + y² = 25, x - y = 1
let f1 = (x, y) => x^2 + y^2 - 25
let f2 = (x, y) => x - y - 1
newton_system(f1, f2, 3, 2) // → (4, 3)
```

## 🧪 Testing

```bash
# Ejecutar tests
cargo test --package achronyme-numerical

# Tests específicos
cargo test --package achronyme-numerical differentiation
cargo test --package achronyme-numerical integration
cargo test --package achronyme-numerical solvers
```

**Nota**: Actualmente los tests están marcados como `#[ignore]` porque usan la API antigua basada en closures. Necesitan ser refactorizados para usar `Evaluator + Function`.

## 🔧 Cómo extender

### Agregar nuevo método de integración (ej: Gauss-Legendre)

1. **Implementar función en `integration.rs`**:
```rust
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
    // Obtener nodos y pesos de Gauss-Legendre
    let (nodes, weights) = gauss_nodes_weights(n);

    // Transformar intervalo [a, b]
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

3. **Agregar handler en `achronyme-eval`**:
```rust
// En achronyme-eval/src/handlers/numerical.rs
"gauss_legendre" => {
    // Extraer argumentos y llamar gauss_legendre()
}
```

4. **Agregar test**:
```rust
#[test]
fn test_gauss_legendre() {
    // Verificar precisión en polinomios de grado 2n-1
}
```

## 📐 Fundamentos matemáticos

### Diferencias finitas

**Central difference** (O(h²) error):
```
f'(x) = lim[h→0] (f(x+h) - f(x-h)) / (2h)
```

**Expansión de Taylor**:
```
f(x+h) = f(x) + f'(x)h + f''(x)h²/2 + O(h³)
f(x-h) = f(x) - f'(x)h + f''(x)h²/2 + O(h³)

f(x+h) - f(x-h) = 2f'(x)h + O(h³)
→ f'(x) = (f(x+h) - f(x-h))/(2h) + O(h²)
```

**Elección de h**:
- Muy pequeño (h < 1e-8): Error de redondeo domina
- Muy grande (h > 1e-3): Error de truncamiento domina
- **Óptimo**: h ≈ √ε ≈ 1e-5 para precisión doble

### Integración numérica

**Regla del trapecio**:
```
∫[a,b] f(x)dx ≈ h/2 [f(x₀) + 2f(x₁) + 2f(x₂) + ... + f(xₙ)]
```

**Regla de Simpson 1/3** (más precisa):
```
∫[a,b] f(x)dx ≈ h/3 [f(x₀) + 4f(x₁) + 2f(x₂) + 4f(x₃) + ... + f(xₙ)]
```

**Extrapolación de Romberg**:
Usa la fórmula de Richardson para mejorar precisión:
```
R(k,j) = (4^j R(k,j-1) - R(k-1,j-1)) / (4^j - 1)
```

### Métodos de búsqueda de raíces

**Teorema del valor intermedio** (bisección):
```
Si f(a)·f(b) < 0 y f continua → ∃c ∈ (a,b): f(c) = 0
```

**Método de Newton**:
```
x_{n+1} = x_n - f(x_n)/f'(x_n)
```

**Convergencia cuadrática**:
```
|e_{n+1}| ≤ C|e_n|² (error se reduce al cuadrado en cada iteración)
```

**Método secante** (aproxima f' con diferencias):
```
x_{n+1} = x_n - f(x_n) · (x_n - x_{n-1})/(f(x_n) - f(x_{n-1}))
```

## 📖 Documentación interna

Para entender la implementación en detalle:
- [src/README.md](src/README.md) - Arquitectura técnica y detalles de implementación

## 🎯 Principios de diseño

1. **Sin dependencias pesadas** - Implementación pura, solo usa `achronyme-types`
2. **API basada en traits** - Usa `LambdaEvaluator` para flexibilidad
3. **Tolerancia configurable** - Permite al usuario controlar precisión vs velocidad
4. **Error handling robusto** - Retorna `Result<f64, String>` con mensajes claros
5. **Numerical stability** - Considera errores de redondeo en step sizes

## 📊 Estadísticas

- **Líneas de código**: ~350 LOC
- **Módulos**: 3 (differentiation, integration, solvers)
- **Funciones públicas**: 15+ funciones
- **Métodos de integración**: 6 métodos
- **Métodos de resolución**: 5 métodos

## 🔗 Ver también

- [achronyme-types](../achronyme-types/README.md) - Define `Function`, `LambdaEvaluator`
- [achronyme-eval](../achronyme-eval/README.md) - Evalúa código SOC que llama estos métodos
- [achronyme-solver](../achronyme-solver/README.md) - Optimización lineal (complementario a métodos numéricos)

## 🚧 Estado actual y trabajo futuro

### Estado actual:
- ✅ Implementación completa de métodos básicos
- ✅ API basada en `LambdaEvaluator`
- ❌ Tests pendientes (marcados como `#[ignore]`)

### Mejoras futuras:
- **Tests**: Refactorizar tests para usar nueva API
- **Adaptive methods**: Mejorar métodos adaptativos (quad, romberg)
- **Error estimates**: Retornar estimación de error junto con resultado
- **Multi-dimensional**: Integración y optimización multidimensional
- **ODE solvers**: Ecuaciones diferenciales ordinarias (Euler, RK4)
- **Root polishing**: Mejorar precisión de raíces con iteraciones extra
- **Caching**: Cache de evaluaciones de función para reducir costo

### Algoritmos potenciales:
- **Diferenciación**: Automatic differentiation (AD), complex step
- **Integración**: Gauss-Legendre, Clenshaw-Curtis, adaptive Simpson
- **Raíces**: Brent's method (híbrido bisección + secante), Muller's method
- **ODEs**: Runge-Kutta (RK4, RK45), Adams-Bashforth, BDF
- **PDEs**: Finite differences, finite elements
