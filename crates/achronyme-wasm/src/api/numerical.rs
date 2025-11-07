use wasm_bindgen::prelude::*;
use crate::state::{Handle, HANDLES, EVALUATOR};
use achronyme_types::value::Value;

// ============================================================================
// Numerical Calculus Functions
// ============================================================================

/// Numerical first derivative using central difference
/// diff(f, x, h) - computes f'(x) ≈ (f(x+h) - f(x-h)) / (2h)
#[wasm_bindgen(js_name = numDiff)]
pub fn num_diff(func_handle: Handle, x: f64, h: f64) -> Result<f64, JsValue> {

    HANDLES.with(|handles| {
        let func = {
            let h = handles.borrow();
            match h.get(func_handle) {
                Some(Value::Function(f)) => Ok(f.clone()),
                Some(_) => Err(JsValue::from_str("diff requires a function handle")),
                None => Err(JsValue::from_str("Invalid function handle")),
            }
        }?;

        EVALUATOR.with(|evaluator| {
            let mut eval = evaluator.borrow_mut();
            use achronyme_numerical::diff_central;
            diff_central(&mut *eval, &func, x, h)
                .map_err(|e| JsValue::from_str(&e))
        })
    })
}

/// Numerical second derivative
/// diff2(f, x, h) - computes f''(x)
#[wasm_bindgen(js_name = numDiff2)]
pub fn num_diff2(func_handle: Handle, x: f64, h: f64) -> Result<f64, JsValue> {

    HANDLES.with(|handles| {
        let func = {
            let h = handles.borrow();
            match h.get(func_handle) {
                Some(Value::Function(f)) => Ok(f.clone()),
                Some(_) => Err(JsValue::from_str("diff2 requires a function handle")),
                None => Err(JsValue::from_str("Invalid function handle")),
            }
        }?;

        EVALUATOR.with(|evaluator| {
            let mut eval = evaluator.borrow_mut();
            use achronyme_numerical::diff2_central;
            diff2_central(&mut *eval, &func, x, h)
                .map_err(|e| JsValue::from_str(&e))
        })
    })
}

/// Numerical third derivative
/// diff3(f, x, h) - computes f'''(x)
#[wasm_bindgen(js_name = numDiff3)]
pub fn num_diff3(func_handle: Handle, x: f64, h: f64) -> Result<f64, JsValue> {

    HANDLES.with(|handles| {
        let func = {
            let h = handles.borrow();
            match h.get(func_handle) {
                Some(Value::Function(f)) => Ok(f.clone()),
                Some(_) => Err(JsValue::from_str("diff3 requires a function handle")),
                None => Err(JsValue::from_str("Invalid function handle")),
            }
        }?;

        EVALUATOR.with(|evaluator| {
            let mut eval = evaluator.borrow_mut();
            use achronyme_numerical::diff3_central;
            diff3_central(&mut *eval, &func, x, h)
                .map_err(|e| JsValue::from_str(&e))
        })
    })
}

/// Numerical integration using trapezoidal rule
/// integral(f, a, b, n) - computes ∫f(x)dx from a to b using n subdivisions
#[wasm_bindgen(js_name = numIntegral)]
pub fn num_integral(func_handle: Handle, a: f64, b: f64, n: usize) -> Result<f64, JsValue> {

    HANDLES.with(|handles| {
        let func = {
            let h = handles.borrow();
            match h.get(func_handle) {
                Some(Value::Function(f)) => Ok(f.clone()),
                Some(_) => Err(JsValue::from_str("integral requires a function handle")),
                None => Err(JsValue::from_str("Invalid function handle")),
            }
        }?;

        EVALUATOR.with(|evaluator| {
            let mut eval = evaluator.borrow_mut();
            use achronyme_numerical::trapz;
            trapz(&mut *eval, &func, a, b, n)
                .map_err(|e| JsValue::from_str(&e))
        })
    })
}

/// Numerical integration using Simpson's rule
/// simpson(f, a, b, n) - more accurate than trapezoidal
#[wasm_bindgen(js_name = numSimpson)]
pub fn num_simpson(func_handle: Handle, a: f64, b: f64, n: usize) -> Result<f64, JsValue> {

    HANDLES.with(|handles| {
        let func = {
            let h = handles.borrow();
            match h.get(func_handle) {
                Some(Value::Function(f)) => Ok(f.clone()),
                Some(_) => Err(JsValue::from_str("simpson requires a function handle")),
                None => Err(JsValue::from_str("Invalid function handle")),
            }
        }?;

        EVALUATOR.with(|evaluator| {
            let mut eval = evaluator.borrow_mut();
            use achronyme_numerical::simpson;
            simpson(&mut *eval, &func, a, b, n)
                .map_err(|e| JsValue::from_str(&e))
        })
    })
}

