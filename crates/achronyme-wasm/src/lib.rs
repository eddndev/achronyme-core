/**
 * Achronyme WASM Bindings
 *
 * Expone el evaluador Rust a JavaScript/WebAssembly con una API compatible
 * con la implementación de C++ para mantener compatibilidad con el SDK TypeScript.
 */

mod api;
mod state;



// Core API
pub use api::core::{eval, reset, eval_to_handle};

// Memory API
pub use api::memory::{
    create_vector_from_buffer,
    create_vector,
    get_vector,
    get_matrix,
    create_matrix,
    create_matrix_from_buffer,
    create_complex_vector,
    get_complex_vector,
    bind_variable_to_handle,
    release_handle,
};

// Math Operations
pub use api::math::{
    math_sin,
    math_cos,
    math_tan,
    math_exp,
    math_ln,
    math_abs,
    math_sqrt,
    math_asin,
    math_acos,
    math_atan,
    math_sinh,
    math_cosh,
    math_tanh,
    math_asinh,
    math_acosh,
    math_atanh,
    math_ceil,
    math_floor,
    math_round,
    math_trunc,
    math_cbrt,
    math_log10,
    linspace,
    vadd,
    vsub,
    vmul,
    vdiv,
    dot,
};

// Statistics Operations
pub use api::stats::{
    sum,
    mean,
    std,
    min,
    max,
    norm,
    norm_l1,
};

// DSP Operations
pub use api::dsp::{
    dsp_fft,
    dsp_fft_mag,
    ifft,
    hanning_window,
    hamming_window,
    blackman_window,
};

// Linear Algebra Operations
pub use api::linalg::{
    LuResult,
    lu_decomposition,
    matrix_inverse,
    QrResult,
    qr_decomposition,
    cholesky_decomposition,
    SvdResult,
    svd_decomposition,
    PowerIterationResult,
    power_iteration,
    qr_eigenvalues,
    EigenResult,
    eigen_symmetric,
    is_symmetric,
    is_positive_definite,
    identity,
};

// Numerical Calculus
pub use api::numerical::{
    num_diff,
    num_diff2,
    num_diff3,
    num_integral,
    num_simpson,
    num_romberg,
    num_quad,
    num_solve,
    num_newton,
    num_secant,
};

// Optimization Solvers
pub use api::solver::{
    simplex,
    linprog,
    dual_simplex,
    two_phase_simplex,
    revised_simplex,
    objective_value,
    shadow_price,
    sensitivity_c,
    sensitivity_b,
    intlinprog,
    binary_linprog,
};