use crate::functions::FunctionRegistry;
use achronyme_types::value::Value;
use std::collections::{HashMap, HashSet, VecDeque, BinaryHeap};
use std::cmp::Ordering;

pub fn register_functions(registry: &mut FunctionRegistry) {
    // Network constructor and basic operations
    registry.register("network", network, 1);
    registry.register("nodes", nodes, 1);
    registry.register("edges", edges, 1);
    registry.register("neighbors", neighbors, 2);
    registry.register("degree", degree, 2);

    // Graph algorithms - Traversal
    registry.register("bfs", bfs, 2);
    registry.register("dfs", dfs, 2);
    registry.register("bfs_path", bfs_path, 3);

    // Graph algorithms - Shortest Paths
    registry.register("dijkstra", dijkstra, 3);

    // Graph algorithms - Cycle Detection
    registry.register("has_cycle", has_cycle, 1);

    // Graph algorithms - Minimum Spanning Tree
    registry.register("kruskal", kruskal, 1);
    registry.register("prim", prim, 2);

    // Graph algorithms - Connectivity
    registry.register("connected_components", connected_components, 1);
    registry.register("is_connected", is_connected, 1);

    // Graph algorithms - Topological Sort
    registry.register("topological_sort", topological_sort, 1);
}

// Implementations

/// Create a network from a vector of edges
/// Returns a record with structure: {nodes: [...], edges: [...]}
fn network(args: &[Value]) -> Result<Value, String> {
    match &args[0] {
        Value::Vector(edges_vec) => {
            // Validate that all elements are edges
            for edge in edges_vec.iter() {
                if !matches!(edge, Value::Edge { .. }) {
                    return Err("network() requires a vector of edges".to_string());
                }
            }

            // Extract unique nodes from edges
            let mut node_set = HashSet::new();

            for edge in edges_vec.iter() {
                if let Value::Edge { from, to, .. } = edge {
                    node_set.insert(from.clone());
                    node_set.insert(to.clone());
                }
            }

            // Convert node set to sorted vector of strings
            let mut node_names: Vec<String> = node_set.into_iter().collect();
            node_names.sort();

            // Create node records with id field
            let nodes: Vec<Value> = node_names
                .into_iter()
                .map(|name| {
                    let mut node_map = HashMap::new();
                    node_map.insert("id".to_string(), Value::String(name));
                    Value::Record(node_map)
                })
                .collect();

            // Build the network record
            let mut network_map = HashMap::new();
            network_map.insert("nodes".to_string(), Value::Vector(nodes));
            network_map.insert("edges".to_string(), Value::Vector(edges_vec.clone()));

            Ok(Value::Record(network_map))
        }
        _ => Err("network() requires a vector of edges".to_string()),
    }
}

/// Extract nodes from a network
fn nodes(args: &[Value]) -> Result<Value, String> {
    match &args[0] {
        Value::Record(map) => {
            map.get("nodes")
                .cloned()
                .ok_or_else(|| "nodes() requires a network record with 'nodes' field".to_string())
        }
        _ => Err("nodes() requires a network record".to_string()),
    }
}

/// Extract edges from a network
fn edges(args: &[Value]) -> Result<Value, String> {
    match &args[0] {
        Value::Record(map) => {
            map.get("edges")
                .cloned()
                .ok_or_else(|| "edges() requires a network record with 'edges' field".to_string())
        }
        _ => Err("edges() requires a network record".to_string()),
    }
}

/// Get neighbors of a node in a network
/// neighbors(network, node_id) returns a vector of node IDs that are neighbors
fn neighbors(args: &[Value]) -> Result<Value, String> {
    let network = match &args[0] {
        Value::Record(map) => map,
        _ => return Err("neighbors() requires a network record as first argument".to_string()),
    };

    let node_id = match &args[1] {
        Value::String(s) => s,
        _ => return Err("neighbors() requires a string node ID as second argument".to_string()),
    };

    // Get edges from network
    let edges_vec = match network.get("edges") {
        Some(Value::Vector(v)) => v,
        _ => return Err("Network must have an 'edges' field with a vector".to_string()),
    };

    // Collect unique neighbors
    let mut neighbor_set = HashSet::new();

    for edge in edges_vec.iter() {
        match edge {
            Value::Edge { from, to, directed, .. } => {
                // If edge is from our node, add 'to' as neighbor
                if from == node_id {
                    neighbor_set.insert(to.clone());
                }
                // If edge is to our node and undirected, add 'from' as neighbor
                if to == node_id && !directed {
                    neighbor_set.insert(from.clone());
                }
                // If edge is to our node and directed, don't add 'from'
                // (directed edges only go one way)
            }
            _ => return Err("edges vector must contain only Edge values".to_string()),
        }
    }

    // Convert to sorted vector of strings
    let mut neighbors_list: Vec<String> = neighbor_set.into_iter().collect();
    neighbors_list.sort();

    let neighbors_values: Vec<Value> = neighbors_list
        .into_iter()
        .map(Value::String)
        .collect();

    Ok(Value::Vector(neighbors_values))
}

