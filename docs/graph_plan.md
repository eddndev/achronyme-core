# Plan de Implementación: Grafos y Redes en Achronyme

**Fecha:** 2025-01-07
**Versión:** 3.1
**Estado:** Diseño Final Aprobado - Identificadores Puros + Sintaxis `:` + Sin Campo `id`

---

## 0. Diseño Final v3.0 - Resumen Rápido ⭐

### **Sintaxis Elegante y Consistente**

```soc
// 1. Preparar datos de nodos (opcional)
let madrid = { lat: 40.4168, lon: -3.7038 }
let barcelona = { lat: 41.3851, lon: 2.1734 }

// 2. Construir red
let mapa = {
    nodes: {
        Madrid: madrid,      // Clave 'Madrid' (ID), valor madrid (datos)
        Barcelona: barcelona
    },
    edges: [
        Madrid -> Barcelona: { km: 620, tiempo_h: 6.5 }
        //  ↑      ↑       ↑
        // IDs puros    Metadata (evaluada)
        // (no evaluar)
    ]
}

// 3. Acceso directo
mapa.nodes.Madrid.lat     // 40.4168
mapa.edges[0].km          // 620 (cuando tengamos indexing)
```

### **Principios Clave:**

| Aspecto | Comportamiento |
|---------|----------------|
| **Identificadores en edges** | `A -> B` → IDs "A" y "B" (NUNCA evalúan variables) |
| **Separador `:`** | `A -> B: metadata` → metadata SÍ se evalúa |
| **Nodos como Record** | `nodes: { A: {...}, B: {...} }` → field access directo |
| **Type promotion** | Record → Network si contiene Edges |
| **Sin strings** | Solo identificadores válidos (no `"New York"`) |

### **Ventajas del Diseño:**

✅ **100% consistente con records:** Identificadores = claves, nunca se evalúan
✅ **Sin ambigüedad:** Separación clara entre IDs y metadata
✅ **Máxima simplicidad:** Sintaxis limpia y predecible
✅ **Field access natural:** `red.nodes.A.color` funciona perfectamente
✅ **Construcción dinámica:** Metadata puede ser variable o expresión

---

## 1. Resumen Ejecutivo

Este documento define el diseño completo para el soporte de grafos/redes en el lenguaje SOC de Achronyme. El diseño sigue los principios de **minimalismo sintáctico**, **type promotion automática**, y **consistencia con tipos existentes** (especialmente Records).

### Objetivos:
- ✅ Sintaxis declarativa simple y **100% consistente con records**
- ✅ **Identificadores puros en edges** (NUNCA evaluados como variables)
- ✅ **Separador `:` para metadata** de edges (evaluada)
- ✅ **Nodos como Record de Records** (field access directo O(1))
- ✅ Soporte para grafos dirigidos (`->`) y no dirigidos (`--`)
- ✅ Type promotion automática (Record → Network cuando contiene Edges)
- ✅ Funciones built-in para consultas y algoritmos básicos
- ✅ Extensibilidad para algoritmos futuros (BFS, DFS, PERT, etc.)

### Principios de Diseño Clave:
- **Consistencia total:** Edges usan identificadores como records usan claves
- **Sin ambigüedad:** Identificadores antes de `->` nunca se evalúan
- **Evaluación explícita:** Metadata después de `:` siempre se evalúa
- **Minimalismo:** Sintaxis limpia sin verbosidad innecesaria

---

## 2. Sistema de Tipos

### 2.1. Jerarquía de Tipos

```
Value (enum)
├── Number(f64)
├── Boolean(bool)
├── String(String)
├── Complex(Complex)
├── Vector(Vector)
├── Matrix(Matrix)
├── Record(HashMap<String, Value>)
├── Function(Function)
├── Edge { ... }          ← NUEVO (tipo primitivo)
└── Network { ... }        ← NUEVO (tipo contenedor)
```

### 2.2. Definición de Edge (Tipo Primitivo)

**Edge es un tipo de primera clase**, como Number o String.

```rust
pub enum Value {
    // ... tipos existentes ...

    Edge {
        from: String,                      // ID del nodo origen
        to: String,                        // ID del nodo destino
        directed: bool,                    // true = ->, false = --
        properties: HashMap<String, Value>, // Propiedades opcionales
    },
}
```

**Sintaxis:**
```soc
// Con IDENTIFICADORES PUROS (única forma permitida)
A -> B                      // Edge dirigido simple (sin metadata)
A -> B: { peso: 5 }         // Edge dirigido con metadata
A -- C                      // Edge no dirigido simple
A -- C: { distancia: 10 }   // Edge no dirigido con metadata

// Metadata puede ser variable o record literal
let meta = { peso: 10, color: "red" }
A -> B: meta                // Variable evaluada
A -> C: { tipo: "directo" } // Record literal
```

