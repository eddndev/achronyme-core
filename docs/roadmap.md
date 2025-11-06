# Roadmap de Achronyme

**Visión**: Crear un ecosistema de cálculo matemático open-source de clase mundial que pueda competir con sistemas propietarios como Wolfram Mathematica, con enfoque en rendimiento, accesibilidad y extensibilidad.

---

## ✅ Completado (v0.1 - v0.3)

- [x] **Phase 1**: Parser y evaluador de expresiones matemáticas
- [x] **Phase 2**: Operaciones aritméticas básicas (+, -, *, /, ^, %)
- [x] **Phase 3A**: Funciones matemáticas estándar (trigonométricas, exponenciales, logaritmos)
- [x] **Phase 3B**: Tipos complejos (Complex, Vector, Matrix)
- [x] **Phase 4A**: Variables, lambdas, closures, higher-order functions
- [x] **Phase 4B**: DSP básico (DFT, FFT Cooley-Tukey, IFFT, convolución, ventanas)
- [x] **Phase 4C**: Funciones estadísticas nativas (sum, mean, std, max, min)
- [x] **Phase 4D**: SDK TypeScript tipo-seguro con gestión de memoria

---

## 🚧 En Desarrollo (v0.4 - v0.6)

### Phase 5: Álgebra Lineal Avanzada (v0.4)

**Descomposición de matrices:**
- [ ] LU decomposition (factorización PA = LU)
- [ ] QR decomposition (Gram-Schmidt, Householder)
- [ ] Cholesky decomposition (matrices positivas definidas)
- [ ] SVD - Singular Value Decomposition

**Eigenvalues y eigenvectors:**
- [ ] Método de potencias (power iteration)
- [ ] Algoritmo QR para eigenvalues
- [ ] Eigenvalues de matrices simétricas (Jacobi)

**Operaciones avanzadas:**
- [ ] Rank, null space, column space
- [ ] Pseudoinversa (Moore-Penrose)
- [ ] Normas matriciales (Frobenius, 1-norm, ∞-norm)
- [ ] Condición de matriz (condition number)

### Phase 6: Cálculo Numérico (v0.5)

**Derivación numérica:**
- [ ] Diferencias finitas (forward, backward, central)
- [ ] Derivadas de orden superior
- [ ] Derivadas parciales
- [ ] Gradiente, divergencia, curl

**Integración numérica:**
- [ ] Regla del trapecio
- [ ] Regla de Simpson (1/3, 3/8)
- [ ] Cuadratura de Gauss
- [ ] Integración adaptativa (Romberg)
- [ ] Integrales dobles y triples

**Solución de ecuaciones:**
- [ ] Métodos de bisección y secante
- [ ] Newton-Raphson (una variable)
- [ ] Newton-Raphson multidimensional
- [ ] Métodos de punto fijo

**Sistemas de ecuaciones lineales:**
- [ ] Eliminación gaussiana con pivoteo
- [ ] Métodos iterativos (Jacobi, Gauss-Seidel)
- [ ] Gradiente conjugado
- [ ] Sparse matrices (matrices dispersas)

### Phase 7: Optimización (v0.6)

**Optimización sin restricciones:**
- [ ] Gradiente descendente (vanilla, momentum, AdaGrad, Adam)
- [ ] Método de Newton
- [ ] Quasi-Newton (BFGS, L-BFGS)
- [ ] Nelder-Mead (simplex)
- [ ] Simulated annealing

**Optimización con restricciones:**
- [ ] Simplex method (programación lineal)
- [ ] Interior point methods
- [ ] Lagrange multipliers
- [ ] Sequential Quadratic Programming (SQP)

**Optimización combinatoria:**
- [ ] Genetic algorithms
- [ ] Particle swarm optimization
- [ ] Ant colony optimization

---

## 🔮 Futuro (v0.7+)

