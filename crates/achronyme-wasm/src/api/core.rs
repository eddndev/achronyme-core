use wasm_bindgen::prelude::*;
use crate::state::{EVALUATOR, HANDLES, Handle};
use achronyme_types::value::Value;
use achronyme_parser::pest_parser;
use crate::api::utils::format_value;

// ============================================================================
// Core Evaluation API (Compatible with C++ SDK)
// ============================================================================

/// Evalua una expresión y retorna el resultado como string
#[wasm_bindgen(js_name = eval)]
pub fn eval(expression: &str) -> Result<String, JsValue> {
    EVALUATOR.with(|evaluator| {
        let mut eval = evaluator.borrow_mut();

        // Parse using Pest parser
        let statements = pest_parser::parse(expression)
            .map_err(|e| JsValue::from_str(&e))?;

        // Evaluate each statement and return the last result
        let mut last_result = Value::Number(0.0);
        for ast in statements {
            last_result = eval.evaluate(&ast)
                .map_err(|e| JsValue::from_str(&e))?;
        }

        // Format result (compatible with C++ output format)
        Ok(format_value(&last_result))
    })
}

/// Resetea el evaluador y libera todos los handles
#[wasm_bindgen]
pub fn reset() {
    EVALUATOR.with(|evaluator| {
        *evaluator.borrow_mut() = achronyme_eval::evaluator::Evaluator::new();
    });
    HANDLES.with(|handles| {
        handles.borrow_mut().clear();
    });
}

/// Evalúa una expresión y retorna un handle al resultado
/// Útil para crear funciones lambda que luego se pasan a funciones numéricas
#[wasm_bindgen(js_name = evalToHandle)]
pub fn eval_to_handle(expression: &str) -> Result<Handle, JsValue> {
    EVALUATOR.with(|evaluator| {
        let mut eval = evaluator.borrow_mut();

        // Parse using Pest parser
        let statements = pest_parser::parse(expression)
            .map_err(|e| JsValue::from_str(&e))?;

        // Evaluate each statement and keep the last result
        let mut last_result = Value::Number(0.0);
        for ast in statements {
            last_result = eval.evaluate(&ast)
                .map_err(|e| JsValue::from_str(&e))?;
        }

        // Store in handle manager and return the handle
        HANDLES.with(|handles| {
            let mut h = handles.borrow_mut();
            Ok(h.create(last_result))
        })
    })
}
