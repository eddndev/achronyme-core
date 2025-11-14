// Helper Functions for Auto-Calculation

use achronyme_types::value::Value;
use std::collections::HashMap;

/// Check if network has ES and EF calculated (from forward_pass)
pub(super) fn has_es_ef_data(network: &HashMap<String, Value>) -> bool {
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
pub(super) fn has_ls_lf_data(network: &HashMap<String, Value>) -> bool {
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
pub(super) fn has_slack_data(network: &HashMap<String, Value>) -> bool {
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