/// Get degree of a node in a network
/// degree(network, node_id) returns the number of edges connected to the node
fn degree(args: &[Value]) -> Result<Value, String> {
    let network = match &args[0] {
        Value::Record(map) => map,
        _ => return Err("degree() requires a network record as first argument".to_string()),
    };

    let node_id = match &args[1] {
        Value::String(s) => s,
        _ => return Err("degree() requires a string node ID as second argument".to_string()),
    };

    // Get edges from network
    let edges_vec = match network.get("edges") {
        Some(Value::Vector(v)) => v,
        _ => return Err("Network must have an 'edges' field with a vector".to_string()),
    };

    // Count edges connected to this node
    let mut edge_count = 0;

    for edge in edges_vec.iter() {
        match edge {
            Value::Edge { from, to, .. } => {
                // Count edge if it's from our node
                if from == node_id {
                    edge_count += 1;
                }
                // Count edge if it's to our node
                // For directed edges, this is in-degree
                // For undirected edges, we already counted it above, so skip
                else if to == node_id {
                    edge_count += 1;
                }
            }
            _ => return Err("edges vector must contain only Edge values".to_string()),
        }
    }

    Ok(Value::Number(edge_count as f64))
}

// ============================================================================
// Helper Functions for Graph Algorithms
// ============================================================================

/// Extract node IDs from a network
fn extract_node_ids(network: &HashMap<String, Value>) -> Result<Vec<String>, String> {
    match network.get("nodes") {
        Some(Value::Vector(nodes)) => {
            let mut node_ids = Vec::new();
            for node in nodes {
                match node {
                    Value::Record(map) => {
                        match map.get("id") {
                            Some(Value::String(id)) => node_ids.push(id.clone()),
                            _ => return Err("Node record must have 'id' field with string value".to_string()),
                        }
                    }
                    _ => return Err("Nodes must be records".to_string()),
                }
            }
            Ok(node_ids)
        }
        _ => Err("Network must have 'nodes' field with vector".to_string()),
    }
}

/// Build adjacency list from network edges
fn build_adjacency_list(edges: &[Value]) -> Result<HashMap<String, Vec<String>>, String> {
    let mut adj_list: HashMap<String, Vec<String>> = HashMap::new();

    for edge in edges {
        match edge {
            Value::Edge { from, to, directed, .. } => {
                // Add forward edge
                adj_list.entry(from.clone())
                    .or_insert_with(Vec::new)
                    .push(to.clone());

                // For undirected edges, add reverse edge
                if !directed {
                    adj_list.entry(to.clone())
                        .or_insert_with(Vec::new)
                        .push(from.clone());
                }
            }
            _ => return Err("Invalid edge in edges vector".to_string()),
        }
    }

    Ok(adj_list)
}

/// Validate that a node exists in the network
fn validate_node_exists(network: &HashMap<String, Value>, node_id: &str) -> Result<(), String> {
    let node_ids = extract_node_ids(network)?;
    if !node_ids.contains(&node_id.to_string()) {
        return Err(format!("Node '{}' not found in network", node_id));
    }
    Ok(())
}

/// Validate that all edges have a specific property with correct type
fn validate_edge_weights(edges: &[Value]) -> Result<(), String> {
    for edge in edges {
        match edge {
            Value::Edge { properties, from, to, .. } => {
                match properties.get("weight") {
                    Some(Value::Number(w)) => {
                        if *w <= 0.0 {
                            return Err(format!(
                                "dijkstra() requires all weights to be positive numbers (edge {} -> {} has weight {})",
                                from, to, w
                            ));
                        }
                    }
                    Some(_) => {
                        return Err(format!(
                            "Edge {} -> {} has non-numeric 'weight' property",
                            from, to
                        ));
                    }
                    None => {
                        return Err("dijkstra() requires all edges to have a 'weight' property".to_string());
                    }
                }
            }
            _ => return Err("Invalid edge in edges vector".to_string()),
        }
    }
    Ok(())
}

