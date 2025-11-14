// PERT - Probabilistic Analysis

use achronyme_types::value::Value;
use achronyme_types::Environment;
use std::collections::HashMap;
use super::validation::{validate_dag, validate_probabilistic_properties};
use super::cpm::calculate_slack;
use super::critical_path::critical_path;
use super::statistics::{normal_cdf, inverse_normal_cdf};
use super::project::project_duration;

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
