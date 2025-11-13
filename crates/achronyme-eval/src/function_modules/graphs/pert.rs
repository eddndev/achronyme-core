// PERT (Program Evaluation and Review Technique) Algorithms
// Includes both CPM (Critical Path Method) and Probabilistic PERT analysis

use achronyme_types::value::Value;
use achronyme_types::Environment;
use std::collections::{HashMap, HashSet};
use super::helpers::build_adjacency_list;
use super::cycles::has_cycle;
use super::topological::topological_sort;

// ============================================================================
// PERT Validation Functions
// ============================================================================

/// Validate that network is a DAG (required for PERT)
fn validate_dag(network: &HashMap<String, Value>, _env: &mut Environment) -> Result<(), String> {
    match has_cycle(&[Value::Record(network.clone())], _env)? {
        Value::Boolean(true) => {
            Err("PERT requires a Directed Acyclic Graph (DAG), but the network contains cycles".to_string())
        }
        _ => Ok(()),
    }
}

/// Validate that all nodes have duration, te, or (op, mo, pe) properties
fn validate_node_durations(network: &HashMap<String, Value>) -> Result<(), String> {
    let nodes_record = match network.get("nodes") {
        Some(Value::Record(r)) => r,
        _ => return Err("Network must have 'nodes' field with a record".to_string()),
    };

    for (node_id, node_data) in nodes_record {
        match node_data {
            Value::Record(props) => {
                // Try to get duration using same priority logic as get_node_duration
                match get_node_duration(props) {
                    Ok(duration) => {
                        // Validate duration is non-negative
                        if duration < 0.0 {
                            return Err(format!(
                                "Node '{}' has negative duration: {}",
                                node_id, duration
                            ));
                        }
                    }
                    Err(e) => {
                        return Err(format!(
                            "Node '{}': {}",
                            node_id, e
                        ));
                    }
                }
            }
            _ => return Err(format!("Node '{}' data must be a record", node_id)),
        }
    }

    Ok(())
}

/// Validate that all nodes have op, mo, pe properties for probabilistic PERT
fn validate_probabilistic_properties(network: &HashMap<String, Value>) -> Result<(), String> {
    let nodes_record = match network.get("nodes") {
        Some(Value::Record(r)) => r,
        _ => return Err("Network must have 'nodes' field with a record".to_string()),
    };

    for (node_id, node_data) in nodes_record {
        match node_data {
            Value::Record(props) => {
                // Check for op, mo, pe
                let op = props.get("op");
                let mo = props.get("mo");
                let pe = props.get("pe");

                if op.is_none() || mo.is_none() || pe.is_none() {
                    return Err(format!(
                        "Node '{}' must have 'op', 'mo', and 'pe' properties for probabilistic PERT",
                        node_id
                    ));
                }

                // Extract values and validate op <= mo <= pe
                let op_val = match op {
                    Some(Value::Number(n)) => *n,
                    _ => return Err(format!("Node '{}' 'op' must be a number", node_id)),
                };

                let mo_val = match mo {
                    Some(Value::Number(n)) => *n,
                    _ => return Err(format!("Node '{}' 'mo' must be a number", node_id)),
                };

                let pe_val = match pe {
                    Some(Value::Number(n)) => *n,
                    _ => return Err(format!("Node '{}' 'pe' must be a number", node_id)),
                };

                if !(op_val <= mo_val && mo_val <= pe_val) {
                    return Err(format!(
                        "Node '{}' must satisfy: op <= mo <= pe (got op={}, mo={}, pe={})",
                        node_id, op_val, mo_val, pe_val
                    ));
                }

                if op_val < 0.0 || mo_val < 0.0 || pe_val < 0.0 {
                    return Err(format!(
                        "Node '{}' times must be non-negative",
                        node_id
                    ));
                }
            }
            _ => return Err(format!("Node '{}' data must be a record", node_id)),
        }
    }

    Ok(())
}

