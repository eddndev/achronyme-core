use crate::function_modules::graphs::helpers::{build_adjacency_list, extract_node_ids};
use achronyme_types::value::Value;
use achronyme_types::Environment;
use std::cmp::Ordering;
use std::collections::HashSet;

/// Connected Components - Find all connected components in a graph
pub fn connected_components(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
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
            component.sort_by(|a, b| match (a, b) {
                (Value::String(s1), Value::String(s2)) => s1.cmp(s2),
                _ => Ordering::Equal,
            });

            components.push(Value::Vector(component));
        }
    }

    Ok(Value::Vector(components))
}

/// Is Connected - Check if graph is connected (all nodes reachable from any node)
pub fn is_connected(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
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
