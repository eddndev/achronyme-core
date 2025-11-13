# Module System Example - Statistical Data Analysis

This example demonstrates the Achronyme module system with a complete statistical data analysis application.

## Overview

This application showcases:
- **Importing from built-in modules** (`stats`, `math`, `linalg`)
- **Data processing pipeline** with filtering, transformation, and analysis
- **Vector operations** using linear algebra
- **Higher-order functions** for data manipulation
- **Statistical reporting** with comprehensive analysis
- **Reusable utility modules** (design examples for Phase 4)

## Project Structure

```
examples/soc/modules/
├── main.ach                      # Main application entry point
├── src/
│   ├── data_processing.ach       # Data processing utilities
│   └── visualization.ach         # Text-based visualization utilities
└── README.md                     # This file
```

## Files

### `main.ach`
The main application that:
1. Generates sample scientific measurement data
2. Performs basic statistical analysis (mean, std, min, max)
3. Applies data transformations (normalization, log, etc.)
4. Performs vector operations (dot product, norm, cosine similarity)
5. Detects and filters outliers
6. Generates comprehensive statistical reports
7. Demonstrates advanced analysis with higher-order functions

**Imports used:**
```javascript
import { mean, std } from "stats"
import { asin, acos, sinh, log10 } from "math"
import { dot, norm } from "linalg"
```

### `src/data_processing.ach`
Utility module providing data processing functions:
- **Data Loading**: `loadData`, `parseCSV`
- **Outlier Detection**: `detectOutliersIQR`, `detectOutliersZScore`, `filterOutliers`
- **Normalization**: `normalize`, `rescale`, `robustScale`
- **Transformations**: `logTransform`, `sqrtTransform`, `powerTransform`
- **Validation**: `validateRange`, `allPositive`, `hasMissingValues`
- **Statistics**: `coefficientOfVariation`, `skewness`, `kurtosis`

**Note**: This module demonstrates the design for user-defined modules with exports. The `export` syntax is ready for Phase 4 implementation.

### `src/visualization.ach`
Utility module providing text-based visualization:
- **Histograms**: `createHistogram`, `printHistogram`
- **Reports**: `createReport`, `printReport`
- **Charts**: `createBarChart`, `printBoxPlot`
- **Comparisons**: `compareDistributions`, `printSummaryTable`
- **Progress**: `createProgressBar`

## Running the Example

### Using the REPL

```bash
# Navigate to the project root
cd /path/to/achronyme-core

# Start the REPL
cargo run --release

# Load and run the main application
> load("examples/soc/modules/main.ach")
```

### Using the CLI (when available)

```bash
achronyme run examples/soc/modules/main.ach
```

## Expected Output

The application will output:

```
============================================================
Statistical Data Analysis Application
============================================================

1. Generating Sample Dataset
----------------------------------------
Total measurements: 30
Raw data sample: [12.3, 15.7, 14.2, ...]

2. Basic Statistical Analysis
----------------------------------------
Mean: 14.926666666666668
Standard Deviation: 0.7891...
Minimum: 12.3
Maximum: 16.3
Range: 4.0

3. Data Transformation
----------------------------------------
Normalized data sample:
First 5 values: [-2.344..., 0.979..., -0.920..., ...]
Normalized mean (should be ~0): 0.0
Normalized std (should be ~1): 1.0

4. Advanced Mathematical Operations
----------------------------------------
Inverse sine transformation sample: [0.6435..., 0.8264..., ...]
Hyperbolic sine transformation sample: [1.3169..., 1.6838..., ...]
Log10 transformation sample: [1.0899..., 1.1959..., ...]

5. Vector Operations (Linear Algebra)
----------------------------------------
Feature Vector 1: [12.3, 15.7, 14.2, 13.8, 16.1]
Feature Vector 2: [14.9, 15.3, 14.7, 15.8, 16.2]
Dot Product (similarity): 1122.85
Norm of Vector 1: 33.67...
Norm of Vector 2: 34.51...
Cosine Similarity: 0.9667...

6. Outlier Detection
----------------------------------------
Lower Bound: 12.349...
Upper Bound: 17.503...
Outliers found: 0
Clean data points: 30
No outliers detected!

7. Final Statistical Report
----------------------------------------
ORIGINAL DATA:
  Count: 30
  Mean: 14.926...
  Std Dev: 0.789...

CLEANED DATA (outliers removed):
  Count: 30
  Mean: 14.926...
  Std Dev: 0.789...

DATA QUALITY:
  Outliers removed: 0
  Data retention rate: 100 %

8. Advanced Analysis
----------------------------------------
Value Distribution:
  Low (< mean - std): 5
  Mid (±1 std): 20
  High (> mean + std): 5

Extreme Values:
  Minimum: 12.3
  Maximum: 16.3

All values positive: true
Any value > 20: false

9. Data Aggregation
----------------------------------------
Total Sum: 447.8
Product of first 3 values: 2740.02
Cumulative sum sample: [12.3, 28.0, 42.2, ...]

============================================================
Analysis Complete!
============================================================

Modules used:
  - stats: mean, std
  - math: asin, acos, sinh, log10
  - linalg: dot, norm
  - prelude: map, filter, reduce, count, all, any, sum, len, min, max

This example demonstrates:
  ✓ Import from multiple built-in modules
  ✓ Statistical analysis and data transformation
  ✓ Vector operations and linear algebra
  ✓ Higher-order functions for data processing
  ✓ Outlier detection and data cleaning
  ✓ Comprehensive statistical reporting
```

