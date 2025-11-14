// PERT/CPM - Critical Path Method (Costos)

use achronyme_types::value::Value;
use achronyme_types::Environment;
use std::collections::HashMap;
use super::super::helpers::build_adjacency_list;
use super::super::topological::topological_sort;
use super::validation::{validate_dag, validate_node_durations, get_node_duration};
use super::state_detection::{has_es_ef_data, has_ls_lf_data};

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

        new_props.insert("ES".to_string(), Value::Number(*es_map.get(node_id.as_str()).unwrap()));
        new_props.insert("EF".to_string(), Value::Number(*ef_map.get(node_id.as_str()).unwrap()));

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

        new_props.insert("LS".to_string(), Value::Number(*ls_map.get(node_id.as_str()).unwrap()));
        new_props.insert("LF".to_string(), Value::Number(*lf_map.get(node_id.as_str()).unwrap()));

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