/// Get duration from node with priority:
/// 1. 'duration' (explicit, deterministic)
/// 2. 'te' (explicit, pre-calculated expected time)
/// 3. Calculate from (op, mo, pe) using PERT formula: (op + 4*mo + pe) / 6
fn get_node_duration(node_props: &HashMap<String, Value>) -> Result<f64, String> {
    // Priority 1: duration
    if let Some(Value::Number(d)) = node_props.get("duration") {
        return Ok(*d);
    }

    // Priority 2: te (expected time)
    if let Some(Value::Number(t)) = node_props.get("te") {
        return Ok(*t);
    }

    // Priority 3: Calculate from op, mo, pe
    if let (Some(Value::Number(op)), Some(Value::Number(mo)), Some(Value::Number(pe))) =
        (node_props.get("op"), node_props.get("mo"), node_props.get("pe")) {
        // Validate op <= mo <= pe
        if !(*op <= *mo && *mo <= *pe) {
            return Err(format!(
                "Invalid PERT estimates: op <= mo <= pe required (got op={}, mo={}, pe={})",
                op, mo, pe
            ));
        }
        // Calculate expected time: te = (op + 4*mo + pe) / 6
        return Ok((op + 4.0 * mo + pe) / 6.0);
    }

    Err("Node must have 'duration', 'te', or ('op', 'mo', 'pe') properties".to_string())
}

// ============================================================================
// Helper Functions for Auto-Calculation
// ============================================================================

/// Check if network has ES and EF calculated (from forward_pass)
fn has_es_ef_data(network: &HashMap<String, Value>) -> bool {
    if let Some(Value::Record(nodes)) = network.get("nodes") {
        return nodes.iter().any(|(_id, node_data)| {
            if let Value::Record(props) = node_data {
                props.contains_key("ES") && props.contains_key("EF")
            } else {
                false
            }
        });
    }
    false
}

/// Check if network has LS and LF calculated (from backward_pass)
fn has_ls_lf_data(network: &HashMap<String, Value>) -> bool {
    if let Some(Value::Record(nodes)) = network.get("nodes") {
        return nodes.iter().any(|(_id, node_data)| {
            if let Value::Record(props) = node_data {
                props.contains_key("LS") && props.contains_key("LF")
            } else {
                false
            }
        });
    }
    false
}

/// Check if network has slack calculated (from calculate_slack)
fn has_slack_data(network: &HashMap<String, Value>) -> bool {
    if let Some(Value::Record(nodes)) = network.get("nodes") {
        return nodes.iter().any(|(_id, node_data)| {
            if let Value::Record(props) = node_data {
                props.contains_key("slack")
            } else {
                false
            }
        });
    }
    false
}

// ============================================================================
// PERT/CPM - Critical Path Method (Costos)
// ============================================================================

