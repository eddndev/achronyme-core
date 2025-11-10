/// Indexing and Slicing Handler
///
/// Handles indexing and slicing operations on tensors, vectors, and arrays.
/// Supports:
/// - Single element access: tensor[0, 1, 2]
/// - Range slicing: tensor[0, .., ..]
/// - Mixed indexing: tensor[0, 1..3]

use achronyme_parser::ast::IndexArg;
use achronyme_types::value::Value;
use achronyme_types::tensor::{RealTensor, ComplexTensor};

use crate::Evaluator;

/// Evaluate an index access operation
pub fn evaluate_index_access(
    evaluator: &mut Evaluator,
    object: &achronyme_parser::ast::AstNode,
    indices: &[IndexArg],
) -> Result<Value, String> {
    // Evaluate the object being indexed
    let obj_value = evaluator.evaluate(object)?;

    // Evaluate all index arguments
    let mut evaluated_indices = Vec::new();
    for idx in indices {
        evaluated_indices.push(evaluate_index_arg(evaluator, idx)?);
    }

    // Apply the indexing based on the object type
    match obj_value {
        Value::Vector(ref vec) => index_vector(vec, &evaluated_indices),
        Value::Tensor(ref tensor) => index_tensor(tensor, &evaluated_indices),
        Value::ComplexTensor(ref tensor) => index_complex_tensor(tensor, &evaluated_indices),
        Value::String(ref s) => index_string(s, &evaluated_indices),
        _ => Err(format!("Cannot index into {:?}", obj_value)),
    }
}

/// Evaluate an index argument (single index or range)
fn evaluate_index_arg(
    evaluator: &mut Evaluator,
    arg: &IndexArg,
) -> Result<EvaluatedIndex, String> {
    match arg {
        IndexArg::Single(expr) => {
            let value = evaluator.evaluate(expr)?;
            match value {
                Value::Number(n) => {
                    let idx = n as isize;
                    Ok(EvaluatedIndex::Single(idx))
                }
                _ => Err("Index must be a number".to_string()),
            }
        }
        IndexArg::Range { start, end } => {
            let start_idx = if let Some(start_expr) = start {
                let value = evaluator.evaluate(start_expr)?;
                match value {
                    Value::Number(n) => Some(n as isize),
                    _ => return Err("Range start must be a number".to_string()),
                }
            } else {
                None
            };

            let end_idx = if let Some(end_expr) = end {
                let value = evaluator.evaluate(end_expr)?;
                match value {
                    Value::Number(n) => Some(n as isize),
                    _ => return Err("Range end must be a number".to_string()),
                }
            } else {
                None
            };

            Ok(EvaluatedIndex::Range { start: start_idx, end: end_idx })
        }
    }
}

/// Represents an evaluated index
#[derive(Debug, Clone)]
enum EvaluatedIndex {
    Single(isize),  // Single index (can be negative for reverse indexing)
    Range {
        start: Option<isize>,
        end: Option<isize>,
    },
}

/// Index into a generic vector
fn index_vector(vec: &[Value], indices: &[EvaluatedIndex]) -> Result<Value, String> {
    if indices.len() != 1 {
        return Err(format!("Vector requires exactly 1 index, got {}", indices.len()));
    }

    match &indices[0] {
        EvaluatedIndex::Single(idx) => {
            let actual_idx = normalize_index(*idx, vec.len())?;
            vec.get(actual_idx)
                .cloned()
                .ok_or_else(|| format!("Index {} out of bounds for vector of length {}", idx, vec.len()))
        }
        EvaluatedIndex::Range { start, end } => {
            let (start_idx, end_idx) = normalize_range(*start, *end, vec.len())?;
            let slice = &vec[start_idx..end_idx];

            // If the vector contains only numbers, return as Tensor for consistency
            if slice.iter().all(|v| matches!(v, Value::Number(_))) {
                let nums: Vec<f64> = slice.iter().map(|v| {
                    if let Value::Number(n) = v {
                        *n
                    } else {
                        unreachable!()
                    }
                }).collect();
                return Ok(Value::Tensor(RealTensor::vector(nums)));
            }

            Ok(Value::Vector(slice.to_vec()))
        }
    }
}