/// Validate that all edges are undirected
fn validate_undirected(edges: &[Value], algorithm: &str) -> Result<(), String> {
    for edge in edges {
        match edge {
            Value::Edge { directed, from, to, .. } => {
                if *directed {
                    return Err(format!(
                        "{}() requires an undirected graph (use <> edges), but found directed edge {} -> {}",
                        algorithm, from, to
                    ));
                }
            }
            _ => return Err("Invalid edge in edges vector".to_string()),
        }
    }
    Ok(())
}

/// Validate MST requirements: undirected + weighted
fn validate_mst_requirements(edges: &[Value], algorithm: &str) -> Result<(), String> {
    // First check undirected
    validate_undirected(edges, algorithm)?;

    // Then check weights
    for edge in edges {
        match edge {
            Value::Edge { properties, from, to, .. } => {
                match properties.get("weight") {
                    Some(Value::Number(w)) => {
                        if *w <= 0.0 {
                            return Err(format!(
                                "{}() requires all weights to be positive numbers (edge {} <> {} has weight {})",
                                algorithm, from, to, w
                            ));
                        }
                    }
                    Some(_) => {
                        return Err(format!(
                            "Edge {} <> {} has non-numeric 'weight' property",
                            from, to
                        ));
                    }
                    None => {
                        return Err(format!("{}() requires all edges to have a 'weight' property", algorithm));
                    }
                }
            }
            _ => return Err("Invalid edge in edges vector".to_string()),
        }
    }
    Ok(())
}

// ============================================================================
// Graph Traversal Algorithms
// ============================================================================

/// BFS (Breadth-First Search) - Returns nodes in BFS order
fn bfs(args: &[Value]) -> Result<Value, String> {
    let network = match &args[0] {
        Value::Record(map) => map,
        _ => return Err("bfs() requires a network record as first argument".to_string()),
    };

    let start_node = match &args[1] {
        Value::String(s) => s,
        _ => return Err("bfs() requires a string node ID as second argument".to_string()),
    };

    // Validate start node exists
    validate_node_exists(network, start_node)?;

    // Get edges and build adjacency list
    let edges_vec = match network.get("edges") {
        Some(Value::Vector(v)) => v,
        _ => return Err("Network must have an 'edges' field with a vector".to_string()),
    };

    let adj_list = build_adjacency_list(edges_vec)?;

    // BFS implementation
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut result = Vec::new();

    queue.push_back(start_node.clone());
    visited.insert(start_node.clone());

    while let Some(current) = queue.pop_front() {
        result.push(Value::String(current.clone()));

        // Visit neighbors
        if let Some(neighbors) = adj_list.get(&current) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    visited.insert(neighbor.clone());
                    queue.push_back(neighbor.clone());
                }
            }
        }
    }

    Ok(Value::Vector(result))
}

/// DFS (Depth-First Search) - Returns nodes in DFS order
fn dfs(args: &[Value]) -> Result<Value, String> {
    let network = match &args[0] {
        Value::Record(map) => map,
        _ => return Err("dfs() requires a network record as first argument".to_string()),
    };

    let start_node = match &args[1] {
        Value::String(s) => s,
        _ => return Err("dfs() requires a string node ID as second argument".to_string()),
    };

    // Validate start node exists
    validate_node_exists(network, start_node)?;

    // Get edges and build adjacency list
    let edges_vec = match network.get("edges") {
        Some(Value::Vector(v)) => v,
        _ => return Err("Network must have an 'edges' field with a vector".to_string()),
    };

    let adj_list = build_adjacency_list(edges_vec)?;

    // DFS implementation using stack
    let mut visited = HashSet::new();
    let mut stack = Vec::new();
    let mut result = Vec::new();

    stack.push(start_node.clone());

    while let Some(current) = stack.pop() {
        if visited.contains(&current) {
            continue;
        }

        visited.insert(current.clone());
        result.push(Value::String(current.clone()));

        // Visit neighbors (in reverse order to maintain left-to-right traversal)
        if let Some(neighbors) = adj_list.get(&current) {
            for neighbor in neighbors.iter().rev() {
                if !visited.contains(neighbor) {
                    stack.push(neighbor.clone());
                }
            }
        }
    }

    Ok(Value::Vector(result))
}