**Características:**
- ✅ Los operadores `->` (dirigido) y `--` (no dirigido) crean valores `Value::Edge`
- ✅ **SOLO identificadores permitidos** (no strings, no números, no expresiones)
- ✅ **Identificadores NUNCA se evalúan** como variables (son IDs puros de nodo)
- ✅ **Metadata después de `:` SÍ se evalúa** (puede ser variable o record literal)
- ✅ Soporta field access: `e.from`, `e.to`, `e.directed`, `e.peso`

---

### 2.3. Definición de Network (Tipo Contenedor)

**Network es un contenedor** que agrupa nodos, aristas y metadata.

```rust
pub enum Value {
    // ... tipos existentes ...

    Network {
        // HashMap: ID → Record de propiedades del nodo
        // SIEMPRE tiene campo 'id', puede tener propiedades adicionales
        nodes: HashMap<String, HashMap<String, Value>>,

        // Vector de Edges (solo Value::Edge permitido)
        edges: Vec<Value>,

        // Metadata opcional (campos extra del record literal)
        metadata: HashMap<String, Value>,
    },
}
```

**Características:**
- ✅ Todos los nodos son internamente Records con campo `id` obligatorio
- ✅ Los edges son valores de tipo `Value::Edge`
- ✅ Metadata almacena campos del record literal que no son `nodes` ni `edges`

---

## 3. Type Promotion y Conversiones

### 3.1. Nodos como Record de Records

**Los nodos se declaran como un Record donde:**
- **Clave:** Identificador del nodo (es el ID)
- **Valor:** Record con propiedades (SIN campo `id`)

```soc
// Usuario escribe:
let red = {
    nodes: {
        A: { peso: 1, color: "red" },
        B: { peso: 2, color: "blue" },
        C: { peso: 3, color: "green" }
    },
    edges: [A -> B, B -> C]
}

// Se almacena EXACTAMENTE así (sin campo 'id'):
// nodes: {
//     "A": { peso: 1, color: "red" },
//     "B": { peso: 2, color: "blue" },
//     "C": { peso: 3, color: "green" }
// }
```

**Reglas:**
- ✅ **NO hay campo `id`** en los records de nodos
- ✅ La **clave del HashMap ES el ID** del nodo
- ✅ **No hay validación** de `id` (no puede haber inconsistencia)
- ✅ **Acceso por clave:** El ID se conoce por la clave usada en el acceso

```soc
// ✅ Sintaxis limpia (sin duplicación)
nodes: {
    A: { x: 10, color: "red" }
}

// Acceso:
red.nodes.A         // → { x: 10, color: "red" }
red.nodes.A.color   // → "red"
// El ID es "A" (se conoce por la clave de acceso)
```

---

### 3.2. Record → Network (Type Promotion)

Un `Value::Record` se promociona automáticamente a `Value::Network` **SOLO si contiene Edges**.

**La presencia de Edges es lo que define una Network**, no la presencia de un campo `nodes`.

```soc
// ✅ ES Network (contiene edges)
let red1 = {
    edges: [A -> B, B -> C]
}

// ✅ ES Network (contiene edges)
let red2 = {
    nodes: { A: {}, B: {} },
    edges: [A -> B]
}

// ✅ ES Network (campo con edge)
let red3 = {
    mi_conexion: A -> B
}

// ❌ NO ES Network (solo Record, no hay edges)
let record1 = {
    nodes: { A: {}, B: {} }  // Solo un record con campo 'nodes'
}

// ❌ NO ES Network (Vector de Strings, no Edges)
let record2 = {
    nombres: ["A", "B", "C"]  // Solo un record normal
}
```

**Algoritmo de detección:**
```rust
fn contains_edges(value: &Value) -> bool {
    match value {
        Value::Edge { .. } => true,
        // Nota: Vector ahora puede contener Edges
        // (requiere extender Vector para soportar Value genérico)
        _ => false,
    }
}

// Para records, verificar si algún valor es Edge
fn record_has_edges(record: &HashMap<String, Value>) -> bool {
    record.values().any(|v| contains_edges(v))
}
```

---

### 3.3. Inferencia de Nodos desde Edges

Si no se declaran nodos explícitamente, se infieren automáticamente desde los edges usando los **identificadores**.

```soc
// Usuario escribe (identificadores):
let red = {
    edges: [A -> B, B -> C]
}

// Nodos inferidos (identificador → record vacío):
nodes: {
    "A": {},
    "B": {},
    "C": {}
}
```

**Si nodos están declarados parcialmente:**
```soc
let red = {
    nodes: {
        A: { color: "red", x: 0 }  // 'A' declarado con propiedades
    },
    edges: [A -> B, B -> C]
}

// Resultado:
nodes: {
    "A": { color: "red", x: 0 },  // ← Declarado explícitamente
    "B": {},                       // ← Inferido de edge (vacío)
    "C": {}                        // ← Inferido de edge (vacío)
}
```

---

## 4. Sintaxis Declarativa

### 4.1. Formas de Crear Networks

