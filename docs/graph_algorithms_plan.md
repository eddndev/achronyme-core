# Plan de Implementación de Algoritmos de Grafos

## Fase 1: Algoritmos de Recorrido (BÁSICOS)

### 1.1 BFS (Breadth-First Search)
**Firma:** `bfs(network, start_node) -> Vector<String>`

**Descripción:** Recorrido en anchura desde un nodo inicial

**Requisitos:**
- Network válido (record con `nodes` y `edges`)
- `start_node` debe ser un String que exista en el network

**Retorna:** Vector de nodos en el orden visitado

**Validaciones:**
- Verificar que `start_node` existe en `nodes(network)`
- Manejar grafos dirigidos y no dirigidos correctamente

---

### 1.2 DFS (Depth-First Search)
**Firma:** `dfs(network, start_node) -> Vector<String>`

**Descripción:** Recorrido en profundidad desde un nodo inicial

**Requisitos:**
- Network válido (record con `nodes` y `edges`)
- `start_node` debe ser un String que exista en el network

**Retorna:** Vector de nodos en el orden visitado

**Validaciones:**
- Verificar que `start_node` existe en `nodes(network)`
- Manejar grafos dirigidos y no dirigidos correctamente

---

## Fase 2: Caminos Más Cortos

### 2.1 Dijkstra
**Firma:** `dijkstra(network, start_node, end_node) -> Record`

**Descripción:** Encuentra el camino más corto entre dos nodos en un grafo ponderado

**Requisitos:**
- Network válido
- **TODOS** los edges deben tener propiedad `weight` (Number > 0)
- `start_node` y `end_node` deben existir en el network

**Retorna:** Record con:
```javascript
{
    path: Vector<String>,      // Nodos en el camino
    distance: Number,          // Distancia total
    found: Boolean             // true si existe camino
}
```

**Validaciones:**
- Verificar que TODOS los edges tienen `weight`
- Verificar que todos los `weight` son números positivos
- Si algún edge no tiene `weight`, retornar error descriptivo:
  `"dijkstra() requires all edges to have a 'weight' property"`

**Ejemplo de uso:**
```javascript
let g = network([
    A -> B: {weight: 4},
    B -> C: {weight: 2},
    A -> C: {weight: 10}
])
let result = dijkstra(g, "A", "C")
// result = {path: ["A", "B", "C"], distance: 6, found: true}
```

---

### 2.2 BFS Path (Camino sin pesos)
**Firma:** `bfs_path(network, start_node, end_node) -> Record`

**Descripción:** Encuentra el camino más corto (en número de edges) entre dos nodos

**Requisitos:**
- Network válido
- NO requiere weights

**Retorna:** Record con:
```javascript
{
    path: Vector<String>,
    found: Boolean
}
```

---

## Fase 3: Árboles de Expansión Mínima

### 3.1 Kruskal
**Firma:** `kruskal(network) -> Record`

**Descripción:** Encuentra el árbol de expansión mínima (MST) usando el algoritmo de Kruskal

**Requisitos:**
- Network debe ser **no dirigido** (todos los edges deben ser `<>`)
- **TODOS** los edges deben tener propiedad `weight`
- El grafo debe ser conexo (warning si no lo es)

**Retorna:** Record con:
```javascript
{
    edges: Vector<Edge>,       // Edges del MST
    total_weight: Number       // Peso total del MST
}
```

**Validaciones:**
- Verificar que TODOS los edges son no dirigidos (`directed == false`)
- Verificar que TODOS los edges tienen `weight`
- Si hay edges dirigidos, retornar error:
  `"kruskal() requires an undirected graph (use <> edges)"`

---

### 3.2 Prim
**Firma:** `prim(network, start_node) -> Record`

**Descripción:** Encuentra el árbol de expansión mínima usando el algoritmo de Prim

**Requisitos:**
- Network debe ser **no dirigido**
- **TODOS** los edges deben tener propiedad `weight`
- `start_node` debe existir

**Retorna:** Mismo formato que Kruskal

**Validaciones:**
- Mismas validaciones que Kruskal
- Verificar que `start_node` existe

---

## Fase 4: Detección de Ciclos y Conectividad

### 4.1 Has Cycle
**Firma:** `has_cycle(network) -> Boolean`

**Descripción:** Detecta si el grafo tiene ciclos

**Requisitos:**
- Network válido

**Retorna:** `true` si hay ciclo, `false` si es acíclico (DAG)

