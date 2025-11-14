// Critical Path Finding

use achronyme_types::value::Value;
use achronyme_types::Environment;
use std::collections::{HashMap, HashSet};
use super::state_detection::has_slack_data;
use super::cpm::calculate_slack;

/// Find one complete critical path from start to finish
/// Returns a single path (vector of node IDs) following nodes with slack = 0
/// Auto-calculates all prerequisites (forward_pass, backward_pass, calculate_slack) if missing
pub fn critical_path(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    let network = match &args[0] {
        Value::Record(map) => map,
        _ => return Err("critical_path() requires a network record".to_string()),
    };

    // Auto-calculate slack if missing
    // (calculate_slack will auto-calculate forward_pass and backward_pass if needed)
    let network_with_slack = if !has_slack_data(network) {
        match calculate_slack(&[Value::Record(network.clone())], _env)? {
            Value::Record(n) => n,
            _ => return Err("Failed to calculate slack".to_string()),
        }
    } else {
        network.clone()
    };

    let nodes_record = match network_with_slack.get("nodes") {
        Some(Value::Record(r)) => r,
        _ => return Err("Network must have 'nodes' field".to_string()),
    };

    let edges_vec = match network_with_slack.get("edges") {
        Some(Value::Vector(v)) => v,
        _ => return Err("Network must have 'edges' field".to_string()),
    };

    // Find nodes with slack ~= 0 (critical nodes)
    let epsilon = 1e-6;
    let mut critical_nodes_set: HashSet<String> = HashSet::new();

    for (node_id, node_data) in nodes_record {
        if let Value::Record(props) = node_data {
            if let Some(Value::Number(slack)) = props.get("slack") {
                if slack.abs() < epsilon {
                    critical_nodes_set.insert(node_id.clone());
                }
            }
        }
    }

    // Build adjacency list for critical nodes only
    let mut critical_adj: HashMap<String, Vec<String>> = HashMap::new();
    for edge in edges_vec {
        if let Value::Edge { from, to, .. } = edge {
            if critical_nodes_set.contains(from) && critical_nodes_set.contains(to) {
                critical_adj.entry(from.clone())
                    .or_insert_with(Vec::new)
                    .push(to.clone());
            }
        }
    }

    // Find start node (ES = 0 and is critical)
    let mut start_node: Option<String> = None;
    for (node_id, node_data) in nodes_record {
        if critical_nodes_set.contains(node_id.as_str()) {
            if let Value::Record(props) = node_data {
                if let Some(Value::Number(es)) = props.get("ES") {
                    if es.abs() < epsilon {
                        start_node = Some(node_id.clone());
                        break;
                    }
                }
            }
        }
    }

    let start = start_node.ok_or("No critical start node found (ES=0)")?;

    // Find project duration to identify end nodes
    let project_duration = nodes_record.values()
        .filter_map(|node_data| {
            if let Value::Record(props) = node_data {
                if let Some(Value::Number(ef)) = props.get("EF") {
                    return Some(*ef);
                }
            }
            None
        })
        .fold(0.0, f64::max);

    // Find end nodes (EF = project_duration and is critical)
    let mut end_nodes: Vec<String> = Vec::new();
    for (node_id, node_data) in nodes_record {
        if critical_nodes_set.contains(node_id.as_str()) {
            if let Value::Record(props) = node_data {
                if let Some(Value::Number(ef)) = props.get("EF") {
                    if (ef - project_duration).abs() < epsilon {
                        end_nodes.push(node_id.clone());
                    }
                }
            }
        }
    }

    if end_nodes.is_empty() {
        return Err("No critical end node found".to_string());
    }

    // DFS to find one complete path from start to any end node
    fn dfs_find_path(
        current: &str,
        end_nodes: &[String],
        adj: &HashMap<String, Vec<String>>,
        path: &mut Vec<String>,
        visited: &mut HashSet<String>,
    ) -> bool {
        path.push(current.to_string());
        visited.insert(current.to_string());

        // Check if we reached an end node
        if end_nodes.contains(&current.to_string()) {
            return true;
        }

        // Explore neighbors
        if let Some(neighbors) = adj.get(current) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    if dfs_find_path(neighbor, end_nodes, adj, path, visited) {
                        return true;
                    }
                }
            }
        }

        // Backtrack
        path.pop();
        visited.remove(current);
        false
    }

    let mut path = Vec::new();
    let mut visited = HashSet::new();

    if !dfs_find_path(&start, &end_nodes, &critical_adj, &mut path, &mut visited) {
        return Err("Could not find complete critical path from start to end".to_string());
    }

    // Convert to Value::Vector
    let path_values: Vec<Value> = path.into_iter().map(Value::String).collect();
    Ok(Value::Vector(path_values))
}