#### **Forma A: Minimalista (Solo Edges)**
```soc
// Nodos inferidos automáticamente de identificadores
let red = {
    edges: [
        A -> B,
        B -> C,
        C -> A
    ]
}
```

#### **Forma B: Con Metadata en Edges**
```soc
let red = {
    edges: [
        A -> B: { peso: 5, tipo: "directo" },
        B -> C: { peso: 3, tipo: "indirecto" },
        C -> A: { peso: 2, tipo: "directo" }
    ]
}
```

#### **Forma C: Con Nodos Explícitos (Record de Records)**
```soc
let red = {
    nodes: {
        A: { x: 0, y: 0, color: "red" },
        B: { x: 10, y: 5, color: "blue" },
        C: { x: 5, y: 10, color: "green" }
    },
    edges: [
        A -> B: { peso: 5 },
        B -> C: { peso: 3 },
        C -> A: { peso: 2 }
    ]
}

// Acceso directo a nodos:
red.nodes.A         // → { x: 0, y: 0, color: "red" }
red.nodes.A.color   // → "red"
```

#### **Forma D: Patrón con Variables del Mismo Nombre**
```soc
// Preparar datos de nodos (patrón común)
let A = { color: "red", peso: 1 }
let B = { color: "blue", peso: 2 }
let C = { color: "green", peso: 3 }

let red = {
    nodes: {
        A: A,  // ← Clave 'A' (ID), valor 'A' (variable con datos)
        B: B,
        C: C
    },
    edges: [
        A -> B: { fuerza: 5 },  // ← 'A' y 'B' son IDs (no se evalúan)
        B -> C: { fuerza: 3 }
    ]
}
```

#### **Forma E: Con Metadata del Grafo**
```soc
let red = {
    autor: "Alice",
    version: 2,
    fecha: "2025-01-07",
    nodes: {
        A: { peso: 1 },
        B: { peso: 2 }
    },
    edges: [A -> B: { relacion: "conecta" }]
}
// metadata: { autor: "Alice", version: 2, fecha: "2025-01-07" }
```

---

### 4.2. Operadores de Edges y Sintaxis con `:`

#### **Operador `->` (Dirigido)**
```soc
// Edge simple (sin metadata)
A -> B

// Edge con metadata (usando ':')
A -> B: { peso: 5 }
Madrid -> Barcelona: { km: 620, tiempo_h: 6.5 }
```

#### **Operador `--` (No Dirigido)**
```soc
// Edge simple
A -- B

// Edge con metadata
A -- B: { distancia: 10 }
Alice -- Bob: { relacion: "amigos", desde: 2020 }
```

#### **Metadata: Variable o Record Literal**
```soc
// Record literal
A -> B: { peso: 5, color: "red" }

// Variable (se evalúa)
let meta_carretera = { tipo: "carretera", peajes: 2 }
Madrid -> Barcelona: meta_carretera

// Expresión
A -> B: { peso: calcular_peso(), urgente: true }
```

#### **Reglas Importantes**

✅ **PERMITIDO:**
```soc
A -> B                // Identificadores simples
Madrid -> Barcelona   // CamelCase
node_1 -> node_2      // snake_case con números
Start -> End          // Nombres descriptivos
A -> B: meta          // Metadata como variable
```

❌ **NO PERMITIDO:**
```soc
// Números puros
1 -> 2  // ❌ Error: número no es identificador

// Strings (ni siquiera con espacios)
"New York" -> "Boston"  // ❌ Error: solo identificadores permitidos

// Variables como nodos (no se evalúan)
let x = "Madrid"
x -> B  // ❌ Crea nodo "x", NO "Madrid"
```

#### **Solución para Nombres con Espacios**
```soc
// Usar identificador corto + campo con nombre completo
let mapa = {
    nodes: {
        NY: { nombre: "New York", poblacion: 8000000 },
        LA: { nombre: "Los Angeles", poblacion: 4000000 }
    },
    edges: [
        NY -> LA: { km: 4500 }
    ]
}

// Acceso:
mapa.nodes.NY.nombre  // "New York"
// El ID es "NY" (por la clave), el nombre completo está en metadata
```

---

## 5. Field Access

### 5.1. Field Access en Edges

Los Edges soportan acceso a campos como Records.

```soc
let e = A -> B: { peso: 5, color: "red" }

// Campos especiales (built-in)
let origen = e.from       // "A" (String - ID del nodo)
let destino = e.to        // "B" (String - ID del nodo)
let dirigido = e.directed // true (Boolean)

// Propiedades de metadata
let p = e.peso            // 5 (Number)
let c = e.color           // "red" (String)
```

**Implementación:**
```rust
match value {
    Value::Edge { from, to, directed, properties } => {
        match field.as_str() {
            "from" => Some(Value::String(from.clone())),
            "to" => Some(Value::String(to.clone())),
            "directed" => Some(Value::Boolean(*directed)),
            _ => properties.get(field).cloned()
        }
    }
    // ...
}
```

---

### 5.2. Field Access en Networks

