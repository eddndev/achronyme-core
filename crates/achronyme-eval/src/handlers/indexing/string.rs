/// String indexing and slicing operations

use achronyme_types::value::Value;

use super::types::EvaluatedIndex;
use super::utils::{normalize_index, normalize_range};

/// Index into a string
pub fn index_string(s: &str, indices: &[EvaluatedIndex]) -> Result<Value, String> {
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
