/// Utilities for index normalization
///
/// Provides functions to handle negative indices (Python-style) and range normalization.

/// Normalize an index (handle negative indices like Python)
pub fn normalize_index(idx: isize, len: usize) -> Result<usize, String> {
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
pub fn normalize_range(start: Option<isize>, end: Option<isize>, len: usize) -> Result<(usize, usize), String> {
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
