use crate::function_modules::graphs::helpers::{validate_edge_weights, validate_node_exists};
use achronyme_types::value::Value;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

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
        other
            .distance
            .partial_cmp(&self.distance)
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
pub fn dijkstra(args: &[Value]) -> Result<Value, String> {
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

                adj_list
                    .entry(from.clone())
                    .or_insert_with(Vec::new)
                    .push((to.clone(), weight));

                if !directed {
                    adj_list
                        .entry(to.clone())
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

                let is_better = distances.get(neighbor).map_or(true, |&d| new_distance < d);

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