/// Romberg integration (adaptive, high accuracy)
/// romberg(f, a, b, tol) - uses Richardson extrapolation
#[wasm_bindgen(js_name = numRomberg)]
pub fn num_romberg(func_handle: Handle, a: f64, b: f64, tol: f64) -> Result<f64, JsValue> {

    HANDLES.with(|handles| {
        let func = {
            let h = handles.borrow();
            match h.get(func_handle) {
                Some(Value::Function(f)) => Ok(f.clone()),
                Some(_) => Err(JsValue::from_str("romberg requires a function handle")),
                None => Err(JsValue::from_str("Invalid function handle")),
            }
        }?;

        EVALUATOR.with(|evaluator| {
            let mut eval = evaluator.borrow_mut();
            use achronyme_numerical::romberg;
            romberg(&mut *eval, &func, a, b, tol, 20)
                .map_err(|e| JsValue::from_str(&e))
        })
    })
}

/// Adaptive quadrature integration
/// quad(f, a, b) - automatically adapts to achieve high accuracy
#[wasm_bindgen(js_name = numQuad)]
pub fn num_quad(func_handle: Handle, a: f64, b: f64) -> Result<f64, JsValue> {

    HANDLES.with(|handles| {
        let func = {
            let h = handles.borrow();
            match h.get(func_handle) {
                Some(Value::Function(f)) => Ok(f.clone()),
                Some(_) => Err(JsValue::from_str("quad requires a function handle")),
                None => Err(JsValue::from_str("Invalid function handle")),
            }
        }?;

        EVALUATOR.with(|evaluator| {
            let mut eval = evaluator.borrow_mut();
            use achronyme_numerical::quad;
            quad(&mut *eval, &func, a, b, 1e-10)
                .map_err(|e| JsValue::from_str(&e))
        })
    })
}

/// Root finding using bisection method
/// solve(f, a, b, tol) - finds x where f(x) = 0 in interval [a,b]
#[wasm_bindgen(js_name = numSolve)]
pub fn num_solve(func_handle: Handle, a: f64, b: f64, tol: f64) -> Result<f64, JsValue> {

    HANDLES.with(|handles| {
        let func = {
            let h = handles.borrow();
            match h.get(func_handle) {
                Some(Value::Function(f)) => Ok(f.clone()),
                Some(_) => Err(JsValue::from_str("solve requires a function handle")),
                None => Err(JsValue::from_str("Invalid function handle")),
            }
        }?;

        EVALUATOR.with(|evaluator| {
            let mut eval = evaluator.borrow_mut();
            use achronyme_numerical::bisect;
            bisect(&mut *eval, &func, a, b, tol)
                .map_err(|e| JsValue::from_str(&e))
        })
    })
}

/// Newton's method for root finding
/// newton(f, df, x0, tol, maxIter) - requires both function and its derivative
#[wasm_bindgen(js_name = numNewton)]
pub fn num_newton(
    func_handle: Handle,
    dfunc_handle: Handle,
    x0: f64,
    tol: f64,
    max_iter: usize,
) -> Result<f64, JsValue> {

    HANDLES.with(|handles| {
        let (func, dfunc) = {
            let h = handles.borrow();
            let f = match h.get(func_handle) {
                Some(Value::Function(f)) => Ok(f.clone()),
                Some(_) => Err(JsValue::from_str("newton requires a function handle")),
                None => Err(JsValue::from_str("Invalid function handle")),
            }?;

            let df = match h.get(dfunc_handle) {
                Some(Value::Function(f)) => Ok(f.clone()),
                Some(_) => Err(JsValue::from_str("newton requires a derivative function handle")),
                None => Err(JsValue::from_str("Invalid derivative handle")),
            }?;

            Ok::<_, JsValue>((f, df))
        }?;

        EVALUATOR.with(|evaluator| {
            let mut eval = evaluator.borrow_mut();
            use achronyme_numerical::newton;
            newton(&mut *eval, &func, &dfunc, x0, tol, max_iter)
                .map_err(|e| JsValue::from_str(&e))
        })
    })
}

/// Secant method for root finding
/// secant(f, x0, x1, tol, maxIter) - doesn't require derivative
#[wasm_bindgen(js_name = numSecant)]
pub fn num_secant(
    func_handle: Handle,
    x0: f64,
    x1: f64,
    tol: f64,
    max_iter: usize,
) -> Result<f64, JsValue> {

    HANDLES.with(|handles| {
        let func = {
            let h = handles.borrow();
            match h.get(func_handle) {
                Some(Value::Function(f)) => Ok(f.clone()),
                Some(_) => Err(JsValue::from_str("secant requires a function handle")),
                None => Err(JsValue::from_str("Invalid function handle")),
            }
        }?;

        EVALUATOR.with(|evaluator| {
            let mut eval = evaluator.borrow_mut();
            use achronyme_numerical::secant;
            secant(&mut *eval, &func, x0, x1, tol, max_iter)
                .map_err(|e| JsValue::from_str(&e))
        })
    })
}