### Phase 8: Estadística y Probabilidad
- [ ] Distribuciones (normal, binomial, Poisson, t, chi-cuadrado, F)
- [ ] Tests estadísticos (t-test, ANOVA, chi-cuadrado)
- [ ] Regresión (lineal, múltiple, polinomial, logística)
- [ ] Correlación y covarianza
- [ ] Análisis de series temporales (ACF, PACF, ARIMA)
- [ ] Bootstrap y métodos de Monte Carlo

### Phase 9: EDOs y EDPs
**Ecuaciones diferenciales ordinarias:**
- [ ] Euler, Runge-Kutta (RK4, RK45)
- [ ] Métodos multipaso (Adams-Bashforth)
- [ ] Stiff solvers (BDF)

**Ecuaciones diferenciales parciales:**
- [ ] Método de diferencias finitas
- [ ] Elementos finitos (FEM)
- [ ] Método espectral

### Phase 10: Cálculo Simbólico
**Álgebra simbólica:**
- [ ] Simplificación de expresiones
- [ ] Expansión y factorización
- [ ] Sustitución y evaluación simbólica

**Cálculo simbólico:**
- [ ] Derivación simbólica (regla de cadena, producto, cociente)
- [ ] Integración simbólica (tablas, sustitución)
- [ ] Límites y series de Taylor

**Ecuaciones simbólicas:**
- [ ] Solución de ecuaciones algebraicas
- [ ] Sistemas de ecuaciones simbólicas

### Phase 11: Procesamiento Avanzado de Señales
**Análisis tiempo-frecuencia:**
- [ ] Short-Time Fourier Transform (STFT)
- [ ] Wavelets (Haar, Daubechies, Morlet)
- [ ] Spectrogram y Mel-spectrogram

**Filtros digitales:**
- [ ] IIR filters (Butterworth, Chebyshev, Elliptic)
- [ ] FIR filter design (windowing, Parks-McClellan)
- [ ] Adaptive filters (LMS, RLS)

**Procesamiento de imágenes:**
- [ ] 2D FFT
- [ ] Convolución 2D
- [ ] Filtros (Gaussian blur, Sobel, Laplacian)

### Phase 12: Machine Learning Básico
- [ ] Redes neuronales (feedforward, backpropagation)
- [ ] k-means clustering
- [ ] PCA (Principal Component Analysis)
- [ ] k-NN, Decision Trees
- [ ] Support Vector Machines

---

## 🌟 Visión de Ecosistema

### @achronyme/core (actual)
Núcleo de cálculo matemático con WebAssembly

### @achronyme/language (futuro)
Procesamiento de lenguaje natural matemático

```javascript
import { AchronymeNLP } from '@achronyme/language';

const nlp = new AchronymeNLP();

nlp.parse("solve x squared plus 5x minus 6 equals 0")
// → "solve(x^2 + 5*x - 6 = 0)"

nlp.parse("integrate x squared from 0 to 10")
// → "integrate(x^2, x, 0, 10)"
```

### @achronyme/plot (futuro)
Visualización matemática

```javascript
import { AchronymePlot } from '@achronyme/plot';

plot.func('x^2', {range: [-10, 10]});
plot.scatter(data);
plot.surface('x^2 + y^2', {x: [-5,5], y: [-5,5]});
```

### @achronyme/cas (futuro)
Sistema de álgebra computacional completo

```javascript
import { AchronymeCAS } from '@achronyme/cas';

cas.simplify('(x+1)^2');           // → x^2 + 2*x + 1
cas.expand('(a+b)*(c+d)');         // → a*c + a*d + b*c + b*d
cas.solve('x^2 + 5*x - 6 = 0');    // → [x = 1, x = -6]
```

---

## 🤝 Cómo Contribuir

Achronyme es open-source y buscamos colaboradores en:
- **C++ developers**: Implementar algoritmos numéricos core
- **TypeScript developers**: SDK, testing, ejemplos
- **Math experts**: Validación de algoritmos, precisión numérica
- **DSP engineers**: Optimización de FFT, nuevos filtros
- **Documentation**: Tutoriales, ejemplos, traducciones
- **Testing**: Benchmarks, validación contra NumPy/MATLAB/Wolfram

