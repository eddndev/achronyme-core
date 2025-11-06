pub mod linear;
pub mod integer;

// Re-exports convenientes para el usuario
pub use linear::simplex::{solve as simplex_solve, objective_value};
pub use linear::linprog::solve as linprog_solve;
pub use linear::dual_simplex::solve as dual_simplex_solve;
pub use linear::two_phase::solve as two_phase_solve;
pub use linear::revised_simplex::solve as revised_simplex_solve;
pub use linear::sensitivity::{shadow_price, sensitivity_b, sensitivity_c};
pub use integer::{intlinprog, binary_linprog};