/// BFS Path - Find shortest path between two nodes (unweighted)
fn bfs_path(args: &[Value]) -> Result<Value, String> {
    let network = match &args[0] {
        Value::Record(map) => map,
        _ => return Err("bfs_path() requires a network record as first argument".to_string()),
    };

    let start_node = match &args[1] {
        Value::String(s) => s,
        _ => return Err("bfs_path() requires a string node ID as second argument".to_string()),
    };

    let end_node = match &args[2] {
        Value::String(s) => s,
        _ => return Err("bfs_path() requires a string node ID as third argument".to_string()),
    };

    // Validate nodes exist
    validate_node_exists(network, start_node)?;
    validate_node_exists(network, end_node)?;

    // Get edges and build adjacency list
    let edges_vec = match network.get("edges") {
        Some(Value::Vector(v)) => v,
        _ => return Err("Network must have an 'edges' field with a vector".to_string()),
    };

    let adj_list = build_adjacency_list(edges_vec)?;

    // BFS with parent tracking for path reconstruction
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut parent: HashMap<String, String> = HashMap::new();

    queue.push_back(start_node.clone());
    visited.insert(start_node.clone());

    let mut found = false;

    while let Some(current) = queue.pop_front() {
        if &current == end_node {
            found = true;
            break;
        }

        if let Some(neighbors) = adj_list.get(&current) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    visited.insert(neighbor.clone());
                    parent.insert(neighbor.clone(), current.clone());
                    queue.push_back(neighbor.clone());
                }
            }
        }
    }

    // Reconstruct path if found
    let path = if found {
        let mut path_nodes = Vec::new();
        let mut current = end_node.clone();

        path_nodes.push(current.clone());

        while let Some(prev) = parent.get(&current) {
            path_nodes.push(prev.clone());
            current = prev.clone();
        }

        path_nodes.reverse();
        path_nodes.into_iter().map(Value::String).collect()
    } else {
        Vec::new()
    };

    // Return record with path and found status
    let mut result = HashMap::new();
    result.insert("path".to_string(), Value::Vector(path));
    result.insert("found".to_string(), Value::Boolean(found));

    Ok(Value::Record(result))
}

// ============================================================================
// Shortest Path Algorithms (Weighted)
// ============================================================================

/// State for Dijkstra's priority queue
#[derive(Clone)]
struct DijkstraState {
    node: String,
    distance: f64,
}

impl Eq for DijkstraState {}

impl PartialEq for DijkstraState {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance && self.node == other.node
    }
}