Los Networks soportan acceso a campos como Records, **incluyendo acceso directo a nodos individuales**.

```soc
let red = {
    autor: "Alice",
    nodes: {
        A: { color: "red", x: 0 },
        B: { color: "blue", x: 10 }
    },
    edges: [A -> B]
}

// Acceso directo a campos
let mis_edges = red.edges      // Vector de Edges
let mis_nodos = red.nodes      // Record de nodos
let autor = red.autor          // "Alice" (metadata)

// ✅ Acceso directo a nodo específico
let nodo_a = red.nodes.A        // { color: "red", x: 0 }
let color_a = red.nodes.A.color // "red"

// Acceso a edges individuales (requiere indexing - futuro)
// let primer_edge = red.edges[0]  // Edge: A -> B (no implementado aún)
```

---

## 6. Funciones Built-in

### 6.1. Funciones de Consulta Básica

#### **Información General**
```soc
nodes(network) → Vector<String>
// Retorna vector de IDs de nodos
// Ejemplo: nodes(red) → ["A", "B", "C"]

edges(network) → Vector<Edge>
// Retorna vector de todos los edges
// Ejemplo: edges(red) → ["A" -> "B", "B" -> "C"]

node_count(network) → Number
// Retorna cantidad de nodos
// Ejemplo: node_count(red) → 3

edge_count(network) → Number
// Retorna cantidad de edges
// Ejemplo: edge_count(red) → 2
```

#### **Consultas de Existencia**
```soc
has_node(network, id) → Boolean
// Verifica si existe un nodo
// Ejemplo: has_node(red, "A") → true

has_edge(network, from, to) → Boolean
// Verifica si existe un edge
// Ejemplo: has_edge(red, "A", "B") → true
```

#### **Acceso a Datos**
```soc
get_node(network, id) → Record
// Retorna propiedades de un nodo (sin campo 'id')
// Ejemplo: get_node(red, "A") → { color: "red", peso: 1 }

get_edge(network, from, to) → Edge
// Retorna edge específico
// Ejemplo: get_edge(red, "A", "B") → "A" -> "B" { peso: 5 }
```

---

### 6.2. Funciones de Análisis de Grafo

```soc
neighbors(network, node) → Vector<String>
// Retorna IDs de nodos adyacentes
// Ejemplo: neighbors(red, "B") → ["A", "C"]

degree(network, node) → Number
// Retorna grado del nodo (total de conexiones)
// Ejemplo: degree(red, "B") → 2

in_degree(network, node) → Number
// Retorna grado de entrada (solo grafos dirigidos)
// Ejemplo: in_degree(red, "B") → 1

out_degree(network, node) → Number
// Retorna grado de salida (solo grafos dirigidos)
// Ejemplo: out_degree(red, "A") → 1

is_connected(network) → Boolean
// Verifica si el grafo es conexo
// Ejemplo: is_connected(red) → true
```

---

### 6.3. Algoritmos (MVP: Dijkstra)

```soc
dijkstra(network, start, end, options) → Record
// Encuentra el camino más corto entre dos nodos

// Ejemplo básico:
let ruta = dijkstra(mapa, "Madrid", "Barcelona", { weight: "km" })
// → { path: ["Madrid", "Zaragoza", "Barcelona"], distance: 915 }

// Opciones:
// { weight: "km" }           → usa propiedad 'km' de edges
// { weight: x => x.tiempo }  → función custom para calcular peso
// { weight: null }           → peso 1 para todos los edges (BFS)

// Retorno:
// {
//     path: ["A", "B", "C"],      // Vector de nodos en orden
//     distance: 15,                // Distancia total
//     edges: [edge1, edge2]        // Edges del camino
// }
```

---

### 6.4. Funciones de Modificación (Futuro - Post-MVP)

**Nota:** Estas funciones retornan nuevas networks (inmutables).

```soc
add_edge(network, edge) → Network
// Agrega un edge a la red

add_node(network, node) → Network
// Agrega un nodo a la red

remove_edge(network, from, to) → Network
// Remueve un edge

remove_node(network, id) → Network
// Remueve un nodo y todos sus edges

// Constructor explícito (futuro)
network(edges) → Network
network(edges, nodes) → Network
network(edges, nodes, metadata) → Network
```

---

## 7. Casos de Uso y Ejemplos

### 7.1. Mapa de Rutas (Dijkstra)