**Repositorio**: https://github.com/eddndev/achronyme-core
**Discusiones**: https://github.com/eddndev/achronyme-core/discussions

---

## 🎯 Objetivo Realista

Convertirse en la alternativa open-source líder para cálculo numérico, DSP y álgebra lineal en los próximos 2-3 años. Competir con Wolfram en cálculo simbólico es un objetivo a largo plazo (5-10 años) que requiere una comunidad activa.

---

## 📚 Referencias

- [Comparación con Wolfram](./wolfram-comparison.md)
- [Especificación del Lenguaje](./language-spec.md)
- [Guía del SDK](./sdk-guide.md)
- [README Principal](../README.md)

---

**Versión**: 0.3.0
**Última actualización**: 2025

---

## Propuesta de Sintaxis para Grafos (Futuro)

*Esta es una propuesta para una futura implementación de una sintaxis de grafos en el lenguaje SOC, diseñada para ser extensible y soportar múltiples algoritmos (PERT, Dijkstra, etc.).*

### Principio de Diseño

La sintaxis debe separar la **topología** del grafo (su estructura de nodos y aristas) de los **datos** asociados a un problema específico (pesos, tiempos, costos), permitiendo máxima flexibilidad.

### Sintaxis General Propuesta

Se introduce un nuevo literal `network` y un operador de arista `->`.

```soc
let mi_red = network {
    // Opcional: Definición de nodos y sus propiedades
    nodes: {
        "ID_Nodo_1": { prop1: valor1, ... },
        "ID_Nodo_2": { prop2: valor2, ... }
    },

    // Opcional: Lista de aristas y sus propiedades
    edges: [
        "ID_Nodo_1" -> "ID_Nodo_2" { prop_arista: valor, ... },
        ...
    ]
}
```

### Ejemplos de Casos de Uso

#### 1. Grafo Simple (Topología Pura)
Para algoritmos de conectividad, recorridos (BFS, DFS), etc.

```soc
let grafo_simple = network {
    // Los nodos se pueden inferir de las aristas
    edges: [
        "A" -> "B",
        "B" -> "C",
        "A" -> "C"
    ]
}
```

#### 2. Grafo con Pesos en Aristas (Para Dijkstra, Kruskal)
Se añaden propiedades a las aristas.

```soc
let mapa_distancias = network {
    edges: [
        "Madrid"   -> "Zaragoza"  { distancia: 325 },
        "Zaragoza" -> "Barcelona" { distancia: 290 },
        "Madrid"   -> "Valencia"  { distancia: 360 }
    ]
}

// Uso: dijkstra(mapa_distancias, "Madrid", "Barcelona", { weight: "distancia" })
```

#### 3. Grafo con Propiedades en Nodos (Para PERT)
Se añaden propiedades a los nodos.

```soc
let proyecto_pert = network {
    nodes: {
        "Diseño":     { to: 3, tm: 5, tp: 10 },
        "Backend":    { to: 7, tm: 10, tp: 15 },
        "Frontend":   { to: 6, tm: 8,  tp: 12 }
    },
    edges: [
        "Diseño" -> "Backend",
        "Diseño" -> "Frontend"
    ]
}

// Uso: find_critical_path(proyecto_pert)
```

### Beneficios de la Propuesta

- **Consistente:** Sigue el estilo declarativo del lenguaje SOC.
- **Flexible:** Permite definir topología pura, datos en nodos, datos en aristas, o una combinación.
- **Extensible:** Los nuevos algoritmos pueden simplemente buscar las propiedades que necesitan en los `records` de los nodos o aristas, sin requerir cambios en la sintaxis.
- **Legible:** La estructura del grafo y sus datos son fáciles de entender de un vistazo.