/// Index into a string
fn index_string(s: &str, indices: &[EvaluatedIndex]) -> Result<Value, String> {
    let chars: Vec<char> = s.chars().collect();

    if indices.len() != 1 {
        return Err(format!("String requires exactly 1 index, got {}", indices.len()));
    }

    match &indices[0] {
        EvaluatedIndex::Single(idx) => {
            let actual_idx = normalize_index(*idx, chars.len())?;
            chars.get(actual_idx)
                .map(|c| Value::String(c.to_string()))
                .ok_or_else(|| format!("Index {} out of bounds for string of length {}", idx, chars.len()))
        }
        EvaluatedIndex::Range { start, end } => {
            let (start_idx, end_idx) = normalize_range(*start, *end, chars.len())?;
            let slice: String = chars[start_idx..end_idx].iter().collect();
            Ok(Value::String(slice))
        }
    }
}

/// Index into a real tensor
fn index_tensor(tensor: &RealTensor, indices: &[EvaluatedIndex]) -> Result<Value, String> {
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
        index_tensor_element(tensor, indices)
    } else {
        // At least one range - perform slicing
        slice_tensor(tensor, indices)
    }
}

/// Index into a complex tensor
fn index_complex_tensor(tensor: &ComplexTensor, indices: &[EvaluatedIndex]) -> Result<Value, String> {
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

/// Extract a single element from a real tensor
fn index_tensor_element(tensor: &RealTensor, indices: &[EvaluatedIndex]) -> Result<Value, String> {
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
        // Extract sub-tensor by fixing the first coords.len() dimensions
        return extract_subtensor(tensor, &coords);
    }

    // Get the element at the coordinates
    tensor.get(&coords)
        .map(|&val| Value::Number(val))
        .map_err(|e| format!("Index {:?} out of bounds for tensor shape {:?}: {}", coords, shape, e))
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

/// Extract a sub-tensor by fixing some dimensions
fn extract_subtensor(tensor: &RealTensor, fixed_coords: &[usize]) -> Result<Value, String> {
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
        Ok(Value::Vector(result_data.iter().map(|&n| Value::Number(n)).collect()))
    } else {
        RealTensor::new(result_data, result_shape)
            .map(Value::Tensor)
            .map_err(|e| format!("Failed to create subtensor: {}", e))
    }
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

/// Slice a real tensor
fn slice_tensor(tensor: &RealTensor, indices: &[EvaluatedIndex]) -> Result<Value, String> {
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
        Ok(Value::Number(result_data[0]))
    } else if result_shape.len() == 1 {
        // Vector
        Ok(Value::Vector(result_data.iter().map(|&n| Value::Number(n)).collect()))
    } else {
        // Tensor
        RealTensor::new(result_data, result_shape)
            .map(Value::Tensor)
            .map_err(|e| format!("Failed to create sliced tensor: {}", e))
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

/// Normalize an index (handle negative indices like Python)
fn normalize_index(idx: isize, len: usize) -> Result<usize, String> {
    let actual = if idx < 0 {
        // Negative index: count from the end
        let pos = len as isize + idx;
        if pos < 0 {
            return Err(format!("Index {} out of bounds for length {}", idx, len));
        }
        pos as usize
    } else {
        idx as usize
    };

    if actual >= len {
        Err(format!("Index {} out of bounds for length {}", idx, len))
    } else {
        Ok(actual)
    }
}

/// Normalize a range (handle None, negative indices)
fn normalize_range(start: Option<isize>, end: Option<isize>, len: usize) -> Result<(usize, usize), String> {
    let start_idx = match start {
        None => 0,
        Some(s) => {
            if s < 0 {
                let pos = len as isize + s;
                if pos < 0 {
                    0
                } else {
                    pos as usize
                }
            } else {
                s as usize
            }
        }
    };

    let end_idx = match end {
        None => len,
        Some(e) => {
            if e < 0 {
                let pos = len as isize + e;
                if pos < 0 {
                    0
                } else {
                    pos as usize
                }
            } else {
                (e as usize).min(len)
            }
        }
    };

    Ok((start_idx, end_idx))
}
