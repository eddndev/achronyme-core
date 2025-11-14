/// Types for representing evaluated indices

/// Represents an evaluated index
#[derive(Debug, Clone)]
pub enum EvaluatedIndex {
    Single(isize),  // Single index (can be negative for reverse indexing)
    Range {
        start: Option<isize>,
        end: Option<isize>,
    },
}