```soc
let mapa_españa = {
    nodes: [
        { id: "Madrid", lat: 40.4168, lon: -3.7038, poblacion: 3223000 },
        { id: "Barcelona", lat: 41.3851, lon: 2.1734, poblacion: 1620000 },
        { id: "Valencia", lat: 39.4699, lon: -0.3763, poblacion: 791000 },
        { id: "Zaragoza", lat: 41.6488, lon: -0.8891, poblacion: 674000 }
    ],
    edges: [
        "Madrid" -> "Zaragoza" { km: 325, tiempo_h: 3.2, peajes: 2 },
        "Zaragoza" -> "Barcelona" { km: 290, tiempo_h: 2.8, peajes: 1 },
        "Madrid" -> "Valencia" { km: 350, tiempo_h: 3.5, peajes: 2 },
        "Barcelona" -> "Valencia" { km: 350, tiempo_h: 3.0, peajes: 1 }
    ]
}

// Ruta más corta por distancia
let ruta_corta = dijkstra(mapa_españa, "Madrid", "Barcelona", { weight: "km" })
// → { path: ["Madrid", "Zaragoza", "Barcelona"], distance: 615 }

// Ruta más rápida por tiempo
let ruta_rapida = dijkstra(mapa_españa, "Madrid", "Barcelona", { weight: "tiempo_h" })

// Consultas
let vecinos_madrid = neighbors(mapa_españa, "Madrid")  // ["Zaragoza", "Valencia"]
let datos_madrid = get_node(mapa_españa, "Madrid")    // { id: "Madrid", lat: ..., ... }
```

---

### 7.2. Red Social

```soc
let red_social = {
    nodes: [
        { id: "alice", nombre: "Alice", edad: 30, ciudad: "Madrid" },
        { id: "bob", nombre: "Bob", edad: 25, ciudad: "Barcelona" },
        { id: "carol", nombre: "Carol", edad: 28, ciudad: "Valencia" }
    ],
    edges: [
        "alice" -- "bob" { relacion: "amigos", desde: 2020 },
        "alice" -- "carol" { relacion: "familia", parentesco: "prima" },
        "bob" -- "carol" { relacion: "conocidos" }
    ]
}

// Amigos de Alice
let amigos_alice = neighbors(red_social, "alice")  // ["bob", "carol"]

// Grado de conexiones
let popularidad_alice = degree(red_social, "alice")  // 2

// Datos de un usuario
let datos_alice = get_node(red_social, "alice")
// → { nombre: "Alice", edad: 30, ciudad: "Madrid" }
```

---

### 7.3. PERT/CPM (Gestión de Proyectos)

```soc
let proyecto = {
    nodes: [
        { id: "Inicio", duracion: 0 },
        { id: "Diseño", duracion_opt: 3, duracion_prob: 5, duracion_pes: 10 },
        { id: "Backend", duracion_opt: 7, duracion_prob: 10, duracion_pes: 15 },
        { id: "Frontend", duracion_opt: 6, duracion_prob: 8, duracion_pes: 12 },
        { id: "Testing", duracion_opt: 2, duracion_prob: 3, duracion_pes: 5 },
        { id: "Fin", duracion: 0 }
    ],
    edges: [
        "Inicio" -> "Diseño",
        "Diseño" -> "Backend",
        "Diseño" -> "Frontend",
        "Backend" -> "Testing",
        "Frontend" -> "Testing",
        "Testing" -> "Fin"
    ]
}

// Algoritmos PERT (futuro)
// let ruta_critica = critical_path(proyecto)
// let holguras = slack_times(proyecto)
```

---

### 7.4. Grafo Simple (Topología Pura)

```soc
// Grafo minimalista (solo estructura)
let grafo = {
    edges: [
        "A" -> "B",
        "B" -> "C",
        "A" -> "C",
        "C" -> "D"
    ]
}

// Consultas básicas
let todos_nodos = nodes(grafo)        // ["A", "B", "C", "D"]
let vecinos_c = neighbors(grafo, "C") // ["B", "A", "D"]
let grado_c = degree(grafo, "C")      // 3
let conexo = is_connected(grafo)      // true
```

---

### 7.5. Grafo Mixto (Dirigido + No Dirigido)

```soc
let red_transporte = {
    edges: [
        "A" -> "B" { tipo: "tren", unidireccional: true },
        "B" -- "C" { tipo: "carretera", bidireccional: true },
        "C" -> "D" { tipo: "bus", unidireccional: true }
    ]
}

// Permitido: mezcla de edges dirigidos y no dirigidos
```

---

## 8. Reglas y Validaciones

### 8.1. Validaciones Obligatorias

✅ **Nodos deben tener campo `id`:**
```soc
// ❌ Error
{ nodes: [{ nombre: "Alice" }] }

// ✅ OK
{ nodes: [{ id: "A", nombre: "Alice" }] }
```

✅ **IDs de nodos deben ser únicos:**
```soc
// ❌ Error: "Duplicate node ID 'A'"
{ nodes: [{ id: "A" }, { id: "A" }] }
```

---

### 8.2. Comportamientos Permitidos

✅ **Auto-inferencia de nodos:**
```soc
// Nodos "A", "B", "C" se crean automáticamente
{ edges: ["A" -> "B", "B" -> "C"] }
```

✅ **Edges duplicados (multigrafo):**
```soc
// Permitido: múltiples edges entre mismos nodos
{
    edges: [
        "A" -> "B" { ruta: "norte", km: 100 },
        "A" -> "B" { ruta: "sur", km: 120 }
    ]
}
```

