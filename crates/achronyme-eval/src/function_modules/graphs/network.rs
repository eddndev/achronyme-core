use achronyme_types::value::Value;
use std::collections::{HashMap, HashSet};

/// Create a network from edges, optionally with node properties
///
/// Syntax 1 (infer nodes from edges):
///   network([A -> B, B -> C])
///   Returns: {nodes: {A: {}, B: {}, C: {}}, edges: [...]}
///
/// Syntax 2 (explicit node properties):
///   network([A -> B, B -> C], {A: {te: 3}, B: {te: 5}, C: {te: 2}})
///   Returns: {nodes: {A: {te: 3}, B: {te: 5}, C: {te: 2}}, edges: [...]}
///
/// Notes:
/// - Nodes referenced in edges but not in properties will have empty properties {}
/// - Nodes in properties but not in edges are allowed (isolated nodes)
/// - Node properties are flexible - any record structure is accepted
pub fn network(args: &[Value]) -> Result<Value, String> {
    // Validate argument count
    if args.is_empty() || args.len() > 2 {
        return Err("network() expects 1 or 2 arguments".to_string());
    }

    // Extract and validate edges (first argument)
    let edges_vec = match &args[0] {
        Value::Vector(v) => {
            // Validate that all elements are edges
            for edge in v.iter() {
                if !matches!(edge, Value::Edge { .. }) {
                    return Err("network() first argument must be a vector of edges".to_string());
                }
            }
            v
        }
        _ => return Err("network() first argument must be a vector of edges".to_string()),
    };

    // Extract node properties if provided (second argument)
    let node_props = if args.len() == 2 {
        match &args[1] {
            Value::Record(r) => {
                // Validate that all values in the record are records (node properties)
                for (node_id, props) in r.iter() {
                    if !matches!(props, Value::Record(_)) {
                        return Err(format!(
                            "network() node properties must be records (node '{}' has invalid type)",
                            node_id
                        ));
                    }
                }
                Some(r)
            }
            _ => return Err("network() second argument must be a record of node properties".to_string()),
        }
    } else {
        None
    };

    // Step 1: Extract nodes referenced in edges
    let mut all_nodes = HashSet::new();
    for edge in edges_vec.iter() {
        if let Value::Edge { from, to, .. } = edge {
            all_nodes.insert(from.clone());
            all_nodes.insert(to.clone());
        }
    }

    // Step 2: Add nodes from properties (includes isolated nodes)
    if let Some(props) = node_props {
        for node_id in props.keys() {
            all_nodes.insert(node_id.clone());
        }
    }

    // Step 3: Build nodes record
    // Merge nodes from edges + nodes from properties
    let mut nodes_record = HashMap::new();
    for node_id in all_nodes {
        let node_data = if let Some(props) = node_props {
            // Use provided properties if available
            props.get(&node_id)
                .cloned()
                .unwrap_or(Value::Record(HashMap::new()))
        } else {
            // No properties provided, use empty record
            Value::Record(HashMap::new())
        };

        nodes_record.insert(node_id, node_data);
    }

    // Step 4: Build the network record
    let mut network_map = HashMap::new();
    network_map.insert("nodes".to_string(), Value::Record(nodes_record));
    network_map.insert("edges".to_string(), Value::Vector(edges_vec.clone()));

    Ok(Value::Record(network_map))
}

/// Extract nodes from a network
/// Returns a record mapping node IDs to their properties
pub fn nodes(args: &[Value]) -> Result<Value, String> {
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
pub fn edges(args: &[Value]) -> Result<Value, String> {
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
pub fn neighbors(args: &[Value]) -> Result<Value, String> {
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
pub fn degree(args: &[Value]) -> Result<Value, String> {
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