---

### 4.2 Is Connected
**Firma:** `is_connected(network) -> Boolean`

**Descripción:** Verifica si el grafo es conexo (todos los nodos alcanzables)

**Requisitos:**
- Network válido

**Retorna:** `true` si es conexo, `false` si no

---

### 4.3 Connected Components
**Firma:** `connected_components(network) -> Vector<Vector<String>>`

**Descripción:** Encuentra todas las componentes conexas

**Requisitos:**
- Network válido

**Retorna:** Vector de componentes, donde cada componente es un vector de nodos

**Ejemplo:**
```javascript
// Para un grafo con 2 componentes: {A, B, C} y {D, E}
// Retorna: [["A", "B", "C"], ["D", "E"]]
```

---

## Fase 5: Ordenamiento Topológico

### 5.1 Topological Sort
**Firma:** `topological_sort(network) -> Vector<String>`

**Descripción:** Ordena los nodos de un DAG topológicamente

**Requisitos:**
- Network debe ser **dirigido** (puede tener edges `->`)
- Network debe ser **acíclico** (no puede tener ciclos)

**Retorna:** Vector de nodos en orden topológico

**Validaciones:**
- Verificar que el grafo es acíclico usando `has_cycle()`
- Si tiene ciclos, retornar error:
  `"topological_sort() requires a Directed Acyclic Graph (DAG)"`

---

## Diseño de Validación Común

Todas las funciones deben validar:

1. **Validación de Network:**
   ```rust
   fn validate_network(network: &HashMap<String, Value>) -> Result<(), String> {
       if !network.contains_key("nodes") || !network.contains_key("edges") {
           return Err("Invalid network: must have 'nodes' and 'edges' fields".to_string());
       }
       // ...
   }
   ```

2. **Validación de Nodo Existe:**
   ```rust
   fn validate_node_exists(network: &HashMap<String, Value>, node_id: &str) -> Result<(), String> {
       let nodes = extract_nodes(network)?;
       if !nodes.iter().any(|n| matches!(n, Value::Record(map) if map.get("id") == Some(&Value::String(node_id.to_string())))) {
           return Err(format!("Node '{}' not found in network", node_id));
       }
       Ok(())
   }
   ```

3. **Validación de Propiedad en Edges:**
   ```rust
   fn validate_edge_property(edges: &[Value], property: &str) -> Result<(), String> {
       for edge in edges {
           match edge {
               Value::Edge { properties, .. } => {
                   if !properties.contains_key(property) {
                       return Err(format!("Edge missing required property '{}'", property));
                   }
               }
               _ => return Err("Invalid edge in network".to_string()),
           }
       }
       Ok(())
   }
   ```

4. **Validación de Grafo No Dirigido:**
   ```rust
   fn validate_undirected(edges: &[Value]) -> Result<(), String> {
       for edge in edges {
           match edge {
               Value::Edge { directed: true, from, to, .. } => {
                   return Err(format!("Edge {} -> {} is directed; expected undirected (<>)", from, to));
               }
               _ => {}
           }
       }
       Ok(())
   }
   ```

---

## Priorización de Implementación

### Sprint 1 (Básico - Siguiente sesión):
1. BFS
2. DFS
3. BFS Path

### Sprint 2 (Caminos):
1. Dijkstra (con validación de weights)
2. Has Cycle

### Sprint 3 (MST):
1. Kruskal (con validación de undirected)
2. Prim

### Sprint 4 (Avanzado):
1. Connected Components
2. Is Connected
3. Topological Sort (con validación de DAG)

---

## Notas de Implementación

- Todas las funciones deben ir en `function_modules/graphs.rs`
- Crear helpers comunes para validaciones
- Mensajes de error descriptivos y claros
- Tests unitarios para cada algoritmo
- Ejemplos de uso en `examples/soc/algorithms/`

---

## Ejemplos de Mensajes de Error

```javascript
// Error: propiedad faltante
"dijkstra() requires all edges to have a 'weight' property"

// Error: grafo dirigido cuando se esperaba no dirigido
"kruskal() requires an undirected graph (use <> edges)"

// Error: peso negativo
"dijkstra() requires all weights to be positive numbers"

// Error: nodo no existe
"Node 'X' not found in network"

// Error: grafo con ciclos para topological sort
"topological_sort() requires a Directed Acyclic Graph (DAG), but the graph contains cycles"
```
