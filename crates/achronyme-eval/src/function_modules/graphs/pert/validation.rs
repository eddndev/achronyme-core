// PERT Validation Functions

use achronyme_types::value::Value;
use achronyme_types::Environment;
use std::collections::HashMap;
use super::super::cycles::has_cycle;

/// Validate that network is a DAG (required for PERT)
pub(super) fn validate_dag(network: &HashMap<String, Value>, _env: &mut Environment) -> Result<(), String> {
    match has_cycle(&[Value::Record(network.clone())], _env)? {
        Value::Boolean(true) => {
            Err("PERT requires a Directed Acyclic Graph (DAG), but the network contains cycles".to_string())
        }
        _ => Ok(()),
    }
}

/// Validate that all nodes have duration, te, or (op, mo, pe) properties
pub(super) fn validate_node_durations(network: &HashMap<String, Value>) -> Result<(), String> {
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
pub(super) fn validate_probabilistic_properties(network: &HashMap<String, Value>) -> Result<(), String> {
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
pub(super) fn get_node_duration(node_props: &HashMap<String, Value>) -> Result<f64, String> {
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
