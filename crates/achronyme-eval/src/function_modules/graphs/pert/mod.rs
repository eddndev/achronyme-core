// PERT (Program Evaluation and Review Technique) Algorithms
// Includes both CPM (Critical Path Method) and Probabilistic PERT analysis

mod validation;
mod state_detection;
mod cpm;
mod critical_path;
mod project;
mod probabilistic;
mod statistics;

// Re-export public API
pub use cpm::{forward_pass, backward_pass, calculate_slack};
pub use critical_path::{critical_path, all_critical_paths};
pub use project::project_duration;
pub use probabilistic::{
    expected_time, task_variance, project_variance, project_std_dev,
    completion_probability, time_for_probability, pert_analysis
};
