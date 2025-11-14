/// Branch & Bound tree node representing a subproblem
///
/// Each node represents a region of the search space defined by
/// variable bounds: lower[i] ≤ xᵢ ≤ upper[i]
#[derive(Clone)]
pub struct BBNode {
    pub lower_bounds: Vec<f64>,
    pub upper_bounds: Vec<f64>,
}

impl BBNode {
    /// Create a new node with specified bounds
    pub fn new(lower_bounds: Vec<f64>, upper_bounds: Vec<f64>) -> Self {
        Self {
            lower_bounds,
            upper_bounds,
        }
    }

    /// Create initial root node with default bounds [0, ∞)
    pub fn initial(n: usize) -> Self {
        Self {
            lower_bounds: vec![0.0; n],
            upper_bounds: vec![f64::INFINITY; n],
        }
    }
}
