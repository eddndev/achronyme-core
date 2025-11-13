use crate::function_modules::graphs::helpers::{extract_node_ids, validate_mst_requirements};
use achronyme_types::value::Value;
use achronyme_types::Environment;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

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
        other
            .weight
            .partial_cmp(&self.weight)
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
pub fn kruskal(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
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
    weighted_edges.sort_by(|a, b| a.weight.partial_cmp(&b.weight).unwrap_or(Ordering::Equal));

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
pub fn prim(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    let network = match &args[0] {
        Value::Record(map) => map,
        _ => return Err("prim() requires a network record as first argument".to_string()),
    };

    let start_node = match &args[1] {
        Value::String(s) => s,
        _ => return Err("prim() requires a string node ID as second argument".to_string()),
    };

    // Validate start node exists
    crate::function_modules::graphs::helpers::validate_node_exists(network, start_node)?;

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
                adj_list
                    .entry(from.clone())
                    .or_insert_with(Vec::new)
                    .push((to.clone(), weight, edge.clone()));

                adj_list
                    .entry(to.clone())
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