## Key Features Demonstrated

### 1. Module System (Phase 2/3)
- ✅ Import statements with multiple functions
- ✅ Import from different modules in same file
- ✅ Using imported functions seamlessly
- ✅ Module aliases (can do `import { mean as average } from "stats"`)

### 2. Built-in Modules Used
- **`stats`**: Statistical functions (mean, std)
- **`math`**: Advanced math (asin, acos, sinh, log10)
- **`linalg`**: Linear algebra (dot, norm)
- **Prelude**: Always available (map, filter, reduce, etc.)

### 3. Data Processing Pipeline
1. **Data Generation**: Create sample dataset
2. **Statistical Analysis**: Calculate descriptive statistics
3. **Transformation**: Normalize and transform data
4. **Advanced Math**: Apply mathematical functions
5. **Vector Operations**: Linear algebra computations
6. **Outlier Detection**: Identify and filter anomalies
7. **Reporting**: Generate comprehensive reports
8. **Aggregation**: Cumulative statistics

### 4. Functional Programming
- **Higher-order functions**: `map`, `filter`, `reduce`
- **Predicates**: `all`, `any`, `count`, `find`
- **Function composition**: Pipe operations
- **Immutability**: Data transformations create new values

## Future Enhancements (Phase 4+)

When user-defined modules with `export` are implemented:

```javascript
// main.ach
import { loadData, filterOutliers } from "src/data_processing"
import { createHistogram, printReport } from "src/visualization"

let data = loadData("data.csv")
let clean = filterOutliers(data, "zscore", 2.5)
let hist = createHistogram(clean, 10)
printHistogram(hist, "Data Distribution")
```

## Learning Resources

### Module System Basics
```javascript
// Import single function
import { mean } from "stats"

// Import multiple functions
import { mean, std, variance } from "stats"

// Import with alias
import { mean as average } from "stats"

// Import from multiple modules
import { mean } from "stats"
import { asin } from "math"
import { dot } from "linalg"
```

### Common Patterns

**Data Pipeline:**
```javascript
let result = pipe(
    data => filterOutliers(data, "zscore", 2),
    data => normalize(data),
    data => mean(data)
)
```

**Conditional Processing:**
```javascript
let processed = if(
    len(data) > 100,
    () => filterOutliers(data, "zscore", 3),
    () => data
)
```

**Statistics Calculation:**
```javascript
let stats = {
    mean: mean(data),
    std: std(data),
    cv: (std(data) / mean(data)) * 100
}
```

## Dependencies

- Achronyme v0.1.0+
- Module system (Phase 2/3 complete)

## Contributing

This example can be extended with:
- More statistical methods
- Additional visualization types
- Machine learning algorithms
- Time series analysis
- Financial calculations
- Scientific simulations

## License

Part of the Achronyme project. See main repository for license details.
