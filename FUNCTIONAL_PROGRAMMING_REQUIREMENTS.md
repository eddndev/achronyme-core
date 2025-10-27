# Requisitos de Programación Funcional - Phase 4A

## 🎯 Objetivo: Entorno de Programación Funcional Completo

Achronyme Core debe soportar programación funcional al estilo de:
- **Wolfram Language** (Mathematica)
- **Python** (con lambdas)
- **JavaScript** (funciones first-class)
- **MATLAB** (function handles)

## ❌ Problemas Actuales (Sin Persistencia)

### 1. No se pueden guardar funciones en variables

**Wolfram Language:**
```mathematica
square[x_] := x^2
square[5]                    (* 25 *)

f = Function[x, x + 10]
f[3]                         (* 13 *)
```

**Achronyme (Esperado):**
```javascript
let square = x => x^2
square(5)                    // DEBERÍA: 25
```

**Achronyme (Actual):**
```javascript
let square = x => x^2        // ✓ Define
square(5)                    // ❌ Error: square no existe
```

**Causa**: Nueva sesión de evaluación = variables desaparecen

---

### 2. No se pueden reutilizar funciones en HOFs

**Wolfram Language:**
```mathematica
double[x_] := 2*x
Map[double, {1, 2, 3}]      (* {2, 4, 6} *)
```

**Python:**
```python
double = lambda x: 2*x
list(map(double, [1, 2, 3])) # [2, 4, 6]
```

**Achronyme (Esperado):**
```javascript
let double = x => x * 2
map(double, [1, 2, 3])       // DEBERÍA: [2, 4, 6]
```

**Achronyme (Actual):**
```javascript
let double = x => x * 2      // ✓ Define
map(double, [1, 2, 3])       // ❌ Error: double no existe

// Workaround (incómodo):
map(x => x * 2, [1, 2, 3])   // ✓ Funciona inline
```

---

### 3. No funcionan los closures

**Wolfram Language:**
```mathematica
x = 5
adder[y_] := x + y
adder[3]                     (* 8 *)

x = 10
adder[3]                     (* 13 - closure ve nuevo x *)
```

**Python:**
```python
x = 5
adder = lambda y: x + y
adder(3)                     # 8

x = 10
adder(3)                     # 13
```

**Achronyme (Esperado):**
```javascript
let x = 5
let adder = y => x + y
adder(3)                     // DEBERÍA: 8
```

**Achronyme (Actual):**
```javascript
let x = 5                    // ✓ Define x
let adder = y => x + y       // ✓ Define adder (captura x en closure)
adder(3)                     // ❌ Error: adder no existe
```

---

### 4. No se puede construir biblioteca de funciones

**Wolfram Language:**
```mathematica
(* Biblioteca de funciones *)
square[x_] := x^2
cube[x_] := x^3
sumOfSquares[a_, b_] := square[a] + square[b]

sumOfSquares[3, 4]           (* 25 *)
```

**Achronyme (Esperado):**
```javascript
// Biblioteca DSP
let lowpass = signal => convolve(signal, [0.2, 0.2, 0.2, 0.2, 0.2])
let highpass = signal => convolve(signal, [-0.25, -0.25, 1, -0.25, -0.25])
let normalize = v => map(x => x / norm(v), v)

// Usar:
let signal = [1, 2, 3, 4, 5]
let filtered = lowpass(signal)
let normal = normalize(filtered)
```

**Achronyme (Actual):**
```javascript
let lowpass = ...            // ✓ Define
let signal = [1, 2, 3, 4, 5] // ❌ lowpass ya desapareció
```

---

### 5. No se puede hacer composición de funciones guardadas

**Wolfram Language:**
```mathematica
f[x_] := x + 1
g[x_] := x * 2
h = Composition[f, g]       (* h[x] = f[g[x]] *)
h[3]                         (* 7 *)
```

**Achronyme (Esperado):**
```javascript
let f = x => x + 1
let g = x => x * 2
// Queremos: compose(f, g) pero compose devolvería función
// Por ahora: pipe
let data = [1, 2, 3]
pipe(data, v => map(g, v), v => map(f, v))
```

**Achronyme (Actual):**
```javascript
let f = x => x + 1           // ✓ Define
let g = x => x * 2           // ❌ f ya desapareció
```

---

## ✅ Lo que SÍ Funciona Actualmente

### Lambdas inline
```javascript
map(x => x^2, [1, 2, 3])                    // ✓ Funciona
filter(x => x > 0, [-1, 0, 1, 2])           // ✓ Funciona
reduce((a, b) => a + b, 0, [1, 2, 3])       // ✓ Funciona
```

### Expresiones compuestas en una línea
```javascript
let square = x => x^2; map(square, [1,2,3]) // ✓ Funciona (misma eval)
```

### Pipe con lambdas inline
```javascript
pipe([1,2,3],
     v => map(x => x^2, v),
     v => reduce((a,b) => a+b, 0, v))       // ✓ Funciona
```

---