✅ **Self-loops:**
```soc
// Permitido: nodo apunta a sí mismo
{ edges: ["A" -> "A"] }
```

✅ **Grafos mixtos:**
```soc
// Permitido: mezcla de dirigidos y no dirigidos
{
    edges: [
        "A" -> "B",   // Dirigido
        "B" -- "C"    // No dirigido
    ]
}
```

---

## 9. Implementación Técnica

### 9.1. Gramática Pest

```pest
// Operadores de edges
edge_op_directed = { "->" }
edge_op_undirected = { "--" }
edge_op = { edge_op_directed | edge_op_undirected }

// Nodos de edge: SOLO identificadores (no strings, no expresiones)
edge_node = { identifier }

// Edge literal: node op node [":" metadata]
// Metadata es opcional y puede ser cualquier expresión que evalúe a Record
edge = {
    edge_node ~ edge_op ~ edge_node ~ (":" ~ expr)?
}

// Integración en la jerarquía de precedencia
// Los edges tienen precedencia baja (después de field_access, antes de comparaciones)

// Nivel 1: Field access (más alta)
field_access = {
    primary ~ ("." ~ identifier)*
}

// Nivel 2: Power
power = {
    field_access ~ ("^" ~ power)?
}

// Nivel 3: Unary
unary = {
    "-" ~ unary
  | "!" ~ unary
  | power
}

// Nivel 4: Multiplicative
multiplicative = {
    unary ~ (mult_op ~ unary)*
}

// Nivel 5: Additive
additive = {
    multiplicative ~ (add_op ~ multiplicative)*
}

// Nivel 6: EDGES (nueva precedencia)
edge_expr = {
    additive ~ (edge_op ~ additive ~ (":" ~ expr)?)?
}

// Nivel 7: Comparison
comparison = {
    edge_expr ~ (cmp_op ~ edge_expr)?
}

// Nivel 8+: Logical operators
logical_and = {
    comparison ~ (logical_and_op ~ comparison)*
}

logical_or = {
    logical_and ~ (logical_or_op ~ logical_and)*
}

// Expression entry point
expr = { logical_or }
```

**Notas importantes:**
- Edge tiene precedencia **baja** (después de aritmética, antes de comparaciones)
- Permite: `a + b -> c + d` evalúa como `(a+b) -> (c+d)`
- El `:` separa el edge de su metadata (que puede ser expresión compleja)

---

### 9.2. AST

```rust
pub enum AstNode {
    // ... existentes ...

    Edge {
        from: String,                    // ID del nodo (identificador, NO evaluar)
        to: String,                      // ID del nodo (identificador, NO evaluar)
        directed: bool,                  // true = ->, false = --
        metadata: Option<Box<AstNode>>,  // Expresión de metadata (evaluar)
    },
}
```

**Cambio clave:** `from` y `to` son **Strings directos** (no `Box<AstNode>`), porque los identificadores nunca se evalúan.

---

### 9.3. Evaluador

#### **Evaluación de Edge:**

```rust
fn evaluate_edge(
    &mut self,
    from: &str,              // Identificador directo (ya parseado)
    to: &str,                // Identificador directo (ya parseado)
    directed: bool,
    metadata_expr: &Option<Box<AstNode>>
) -> Result<Value, String> {
    // Los identificadores YA vienen como strings desde el parser
    // NO necesitamos verificar variables, porque nunca se evalúan
    let from_id = from.to_string();
    let to_id = to.to_string();

    // Evaluar metadata (SÍ se evalúa, puede ser variable o expresión)
    let properties = match metadata_expr {
        Some(expr) => {
            let value = self.evaluate(expr)?;
            match value {
                Value::Record(map) => map,
                _ => return Err(format!(
                    "Edge metadata must be a record, got {:?}",
                    value
                )),
            }
        },
        None => HashMap::new(),
    };

    Ok(Value::Edge {
        from: from_id,
        to: to_id,
        directed,
        properties,
    })
}
```

**Simplificación clave:** Como los identificadores ya vienen como `String` del AST (no como `AstNode`), el evaluador es mucho más simple. No hay verificación de variables porque el parser ya los convirtió a strings puros.