impl Ord for DijkstraState {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for min-heap (BinaryHeap is max-heap by default)
        other.distance.partial_cmp(&self.distance)
            .unwrap_or(Ordering::Equal)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl PartialOrd for DijkstraState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Dijkstra - Find shortest path in weighted graph
fn dijkstra(args: &[Value]) -> Result<Value, String> {
    let network = match &args[0] {
        Value::Record(map) => map,
        _ => return Err("dijkstra() requires a network record as first argument".to_string()),
    };

    let start_node = match &args[1] {
        Value::String(s) => s,
        _ => return Err("dijkstra() requires a string node ID as second argument".to_string()),
    };

    let end_node = match &args[2] {
        Value::String(s) => s,
        _ => return Err("dijkstra() requires a string node ID as third argument".to_string()),
    };

    // Validate nodes exist
    validate_node_exists(network, start_node)?;
    validate_node_exists(network, end_node)?;

    // Get edges and validate weights
    let edges_vec = match network.get("edges") {
        Some(Value::Vector(v)) => v,
        _ => return Err("Network must have an 'edges' field with a vector".to_string()),
    };

    validate_edge_weights(edges_vec)?;

    // Build weighted adjacency list
    let mut adj_list: HashMap<String, Vec<(String, f64)>> = HashMap::new();

    for edge in edges_vec {
        match edge {
            Value::Edge { from, to, directed, properties } => {
                let weight = match properties.get("weight") {
                    Some(Value::Number(w)) => *w,
                    _ => unreachable!(), // Already validated
                };

                adj_list.entry(from.clone())
                    .or_insert_with(Vec::new)
                    .push((to.clone(), weight));

                if !directed {
                    adj_list.entry(to.clone())
                        .or_insert_with(Vec::new)
                        .push((from.clone(), weight));
                }
            }
            _ => return Err("Invalid edge in edges vector".to_string()),
        }
    }

    // Dijkstra's algorithm
    let mut distances: HashMap<String, f64> = HashMap::new();
    let mut parent: HashMap<String, String> = HashMap::new();
    let mut heap = BinaryHeap::new();

    distances.insert(start_node.clone(), 0.0);
    heap.push(DijkstraState {
        node: start_node.clone(),
        distance: 0.0,
    });

    while let Some(DijkstraState { node, distance }) = heap.pop() {
        // Skip if we've found a better path
        if let Some(&d) = distances.get(&node) {
            if distance > d {
                continue;
            }
        }

        // If we reached the end node, we're done
        if &node == end_node {
            break;
        }

        // Explore neighbors
        if let Some(neighbors) = adj_list.get(&node) {
            for (neighbor, edge_weight) in neighbors {
                let new_distance = distance + edge_weight;

                let is_better = distances
                    .get(neighbor)
                    .map_or(true, |&d| new_distance < d);

                if is_better {
                    distances.insert(neighbor.clone(), new_distance);
                    parent.insert(neighbor.clone(), node.clone());
                    heap.push(DijkstraState {
                        node: neighbor.clone(),
                        distance: new_distance,
                    });
                }
            }
        }
    }

    // Check if path was found
    let found = distances.contains_key(end_node);
    let total_distance = distances.get(end_node).copied().unwrap_or(f64::INFINITY);

    // Reconstruct path
    let path = if found {
        let mut path_nodes = Vec::new();
        let mut current = end_node.clone();

        path_nodes.push(current.clone());

        while let Some(prev) = parent.get(&current) {
            path_nodes.push(prev.clone());
            current = prev.clone();
        }

        path_nodes.reverse();
        path_nodes.into_iter().map(Value::String).collect()
    } else {
        Vec::new()
    };

    // Return record
    let mut result = HashMap::new();
    result.insert("path".to_string(), Value::Vector(path));
    result.insert("distance".to_string(), Value::Number(total_distance));
    result.insert("found".to_string(), Value::Boolean(found));

    Ok(Value::Record(result))
}

// ============================================================================
// Cycle Detection
// ============================================================================

/// Has Cycle - Detect if graph contains a cycle
fn has_cycle(args: &[Value]) -> Result<Value, String> {
    let network = match &args[0] {
        Value::Record(map) => map,
        _ => return Err("has_cycle() requires a network record as first argument".to_string()),
    };

    // Get edges
    let edges_vec = match network.get("edges") {
        Some(Value::Vector(v)) => v,
        _ => return Err("Network must have an 'edges' field with a vector".to_string()),
    };

    // Get all nodes
    let node_ids = extract_node_ids(network)?;

    // Build adjacency list
    let adj_list = build_adjacency_list(edges_vec)?;

    // Check if graph has any directed edges
    let has_directed = edges_vec.iter().any(|e| {
        matches!(e, Value::Edge { directed: true, .. })
    });

    if has_directed {
        // For directed graphs, use DFS with three colors (white, gray, black)
        detect_cycle_directed(&node_ids, &adj_list)
    } else {
        // For undirected graphs, use DFS with parent tracking
        detect_cycle_undirected(&node_ids, &adj_list)
    }
}

/// Detect cycle in directed graph using DFS with three colors
fn detect_cycle_directed(nodes: &[String], adj_list: &HashMap<String, Vec<String>>) -> Result<Value, String> {
    #[derive(PartialEq)]
    enum Color {
        White,  // Not visited
        Gray,   // Currently being explored
        Black,  // Fully explored
    }

    let mut colors: HashMap<String, Color> = HashMap::new();

    // Initialize all nodes as white
    for node in nodes {
        colors.insert(node.clone(), Color::White);
    }

    fn dfs_directed(
        node: &str,
        colors: &mut HashMap<String, Color>,
        adj_list: &HashMap<String, Vec<String>>,
    ) -> bool {
        // Mark as gray (currently exploring)
        colors.insert(node.to_string(), Color::Gray);

        // Visit neighbors
        if let Some(neighbors) = adj_list.get(node) {
            for neighbor in neighbors {
                match colors.get(neighbor) {
                    Some(Color::Gray) => {
                        // Back edge found - cycle detected!
                        return true;
                    }
                    Some(Color::White) => {
                        if dfs_directed(neighbor, colors, adj_list) {
                            return true;
                        }
                    }
                    Some(Color::Black) => {
                        // Already fully explored, skip
                    }
                    None => {
                        // Node not in graph, skip
                    }
                }
            }
        }

        // Mark as black (fully explored)
        colors.insert(node.to_string(), Color::Black);
        false
    }

    // Check all components
    for node in nodes {
        if colors.get(node) == Some(&Color::White) {
            if dfs_directed(node, &mut colors, adj_list) {
                return Ok(Value::Boolean(true));
            }
        }
    }

    Ok(Value::Boolean(false))
}

/// Detect cycle in undirected graph using DFS with parent tracking
fn detect_cycle_undirected(nodes: &[String], adj_list: &HashMap<String, Vec<String>>) -> Result<Value, String> {
    let mut visited: HashSet<String> = HashSet::new();

    fn dfs_undirected(
        node: &str,
        parent: Option<&str>,
        visited: &mut HashSet<String>,
        adj_list: &HashMap<String, Vec<String>>,
    ) -> bool {
        visited.insert(node.to_string());

        if let Some(neighbors) = adj_list.get(node) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    if dfs_undirected(neighbor, Some(node), visited, adj_list) {
                        return true;
                    }
                } else if Some(neighbor.as_str()) != parent {
                    // Visited neighbor that's not the parent - cycle found!
                    return true;
                }
            }
        }

        false
    }

    // Check all components
    for node in nodes {
        if !visited.contains(node) {
            if dfs_undirected(node, None, &mut visited, adj_list) {
                return Ok(Value::Boolean(true));
            }
        }
    }

    Ok(Value::Boolean(false))
}

