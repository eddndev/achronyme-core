use achronyme_types::value::Value;
use std::collections::HashMap;

/// Extract node IDs from a network
pub fn extract_node_ids(network: &HashMap<String, Value>) -> Result<Vec<String>, String> {
    match network.get("nodes") {
        Some(Value::Record(nodes)) => {
            // Keys of the nodes record are the node IDs
            Ok(nodes.keys().cloned().collect())
        }
        _ => Err("Network must have 'nodes' field with a record".to_string()),
    }
}

/// Build adjacency list from network edges
pub fn build_adjacency_list(edges: &[Value]) -> Result<HashMap<String, Vec<String>>, String> {
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

/// Build adjacency list treating all edges as undirected (weak connectivity)
/// Used for connected_components to find weakly connected components
pub fn build_undirected_adjacency_list(edges: &[Value]) -> Result<HashMap<String, Vec<String>>, String> {
    let mut adj_list: HashMap<String, Vec<String>> = HashMap::new();

    for edge in edges {
        match edge {
            Value::Edge { from, to, .. } => {
                // Always add both directions (weak connectivity)
                adj_list.entry(from.clone())
                    .or_insert_with(Vec::new)
                    .push(to.clone());

                adj_list.entry(to.clone())
                    .or_insert_with(Vec::new)
                    .push(from.clone());
            }
            _ => return Err("Invalid edge in edges vector".to_string()),
        }
    }

    Ok(adj_list)
}

/// Validate that a node exists in the network
pub fn validate_node_exists(network: &HashMap<String, Value>, node_id: &str) -> Result<(), String> {
    let node_ids = extract_node_ids(network)?;
    if !node_ids.contains(&node_id.to_string()) {
        return Err(format!("Node '{}' not found in network", node_id));
    }
    Ok(())
}

/// Validate that all edges have a specific property with correct type
pub fn validate_edge_weights(edges: &[Value]) -> Result<(), String> {
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
pub fn validate_undirected(edges: &[Value], algorithm: &str) -> Result<(), String> {
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
pub fn validate_mst_requirements(edges: &[Value], algorithm: &str) -> Result<(), String> {
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