/// Forward pass: Calculate Early Start (ES) and Early Finish (EF) for all tasks
/// ES[task] = max(EF[predecessors])
/// EF[task] = ES[task] + duration[task]
pub fn forward_pass(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    let network = match &args[0] {
        Value::Record(map) => map,
        _ => return Err("forward_pass() requires a network record".to_string()),
    };

    // Validate DAG and durations
    validate_dag(network, _env)?;
    validate_node_durations(network)?;

    // Get nodes and edges
    let nodes_record = match network.get("nodes") {
        Some(Value::Record(r)) => r,
        _ => return Err("Network must have 'nodes' field".to_string()),
    };

    let edges_vec = match network.get("edges") {
        Some(Value::Vector(v)) => v,
        _ => return Err("Network must have 'edges' field".to_string()),
    };

    // Build reverse adjacency list (predecessors)
    let mut predecessors: HashMap<String, Vec<String>> = HashMap::new();
    for edge in edges_vec {
        if let Value::Edge { from, to, .. } = edge {
            predecessors.entry(to.clone())
                .or_insert_with(Vec::new)
                .push(from.clone());
        }
    }

    // Get topological order
    let topo_order = match topological_sort(&[Value::Record(network.clone())], _env)? {
        Value::Vector(v) => v,
        _ => return Err("Failed to get topological order".to_string()),
    };

    // Calculate ES and EF for each node in topological order
    let mut es_map: HashMap<String, f64> = HashMap::new();
    let mut ef_map: HashMap<String, f64> = HashMap::new();

    for node_val in topo_order {
        let node_id = match node_val {
            Value::String(s) => s,
            _ => return Err("Invalid node in topological order".to_string()),
        };

        let node_props = match nodes_record.get(&node_id) {
            Some(Value::Record(p)) => p,
            _ => return Err(format!("Node '{}' not found", node_id)),
        };

        let duration = get_node_duration(node_props)?;

        // ES = max(EF of all predecessors), or 0 if no predecessors
        let es = if let Some(preds) = predecessors.get(&node_id) {
            let mut max_ef: f64 = 0.0;
            for pred in preds {
                if let Some(pred_ef) = ef_map.get(pred) {
                    max_ef = max_ef.max(*pred_ef);
                }
            }
            max_ef
        } else {
            0.0 // Start nodes have ES = 0
        };

        let ef = es + duration;

        es_map.insert(node_id.clone(), es);
        ef_map.insert(node_id.clone(), ef);
    }

    // Build new network with ES and EF added to nodes
    let mut new_nodes = HashMap::new();
    for (node_id, node_data) in nodes_record {
        let mut new_props = match node_data {
            Value::Record(p) => p.clone(),
            _ => HashMap::new(),
        };

        new_props.insert("ES".to_string(), Value::Number(*es_map.get(node_id).unwrap()));
        new_props.insert("EF".to_string(), Value::Number(*ef_map.get(node_id).unwrap()));

        new_nodes.insert(node_id.clone(), Value::Record(new_props));
    }

    let mut new_network = network.clone();
    new_network.insert("nodes".to_string(), Value::Record(new_nodes));

    Ok(Value::Record(new_network))
}

/// Backward pass: Calculate Late Start (LS) and Late Finish (LF) for all tasks
/// LF[task] = min(LS[successors])
/// LS[task] = LF[task] - duration[task]
/// Auto-calculates forward_pass if ES/EF data is missing
pub fn backward_pass(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    let network = match &args[0] {
        Value::Record(map) => map,
        _ => return Err("backward_pass() requires a network record".to_string()),
    };

    // Auto-calculate forward pass if ES/EF data is missing
    let network_with_es_ef = if !has_es_ef_data(network) {
        match forward_pass(&[Value::Record(network.clone())], _env)? {
            Value::Record(n) => n,
            _ => return Err("Failed to calculate forward pass".to_string()),
        }
    } else {
        network.clone()
    };

    // Validate that network has ES/EF (from forward_pass)
    let nodes_record = match network_with_es_ef.get("nodes") {
        Some(Value::Record(r)) => r,
        _ => return Err("Network must have 'nodes' field".to_string()),
    };

    let edges_vec = match network_with_es_ef.get("edges") {
        Some(Value::Vector(v)) => v,
        _ => return Err("Network must have 'edges' field".to_string()),
    };

    // Build adjacency list (successors)
    let adj_list = build_adjacency_list(edges_vec)?;

    // Get topological order (reversed for backward pass)
    let topo_order = match topological_sort(&[Value::Record(network_with_es_ef.clone())], _env)? {
        Value::Vector(mut v) => {
            v.reverse();
            v
        }
        _ => return Err("Failed to get topological order".to_string()),
    };

    // Find project completion time (max EF)
    let project_completion = nodes_record.values()
        .filter_map(|node_data| {
            if let Value::Record(props) = node_data {
                if let Some(Value::Number(ef)) = props.get("EF") {
                    return Some(*ef);
                }
            }
            None
        })
        .fold(0.0, f64::max);

    // Calculate LS and LF for each node in reverse topological order
    let mut ls_map: HashMap<String, f64> = HashMap::new();
    let mut lf_map: HashMap<String, f64> = HashMap::new();

    for node_val in topo_order {
        let node_id = match node_val {
            Value::String(s) => s,
            _ => return Err("Invalid node in topological order".to_string()),
        };

        let node_props = match nodes_record.get(&node_id) {
            Some(Value::Record(p)) => p,
            _ => return Err(format!("Node '{}' not found", node_id)),
        };

        let duration = get_node_duration(node_props)?;

        // LF = min(LS of all successors), or project_completion if no successors
        let lf = if let Some(succs) = adj_list.get(&node_id) {
            let mut min_ls = f64::INFINITY;
            for succ in succs {
                if let Some(succ_ls) = ls_map.get(succ) {
                    min_ls = min_ls.min(*succ_ls);
                }
            }
            if min_ls == f64::INFINITY {
                project_completion
            } else {
                min_ls
            }
        } else {
            project_completion // End nodes have LF = project completion
        };

        let ls = lf - duration;

        ls_map.insert(node_id.clone(), ls);
        lf_map.insert(node_id.clone(), lf);
    }

    // Build new network with LS and LF added to nodes
    let mut new_nodes = HashMap::new();
    for (node_id, node_data) in nodes_record {
        let mut new_props = match node_data {
            Value::Record(p) => p.clone(),
            _ => HashMap::new(),
        };

        new_props.insert("LS".to_string(), Value::Number(*ls_map.get(node_id).unwrap()));
        new_props.insert("LF".to_string(), Value::Number(*lf_map.get(node_id).unwrap()));

        new_nodes.insert(node_id.clone(), Value::Record(new_props));
    }

    let mut new_network = network_with_es_ef.clone();
    new_network.insert("nodes".to_string(), Value::Record(new_nodes));

    Ok(Value::Record(new_network))
}