// ============================================================================
// Minimum Spanning Tree Algorithms
// ============================================================================

/// Union-Find (Disjoint Set Union) data structure for Kruskal's algorithm
struct UnionFind {
    parent: HashMap<String, String>,
    rank: HashMap<String, usize>,
}

impl UnionFind {
    fn new(nodes: &[String]) -> Self {
        let mut parent = HashMap::new();
        let mut rank = HashMap::new();

        for node in nodes {
            parent.insert(node.clone(), node.clone());
            rank.insert(node.clone(), 0);
        }

        UnionFind { parent, rank }
    }

    fn find(&mut self, node: &str) -> String {
        if self.parent.get(node).map(|p| p.as_str()) != Some(node) {
            let parent = self.parent.get(node).unwrap().clone();
            let root = self.find(&parent);
            self.parent.insert(node.to_string(), root.clone());
            root
        } else {
            node.to_string()
        }
    }

    fn union(&mut self, node1: &str, node2: &str) -> bool {
        let root1 = self.find(node1);
        let root2 = self.find(node2);

        if root1 == root2 {
            return false; // Already in same set (would create cycle)
        }

        let rank1 = self.rank.get(&root1).copied().unwrap_or(0);
        let rank2 = self.rank.get(&root2).copied().unwrap_or(0);

        if rank1 < rank2 {
            self.parent.insert(root1, root2);
        } else if rank1 > rank2 {
            self.parent.insert(root2, root1);
        } else {
            self.parent.insert(root2, root1.clone());
            self.rank.insert(root1, rank1 + 1);
        }

        true
    }
}

/// Edge struct for MST algorithms
#[derive(Clone)]
struct WeightedEdge {
    from: String,
    to: String,
    weight: f64,
    edge_value: Value,
}

impl Eq for WeightedEdge {}

impl PartialEq for WeightedEdge {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

impl Ord for WeightedEdge {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for min-heap (BinaryHeap is max-heap by default)
        other.weight.partial_cmp(&self.weight)
            .unwrap_or(Ordering::Equal)
            .then_with(|| self.from.cmp(&other.from))
            .then_with(|| self.to.cmp(&other.to))
    }
}

impl PartialOrd for WeightedEdge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Kruskal's algorithm - Minimum Spanning Tree
fn kruskal(args: &[Value]) -> Result<Value, String> {
    let network = match &args[0] {
        Value::Record(map) => map,
        _ => return Err("kruskal() requires a network record as first argument".to_string()),
    };

    // Get edges
    let edges_vec = match network.get("edges") {
        Some(Value::Vector(v)) => v,
        _ => return Err("Network must have an 'edges' field with a vector".to_string()),
    };

    // Validate MST requirements (undirected + weighted)
    validate_mst_requirements(edges_vec, "kruskal")?;

    // Get all nodes
    let node_ids = extract_node_ids(network)?;

    // Extract and sort edges by weight
    let mut weighted_edges: Vec<WeightedEdge> = Vec::new();

    for edge in edges_vec {
        match edge {
            Value::Edge { from, to, properties, .. } => {
                let weight = match properties.get("weight") {
                    Some(Value::Number(w)) => *w,
                    _ => unreachable!(), // Already validated
                };

                weighted_edges.push(WeightedEdge {
                    from: from.clone(),
                    to: to.clone(),
                    weight,
                    edge_value: edge.clone(),
                });
            }
            _ => return Err("Invalid edge in edges vector".to_string()),
        }
    }

    // Sort edges by weight (ascending order)
    weighted_edges.sort_by(|a, b| {
        a.weight.partial_cmp(&b.weight).unwrap_or(Ordering::Equal)
    });

    // Kruskal's algorithm using Union-Find
    let mut uf = UnionFind::new(&node_ids);
    let mut mst_edges = Vec::new();
    let mut total_weight = 0.0;

    for edge in weighted_edges {
        // Try to add edge to MST (will succeed if it doesn't create a cycle)
        if uf.union(&edge.from, &edge.to) {
            total_weight += edge.weight;
            mst_edges.push(edge.edge_value);
        }
    }

    // Build result record
    let mut result = HashMap::new();
    result.insert("edges".to_string(), Value::Vector(mst_edges));
    result.insert("total_weight".to_string(), Value::Number(total_weight));

    Ok(Value::Record(result))
}

