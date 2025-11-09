use crate::function_modules::graphs::helpers::{build_adjacency_list, validate_node_exists};
use achronyme_types::value::Value;
use std::collections::{HashMap, HashSet, VecDeque};

/// BFS (Breadth-First Search) - Returns nodes in BFS order
pub fn bfs(args: &[Value]) -> Result<Value, String> {
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
pub fn dfs(args: &[Value]) -> Result<Value, String> {
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
pub fn bfs_path(args: &[Value]) -> Result<Value, String> {
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
