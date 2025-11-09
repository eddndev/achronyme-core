# Recomendaciones de Diseño y Evolución del Lenguaje

Este documento resume una serie de recomendaciones para extender y mejorar el lenguaje de cálculo científico Achronyme, enfocándose en la manipulación de datos, la eficiencia y la ergonomía del programador.

## Plan de Evolución Sugerido

Se propone un plan de cuatro pasos que construye una base sólida para el futuro del lenguaje, yendo desde cambios en la arquitectura interna hasta la adición de nuevas funcionalidades de cara al usuario.

### Paso 1: Unificar Tipos de Datos a un `Tensor` Interno

El paso más estratégico es refactorizar las estructuras de datos internas. En lugar de tener tipos separados para `Vector` y `Matrix`, se deben unificar en una sola estructura `Tensor`.

Un `Tensor` internamente tendría:
- `data: Vec<T>`: Un vector plano con los datos (donde `T` podría ser `f64`, `Complex`, etc.).
- `shape: Vec<usize>`: Un vector que define las dimensiones del tensor.
- `strides: Vec<usize>`: (Opcional, para vistas y *slicing* eficientes) Define cómo saltar en el vector `data` para moverte a lo largo de cada dimensión.

**Beneficios:**
- **Código Unificado:** Todas las operaciones (`+`, `*`, `sin`, etc.) se escriben una sola vez para `Tensor`, sin importar si es un escalar (rango 0), vector (rango 1), matriz (rango 2) o de mayor dimensionalidad.
- **Eficiencia:** Las operaciones sobre el vector plano `data` son muy rápidas y amigables con el caché de la CPU.
- **Preparado para el Futuro:** Añadir soporte para GPU (con librerías como `wgpu`) sería más sencillo partiendo de esta estructura.

### Paso 2: Extender el Broadcasting (Reglas Claras)

Con la estructura `Tensor` unificada, el siguiente paso es definir reglas claras de broadcasting para operaciones entre tensores de distintas formas, similar a como funciona en NumPy.

**Reglas de Comportamiento Sugeridas:**
Para una operación `T1 op T2`, el broadcasting funciona si, para cada dimensión (empezando por la última y avanzando hacia la primera):
1. Las dimensiones de ambos tensores son iguales, o
2. Una de las dimensiones es 1.

El tensor con dimensión 1 se "estira" virtualmente para igualar la dimensión del otro tensor.

**Ejemplo Práctico:**
```
// Matriz (shape: [2, 3])
let m = [[1, 2, 3], [4, 5, 6]] 

// Vector (shape: [3])
let v = [10, 20, 30]

// Broadcasting m + v (shape [2,3] + [3])
// El evaluador alinea las formas por la derecha:
//   m.shape: [2, 3]
//   v.shape:    [3]
// Las dimensiones son compatibles. El vector 'v' se suma a CADA fila de 'm'.
let result = m + v 
// >> [[11, 22, 33], [14, 25, 36]]
```
Este comportamiento es increíblemente potente y es el estándar de facto en lenguajes de este tipo. No requiere cambios de gramática, solo lógica en el evaluador.

### Paso 3: Implementar Indexación y "Slicing" (Cambio de Gramática)

Para que los tensores sean verdaderamente útiles, se necesita una forma ergonómica de acceder a sus elementos y sub-secciones (*slicing*). La notación de corchetes `[]` es el estándar.

**Sugerencia de Sintaxis:**
```
let t = zeros([3, 4, 5]) // Un tensor 3x4x5

// Acceso a un elemento
let elem = t[0, 2, 1]

// Slicing para obtener sub-tensores
let sub_matrix = t[0, .., ..] // Obtiene la primera matriz 4x5
let sub_vector = t[0, 1, ..]  // Obtiene la segunda fila de la primera matriz
```

**Cambios Sugeridos en `grammar.pest`:**
Se deben introducir dos conceptos: el acceso por índice y las expresiones de rango.

```pest
// 1. Añadir una expresión de rango
range_expr = { expr? ~ ".." ~ expr? }

// 2. Un argumento de acceso puede ser un número o un rango
access_arg = { range_expr | expr }

// 3. Añadir la regla de acceso después de 'primary'
// Esto permite encadenar accesos: tensor[0][1]
access = {
    primary ~ ("[" ~ access_arg ~ ("," ~ access_arg)* ~ "]")*
}

// 4. Actualizar la jerarquía para usar 'access'
// La regla 'field_access' se modificaría para usar 'access' como base.
field_access = {
    access ~ ("." ~ identifier)*
}
```
Esta es la mejora de sintaxis más importante para el manejo de tensores.

### Paso 4: Añadir el Operador de Tubería (`|>`)

Este es un cambio ergonómico que mejora drásticamente la legibilidad de las cadenas de procesamiento de datos, encajando perfectamente en un flujo de trabajo basado en la transformación de tensores.

**Cambio Sugerido en `grammar.pest`:**
```pest
// Añadir el operador con una precedencia baja
pipe = { logical_or ~ ("|>" ~ (function_call | identifier))* }

// La expresión principal del lenguaje ahora es 'pipe'
expr = { pipe }
```

**Ejemplo de Uso:**
```
// Sin tubería (difícil de leer)
let result = sum(filter(map(data, x => x*2), x => x > 10))

// Con tubería (claro y secuencial)
let result = data |> map(x => x*2) |> filter(x => x > 10) |> sum
```
En el evaluador, se implementa como un bucle que toma el resultado de la izquierda y lo alimenta como primer argumento a la función de la derecha.