/// Prim's algorithm - Minimum Spanning Tree
fn prim(args: &[Value]) -> Result<Value, String> {
    let network = match &args[0] {
        Value::Record(map) => map,
        _ => return Err("prim() requires a network record as first argument".to_string()),
    };

    let start_node = match &args[1] {
        Value::String(s) => s,
        _ => return Err("prim() requires a string node ID as second argument".to_string()),
    };

    // Validate start node exists
    validate_node_exists(network, start_node)?;

    // Get edges
    let edges_vec = match network.get("edges") {
        Some(Value::Vector(v)) => v,
        _ => return Err("Network must have an 'edges' field with a vector".to_string()),
    };

    // Validate MST requirements (undirected + weighted)
    validate_mst_requirements(edges_vec, "prim")?;

    // Build adjacency list with edges and weights
    let mut adj_list: HashMap<String, Vec<(String, f64, Value)>> = HashMap::new();

    for edge in edges_vec {
        match edge {
            Value::Edge { from, to, properties, .. } => {
                let weight = match properties.get("weight") {
                    Some(Value::Number(w)) => *w,
                    _ => unreachable!(), // Already validated
                };

                // Undirected graph: add both directions
                adj_list.entry(from.clone())
                    .or_insert_with(Vec::new)
                    .push((to.clone(), weight, edge.clone()));

                adj_list.entry(to.clone())
                    .or_insert_with(Vec::new)
                    .push((from.clone(), weight, edge.clone()));
            }
            _ => return Err("Invalid edge in edges vector".to_string()),
        }
    }

    // Prim's algorithm using priority queue
    let mut visited = HashSet::new();
    let mut heap = BinaryHeap::new();
    let mut mst_edges = Vec::new();
    let mut total_weight = 0.0;

    // Start from the given node
    visited.insert(start_node.clone());

    // Add all edges from start node to heap
    if let Some(neighbors) = adj_list.get(start_node) {
        for (neighbor, weight, edge) in neighbors {
            heap.push(WeightedEdge {
                from: start_node.clone(),
                to: neighbor.clone(),
                weight: *weight,
                edge_value: edge.clone(),
            });
        }
    }

    // Process edges in order of weight
    while let Some(edge) = heap.pop() {
        // Skip if destination already visited
        if visited.contains(&edge.to) {
            continue;
        }

        // Add node to visited set
        visited.insert(edge.to.clone());

        // Add edge to MST
        total_weight += edge.weight;
        mst_edges.push(edge.edge_value);

        // Add all edges from newly added node
        if let Some(neighbors) = adj_list.get(&edge.to) {
            for (neighbor, weight, neighbor_edge) in neighbors {
                if !visited.contains(neighbor) {
                    heap.push(WeightedEdge {
                        from: edge.to.clone(),
                        to: neighbor.clone(),
                        weight: *weight,
                        edge_value: neighbor_edge.clone(),
                    });
                }
            }
        }
    }

    // Build result record
    let mut result = HashMap::new();
    result.insert("edges".to_string(), Value::Vector(mst_edges));
    result.insert("total_weight".to_string(), Value::Number(total_weight));

    Ok(Value::Record(result))
}

// ============================================================================
// Connectivity Algorithms
// ============================================================================

