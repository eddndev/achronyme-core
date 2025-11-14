/// Complex tensor indexing and slicing operations

use achronyme_types::value::Value;
use achronyme_types::tensor::ComplexTensor;

use crate::handlers::indexing::types::EvaluatedIndex;
use crate::handlers::indexing::utils::{normalize_index, normalize_range};

/// Index into a complex tensor
pub fn index_complex_tensor(tensor: &ComplexTensor, indices: &[EvaluatedIndex]) -> Result<Value, String> {
    let shape = tensor.shape();

    if indices.len() > shape.len() {
        return Err(format!(
            "Too many indices: tensor has {} dimensions, got {} indices",
            shape.len(),
            indices.len()
        ));
    }

    // Check if all indices are single (element access) or if there are any ranges (slicing)
    let has_range = indices.iter().any(|idx| matches!(idx, EvaluatedIndex::Range { .. }));

    if !has_range {
        // All indices are single - extract a single element
        index_complex_tensor_element(tensor, indices)
    } else {
        // At least one range - perform slicing
        slice_complex_tensor(tensor, indices)
    }
}

/// Extract a single element from a complex tensor
fn index_complex_tensor_element(tensor: &ComplexTensor, indices: &[EvaluatedIndex]) -> Result<Value, String> {
    let shape = tensor.shape();
    let mut coords = Vec::new();

    // Convert all indices to coordinates
    for (i, idx) in indices.iter().enumerate() {
        match idx {
            EvaluatedIndex::Single(n) => {
                let actual_idx = normalize_index(*n, shape[i])?;
                coords.push(actual_idx);
            }
            _ => unreachable!("Should not have ranges in element access"),
        }
    }

    // If fewer indices than dimensions, we're extracting a sub-tensor
    if coords.len() < shape.len() {
        return extract_complex_subtensor(tensor, &coords);
    }

    // Get the element at the coordinates
    tensor.get(&coords)
        .map(|&val| Value::Complex(val))
        .map_err(|e| format!("Index {:?} out of bounds for tensor shape {:?}: {}", coords, shape, e))
}

/// Extract a sub-tensor from complex tensor by fixing some dimensions
fn extract_complex_subtensor(tensor: &ComplexTensor, fixed_coords: &[usize]) -> Result<Value, String> {
    let shape = tensor.shape();

    // Calculate the shape of the result (remaining dimensions)
    let result_shape: Vec<usize> = shape[fixed_coords.len()..].to_vec();

    // Calculate how many elements we need
    let result_size: usize = result_shape.iter().product();
    let mut result_data = Vec::with_capacity(result_size);

    // Generate all combinations for the remaining dimensions
    let mut remaining_coords = vec![0; result_shape.len()];

    loop {
        // Build full coordinates
        let mut full_coords = fixed_coords.to_vec();
        full_coords.extend_from_slice(&remaining_coords);

        // Get the element
        match tensor.get(&full_coords) {
            Ok(&val) => result_data.push(val),
            Err(e) => return Err(format!("Invalid coordinates {:?} for tensor: {}", full_coords, e)),
        }

        // Increment remaining_coords
        let mut carry = true;
        for i in (0..remaining_coords.len()).rev() {
            if carry {
                remaining_coords[i] += 1;
                if remaining_coords[i] < result_shape[i] {
                    carry = false;
                } else {
                    remaining_coords[i] = 0;
                }
            }
        }

        if carry {
            break; // We've iterated through all combinations
        }
    }

    // Create the result tensor
    if result_shape.len() == 1 {
        // Return as vector for 1D
        Ok(Value::Vector(result_data.iter().map(|&c| Value::Complex(c)).collect()))
    } else {
        ComplexTensor::new(result_data, result_shape)
            .map(Value::ComplexTensor)
            .map_err(|e| format!("Failed to create subtensor: {}", e))
    }
}

/// Slice a complex tensor
fn slice_complex_tensor(tensor: &ComplexTensor, indices: &[EvaluatedIndex]) -> Result<Value, String> {
    let shape = tensor.shape();

    // Build ranges for each dimension
    let mut ranges = Vec::new();
    for (i, idx) in indices.iter().enumerate() {
        match idx {
            EvaluatedIndex::Single(n) => {
                let actual = normalize_index(*n, shape[i])?;
                ranges.push((actual, actual + 1));
            }
            EvaluatedIndex::Range { start, end } => {
                let (s, e) = normalize_range(*start, *end, shape[i])?;
                ranges.push((s, e));
            }
        }
    }

    // Add full ranges for remaining dimensions
    for i in indices.len()..shape.len() {
        ranges.push((0, shape[i]));
    }

    // Calculate result shape (exclude single-index dimensions)
    let mut result_shape = Vec::new();
    for (i, idx) in indices.iter().enumerate() {
        if let EvaluatedIndex::Range { .. } = idx {
            result_shape.push(ranges[i].1 - ranges[i].0);
        }
    }
    // Add remaining full dimensions
    for i in indices.len()..shape.len() {
        result_shape.push(ranges[i].1 - ranges[i].0);
    }

    // Extract data
    let result_size: usize = result_shape.iter().product();
    let mut result_data = Vec::with_capacity(result_size);

    // Generate coordinates within the ranges
    let mut coords = ranges.iter().map(|(s, _)| *s).collect::<Vec<_>>();

    loop {
        // Get element at current coordinates
        match tensor.get(&coords) {
            Ok(&val) => result_data.push(val),
            Err(e) => return Err(format!("Invalid coordinates {:?} for tensor: {}", coords, e)),
        }

        // Increment coordinates
        let mut carry = true;
        for i in (0..coords.len()).rev() {
            if carry {
                coords[i] += 1;
                if coords[i] < ranges[i].1 {
                    carry = false;
                } else {
                    coords[i] = ranges[i].0;
                }
            }
        }

        if carry {
            break;
        }
    }

    // Create result
    if result_shape.is_empty() || (result_shape.len() == 1 && result_shape[0] == 1) {
        // Single element
        Ok(Value::Complex(result_data[0]))
    } else if result_shape.len() == 1 {
        // Vector
        Ok(Value::Vector(result_data.iter().map(|&c| Value::Complex(c)).collect()))
    } else {
        // Tensor
        ComplexTensor::new(result_data, result_shape)
            .map(Value::ComplexTensor)
            .map_err(|e| format!("Failed to create sliced tensor: {}", e))
    }
}
