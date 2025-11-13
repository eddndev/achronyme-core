use crate::function_modules::graphs::helpers::{build_adjacency_list, extract_node_ids};
use achronyme_types::value::Value;
use achronyme_types::Environment;
use std::collections::{HashMap, HashSet};

/// Has Cycle - Detect if graph contains a cycle
pub fn has_cycle(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
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
    let has_directed = edges_vec.iter().any(|e| matches!(e, Value::Edge { directed: true, .. }));

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
        White, // Not visited
        Gray,  // Currently being explored
        Black, // Fully explored
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
