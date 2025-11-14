/// Vector indexing and slicing operations

use achronyme_types::value::Value;
use achronyme_types::tensor::RealTensor;

use super::types::EvaluatedIndex;
use super::utils::{normalize_index, normalize_range};

/// Index into a generic vector
pub fn index_vector(vec: &[Value], indices: &[EvaluatedIndex]) -> Result<Value, String> {
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