/// Connected Components - Find all connected components in a graph
fn connected_components(args: &[Value]) -> Result<Value, String> {
    let network = match &args[0] {
        Value::Record(map) => map,
        _ => return Err("connected_components() requires a network record as first argument".to_string()),
    };

    // Get all nodes
    let node_ids = extract_node_ids(network)?;

    // Get edges and build adjacency list
    let edges_vec = match network.get("edges") {
        Some(Value::Vector(v)) => v,
        _ => return Err("Network must have an 'edges' field with a vector".to_string()),
    };

    let adj_list = build_adjacency_list(edges_vec)?;

    // Find connected components using DFS
    let mut visited: HashSet<String> = HashSet::new();
    let mut components: Vec<Value> = Vec::new();

    for node in &node_ids {
        if !visited.contains(node) {
            let mut component = Vec::new();
            let mut stack = vec![node.clone()];

            while let Some(current) = stack.pop() {
                if visited.contains(&current) {
                    continue;
                }

                visited.insert(current.clone());
                component.push(Value::String(current.clone()));

                if let Some(neighbors) = adj_list.get(&current) {
                    for neighbor in neighbors {
                        if !visited.contains(neighbor) {
                            stack.push(neighbor.clone());
                        }
                    }
                }
            }

            // Sort component for consistent output
            component.sort_by(|a, b| {
                match (a, b) {
                    (Value::String(s1), Value::String(s2)) => s1.cmp(s2),
                    _ => Ordering::Equal,
                }
            });

            components.push(Value::Vector(component));
        }
    }

    Ok(Value::Vector(components))
}

/// Is Connected - Check if graph is connected (all nodes reachable from any node)
fn is_connected(args: &[Value]) -> Result<Value, String> {
    let network = match &args[0] {
        Value::Record(map) => map,
        _ => return Err("is_connected() requires a network record as first argument".to_string()),
    };

    // Get all nodes
    let node_ids = extract_node_ids(network)?;

    // Empty graph is considered connected
    if node_ids.is_empty() {
        return Ok(Value::Boolean(true));
    }

    // Get edges and build adjacency list
    let edges_vec = match network.get("edges") {
        Some(Value::Vector(v)) => v,
        _ => return Err("Network must have an 'edges' field with a vector".to_string()),
    };

    let adj_list = build_adjacency_list(edges_vec)?;

    // DFS from first node to see if all nodes are reachable
    let start_node = &node_ids[0];
    let mut visited: HashSet<String> = HashSet::new();
    let mut stack = vec![start_node.clone()];

    while let Some(current) = stack.pop() {
        if visited.contains(&current) {
            continue;
        }

        visited.insert(current.clone());

        if let Some(neighbors) = adj_list.get(&current) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    stack.push(neighbor.clone());
                }
            }
        }
    }

    // Graph is connected if all nodes were visited
    Ok(Value::Boolean(visited.len() == node_ids.len()))
}

// ============================================================================
// Topological Sort
// ============================================================================

/// Topological Sort - Order nodes in a DAG such that for every edge u -> v, u comes before v
fn topological_sort(args: &[Value]) -> Result<Value, String> {
    let network = match &args[0] {
        Value::Record(map) => map,
        _ => return Err("topological_sort() requires a network record as first argument".to_string()),
    };

    // Check if graph has cycles (must be DAG)
    match has_cycle(&[args[0].clone()])? {
        Value::Boolean(true) => {
            return Err("topological_sort() requires a Directed Acyclic Graph (DAG), but the graph contains cycles".to_string());
        }
        _ => {}
    }

    // Get all nodes
    let node_ids = extract_node_ids(network)?;

    // Get edges and build adjacency list
    let edges_vec = match network.get("edges") {
        Some(Value::Vector(v)) => v,
        _ => return Err("Network must have an 'edges' field with a vector".to_string()),
    };

    let adj_list = build_adjacency_list(edges_vec)?;

    // Kahn's algorithm (BFS-based topological sort)
    // Calculate in-degree for each node
    let mut in_degree: HashMap<String, usize> = HashMap::new();
    for node in &node_ids {
        in_degree.insert(node.clone(), 0);
    }

    for neighbors in adj_list.values() {
        for neighbor in neighbors {
            *in_degree.entry(neighbor.clone()).or_insert(0) += 1;
        }
    }

    // Queue of nodes with in-degree 0
    let mut queue: VecDeque<String> = VecDeque::new();
    for (node, &degree) in &in_degree {
        if degree == 0 {
            queue.push_back(node.clone());
        }
    }

    // Process nodes in topological order
    let mut result = Vec::new();

    while let Some(node) = queue.pop_front() {
        result.push(Value::String(node.clone()));

        // Reduce in-degree of neighbors
        if let Some(neighbors) = adj_list.get(&node) {
            for neighbor in neighbors {
                if let Some(degree) = in_degree.get_mut(neighbor) {
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push_back(neighbor.clone());
                    }
                }
            }
        }
    }

    Ok(Value::Vector(result))
}
