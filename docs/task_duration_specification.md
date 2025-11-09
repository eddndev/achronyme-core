# Task Duration Specification in PERT/CPM

Achronyme supports three flexible ways to specify task durations in PERT/CPM networks.

## Priority Order

When extracting duration from a node, the system uses the following priority:

1. **`duration`** - Explicit deterministic duration (highest priority)
2. **`te`** - Explicit expected time (pre-calculated)
3. **`(op, mo, pe)`** - Three-point estimate (calculated automatically using PERT formula)

This priority system allows you to:
- Override calculated values when needed (use explicit `duration` or `te`)
- Get automatic calculation for convenience (use only `op`, `mo`, `pe`)
- Mix different specification methods in the same network

## Method 1: Deterministic Duration (`duration`)

**Use when**: Task has a fixed, known duration

**Properties**: `{duration: number}`

**Example**:
```javascript
let project = network(
    [A -> B, B -> C],
    {
        A: {duration: 5},
        B: {duration: 3},
        C: {duration: 2}
    }
)
```

**Use cases**:
- CPM (Critical Path Method) analysis
- Projects with well-defined, fixed task durations
- Historical data with accurate time estimates

---

## Method 2: Explicit Expected Time (`te`)

**Use when**: You have a pre-calculated expected time

**Properties**: `{te: number}`

**Example**:
```javascript
let project = network(
    [A -> B, B -> C],
    {
        A: {te: 3.5},
        B: {te: 5.2},
        C: {te: 2.8}
    }
)
```

**Use cases**:
- You've calculated `te` using a custom formula
- You want explicit control over the expected time
- You're importing data where `te` is already computed

---

## Method 3: Three-Point Estimate (`op`, `mo`, `pe`)

**Use when**: Task duration has uncertainty

**Properties**: `{op: number, mo: number, pe: number}`

**Automatic Calculation**: `te = (op + 4*mo + pe) / 6`

**Example**:
```javascript
let project = network(
    [A -> B, B -> C],
    {
        A: {op: 2, mo: 3, pe: 5},    // te calculated as 3.17
        B: {op: 4, mo: 5, pe: 8},    // te calculated as 5.33
        C: {op: 1, mo: 2, pe: 3}     // te calculated as 2.0
    }
)
```

**Validation**: Must satisfy `op ≤ mo ≤ pe`

**Use cases**:
- PERT probabilistic analysis
- Projects with uncertain task durations
- Need variance and standard deviation calculations

---

## Mixing Methods

You can mix different methods in the same network:

```javascript
let project = network(
    [A -> B, B -> C, C -> D],
    {
        A: {duration: 5},              // Fixed duration
        B: {op: 3, mo: 4, pe: 6},      // Calculated te = 4.17
        C: {te: 2.5},                  // Explicit te
        D: {op: 1, mo: 1, pe: 1}       // Calculated te = 1.0
    }
)

project_duration(project)  // Works: 5 + 4.17 + 2.5 + 1.0 = 12.67
```

**Priority Rules**:
- If a node has `duration`, it's always used (even if `te` or `op/mo/pe` exist)
- If a node has `te` but no `duration`, `te` is used (even if `op/mo/pe` exist)
- If a node has only `op/mo/pe`, `te` is calculated automatically

---

## Validation

### For `duration` or `te`:
- Must be a number ≥ 0

### For `op/mo/pe`:
- All three must be numbers ≥ 0
- Must satisfy: `op ≤ mo ≤ pe`

### Error Messages:

```javascript
// Missing duration specification
{A: {cost: 100}}
// Error: Node 'A' must have 'duration', 'te', or ('op', 'mo', 'pe') properties

// Invalid PERT estimates
{A: {op: 5, mo: 3, pe: 2}}
// Error: Invalid PERT estimates: op <= mo <= pe required (got op=5, mo=3, pe=2)

// Negative duration
{A: {duration: -5}}
// Error: Node 'A' has negative duration: -5
```

---

## Best Practices

### Use `duration` when:
✅ Task has fixed, well-known duration
✅ Doing deterministic CPM analysis
✅ Historical data provides accurate estimates

### Use `te` when:
✅ You've calculated expected time using custom formula
✅ You want explicit control (override automatic calculation)
✅ Importing pre-calculated PERT data

### Use `op/mo/pe` when:
✅ Task duration has uncertainty
✅ Need probabilistic PERT analysis
✅ Want automatic `te` calculation
✅ Need variance/std deviation for risk analysis

---

## Complete Example

```javascript
// Software development project mixing all methods
let dev_project = network(
    [
        Planning -> Design,
        Design -> Frontend,
        Design -> Backend,
        Frontend -> Testing,
        Backend -> Testing,
        Testing -> Deployment
    ],
    {
        Planning: {duration: 5},                    // Fixed: planning always 5 days
        Design: {op: 3, mo: 5, pe: 8},             // Uncertain: te = 5.17
        Frontend: {op: 10, mo: 12, pe: 16},        // Uncertain: te = 12.33
        Backend: {op: 12, mo: 15, pe: 20},         // Uncertain: te = 15.33
        Testing: {te: 4.5},                        // Explicit: team's estimate
        Deployment: {duration: 1}                   // Fixed: always 1 day
    }
)

// All PERT/CPM functions work seamlessly
let duration = project_duration(dev_project)              // 43.33 days
let critical = critical_path(...)                         // Find critical path
let variance = project_variance(dev_project)              // Calculate risk
let prob = completion_probability(dev_project, 45)        // 68% chance in 45 days
```

---

## Implementation Details

The `get_node_duration()` function implements this logic:

```rust
fn get_node_duration(node_props: &HashMap<String, Value>) -> Result<f64, String> {
    // Priority 1: duration
    if let Some(Value::Number(d)) = node_props.get("duration") {
        return Ok(*d);
    }

    // Priority 2: te (expected time)
    if let Some(Value::Number(t)) = node_props.get("te") {
        return Ok(*t);
    }

    // Priority 3: Calculate from op, mo, pe
    if let (Some(Value::Number(op)), Some(Value::Number(mo)), Some(Value::Number(pe))) =
        (node_props.get("op"), node_props.get("mo"), node_props.get("pe")) {
        // Validate and calculate
        return Ok((op + 4.0 * mo + pe) / 6.0);
    }

    Err("Node must have 'duration', 'te', or ('op', 'mo', 'pe') properties".to_string())
}
```
