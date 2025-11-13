use crate::function_modules::graphs::cycles::has_cycle;
use crate::function_modules::graphs::helpers::{build_adjacency_list, extract_node_ids};
use achronyme_types::value::Value;
use achronyme_types::Environment;
use std::collections::{HashMap, VecDeque};

/// Topological Sort - Order nodes in a DAG such that for every edge u -> v, u comes before v
pub fn topological_sort(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    let network = match &args[0] {
        Value::Record(map) => map,
        _ => return Err("topological_sort() requires a network record as first argument".to_string()),
    };

    // Check if graph has cycles (must be DAG)
    match has_cycle(&[args[0].clone()], _env)? {
        Value::Boolean(true) => {
            return Err(
                "topological_sort() requires a Directed Acyclic Graph (DAG), but the graph contains cycles"
                    .to_string(),
            );
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