#### **Type Promotion: Record → Network:**
```rust
fn evaluate_record(
    &mut self,
    fields: &[(String, AstNode)]
) -> Result<Value, String> {
    let mut record = HashMap::new();
    let mut has_edges = false;

    for (key, value_node) in fields {
        let value = self.evaluate(value_node)?;

        // Detectar si contiene edges
        if contains_edges(&value) {
            has_edges = true;
        }

        record.insert(key.clone(), value);
    }

    if has_edges {
        // Promover a Network
        promote_to_network(record)
    } else {
        Ok(Value::Record(record))
    }
}

fn contains_edges(value: &Value) -> bool {
    match value {
        Value::Edge { .. } => true,
        Value::Vector(v) => v.iter().any(|item| contains_edges(item)),
        Value::Record(map) => map.values().any(|v| contains_edges(v)),
        _ => false,
    }
}

fn promote_to_network(record: HashMap<String, Value>) -> Result<Value, String> {
    let mut nodes = HashMap::new();
    let mut edges = Vec::new();
    let mut metadata = HashMap::new();

    for (key, value) in record {
        match key.as_str() {
            "nodes" => {
                // Procesar nodos (puede ser Vector de Strings o Records)
                nodes = extract_nodes(value)?;
            },
            "edges" => {
                // Procesar edges
                edges = extract_edges(value)?;
            },
            _ => {
                // Cualquier otro campo va a metadata
                metadata.insert(key, value);
            }
        }
    }

    // Inferir nodos desde edges si no están declarados
    if nodes.is_empty() {
        nodes = infer_nodes_from_edges(&edges);
    }

    Ok(Value::Network { nodes, edges, metadata })
}
```

---

### 9.4. Field Access en Edge

```rust
// En el handler de FieldAccess:
match record_value {
    Value::Record(ref map) => {
        map.get(field).cloned()
            .ok_or_else(|| format!("Field '{}' not found", field))
    },
    Value::Edge { from, to, directed, properties } => {
        match field.as_str() {
            "from" => Ok(Value::String(from.clone())),
            "to" => Ok(Value::String(to.clone())),
            "directed" => Ok(Value::Boolean(*directed)),
            _ => properties.get(field).cloned()
                .ok_or_else(|| format!("Field '{}' not found", field))
        }
    },
    Value::Network { metadata, .. } => {
        // Acceso a campos de metadata
        metadata.get(field).cloned()
            .ok_or_else(|| format!("Field '{}' not found", field))
    },
    _ => Err(format!("Cannot access field on {:?}", record_value))
}
```

---

## 10. Plan de Implementación (Fases)

### **Fase 1: Fundamentos (MVP)** ✅

1. ✅ Agregar `Edge` y `Network` a `Value` enum
2. ✅ Extender gramática Pest con operadores `->` y `--`
3. ✅ Agregar `Edge` al AST
4. ✅ Implementar evaluación de Edge
5. ✅ Implementar type promotion (String→Record, Record→Network)
6. ✅ Implementar field access para Edge y Network
7. ✅ Tests unitarios básicos

**Entregable:** Sintaxis funcional, creación de networks, acceso a datos.

---

### **Fase 2: Consultas Básicas** ✅

1. ✅ Implementar funciones: `nodes()`, `edges()`, `node_count()`, `edge_count()`
2. ✅ Implementar funciones: `has_node()`, `has_edge()`, `get_node()`, `get_edge()`
3. ✅ Implementar funciones: `neighbors()`, `degree()`, `in_degree()`, `out_degree()`
4. ✅ Tests de integración
5. ✅ Ejemplos .soc

**Entregable:** API completa de consultas sobre grafos.

---

### **Fase 3: Dijkstra (Algoritmo MVP)** ✅

1. ✅ Implementar algoritmo de Dijkstra
2. ✅ Soporte para propiedades de peso custom
3. ✅ Soporte para funciones lambda como peso
4. ✅ Tests con casos reales (mapas de rutas)
5. ✅ Ejemplo completo: mapa de España

**Entregable:** Primer algoritmo completo de grafos.

---

### **Fase 4: Algoritmos Adicionales** (Post-MVP)

1. ⏳ BFS (Breadth-First Search)
2. ⏳ DFS (Depth-First Search)
3. ⏳ `is_connected()`, `connected_components()`
4. ⏳ Bellman-Ford
5. ⏳ Floyd-Warshall

---

### **Fase 5: Árboles de Expansión** (Futuro)

1. ⏳ Kruskal (MST)
2. ⏳ Prim (MST)

---

### **Fase 6: PERT/CPM** (Futuro)

1. ⏳ `critical_path()`
2. ⏳ `earliest_start_times()`, `latest_start_times()`
3. ⏳ `slack_times()`

---

### **Fase 7: Modificación de Grafos** (Futuro)

1. ⏳ `add_edge()`, `add_node()`
2. ⏳ `remove_edge()`, `remove_node()`
3. ⏳ Constructor `network()`

---

## 11. Tests y Validación

### 11.1. Tests Unitarios

```rust
#[test]
fn test_edge_creation() {
    // "A" -> "B"
    let result = eval_str(r#""A" -> "B""#).unwrap();
    assert!(matches!(result, Value::Edge { .. }));
}

#[test]
fn test_edge_with_properties() {
    // "A" -> "B" { peso: 5 }
    let result = eval_str(r#""A" -> "B" { peso: 5 }"#).unwrap();
    // Verificar propiedades
}

#[test]
fn test_network_creation() {
    let result = eval_str(r#"{ edges: ["A" -> "B"] }"#).unwrap();
    assert!(matches!(result, Value::Network { .. }));
}

#[test]
fn test_node_inference() {
    // Nodos "A", "B", "C" deben inferirse
    let result = eval_str(r#"{ edges: ["A" -> "B", "B" -> "C"] }"#).unwrap();
    // Verificar que nodes contiene 3 nodos
}

#[test]
fn test_field_access_edge() {
    let result = eval_str(r#"let e = "A" -> "B"; e.from"#).unwrap();
    assert_eq!(result, Value::String("A".to_string()));
}
```