/// Calculate slack (float) for all tasks
/// Slack = LS - ES (or LF - EF)
/// Auto-calculates forward_pass and backward_pass if ES/EF or LS/LF data is missing
pub fn calculate_slack(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    let network = match &args[0] {
        Value::Record(map) => map,
        _ => return Err("calculate_slack() requires a network record".to_string()),
    };

    // Auto-calculate backward pass if LS/LF data is missing
    // (backward_pass will auto-calculate forward_pass if ES/EF is also missing)
    let network_with_all_data = if !has_ls_lf_data(network) {
        match backward_pass(&[Value::Record(network.clone())], _env)? {
            Value::Record(n) => n,
            _ => return Err("Failed to calculate backward pass".to_string()),
        }
    } else {
        network.clone()
    };

    let nodes_record = match network_with_all_data.get("nodes") {
        Some(Value::Record(r)) => r,
        _ => return Err("Network must have 'nodes' field".to_string()),
    };

    // Calculate slack for each node
    let mut new_nodes = HashMap::new();
    for (node_id, node_data) in nodes_record {
        let mut new_props = match node_data {
            Value::Record(p) => p.clone(),
            _ => HashMap::new(),
        };

        let es = match new_props.get("ES") {
            Some(Value::Number(n)) => *n,
            _ => return Err(format!("Node '{}' missing ES", node_id)),
        };

        let ls = match new_props.get("LS") {
            Some(Value::Number(n)) => *n,
            _ => return Err(format!("Node '{}' missing LS", node_id)),
        };

        let slack = ls - es;
        new_props.insert("slack".to_string(), Value::Number(slack));

        new_nodes.insert(node_id.clone(), Value::Record(new_props));
    }

    let mut new_network = network_with_all_data.clone();
    new_network.insert("nodes".to_string(), Value::Record(new_nodes));

    Ok(Value::Record(new_network))
}

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
        if critical_nodes_set.contains(node_id) {
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
        if critical_nodes_set.contains(node_id) {
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
        if critical_nodes_set.contains(node_id) {
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
        if critical_nodes_set.contains(node_id) {
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

/// Calculate total project duration (max EF across all nodes)
pub fn project_duration(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    let network = match &args[0] {
        Value::Record(map) => map,
        _ => return Err("project_duration() requires a network record".to_string()),
    };

    // Validate and calculate
    validate_dag(network, _env)?;
    validate_node_durations(network)?;

    // Run forward pass to get EF values
    let network_with_times = forward_pass(&[Value::Record(network.clone())], _env)?;

    let nodes_record = match network_with_times {
        Value::Record(ref map) => match map.get("nodes") {
            Some(Value::Record(r)) => r,
            _ => return Err("Invalid network structure".to_string()),
        },
        _ => return Err("Invalid network structure".to_string()),
    };

    // Find max EF
    let max_ef = nodes_record.values()
        .filter_map(|node_data| {
            if let Value::Record(props) = node_data {
                if let Some(Value::Number(ef)) = props.get("EF") {
                    return Some(*ef);
                }
            }
            None
        })
        .fold(0.0, f64::max);

    Ok(Value::Number(max_ef))
}

// ============================================================================
// PERT - Probabilistic Analysis
// ============================================================================

/// Calculate expected time using PERT formula: te = (op + 4*mo + pe) / 6
pub fn expected_time(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    let op = match &args[0] {
        Value::Number(n) => *n,
        _ => return Err("expected_time() requires three numbers (op, mo, pe)".to_string()),
    };

    let mo = match &args[1] {
        Value::Number(n) => *n,
        _ => return Err("expected_time() requires three numbers (op, mo, pe)".to_string()),
    };

    let pe = match &args[2] {
        Value::Number(n) => *n,
        _ => return Err("expected_time() requires three numbers (op, mo, pe)".to_string()),
    };

    // Validate op <= mo <= pe
    if !(op <= mo && mo <= pe) {
        return Err(format!(
            "expected_time() requires op <= mo <= pe (got op={}, mo={}, pe={})",
            op, mo, pe
        ));
    }

    let te = (op + 4.0 * mo + pe) / 6.0;
    Ok(Value::Number(te))
}

/// Calculate task variance: variance = ((pe - op) / 6)^2
pub fn task_variance(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    let op = match &args[0] {
        Value::Number(n) => *n,
        _ => return Err("task_variance() requires three numbers (op, mo, pe)".to_string()),
    };

    // mo is not used in variance calculation, but we validate it's a number
    let _mo = match &args[1] {
        Value::Number(_n) => {},
        _ => return Err("task_variance() requires three numbers (op, mo, pe)".to_string()),
    };

    let pe = match &args[2] {
        Value::Number(n) => *n,
        _ => return Err("task_variance() requires three numbers (op, mo, pe)".to_string()),
    };

    if pe < op {
        return Err("task_variance() requires pe >= op".to_string());
    }

    let variance = ((pe - op) / 6.0).powi(2);
    Ok(Value::Number(variance))
}

/// Calculate project variance (sum of variances on critical path)
/// Auto-calculates critical path if needed
pub fn project_variance(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    let network = match &args[0] {
        Value::Record(map) => map,
        _ => return Err("project_variance() requires a network record".to_string()),
    };

    // Validate probabilistic properties
    validate_dag(network, _env)?;
    validate_probabilistic_properties(network)?;

    // Get critical path (auto-calculates all prerequisites if needed)
    let critical_nodes = critical_path(&[Value::Record(network.clone())], _env)?;

    let nodes_record = match network.get("nodes") {
        Some(Value::Record(r)) => r,
        _ => return Err("Network must have 'nodes' field".to_string()),
    };

    // Sum variances of critical path tasks
    let mut total_variance = 0.0;

    if let Value::Vector(critical) = critical_nodes {
        for node_val in critical {
            let node_id = match node_val {
                Value::String(s) => s,
                _ => continue,
            };

            if let Some(Value::Record(props)) = nodes_record.get(&node_id) {
                let op = match props.get("op") {
                    Some(Value::Number(n)) => *n,
                    _ => continue,
                };
                // mo is extracted for validation but not used in variance calculation
                let _mo = match props.get("mo") {
                    Some(Value::Number(n)) => *n,
                    _ => continue,
                };
                let pe = match props.get("pe") {
                    Some(Value::Number(n)) => *n,
                    _ => continue,
                };

                let variance = ((pe - op) / 6.0).powi(2);
                total_variance += variance;
            }
        }
    }

    Ok(Value::Number(total_variance))
}

/// Calculate project standard deviation
pub fn project_std_dev(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    let variance = match project_variance(args, _env)? {
        Value::Number(v) => v,
        _ => return Err("Failed to calculate project variance".to_string()),
    };

    Ok(Value::Number(variance.sqrt()))
}

/// Calculate probability of completing project by target time
/// Uses normal distribution: P(T <= target) = Φ((target - te) / σ)
pub fn completion_probability(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    let network = match &args[0] {
        Value::Record(map) => map,
        _ => return Err("completion_probability() requires a network record and target time".to_string()),
    };

    let target_time = match &args[1] {
        Value::Number(n) => *n,
        _ => return Err("completion_probability() requires a target time (number)".to_string()),
    };

    // Calculate project duration (te) and standard deviation
    let te = match project_duration(&[Value::Record(network.clone())], _env)? {
        Value::Number(n) => n,
        _ => return Err("Failed to calculate project duration".to_string()),
    };

    let std_dev = match project_std_dev(&[Value::Record(network.clone())], _env)? {
        Value::Number(n) => n,
        _ => return Err("Failed to calculate project standard deviation".to_string()),
    };

    if std_dev == 0.0 {
        // Deterministic: probability is 0 if target < te, 1 if target >= te
        return Ok(Value::Number(if target_time >= te { 1.0 } else { 0.0 }));
    }

    // Calculate z-score
    let z = (target_time - te) / std_dev;

    // Calculate probability using normal CDF approximation
    let prob = normal_cdf(z);

    Ok(Value::Number(prob))
}

/// Calculate time needed for desired completion probability
pub fn time_for_probability(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    let network = match &args[0] {
        Value::Record(map) => map,
        _ => return Err("time_for_probability() requires a network record and probability".to_string()),
    };

    let probability = match &args[1] {
        Value::Number(n) => *n,
        _ => return Err("time_for_probability() requires a probability (0-1)".to_string()),
    };

    if !(0.0..=1.0).contains(&probability) {
        return Err("Probability must be between 0 and 1".to_string());
    }

    // Calculate project duration (te) and standard deviation
    let te = match project_duration(&[Value::Record(network.clone())], _env)? {
        Value::Number(n) => n,
        _ => return Err("Failed to calculate project duration".to_string()),
    };

    let std_dev = match project_std_dev(&[Value::Record(network.clone())], _env)? {
        Value::Number(n) => n,
        _ => return Err("Failed to calculate project standard deviation".to_string()),
    };

    if std_dev == 0.0 {
        // Deterministic: return te
        return Ok(Value::Number(te));
    }

    // Find z-score for probability using inverse normal CDF
    let z = inverse_normal_cdf(probability);

    // Calculate time: time = te + z * σ
    let time = te + z * std_dev;

    Ok(Value::Number(time))
}

/// Complete PERT analysis - one-stop function for all PERT calculations
/// Returns a record with: network (with ES/EF/LS/LF/slack), critical_path, duration,
/// and if probabilistic properties exist: variance and std_dev
pub fn pert_analysis(args: &[Value], _env: &mut Environment) -> Result<Value, String> {
    let network = match &args[0] {
        Value::Record(map) => map,
        _ => return Err("pert_analysis() requires a network record".to_string()),
    };

    // Calculate network with all properties (auto-calculates prerequisites)
    let network_with_slack = match calculate_slack(&[Value::Record(network.clone())], _env)? {
        Value::Record(n) => n,
        _ => return Err("Failed to calculate slack".to_string()),
    };

    // Get critical path
    let critical_path_nodes = critical_path(&[Value::Record(network_with_slack.clone())], _env)?;

    // Calculate project duration
    let duration = project_duration(&[Value::Record(network_with_slack.clone())], _env)?;

    // Check if network has probabilistic properties (op, mo, pe)
    let has_probabilistic = if let Some(Value::Record(nodes)) = network.get("nodes") {
        nodes.values().any(|node_data| {
            if let Value::Record(props) = node_data {
                props.contains_key("op") && props.contains_key("mo") && props.contains_key("pe")
            } else {
                false
            }
        })
    } else {
        false
    };

    // Build result record
    let mut result = HashMap::new();
    result.insert("network".to_string(), Value::Record(network_with_slack.clone()));
    result.insert("critical_path".to_string(), critical_path_nodes);
    result.insert("duration".to_string(), duration);

    // Add probabilistic analysis if applicable
    if has_probabilistic {
        let variance = project_variance(&[Value::Record(network.clone())], _env)?;
        let std_dev = project_std_dev(&[Value::Record(network.clone())], _env)?;
        result.insert("variance".to_string(), variance);
        result.insert("std_dev".to_string(), std_dev);
    }

    Ok(Value::Record(result))
}

// ============================================================================
// Helper Functions for Normal Distribution
// ============================================================================

/// Cumulative Distribution Function for standard normal distribution
/// Approximation using error function
fn normal_cdf(x: f64) -> f64 {
    0.5 * (1.0 + erf(x / std::f64::consts::SQRT_2))
}

/// Inverse CDF for standard normal distribution
/// Approximation for z-score given probability
fn inverse_normal_cdf(p: f64) -> f64 {
    // Beasley-Springer-Moro algorithm approximation
    let a = [
        -3.969683028665376e+01,
        2.209460984245205e+02,
        -2.759285104469687e+02,
        1.383577518672690e+02,
        -3.066479806614716e+01,
        2.506628277459239e+00,
    ];

    let b = [
        -5.447609879822406e+01,
        1.615858368580409e+02,
        -1.556989798598866e+02,
        6.680131188771972e+01,
        -1.328068155288572e+01,
    ];

    let c = [
        -7.784894002430293e-03,
        -3.223964580411365e-01,
        -2.400758277161838e+00,
        -2.549732539343734e+00,
        4.374664141464968e+00,
        2.938163982698783e+00,
    ];

    let d = [
        7.784695709041462e-03,
        3.224671290700398e-01,
        2.445134137142996e+00,
        3.754408661907416e+00,
    ];

    let p_low = 0.02425;
    let p_high = 1.0 - p_low;

    if p < p_low {
        // Rational approximation for lower region
        let q = (-2.0 * p.ln()).sqrt();
        return (((((c[0] * q + c[1]) * q + c[2]) * q + c[3]) * q + c[4]) * q + c[5])
            / ((((d[0] * q + d[1]) * q + d[2]) * q + d[3]) * q + 1.0);
    }

    if p > p_high {
        // Rational approximation for upper region
        let q = (-2.0 * (1.0 - p).ln()).sqrt();
        return -(((((c[0] * q + c[1]) * q + c[2]) * q + c[3]) * q + c[4]) * q + c[5])
            / ((((d[0] * q + d[1]) * q + d[2]) * q + d[3]) * q + 1.0);
    }

    // Rational approximation for central region
    let q = p - 0.5;
    let r = q * q;
    (((((a[0] * r + a[1]) * r + a[2]) * r + a[3]) * r + a[4]) * r + a[5]) * q
        / (((((b[0] * r + b[1]) * r + b[2]) * r + b[3]) * r + b[4]) * r + 1.0)
}

/// Error function approximation
fn erf(x: f64) -> f64 {
    // Abramowitz and Stegun approximation
    let a1 = 0.254829592;
    let a2 = -0.284496736;
    let a3 = 1.421413741;
    let a4 = -1.453152027;
    let a5 = 1.061405429;
    let p = 0.3275911;

    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let x = x.abs();

    let t = 1.0 / (1.0 + p * x);
    let y = 1.0 - (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1) * t * (-x * x).exp();

    sign * y
}