## 🔴 Por Qué es Crítico Resolver Esto

### 1. Programación Funcional Incompleta
Sin persistencia de variables:
- ❌ No es un lenguaje funcional real
- ❌ No puedes definir funciones reutilizables
- ❌ No puedes hacer abstracción
- ❌ No puedes construir bibliotecas

### 2. Incompatible con Objetivos DSP
Para DSP necesitas definir filtros, ventanas, transformaciones:
```javascript
// Esto DEBE funcionar:
let hanning = n => /* implementación */
let applyWindow = (signal, window) => signal * window(length(signal))
let analyze = signal => fft(applyWindow(signal, hanning))

let mySignal = [1, 2, 3, 4, 5, 6, 7, 8]
analyze(mySignal)
```

### 3. No Compatible con Notebooks/REPL
Los notebooks científicos (Jupyter, Mathematica, MATLAB) esperan:
```
Cell 1:  f = x => x^2
Cell 2:  map(f, [1, 2, 3])      # ✓ Debe funcionar
```

### 4. Inconsistente con el Plan Original
Del `PHASE4_PLAN.md` líneas 128-139:
```javascript
let x = 5
let y = 10
let adder = y => x + y
adder(3)              // → 8
```

**Esto es un requisito explícito de Phase 4A**, no un extra.

---

## 💡 Solución: Evaluador Persistente

### Implementación Mínima

**Cambio en `main.cpp` (1 línea):**
```cpp
// Antes (stateless):
parser::Evaluator evaluator;  // Local, se destruye

// Después (stateful):
static parser::Evaluator evaluator;  // Global, persiste
```

### Casos de Uso Habilitados

#### 1. Biblioteca de Funciones DSP
```javascript
// Definir biblioteca
Module.eval('let hanning = n => /* window */')
Module.eval('let hamming = n => /* window */')
Module.eval('let blackman = n => /* window */')

// Usar en cualquier momento
Module.eval('let signal = [1,2,3,4,5,6,7,8]')
Module.eval('let windowed = signal * hanning(8)')
Module.eval('fft(windowed)')
```

#### 2. Procesamiento de Audio
```javascript
// Definir pipeline
Module.eval('let preprocess = s => normalize(highpass(s))')
Module.eval('let analyze = s => fft_mag(preprocess(s))')
Module.eval('let findPeak = spectrum => reduce((a,b) => max(a,b), 0, spectrum)')

// Procesar
Module.eval('let audio = loadAudio("test.wav")')
Module.eval('let peak = findPeak(analyze(audio))')
```

#### 3. Algebra Lineal
```javascript
// Operaciones
Module.eval('let matmul = (A, B) => A * B')
Module.eval('let solve = (A, b) => inverse(A) * b')

// Usar
Module.eval('let A = [[1, 2], [3, 4]]')
Module.eval('let b = [5, 6]')
Module.eval('solve(A, b)')
```

#### 4. Machine Learning (futuro)
```javascript
// Definir funciones
Module.eval('let sigmoid = x => 1 / (1 + exp(-x))')
Module.eval('let relu = x => max(0, x)')
Module.eval('let softmax = v => map(x => exp(x), v) / sum(map(x => exp(x), v))')

// Usar
Module.eval('let logits = [2.0, 1.0, 0.1]')
Module.eval('softmax(logits)')
```

---

## 📊 Comparación: Antes vs Después

### Antes (Stateless - Actual)

```javascript
// ❌ No funciona:
Module.eval('let square = x => x^2')
Module.eval('square(5)')  // Error

// ✓ Workaround (incómodo):
Module.eval('let square = x => x^2; square(5)')  // Funciona

// ❌ Biblioteca imposible:
Module.eval('let f1 = ...')
Module.eval('let f2 = ...')
Module.eval('f1(f2(5))')  // Error: f1, f2 no existen
```

### Después (Stateful - Con evaluador global)

```javascript
// ✅ Funciona naturalmente:
Module.eval('let square = x => x^2')
Module.eval('square(5)')  // 25 ✓

// ✅ Biblioteca completa:
Module.eval('let f1 = x => x + 1')
Module.eval('let f2 = x => x * 2')
Module.eval('let f3 = x => x^2')
Module.eval('f3(f2(f1(5)))')  // ((5+1)*2)^2 = 144 ✓

// ✅ Closures:
Module.eval('let x = 10')
Module.eval('let adder = y => x + y')
Module.eval('adder(5)')  // 15 ✓

// ✅ Reset cuando necesites:
Module.reset()  // Limpia todo
```

---

## 🎯 Decisión

**La persistencia de variables NO es opcional** para Phase 4A.

Es un **requisito fundamental** para:
1. ✅ Programación funcional completa
2. ✅ Bibliotecas de funciones DSP
3. ✅ Compatibilidad con REPL/Notebooks
4. ✅ Cumplir objetivos del plan original

**Recomendación**: Implementar evaluador global **inmediatamente**.

Sin esto, Phase 4A está **incompleto** aunque los tests pasen.