---

### 11.2. Tests de Integración

```soc
// Test: Grafo simple
let g = { edges: ["A" -> "B", "B" -> "C"] }
assert(node_count(g) == 3)
assert(edge_count(g) == 2)
assert(has_node(g, "A") == true)
assert(has_edge(g, "A", "B") == true)

// Test: Neighbors
let vecinos = neighbors(g, "B")
assert(length(vecinos) == 2)

// Test: Dijkstra
let mapa = {
    edges: [
        "A" -> "B" { km: 10 },
        "B" -> "C" { km: 20 },
        "A" -> "C" { km: 50 }
    ]
}
let ruta = dijkstra(mapa, "A", "C", { weight: "km" })
assert(ruta.distance == 30)
assert(ruta.path == ["A", "B", "C"])
```

---

## 12. Limitaciones Conocidas (MVP)

1. ❌ No hay constructor `network()` explícito (solo literal)
2. ❌ No hay funciones de modificación (`add_edge`, `remove_node`)
3. ❌ Solo Dijkstra implementado (no BFS, DFS, etc.)
4. ❌ No hay validación de ciclos o DAGs
5. ❌ No hay soporte para grafos ponderados negativos (Bellman-Ford futuro)
6. ❌ No hay serialización/visualización de grafos

---

## 13. Decisiones de Diseño Finales (v3.0)

| Aspecto | Decisión | Razón |
|---------|----------|-------|
| **Nodo como tipo** | NO es tipo separado, siempre Record | Simplicidad, consistencia |
| **Nodos en sintaxis** | Record de Records: `{ A: {...}, B: {...} }` | Field access directo O(1), consistente |
| **Campo `id` en nodos** | ❌ NO existe (clave del HashMap es el ID) | Sin duplicación, sin validación necesaria |
| **Edge como tipo** | SÍ es tipo primitivo | Primera clase, operadores binarios |
| **Sintaxis de edges** | **SOLO identificadores** (no strings, no variables) | Máxima consistencia con records |
| **Separador metadata** | **`:` entre edge y metadata** | Distingue IDs (no evaluar) de metadata (evaluar) |
| **Identificadores en edges** | **NUNCA se evalúan** como variables | Sin ambigüedad, IDs puros |
| **Metadata de edges** | **SÍ se evalúa** (puede ser variable o literal) | Flexibilidad, construcción dinámica |
| **Network como tipo** | SÍ es tipo contenedor | Agrupa nodos + edges + metadata |
| **Type promotion** | Record → Network **solo si contiene Edges** | Edges son el distintivo, no campo `nodes` |
| **Nodos no declarados** | Auto-inferir desde identificadores en edges | Flexibilidad, menos boilerplate |
| **Nombres con espacios** | ❌ No permitidos, usar metadata en nodo | Mantiene field access funcional |
| **Edges duplicados** | Permitir (multigrafo) | Útil para rutas múltiples |
| **Self-loops** | Permitir | Válido en teoría de grafos |
| **Grafos mixtos** | Permitir (dirigidos + no dirigidos) | Máxima flexibilidad |
| **Field access en Edge** | Sí: `e.from`, `e.to`, `e.peso` | Consistencia con Record |
| **Field access en Network** | Sí: `n.edges`, `n.nodes.A`, `n.metadata` | Acceso directo a nodos individuales |
| **Constructor `network()`** | Post-MVP | No crítico, literal funciona |

---

## 14. Referencias

- [Roadmap Original](./roadmap.md#propuesta-de-sintaxis-para-grafos-futuro) (líneas 247-331)
- [Especificación del Lenguaje](./language-spec.md)
- Algoritmo Dijkstra: [Wikipedia](https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm)
- Teoría de Grafos: Cormen et al., "Introduction to Algorithms"

---

## 15. Changelog

| Versión | Fecha | Cambios |
|---------|-------|---------|
| 1.0 | 2025-01-07 | Diseño inicial completo aprobado |
| 2.0 | 2025-01-07 | **CAMBIOS MAYORES:** Sintaxis con identificadores en edges (no strings por defecto), nodos como Record de Records (no vector), validación de identificadores vs variables |
| 3.0 | 2025-01-07 | **DISEÑO FINAL:** Identificadores NUNCA se evalúan (IDs puros), sintaxis `:` para metadata (evaluada), eliminación total de strings en edges, máxima consistencia con records |
| 3.1 | 2025-01-07 | **Simplificación:** Eliminación del campo `id` en nodos (clave del HashMap ES el ID), sin duplicación ni validación |

---

**Fin del documento**
