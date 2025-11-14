// Project Duration Calculation

use achronyme_types::value::Value;
use achronyme_types::Environment;
use super::validation::{validate_dag, validate_node_durations};
use super::cpm::forward_pass;

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
