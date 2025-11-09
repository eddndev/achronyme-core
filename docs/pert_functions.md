# PERT Functions Documentation

This document describes all PERT (Program Evaluation and Review Technique) functions implemented in Achronyme.

## Overview

PERT analysis is divided into two main approaches:
1. **PERT/CPM (Critical Path Method)** - Deterministic analysis for finding critical paths and project duration
2. **PERT/Probabilistic** - Stochastic analysis using three-point estimates for uncertainty quantification

## PERT/CPM Functions (Costos)

### `forward_pass(network)`
Calculates Early Start (ES) and Early Finish (EF) for all tasks.

**Input**: Network with nodes having `duration` or `te` property
**Output**: Network with ES and EF added to each node
**Algorithm**: Topological traversal from start nodes
- `ES[start] = 0`
- `ES[task] = max(EF[predecessors])`
- `EF[task] = ES[task] + duration[task]`

**Example**:
```javascript
let g = network([A -> B, B -> C], {A: {duration: 5}, B: {duration: 3}, C: {duration: 2}})
let with_times = forward_pass(g)
// with_times.nodes.B will have: {duration: 3, ES: 5, EF: 8}
```

---

### `backward_pass(network)`
Calculates Late Start (LS) and Late Finish (LF) for all tasks.

**Input**: Network with ES/EF already calculated (from forward_pass)
**Output**: Network with LS and LF added to each node
**Algorithm**: Reverse topological traversal from end nodes
- `LF[end] = EF[end]` (project completion time)
- `LF[task] = min(LS[successors])`
- `LS[task] = LF[task] - duration[task]`

**Example**:
```javascript
let with_times = forward_pass(g)
let with_late = backward_pass(with_times)
// with_late.nodes.B will have: {duration: 3, ES: 5, EF: 8, LS: 5, LF: 8}
```

---

### `calculate_slack(network)`
Calculates slack (float) for all tasks.

**Input**: Network with ES, EF, LS, LF calculated
**Output**: Network with `slack` added to each node
**Formula**: `slack = LS - ES` (or equivalently `LF - EF`)

**Interpretation**:
- `slack = 0`: Task is on critical path (no room for delay)
- `slack > 0`: Task has flexibility (can be delayed without affecting project)

**Example**:
```javascript
let with_slack = calculate_slack(backward_pass(forward_pass(g)))
// with_slack.nodes.B.slack = 0 (if B is on critical path)
```

---

### `critical_path(network)`
Identifies the critical path (sequence of tasks with zero slack).

**Input**: Network with slack calculated
**Output**: Vector of node IDs on the critical path (in topological order)

**Example**:
```javascript
let critical = critical_path(with_slack)
// Returns: ["A", "B", "C"] if all are on critical path
```

---

### `project_duration(network)`
Calculates total project duration.

**Input**: Network with `duration` or `te` on all nodes
**Output**: Number (maximum EF across all nodes)

**Example**:
```javascript
let duration = project_duration(g)
// Returns: 10 (sum along critical path)
```

---

## PERT/Probabilistic Functions

### `expected_time(op, mo, pe)`
Calculates expected time using PERT formula.

**Input**: Three numbers (optimistic, most likely, pessimistic)
**Output**: Expected time `te = (op + 4*mo + pe) / 6`
**Validation**: Requires `op ≤ mo ≤ pe`

**Example**:
```javascript
expected_time(2, 5, 8)
// Returns: 5.0
```

---

### `task_variance(op, mo, pe)`
Calculates variance for a single task.

**Input**: Three numbers (op, mo, pe)
**Output**: Variance `σ² = ((pe - op) / 6)²`

**Example**:
```javascript
task_variance(2, 5, 8)
// Returns: 1.0
```

---

### `project_variance(network)`
Calculates total project variance (sum of variances on critical path).

**Input**: Network with `op`, `mo`, `pe` on all nodes
**Output**: Total variance (number)
**Note**: Only tasks on the critical path contribute to project variance

**Example**:
```javascript
let variance = project_variance(g)
// Returns sum of variances along critical path
```

---

### `project_std_dev(network)`
Calculates project standard deviation.

**Input**: Network with `op`, `mo`, `pe`
**Output**: Standard deviation `σ = sqrt(project_variance)`

**Example**:
```javascript
let std_dev = project_std_dev(g)
// Returns: 2.5 (if variance is 6.25)
```

---

### `completion_probability(network, target_time)`
Calculates probability of completing project by target time.

**Input**:
- Network with `op`, `mo`, `pe`, `te` on all nodes
- Target completion time (number)

**Output**: Probability between 0 and 1
**Algorithm**: Uses normal distribution CDF
- Calculate z-score: `z = (target - te) / σ`
- Return `P(T ≤ target) = Φ(z)`

**Example**:
```javascript
completion_probability(g, 25)
// Returns: 0.84 (84% probability of completing in 25 days)
```

---

### `time_for_probability(network, probability)`
Calculates time needed for desired completion probability.

**Input**:
- Network with `op`, `mo`, `pe`, `te` on all nodes
- Desired probability (0 to 1)

**Output**: Time required (number)
**Algorithm**: Inverse normal CDF
- Find z-score for probability
- Calculate `time = te + z * σ`

**Example**:
```javascript
time_for_probability(g, 0.95)
// Returns: 27.3 (need 27.3 days for 95% confidence)
```

---

## Complete Workflow Example

```javascript
// Define project with uncertainty
let project = network(
    [A -> B, B -> C, B -> D, C -> E, D -> E],
    {
        A: {op: 2, mo: 3, pe: 5, te: 3.17},
        B: {op: 4, mo: 5, pe: 8, te: 5.33},
        C: {op: 1, mo: 2, pe: 3, te: 2.0},
        D: {op: 3, mo: 4, pe: 6, te: 4.17},
        E: {op: 2, mo: 3, pe: 4, te: 3.0}
    }
)

// CPM Analysis
let with_es_ef = forward_pass(project)
let with_ls_lf = backward_pass(with_es_ef)
let with_slack = calculate_slack(with_ls_lf)
let critical = critical_path(with_slack)
let duration = project_duration(project)

// Probabilistic Analysis
let variance = project_variance(project)
let std_dev = project_std_dev(project)
let prob_15 = completion_probability(project, 15)
let time_90 = time_for_probability(project, 0.90)
```

---

## Validation Requirements

### For CPM Functions:
- Network must be a DAG (no cycles)
- All nodes must have `duration` or `te` property (≥ 0)
- At least one start node (no predecessors)
- At least one end node (no successors)

### For Probabilistic Functions:
- All CPM requirements
- All nodes must have `op`, `mo`, `pe` properties
- Must satisfy: `op ≤ mo ≤ pe` for each node
- All times must be ≥ 0

---

## Implementation Notes

### Normal Distribution
- CDF and inverse CDF use high-precision approximations
- Error function uses Abramowitz and Stegun method
- Inverse CDF uses Beasley-Springer-Moro algorithm

### Critical Path Identification
- Uses floating-point comparison with epsilon (1e-6)
- Ensures slack ~= 0 accounts for rounding errors
- Preserves topological order in output

### Performance
- Forward/backward pass: O(V + E) where V=nodes, E=edges
- Critical path: O(V) after passes complete
- Probabilistic functions: O(V + E) plus small constant for distribution calculations
