//! Type Checker Module for Achronyme's Gradual Type System
//!
//! This module provides runtime type checking for the gradual type system.
//! It bridges the gap between AST-level type annotations (from the parser)
//! and runtime values.
//!
//! Key features:
//! - Union type support (value must match ANY type in the union)
//! - Structural typing for Records (extra fields are allowed)
//! - Tensor shape checking (optional shape constraints)
//! - Any type (always matches - opt-out of type checking)
//! - Null type support for optional values
//! - Automatic dereferencing of MutableRef values

use achronyme_parser::TypeAnnotation;
use achronyme_types::value::Value;
use std::collections::HashMap;

/// Error details for type mismatches
#[derive(Debug, Clone, PartialEq)]
pub struct TypeError {
    pub expected: String,
    pub actual: String,
    pub context: Option<String>,
}

impl TypeError {
    pub fn new(expected: String, actual: String) -> Self {
        Self {
            expected,
            actual,
            context: None,
        }
    }

    pub fn with_context(mut self, context: String) -> Self {
        self.context = Some(context);
        self
    }
}

impl std::fmt::Display for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.context {
            Some(ctx) => write!(
                f,
                "Type mismatch in {}: expected {}, got {}",
                ctx, self.expected, self.actual
            ),
            None => write!(
                f,
                "Type mismatch: expected {}, got {}",
                self.expected, self.actual
            ),
        }
    }
}

/// Get a human-readable type name for a runtime Value
fn get_value_type_name(value: &Value) -> String {
    match value {
        Value::Number(_) => "Number".to_string(),
        Value::Boolean(_) => "Boolean".to_string(),
        Value::String(_) => "String".to_string(),
        Value::Complex(_) => "Complex".to_string(),
        Value::Vector(_) => "Vector".to_string(),
        Value::Tensor(t) => format!("Tensor<Number, [{:?}]>", t.shape()),
        Value::ComplexTensor(t) => format!("Tensor<Complex, [{:?}]>", t.shape()),
        Value::Function(_) => "Function".to_string(),
        Value::Record(fields) => {
            let field_names: Vec<_> = fields.keys().cloned().collect();
            format!("Record{{{}}}", field_names.join(", "))
        }
        Value::Edge { .. } => "Edge".to_string(),
        Value::TailCall(_) => "TailCall (internal)".to_string(),
        Value::EarlyReturn(_) => "EarlyReturn (internal)".to_string(),
        Value::MutableRef(_) => "MutableRef".to_string(),
        Value::Null => "null".to_string(),
        Value::Generator(_) => "Generator".to_string(),
        Value::GeneratorYield(_) => "GeneratorYield (internal)".to_string(),
    }
}

