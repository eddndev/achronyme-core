// crates/achronyme-eval/src/function_modules/graphs/mod.rs

use crate::functions::FunctionRegistry;

pub mod helpers;
pub mod network;
pub mod traversal;
pub mod shortest_path;
pub mod cycles;
pub mod mst;
pub mod connectivity;
pub mod topological;
pub mod pert;

pub fn register_functions(registry: &mut FunctionRegistry) {
    // Network constructor and basic operations
    registry.register("network", network::network, -1); // Variadic: 1 or 2 arguments
    registry.register("nodes", network::nodes, 1);
    registry.register("edges", network::edges, 1);
    registry.register("neighbors", network::neighbors, 2);
    registry.register("degree", network::degree, 2);

    // Graph algorithms - Traversal
    registry.register("bfs", traversal::bfs, 2);
    registry.register("dfs", traversal::dfs, 2);
    registry.register("bfs_path", traversal::bfs_path, 3);

    // Graph algorithms - Shortest Paths
    registry.register("dijkstra", shortest_path::dijkstra, 3);

    // Graph algorithms - Cycle Detection
    registry.register("has_cycle", cycles::has_cycle, 1);

    // Graph algorithms - Minimum Spanning Tree
    registry.register("kruskal", mst::kruskal, 1);
    registry.register("prim", mst::prim, 2);

    // Graph algorithms - Connectivity
    registry.register("connected_components", connectivity::connected_components, 1);
    registry.register("is_connected", connectivity::is_connected, 1);

    // Graph algorithms - Topological Sort
    registry.register("topological_sort", topological::topological_sort, 1);

    // PERT/CPM - Critical Path Method (Costos)
    registry.register("forward_pass", pert::forward_pass, 1);
    registry.register("backward_pass", pert::backward_pass, 1);
    registry.register("calculate_slack", pert::calculate_slack, 1);
    registry.register("critical_path", pert::critical_path, 1);
    registry.register("all_critical_paths", pert::all_critical_paths, 1);
    registry.register("project_duration", pert::project_duration, 1);

    // PERT - Probabilistic Analysis
    registry.register("expected_time", pert::expected_time, 3);
    registry.register("task_variance", pert::task_variance, 3);
    registry.register("project_variance", pert::project_variance, 1);
    registry.register("project_std_dev", pert::project_std_dev, 1);
    registry.register("completion_probability", pert::completion_probability, 2);
    registry.register("time_for_probability", pert::time_for_probability, 2);

    // PERT - Comprehensive Analysis
    registry.register("pert_analysis", pert::pert_analysis, 1);
}