/// Find all complete critical paths from start to finish
/// Returns a vector of paths (each path is a vector of node IDs)
/// Shows all parallel critical paths in the network
/// Auto-calculates all prerequisites if missing
pub fn all_critical_paths(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    let network = match &args[0] {
        Value::Record(map) => map,
        _ => return Err("all_critical_paths() requires a network record".to_string()),
    };

    // Auto-calculate slack if missing
    let network_with_slack = if !has_slack_data(network) {
        match calculate_slack(&[Value::Record(network.clone())], _env)? {
            Value::Record(n) => n,
            _ => return Err("Failed to calculate slack".to_string()),
        }
    } else {
        network.clone()
    };

    let nodes_record = match network_with_slack.get("nodes") {
        Some(Value::Record(r)) => r,
        _ => return Err("Network must have 'nodes' field".to_string()),
    };

    let edges_vec = match network_with_slack.get("edges") {
        Some(Value::Vector(v)) => v,
        _ => return Err("Network must have 'edges' field".to_string()),
    };

    // Find nodes with slack ~= 0 (critical nodes)
    let epsilon = 1e-6;
    let mut critical_nodes_set: HashSet<String> = HashSet::new();

    for (node_id, node_data) in nodes_record {
        if let Value::Record(props) = node_data {
            if let Some(Value::Number(slack)) = props.get("slack") {
                if slack.abs() < epsilon {
                    critical_nodes_set.insert(node_id.clone());
                }
            }
        }
    }

    // Build adjacency list for critical nodes only
    let mut critical_adj: HashMap<String, Vec<String>> = HashMap::new();
    for edge in edges_vec {
        if let Value::Edge { from, to, .. } = edge {
            if critical_nodes_set.contains(from) && critical_nodes_set.contains(to) {
                critical_adj.entry(from.clone())
                    .or_insert_with(Vec::new)
                    .push(to.clone());
            }
        }
    }

    // Find start node (ES = 0 and is critical)
    let mut start_node: Option<String> = None;
    for (node_id, node_data) in nodes_record {
        if critical_nodes_set.contains(node_id.as_str()) {
            if let Value::Record(props) = node_data {
                if let Some(Value::Number(es)) = props.get("ES") {
                    if es.abs() < epsilon {
                        start_node = Some(node_id.clone());
                        break;
                    }
                }
            }
        }
    }

    let start = start_node.ok_or("No critical start node found (ES=0)")?;

    // Find project duration to identify end nodes
    let project_duration = nodes_record.values()
        .filter_map(|node_data| {
            if let Value::Record(props) = node_data {
                if let Some(Value::Number(ef)) = props.get("EF") {
                    return Some(*ef);
                }
            }
            None
        })
        .fold(0.0, f64::max);

    // Find end nodes (EF = project_duration and is critical)
    let mut end_nodes: Vec<String> = Vec::new();
    for (node_id, node_data) in nodes_record {
        if critical_nodes_set.contains(node_id.as_str()) {
            if let Value::Record(props) = node_data {
                if let Some(Value::Number(ef)) = props.get("EF") {
                    if (ef - project_duration).abs() < epsilon {
                        end_nodes.push(node_id.clone());
                    }
                }
            }
        }
    }

    if end_nodes.is_empty() {
        return Err("No critical end node found".to_string());
    }

    // DFS to find ALL complete paths from start to any end node
    fn dfs_find_all_paths(
        current: &str,
        end_nodes: &[String],
        adj: &HashMap<String, Vec<String>>,
        path: &mut Vec<String>,
        visited: &mut HashSet<String>,
        all_paths: &mut Vec<Vec<String>>,
    ) {
        path.push(current.to_string());
        visited.insert(current.to_string());

        // Check if we reached an end node
        if end_nodes.contains(&current.to_string()) {
            all_paths.push(path.clone());
        } else {
            // Explore neighbors
            if let Some(neighbors) = adj.get(current) {
                for neighbor in neighbors {
                    if !visited.contains(neighbor) {
                        dfs_find_all_paths(neighbor, end_nodes, adj, path, visited, all_paths);
                    }
                }
            }
        }

        // Backtrack
        path.pop();
        visited.remove(current);
    }

    let mut all_paths = Vec::new();
    let mut path = Vec::new();
    let mut visited = HashSet::new();

    dfs_find_all_paths(&start, &end_nodes, &critical_adj, &mut path, &mut visited, &mut all_paths);

    if all_paths.is_empty() {
        return Err("Could not find any critical path from start to end".to_string());
    }

    // Convert to Value::Vector of Value::Vector
    let paths_values: Vec<Value> = all_paths.into_iter()
        .map(|p| Value::Vector(p.into_iter().map(Value::String).collect()))
        .collect();

    Ok(Value::Vector(paths_values))
}