/// Get a human-readable string representation of a TypeAnnotation
fn type_annotation_to_string(ty: &TypeAnnotation) -> String {
    match ty {
        TypeAnnotation::Number => "Number".to_string(),
        TypeAnnotation::Boolean => "Boolean".to_string(),
        TypeAnnotation::String => "String".to_string(),
        TypeAnnotation::Complex => "Complex".to_string(),
        TypeAnnotation::Vector => "Vector".to_string(),
        TypeAnnotation::Edge => "Edge".to_string(),
        TypeAnnotation::Null => "null".to_string(),
        TypeAnnotation::Any => "Any".to_string(),

        TypeAnnotation::Tensor {
            element_type,
            shape,
        } => {
            let elem_str = type_annotation_to_string(element_type);
            match shape {
                None => format!("Tensor<{}>", elem_str),
                Some(dims) => {
                    let dims_str = dims
                        .iter()
                        .map(|d| d.map_or("_".to_string(), |n| n.to_string()))
                        .collect::<Vec<_>>()
                        .join(", ");
                    format!("Tensor<{}, [{}]>", elem_str, dims_str)
                }
            }
        }

        TypeAnnotation::Record { fields } => {
            if fields.is_empty() {
                "{}".to_string()
            } else {
                let fields_str = fields
                    .iter()
                    .map(|(name, (is_mut, ty))| {
                        if *is_mut {
                            format!("mut {}: {}", name, type_annotation_to_string(ty))
                        } else {
                            format!("{}: {}", name, type_annotation_to_string(ty))
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{{{}}}", fields_str)
            }
        }

        TypeAnnotation::Function {
            params,
            return_type,
        } => {
            let params_str = params
                .iter()
                .map(|p| {
                    p.as_ref()
                        .map_or("Any".to_string(), type_annotation_to_string)
                })
                .collect::<Vec<_>>()
                .join(", ");
            format!(
                "({}) -> {}",
                params_str,
                type_annotation_to_string(return_type)
            )
        }

        TypeAnnotation::Union(types) => types
            .iter()
            .map(type_annotation_to_string)
            .collect::<Vec<_>>()
            .join(" | "),

        TypeAnnotation::TypeReference(name) => {
            // Type references should be resolved before reaching here
            format!("<unresolved type: {}>", name)
        }
    }
}

/// Check if a runtime Value matches a TypeAnnotation
///
/// This function performs runtime type checking according to the gradual type system rules.
/// It automatically dereferences MutableRef values for checking.
///
/// # Arguments
/// * `value` - The runtime value to check
/// * `expected` - The expected type annotation
///
/// # Returns
/// * `Ok(())` if the value matches the expected type
/// * `Err(String)` with a descriptive error message if there's a mismatch
///
/// # Examples
/// ```ignore
/// use achronyme_parser::TypeAnnotation;
/// use achronyme_types::value::Value;
/// use achronyme_eval::type_checker::check_type;
///
/// // Simple type checking
/// assert!(check_type(&Value::Number(42.0), &TypeAnnotation::Number).is_ok());
///
/// // Union types
/// let union = TypeAnnotation::Union(vec![TypeAnnotation::Number, TypeAnnotation::String]);
/// assert!(check_type(&Value::Number(42.0), &union).is_ok());
/// assert!(check_type(&Value::String("hello".into()), &union).is_ok());
///
/// // Any type
/// assert!(check_type(&Value::Boolean(true), &TypeAnnotation::Any).is_ok());
/// ```
pub fn check_type(value: &Value, expected: &TypeAnnotation) -> Result<(), String> {
    // Auto-dereference MutableRef for type checking
    let actual_value = match value {
        Value::MutableRef(rc) => &*rc.borrow(),
        v => v,
    };

    if matches_type(actual_value, expected) {
        Ok(())
    } else {
        Err(format!(
            "Type mismatch: expected {}, got {}",
            type_annotation_to_string(expected),
            get_value_type_name(actual_value)
        ))
    }
}

/// Check if a Value matches a TypeAnnotation (internal helper)
fn matches_type(value: &Value, expected: &TypeAnnotation) -> bool {
    match expected {
        // Any type matches everything
        TypeAnnotation::Any => true,

        // Null type only matches null
        TypeAnnotation::Null => matches!(value, Value::Null),

        // Simple types
        TypeAnnotation::Number => matches!(value, Value::Number(_)),
        TypeAnnotation::Boolean => matches!(value, Value::Boolean(_)),
        TypeAnnotation::String => matches!(value, Value::String(_)),
        TypeAnnotation::Complex => matches!(value, Value::Complex(_)),

        // Vector type (heterogeneous, no element type checking)
        TypeAnnotation::Vector => matches!(value, Value::Vector(_)),

        // Edge type (graph edges)
        TypeAnnotation::Edge => matches!(value, Value::Edge { .. }),

        // Union type: value must match at least one variant
        TypeAnnotation::Union(types) => types.iter().any(|t| matches_type(value, t)),

        // Record type with structural typing
        TypeAnnotation::Record { fields } => match value {
            Value::Record(actual_fields) => check_record_structural_type(fields, actual_fields),
            _ => false,
        },

        // Tensor type with optional shape checking
        TypeAnnotation::Tensor {
            element_type,
            shape,
        } => match value {
            Value::Tensor(tensor) => {
                check_real_tensor_type(tensor, element_type, shape.as_ref())
            }
            Value::ComplexTensor(tensor) => {
                check_complex_tensor_type(tensor, element_type, shape.as_ref())
            }
            _ => false,
        },

        // Function type checking
        TypeAnnotation::Function {
            params,
            return_type: _,
        } => match value {
            Value::Function(func) => check_function_type(func, params),
            _ => false,
        },

        // Type reference should be resolved before type checking
        TypeAnnotation::TypeReference(_) => {
            // This is an error condition - type references should be resolved first
            // Return false to indicate a type mismatch
            false
        }
    }
}

/// Check structural typing for records
/// A record matches if it has all required fields with correct types (extra fields are OK)
fn check_record_structural_type(
    required_fields: &HashMap<String, (bool, TypeAnnotation)>,
    actual_fields: &HashMap<String, Value>,
) -> bool {
    required_fields.iter().all(|(field_name, (_is_mut, field_type))| {
        actual_fields
            .get(field_name)
            .map(|actual_value| matches_type(actual_value, field_type))
            .unwrap_or(false)
    })
}

/// Check if a RealTensor matches a Tensor type annotation
fn check_real_tensor_type(
    tensor: &achronyme_types::tensor::RealTensor,
    element_type: &TypeAnnotation,
    expected_shape: Option<&Vec<Option<usize>>>,
) -> bool {
    // Check element type - RealTensor contains f64, so element type must be Number
    let element_type_matches = matches!(element_type, TypeAnnotation::Number | TypeAnnotation::Any);

    if !element_type_matches {
        return false;
    }

    // Check shape if specified
    match expected_shape {
        None => true, // No shape constraint
        Some(expected) => {
            let actual_shape = tensor.shape();
            // Rank must match
            if expected.len() != actual_shape.len() {
                return false;
            }
            // Each dimension must match (None means wildcard)
            expected.iter().zip(actual_shape).all(|(exp_dim, &act_dim)| {
                exp_dim.map_or(true, |e| e == act_dim)
            })
        }
    }
}

/// Check if a ComplexTensor matches a Tensor type annotation
fn check_complex_tensor_type(
    tensor: &achronyme_types::tensor::ComplexTensor,
    element_type: &TypeAnnotation,
    expected_shape: Option<&Vec<Option<usize>>>,
) -> bool {
    // Check element type - ComplexTensor contains Complex, so element type must be Complex
    let element_type_matches = matches!(element_type, TypeAnnotation::Complex | TypeAnnotation::Any);

    if !element_type_matches {
        return false;
    }

    // Check shape if specified
    match expected_shape {
        None => true, // No shape constraint
        Some(expected) => {
            let actual_shape = tensor.shape();
            // Rank must match
            if expected.len() != actual_shape.len() {
                return false;
            }
            // Each dimension must match (None means wildcard)
            expected.iter().zip(actual_shape).all(|(exp_dim, &act_dim)| {
                exp_dim.map_or(true, |e| e == act_dim)
            })
        }
    }
}

/// Check if a Function matches a Function type annotation
fn check_function_type(
    func: &achronyme_types::function::Function,
    expected_params: &Vec<Option<TypeAnnotation>>,
) -> bool {
    // Get the actual parameter count from the function
    let actual_param_count = func.arity();

    // If expected_params is empty, we don't check parameter count
    if expected_params.is_empty() {
        return true;
    }

    // Otherwise, check parameter count matches
    actual_param_count == expected_params.len()
}

/// Check if a Value can be assigned to a type (with coercion rules)
///
/// This function is similar to `check_type` but returns a boolean and is intended
/// for use cases where you need to check assignability without throwing errors.
///
/// # Arguments
/// * `value` - The runtime value to check
/// * `expected` - The expected type annotation
///
/// # Returns
/// * `true` if the value can be assigned to the type
/// * `false` otherwise
pub fn is_assignable(value: &Value, expected: &TypeAnnotation) -> bool {
    // Auto-dereference MutableRef for type checking
    let actual_value = match value {
        Value::MutableRef(rc) => &*rc.borrow(),
        v => v,
    };

    matches_type(actual_value, expected)
}

/// Check if a Value matches a TypeAnnotation and return detailed error information
///
/// This function provides more detailed error information than `check_type`,
/// useful for generating comprehensive error messages.
///
/// # Arguments
/// * `value` - The runtime value to check
/// * `expected` - The expected type annotation
///
/// # Returns
/// * `Ok(())` if the value matches the expected type
/// * `Err(TypeError)` with detailed error information if there's a mismatch
pub fn check_type_detailed(value: &Value, expected: &TypeAnnotation) -> Result<(), TypeError> {
    // Auto-dereference MutableRef for type checking
    let actual_value = match value {
        Value::MutableRef(rc) => &*rc.borrow(),
        v => v,
    };

    if matches_type(actual_value, expected) {
        Ok(())
    } else {
        Err(TypeError::new(
            type_annotation_to_string(expected),
            get_value_type_name(actual_value),
        ))
    }
}

/// Infer the TypeAnnotation from a runtime Value
///
/// This function attempts to infer the most specific type annotation
/// for a given runtime value. Useful for type inference in gradual typing.
///
/// # Arguments
/// * `value` - The runtime value to infer the type from
///
/// # Returns
/// The inferred TypeAnnotation
pub fn infer_type(value: &Value) -> TypeAnnotation {
    // Auto-dereference MutableRef
    let actual_value = match value {
        Value::MutableRef(rc) => &*rc.borrow(),
        v => v,
    };

    match actual_value {
        Value::Number(_) => TypeAnnotation::Number,
        Value::Boolean(_) => TypeAnnotation::Boolean,
        Value::String(_) => TypeAnnotation::String,
        Value::Complex(_) => TypeAnnotation::Complex,
        Value::Vector(_) => TypeAnnotation::Vector,
        Value::Null => TypeAnnotation::Null,
        Value::Function(_) => TypeAnnotation::Function {
            params: vec![],
            return_type: Box::new(TypeAnnotation::Any),
        },
        Value::Tensor(t) => TypeAnnotation::Tensor {
            element_type: Box::new(TypeAnnotation::Number),
            shape: Some(t.shape().iter().map(|&d| Some(d)).collect()),
        },
        Value::ComplexTensor(t) => TypeAnnotation::Tensor {
            element_type: Box::new(TypeAnnotation::Complex),
            shape: Some(t.shape().iter().map(|&d| Some(d)).collect()),
        },
        Value::Record(fields) => {
            let type_fields: HashMap<String, (bool, TypeAnnotation)> = fields
                .iter()
                .map(|(name, value)| (name.clone(), (false, infer_type(value))))
                .collect();
            TypeAnnotation::Record {
                fields: type_fields,
            }
        }
        Value::Edge { .. } => TypeAnnotation::Edge,
        // Internal values - should not appear in user code
        Value::TailCall(_) => TypeAnnotation::Any,
        Value::EarlyReturn(_) => TypeAnnotation::Any,
        Value::MutableRef(_) => unreachable!("MutableRef should be dereferenced"),
        // Generator type - represents an iterator
        Value::Generator(_) => TypeAnnotation::Any, // TODO: Add Generator type annotation
        // GeneratorYield is internal marker - should not appear in type inference
        Value::GeneratorYield(_) => TypeAnnotation::Any,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use achronyme_types::complex::Complex;
    use achronyme_types::function::Function;
    use achronyme_types::tensor::{ComplexTensor, RealTensor};
    use std::cell::RefCell;
    use std::rc::Rc;

    // Helper to create a simple lambda function for testing
    fn create_test_function(param_count: usize) -> Function {
        let params: Vec<String> = (0..param_count).map(|i| format!("p{}", i)).collect();
        let env = achronyme_types::Environment::new();
        Function::new_with_env(
            params,
            achronyme_parser::AstNode::Number(0.0),
            Rc::new(RefCell::new(env)),
        )
    }

    #[test]
    fn test_simple_number_type() {
        let value = Value::Number(42.0);
        assert!(check_type(&value, &TypeAnnotation::Number).is_ok());
        assert!(check_type(&value, &TypeAnnotation::String).is_err());
        assert!(check_type(&value, &TypeAnnotation::Boolean).is_err());
    }

    #[test]
    fn test_simple_boolean_type() {
        let value = Value::Boolean(true);
        assert!(check_type(&value, &TypeAnnotation::Boolean).is_ok());
        assert!(check_type(&value, &TypeAnnotation::Number).is_err());
    }

    #[test]
    fn test_simple_string_type() {
        let value = Value::String("hello".to_string());
        assert!(check_type(&value, &TypeAnnotation::String).is_ok());
        assert!(check_type(&value, &TypeAnnotation::Number).is_err());
    }

    #[test]
    fn test_simple_complex_type() {
        let value = Value::Complex(Complex::new(3.0, 4.0));
        assert!(check_type(&value, &TypeAnnotation::Complex).is_ok());
        assert!(check_type(&value, &TypeAnnotation::Number).is_err());
    }

    #[test]
    fn test_null_type() {
        let value = Value::Null;
        assert!(check_type(&value, &TypeAnnotation::Null).is_ok());
        assert!(check_type(&value, &TypeAnnotation::Number).is_err());

        let number = Value::Number(42.0);
        assert!(check_type(&number, &TypeAnnotation::Null).is_err());
    }

    #[test]
    fn test_any_type_matches_everything() {
        assert!(check_type(&Value::Number(42.0), &TypeAnnotation::Any).is_ok());
        assert!(check_type(&Value::Boolean(true), &TypeAnnotation::Any).is_ok());
        assert!(check_type(&Value::String("test".into()), &TypeAnnotation::Any).is_ok());
        assert!(check_type(&Value::Null, &TypeAnnotation::Any).is_ok());
        assert!(check_type(&Value::Vector(vec![]), &TypeAnnotation::Any).is_ok());
    }

    #[test]
    fn test_union_type_matches_any_variant() {
        let union = TypeAnnotation::Union(vec![
            TypeAnnotation::Number,
            TypeAnnotation::String,
        ]);

        assert!(check_type(&Value::Number(42.0), &union).is_ok());
        assert!(check_type(&Value::String("hello".into()), &union).is_ok());
        assert!(check_type(&Value::Boolean(true), &union).is_err());
    }

    #[test]
    fn test_optional_type_with_null() {
        let optional_number = TypeAnnotation::Union(vec![
            TypeAnnotation::Number,
            TypeAnnotation::Null,
        ]);

        assert!(check_type(&Value::Number(42.0), &optional_number).is_ok());
        assert!(check_type(&Value::Null, &optional_number).is_ok());
        assert!(check_type(&Value::String("hello".into()), &optional_number).is_err());
    }

    #[test]
    fn test_triple_union_type() {
        let union = TypeAnnotation::Union(vec![
            TypeAnnotation::Number,
            TypeAnnotation::String,
            TypeAnnotation::Boolean,
        ]);

        assert!(check_type(&Value::Number(42.0), &union).is_ok());
        assert!(check_type(&Value::String("test".into()), &union).is_ok());
        assert!(check_type(&Value::Boolean(false), &union).is_ok());
        assert!(check_type(&Value::Null, &union).is_err());
    }

    #[test]
    fn test_vector_type() {
        let empty_vec = Value::Vector(vec![]);
        let mixed_vec = Value::Vector(vec![
            Value::Number(1.0),
            Value::String("two".into()),
            Value::Boolean(true),
        ]);

        // Vector type matches any vector (heterogeneous, no element checking)
        assert!(check_type(&empty_vec, &TypeAnnotation::Vector).is_ok());
        assert!(check_type(&mixed_vec, &TypeAnnotation::Vector).is_ok());

        // Non-vectors don't match
        assert!(check_type(&Value::Number(42.0), &TypeAnnotation::Vector).is_err());
    }

    #[test]
    fn test_record_structural_typing_exact_match() {
        let mut fields = HashMap::new();
        fields.insert("name".to_string(), (false, TypeAnnotation::String));
        fields.insert("age".to_string(), (false, TypeAnnotation::Number));
        let record_type = TypeAnnotation::Record { fields };

        let mut actual_fields = HashMap::new();
        actual_fields.insert("name".to_string(), Value::String("John".into()));
        actual_fields.insert("age".to_string(), Value::Number(30.0));
        let value = Value::Record(actual_fields);

        assert!(check_type(&value, &record_type).is_ok());
    }

    #[test]
    fn test_record_structural_typing_extra_fields_allowed() {
        let mut required_fields = HashMap::new();
        required_fields.insert("name".to_string(), (false, TypeAnnotation::String));
        let record_type = TypeAnnotation::Record {
            fields: required_fields,
        };

        let mut actual_fields = HashMap::new();
        actual_fields.insert("name".to_string(), Value::String("John".into()));
        actual_fields.insert("age".to_string(), Value::Number(30.0)); // Extra field
        actual_fields.insert(
            "email".to_string(),
            Value::String("john@example.com".into()),
        ); // Another extra field
        let value = Value::Record(actual_fields);

        // Should match because all required fields are present with correct types
        assert!(check_type(&value, &record_type).is_ok());
    }

    #[test]
    fn test_record_missing_required_field() {
        let mut required_fields = HashMap::new();
        required_fields.insert("name".to_string(), (false, TypeAnnotation::String));
        required_fields.insert("age".to_string(), (false, TypeAnnotation::Number));
        let record_type = TypeAnnotation::Record {
            fields: required_fields,
        };

        let mut actual_fields = HashMap::new();
        actual_fields.insert("name".to_string(), Value::String("John".into()));
        // Missing "age" field
        let value = Value::Record(actual_fields);

        assert!(check_type(&value, &record_type).is_err());
    }

    #[test]
    fn test_record_wrong_field_type() {
        let mut required_fields = HashMap::new();
        required_fields.insert("name".to_string(), (false, TypeAnnotation::String));
        let record_type = TypeAnnotation::Record {
            fields: required_fields,
        };

        let mut actual_fields = HashMap::new();
        actual_fields.insert("name".to_string(), Value::Number(42.0)); // Wrong type
        let value = Value::Record(actual_fields);

        assert!(check_type(&value, &record_type).is_err());
    }

    #[test]
    fn test_nested_record_types() {
        let mut inner_fields = HashMap::new();
        inner_fields.insert("street".to_string(), (false, TypeAnnotation::String));
        let inner_record = TypeAnnotation::Record {
            fields: inner_fields,
        };

        let mut outer_fields = HashMap::new();
        outer_fields.insert("name".to_string(), (false, TypeAnnotation::String));
        outer_fields.insert("address".to_string(), (false, inner_record));
        let outer_record = TypeAnnotation::Record {
            fields: outer_fields,
        };

        let mut inner_value = HashMap::new();
        inner_value.insert("street".to_string(), Value::String("123 Main St".into()));

        let mut outer_value = HashMap::new();
        outer_value.insert("name".to_string(), Value::String("John".into()));
        outer_value.insert("address".to_string(), Value::Record(inner_value));

        let value = Value::Record(outer_value);

        assert!(check_type(&value, &outer_record).is_ok());
    }

    #[test]
    fn test_real_tensor_type_no_shape() {
        let tensor = RealTensor::new(vec![1.0, 2.0, 3.0], vec![3]).unwrap();
        let value = Value::Tensor(tensor);

        let tensor_type = TypeAnnotation::Tensor {
            element_type: Box::new(TypeAnnotation::Number),
            shape: None,
        };

        assert!(check_type(&value, &tensor_type).is_ok());
    }

    #[test]
    fn test_real_tensor_type_with_exact_shape() {
        let tensor = RealTensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], vec![2, 3]).unwrap();
        let value = Value::Tensor(tensor);

        let tensor_type = TypeAnnotation::Tensor {
            element_type: Box::new(TypeAnnotation::Number),
            shape: Some(vec![Some(2), Some(3)]),
        };

        assert!(check_type(&value, &tensor_type).is_ok());

        // Wrong shape should fail
        let wrong_shape_type = TypeAnnotation::Tensor {
            element_type: Box::new(TypeAnnotation::Number),
            shape: Some(vec![Some(3), Some(2)]),
        };
        assert!(check_type(&value, &wrong_shape_type).is_err());
    }

    #[test]
    fn test_real_tensor_type_with_wildcard_shape() {
        let tensor = RealTensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], vec![2, 3]).unwrap();
        let value = Value::Tensor(tensor);

        // Wildcard for first dimension
        let tensor_type = TypeAnnotation::Tensor {
            element_type: Box::new(TypeAnnotation::Number),
            shape: Some(vec![None, Some(3)]), // [_, 3]
        };

        assert!(check_type(&value, &tensor_type).is_ok());

        // Wildcard for second dimension
        let tensor_type2 = TypeAnnotation::Tensor {
            element_type: Box::new(TypeAnnotation::Number),
            shape: Some(vec![Some(2), None]), // [2, _]
        };

        assert!(check_type(&value, &tensor_type2).is_ok());

        // All wildcards
        let tensor_type3 = TypeAnnotation::Tensor {
            element_type: Box::new(TypeAnnotation::Number),
            shape: Some(vec![None, None]), // [_, _]
        };

        assert!(check_type(&value, &tensor_type3).is_ok());
    }

    #[test]
    fn test_real_tensor_wrong_rank() {
        let tensor = RealTensor::new(vec![1.0, 2.0, 3.0], vec![3]).unwrap(); // 1D
        let value = Value::Tensor(tensor);

        let tensor_type = TypeAnnotation::Tensor {
            element_type: Box::new(TypeAnnotation::Number),
            shape: Some(vec![Some(3), Some(1)]), // Expects 2D
        };

        assert!(check_type(&value, &tensor_type).is_err());
    }

    #[test]
    fn test_complex_tensor_type() {
        let tensor = ComplexTensor::new(
            vec![Complex::new(1.0, 2.0), Complex::new(3.0, 4.0)],
            vec![2],
        )
        .unwrap();
        let value = Value::ComplexTensor(tensor);

        let tensor_type = TypeAnnotation::Tensor {
            element_type: Box::new(TypeAnnotation::Complex),
            shape: None,
        };

        assert!(check_type(&value, &tensor_type).is_ok());

        // Wrong element type should fail
        let wrong_element_type = TypeAnnotation::Tensor {
            element_type: Box::new(TypeAnnotation::Number),
            shape: None,
        };
        assert!(check_type(&value, &wrong_element_type).is_err());
    }

    #[test]
    fn test_function_type_basic() {
        let func = create_test_function(2);
        let value = Value::Function(func);

        let func_type = TypeAnnotation::Function {
            params: vec![Some(TypeAnnotation::Number), Some(TypeAnnotation::String)],
            return_type: Box::new(TypeAnnotation::Number),
        };

        assert!(check_type(&value, &func_type).is_ok());
    }

    #[test]
    fn test_function_type_wrong_param_count() {
        let func = create_test_function(2);
        let value = Value::Function(func);

        let func_type = TypeAnnotation::Function {
            params: vec![
                Some(TypeAnnotation::Number),
                Some(TypeAnnotation::String),
                Some(TypeAnnotation::Boolean),
            ],
            return_type: Box::new(TypeAnnotation::Number),
        };

        assert!(check_type(&value, &func_type).is_err());
    }

    #[test]
    fn test_function_type_empty_params() {
        let func = create_test_function(5);
        let value = Value::Function(func);

        // Empty params means no checking of parameter count
        let func_type = TypeAnnotation::Function {
            params: vec![],
            return_type: Box::new(TypeAnnotation::Any),
        };

        assert!(check_type(&value, &func_type).is_ok());
    }

    #[test]
    fn test_mutable_ref_auto_deref() {
        let inner_value = Value::Number(42.0);
        let mutable_ref = Value::MutableRef(Rc::new(RefCell::new(inner_value)));

        // Should auto-dereference and check the inner value
        assert!(check_type(&mutable_ref, &TypeAnnotation::Number).is_ok());
        assert!(check_type(&mutable_ref, &TypeAnnotation::String).is_err());
    }

    #[test]
    fn test_mutable_ref_with_union() {
        let inner_value = Value::String("hello".into());
        let mutable_ref = Value::MutableRef(Rc::new(RefCell::new(inner_value)));

        let union = TypeAnnotation::Union(vec![
            TypeAnnotation::Number,
            TypeAnnotation::String,
        ]);

        assert!(check_type(&mutable_ref, &union).is_ok());
    }

    #[test]
    fn test_is_assignable() {
        assert!(is_assignable(&Value::Number(42.0), &TypeAnnotation::Number));
        assert!(!is_assignable(
            &Value::Number(42.0),
            &TypeAnnotation::String
        ));

        let union = TypeAnnotation::Union(vec![
            TypeAnnotation::Number,
            TypeAnnotation::String,
        ]);
        assert!(is_assignable(&Value::Number(42.0), &union));
        assert!(is_assignable(&Value::String("test".into()), &union));
        assert!(!is_assignable(&Value::Boolean(true), &union));
    }

    #[test]
    fn test_check_type_detailed() {
        let result = check_type_detailed(&Value::Number(42.0), &TypeAnnotation::String);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.expected, "String");
        assert_eq!(error.actual, "Number");
    }

    #[test]
    fn test_infer_type_simple() {
        assert_eq!(infer_type(&Value::Number(42.0)), TypeAnnotation::Number);
        assert_eq!(infer_type(&Value::Boolean(true)), TypeAnnotation::Boolean);
        assert_eq!(
            infer_type(&Value::String("test".into())),
            TypeAnnotation::String
        );
        assert_eq!(
            infer_type(&Value::Complex(Complex::new(1.0, 2.0))),
            TypeAnnotation::Complex
        );
        assert_eq!(infer_type(&Value::Null), TypeAnnotation::Null);
        assert_eq!(infer_type(&Value::Vector(vec![])), TypeAnnotation::Vector);
    }

    #[test]
    fn test_infer_type_tensor() {
        let tensor = RealTensor::new(vec![1.0, 2.0, 3.0], vec![3]).unwrap();
        let value = Value::Tensor(tensor);

        let inferred = infer_type(&value);
        match inferred {
            TypeAnnotation::Tensor {
                element_type,
                shape,
            } => {
                assert_eq!(*element_type, TypeAnnotation::Number);
                assert_eq!(shape, Some(vec![Some(3)]));
            }
            _ => panic!("Expected Tensor type annotation"),
        }
    }

    #[test]
    fn test_infer_type_record() {
        let mut fields = HashMap::new();
        fields.insert("name".to_string(), Value::String("John".into()));
        fields.insert("age".to_string(), Value::Number(30.0));
        let value = Value::Record(fields);

        let inferred = infer_type(&value);
        match inferred {
            TypeAnnotation::Record { fields } => {
                assert_eq!(fields.len(), 2);
                assert_eq!(fields.get("name").unwrap().1, TypeAnnotation::String);
                assert_eq!(fields.get("age").unwrap().1, TypeAnnotation::Number);
            }
            _ => panic!("Expected Record type annotation"),
        }
    }

    #[test]
    fn test_infer_type_mutable_ref() {
        let inner = Value::Boolean(true);
        let mutable_ref = Value::MutableRef(Rc::new(RefCell::new(inner)));

        // Should auto-dereference and infer the inner type
        assert_eq!(infer_type(&mutable_ref), TypeAnnotation::Boolean);
    }

    #[test]
    fn test_error_message_clarity() {
        let result = check_type(&Value::Number(42.0), &TypeAnnotation::String);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("expected String"));
        assert!(error.contains("got Number"));
    }

    #[test]
    fn test_complex_union_with_records() {
        let mut record_fields = HashMap::new();
        record_fields.insert("value".to_string(), (false, TypeAnnotation::Number));
        let record_type = TypeAnnotation::Record {
            fields: record_fields,
        };

        let union = TypeAnnotation::Union(vec![
            TypeAnnotation::String,
            record_type,
        ]);

        // String matches
        assert!(check_type(&Value::String("test".into()), &union).is_ok());

        // Record with value field matches
        let mut record_value = HashMap::new();
        record_value.insert("value".to_string(), Value::Number(42.0));
        assert!(check_type(&Value::Record(record_value), &union).is_ok());

        // Number doesn't match
        assert!(check_type(&Value::Number(42.0), &union).is_err());
    }
}
