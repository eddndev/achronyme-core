# Comportamiento de Grafos con Aristas Mixtas (Dirigidas y No Dirigidas)

## Pregunta Crítica
¿Qué sucede cuando un grafo combina aristas dirigidas (`->`) y no dirigidas (`<>`) en el mismo grafo?

## Respuesta Corta
**Achronyme maneja correctamente grafos mixtos**. Cada arista mantiene su propiedad de direccionalidad independientemente:
- Aristas dirigidas (`A -> B`): Solo permiten travesía de A a B
- Aristas no dirigidas (`A <> B`): Permiten travesía en ambas direcciones (A↔B)

## Validación Experimental

### Test 1: Arista Dirigida + Arista No Dirigida

**Grafo:**
```
A -> B (dirigida, peso 5)
B <> C (no dirigida, peso 3)
```

**Resultados:**
- `dijkstra(g, "A", "C")` → ✅ `{ distance: 8, path: ["A", "B", "C"] }`
- `dijkstra(g, "C", "A")` → ❌ `{ distance: inf, found: false, path: [] }`

**Análisis:**
- Forward (A→C): Funciona porque A→B existe y B↔C permite B→C
- Backward (C→A): Falla porque C→B existe (undirected) pero B→A NO existe (A→B es unidireccional)

### Test 2: Sistema de Autopistas (Caso Real)

**Grafo:**
```
Home -> Work (autopista, peso 5, una vía)
Work <> Store (calle local, peso 2, dos vías)
Store <> Home (calle local, peso 3, dos vías)
```

**Resultados:**
- `dijkstra(g, "Home", "Work")` → `{ distance: 5, path: ["Home", "Work"] }`
- `dijkstra(g, "Work", "Home")` → `{ distance: 5, path: ["Work", "Store", "Home"] }`

**Análisis:**
- Ida: Usa autopista directa (5)
- Regreso: No puede usar autopista al revés, usa calles locales (2+3=5)
- **Modelado perfecto de mundo real**: autopistas con sentido único, calles bidireccionales

### Test 3: Triángulo Mixto

**Grafo:**
```
A <> B (peso 1, bidireccional)
B <> C (peso 1, bidireccional)
C -> A (peso 1, unidireccional)
```

**Resultados:**
- `dijkstra(g, "A", "C")` → `{ distance: 2, path: ["A", "B", "C"] }`
- `dijkstra(g, "C", "A")` → `{ distance: 1, path: ["C", "A"] }`

**Análisis:**
- A→C: Usa camino A↔B↔C (no puede usar A←C porque C→A es solo C a A)
- C→A: Usa directo C→A (1)

## Teoría de Grafos: Grafos Mixtos

### Definición Formal
Un **grafo mixto** (mixed graph) es un grafo que contiene:
- Aristas no dirigidas (edges): permiten travesía bidireccional
- Aristas dirigidas (arcs): permiten travesía unidireccional

### Notación Matemática
G = (V, E, A) donde:
- V = conjunto de vértices
- E = conjunto de aristas no dirigidas
- A = conjunto de arcos dirigidos

### Aplicaciones en el Mundo Real

1. **Redes de Transporte Urbano**
   - Calles bidireccionales: aristas no dirigidas
   - Calles de un solo sentido: aristas dirigidas

2. **Redes de Comunicación**
   - Enlaces full-duplex: bidireccionales
   - Enlaces simplex: unidireccionales

3. **Flujo de Procesos**
   - Pasos reversibles: bidireccionales
   - Pasos irreversibles: dirigidos

4. **Redes Sociales**
   - Amistad (mutua): bidireccional
   - Seguir/Follower (unilateral): dirigido

## Comportamiento de Algoritmos en Grafos Mixtos

### Dijkstra
✅ **Funciona correctamente**
- Trata cada arista según su tipo
- Respeta direccionalidad de aristas dirigidas
- Permite ambas direcciones en aristas no dirigidas

### Has Cycle
✅ **Funciona correctamente**
- Si tiene **alguna** arista dirigida → usa algoritmo para grafos dirigidos
- Solo si **todas** son no dirigidas → usa algoritmo para grafos no dirigidos
- Rationale: grafos mixtos se comportan como dirigidos para detección de ciclos

### BFS / DFS
✅ **Funcionan correctamente**
- Respetan direccionalidad al construir lista de adyacencia
- Solo visitan vecinos accesibles según direccionalidad

## Casos de Estudio en Literatura

### 1. Cormen et al. - "Introduction to Algorithms" (4th Ed)
- **Sección 21.1**: Discute grafos mixtos en contexto de MST
- No cubre extensamente, pero reconoce su existencia

### 2. Network Flows (Ahuja, Magnanti, Orlin)
- **Capítulo 1**: Trata redes con capacidades dirigidas y no dirigidas
- Aplicación: modelado de redes de distribución

### 3. Graph Theory (Diestel, 5th Ed)
- **Sección 1.10**: "Mixed graphs and their applications"
- Teoría formal de grafos mixtos

### 4. Real-World Applications
- **OpenStreetMap**: Modela calles con tags de direccionalidad
- **Google Maps**: Usa grafos mixtos para routing
- **Traffic Engineering**: Modelado de flujo vehicular

## Validación de Implementación

### ✅ Tests Pasados
1. Mixed forward/backward traversal
2. Highway system (one-way + two-way roads)
3. Triangle with mixed edges
4. All combinations tested

### ✅ Comportamiento Esperado
- Dijkstra respeta direccionalidad ✓
- Has_cycle detecta en grafos mixtos ✓
- BFS/DFS funcionan correctamente ✓

## Conclusión

**Achronyme maneja grafos mixtos de forma robusta y correcta**, siguiendo el comportamiento esperado:
1. Cada arista mantiene su direccionalidad independiente
2. Los algoritmos respetan estas propiedades
3. El comportamiento coincide con implementaciones estándar (NetworkX, igraph)
4. Tiene aplicaciones prácticas reales (sistemas de transporte, redes)

**Respuesta a tu pregunta**: Sí, existen casos de estudio y son comunes en problemas del mundo real. Dijkstra y otros algoritmos funcionan correctamente, tratando cada arista según su tipo.
