// Type Annotation System for Gradual Typing
//
// This module implements the type annotation system for Achronyme's gradual typing.
// It supports:
// - Simple types (Number, Boolean, String, Complex)
// - Tensor types with optional shape specifications
// - Union types (A | B | C) - CORE FEATURE
// - Record types with structural subtyping (duck typing)
// - Function types
// - Null type for optional values
// - Any type (opt-out of type checking)

use std::collections::HashMap;
use crate::value::Value;

/// Type annotation for gradual typing system
#[derive(Debug, Clone, PartialEq)]
pub enum TypeAnnotation {
    /// Number type (f64)
    Number,

    /// Boolean type
    Boolean,

    /// String type
    String,

    /// Complex number type
    Complex,

    /// Tensor type with optional element type and shape
    /// shape: None = unknown rank, Some(vec) = known rank with optional dimensions
    /// Example: Tensor<Number> has shape=None
    /// Example: Tensor<Number, [2, 3]> has shape=Some(vec![Some(2), Some(3)])
    /// Example: Tensor<Number, [_, _]> has shape=Some(vec![None, None])
    Tensor {
        element_type: Box<TypeAnnotation>,
        shape: Option<Vec<Option<usize>>>,
    },

    /// Vector type (heterogeneous array)
    Vector,

    /// Record type with structural typing
    /// HashMap<field_name, (is_mutable, field_type)>
    Record {
        fields: HashMap<String, (bool, TypeAnnotation)>,
    },

    /// Function type
    /// params: parameter types (None for untyped parameters in gradual typing)
    /// return_type: return type
    Function {
        params: Vec<Option<TypeAnnotation>>,
        return_type: Box<TypeAnnotation>,
    },

    /// Union type (CORE FEATURE)
    /// Represents "one of these types"
    /// Example: Number | String | null
    Union(Vec<TypeAnnotation>),

    /// Null type (for optional values)
    /// Example: Number | null for optional numbers
    Null,

    /// Any type (opt-out of type checking)
    /// Accepts any value
    Any,
}

impl TypeAnnotation {
    /// Check if a runtime value matches this type annotation
    ///
    /// # Arguments
    /// * `value` - The runtime value to check
    ///
    /// # Returns
    /// `true` if the value matches the type annotation, `false` otherwise
    pub fn matches(&self, value: &Value) -> bool {
        match (self, value) {
            // Simple types
            (TypeAnnotation::Number, Value::Number(_)) => true,
            (TypeAnnotation::Boolean, Value::Boolean(_)) => true,
            (TypeAnnotation::String, Value::String(_)) => true,
            (TypeAnnotation::Complex, Value::Complex(_)) => true,
            (TypeAnnotation::Vector, Value::Vector(_)) => true,

            // Null type
            (TypeAnnotation::Null, Value::Null) => true,

            // Union: matches if value matches ANY of the types
            (TypeAnnotation::Union(types), val) => {
                types.iter().any(|t| t.matches(val))
            }

            // Record: structural subtyping (must have all required fields with correct types)
            (TypeAnnotation::Record { fields: required_fields }, Value::Record(actual_fields)) => {
                required_fields.iter().all(|(field_name, (_is_mut, field_type))| {
                    actual_fields.get(field_name)
                        .map(|actual_value| field_type.matches(actual_value))
                        .unwrap_or(false)  // Missing field = no match
                })
            }

            // Tensor: check element types and optionally shape
            (TypeAnnotation::Tensor { element_type, shape }, Value::Tensor(tensor)) => {
                // Check element type for all elements
                let elements_match = tensor.data().iter()
                    .all(|elem| element_type.matches(&Value::Number(*elem)));

                // Check shape if specified
                let shape_match = match shape {
                    None => true,  // No shape constraint
                    Some(expected_shape) => {
                        let actual_shape = tensor.shape();
                        expected_shape.len() == actual_shape.len() &&
                        expected_shape.iter().zip(actual_shape).all(|(exp, act)| {
                            exp.map_or(true, |e| e == *act)  // None (_) matches any dimension
                        })
                    }
                };

                elements_match && shape_match
            }

            // ComplexTensor
            (TypeAnnotation::Tensor { element_type, shape }, Value::ComplexTensor(tensor)) => {
                // Check element type for all elements
                let elements_match = tensor.data().iter()
                    .all(|elem| element_type.matches(&Value::Complex(*elem)));

                // Check shape if specified
                let shape_match = match shape {
                    None => true,
                    Some(expected_shape) => {
                        let actual_shape = tensor.shape();
                        expected_shape.len() == actual_shape.len() &&
                        expected_shape.iter().zip(actual_shape).all(|(exp, act)| {
                            exp.map_or(true, |e| e == *act)
                        })
                    }
                };

                elements_match && shape_match
            }

            // Function type matching (approximate - checks if it's a function)
            (TypeAnnotation::Function { .. }, Value::Function(_)) => true,

            // Any type matches everything
            (TypeAnnotation::Any, _) => true,

            // No match
            _ => false,
        }
    }

    /// Check if this type can be assigned from another type (subtyping)
    ///
    /// # Arguments
    /// * `other` - The type to check assignability from
    ///
    /// # Returns
    /// `true` if `other` can be assigned to `self`, `false` otherwise
    ///
    /// # Examples
    /// ```
    /// // Number can be assigned to Number
    /// assert!(TypeAnnotation::Number.is_assignable_from(&TypeAnnotation::Number));
    ///
    /// // Number can be assigned to Number | String
    /// let union = TypeAnnotation::Union(vec![TypeAnnotation::Number, TypeAnnotation::String]);
    /// assert!(union.is_assignable_from(&TypeAnnotation::Number));
    ///
    /// // {name: String, age: Number} can be assigned to {name: String} (structural subtyping)
    /// ```
    pub fn is_assignable_from(&self, other: &TypeAnnotation) -> bool {
        match (self, other) {
            // Same types are assignable
            (a, b) if a == b => true,

            // Any accepts anything
            (TypeAnnotation::Any, _) => true,

            // Anything can be assigned to Any
            (_, TypeAnnotation::Any) => true,

            // Union assignability
            (TypeAnnotation::Union(self_types), TypeAnnotation::Union(other_types)) => {
                // All types in 'other' must be assignable to at least one type in 'self'
                other_types.iter().all(|ot|
                    self_types.iter().any(|st| st.is_assignable_from(ot))
                )
            }
            (TypeAnnotation::Union(types), other) => {
                // Single type is assignable to union if it's assignable to any member
                types.iter().any(|t| t.is_assignable_from(other))
            }
            (self_type, TypeAnnotation::Union(other_types)) => {
                // Union is assignable to single type if ALL union members are assignable
                other_types.iter().all(|ot| self_type.is_assignable_from(ot))
            }

            // Record structural subtyping
            // 'other' must have all fields of 'self' with compatible types (can have extras)
            (TypeAnnotation::Record { fields: self_fields }, TypeAnnotation::Record { fields: other_fields }) => {
                self_fields.iter().all(|(field_name, (self_mut, self_type))| {
                    other_fields.get(field_name).map_or(false, |(other_mut, other_type)| {
                        // Mutability must match
                        self_mut == other_mut &&
                        // Type must be assignable
                        self_type.is_assignable_from(other_type)
                    })
                })
            }

            // Tensor subtyping (element types and shapes must match)
            (
                TypeAnnotation::Tensor { element_type: et1, shape: s1 },
                TypeAnnotation::Tensor { element_type: et2, shape: s2 }
            ) => {
                // Element types must be assignable
                let element_assignable = et1.is_assignable_from(et2);

                // Shapes must be compatible
                let shape_compatible = match (s1, s2) {
                    (None, _) => true,  // No shape constraint in target
                    (Some(_), None) => false,  // Target has shape, source doesn't
                    (Some(shape1), Some(shape2)) => {
                        shape1.len() == shape2.len() &&
                        shape1.iter().zip(shape2).all(|(d1, d2)| {
                            match (d1, d2) {
                                (None, _) => true,  // _ in target matches any
                                (Some(dim1), Some(dim2)) => dim1 == dim2,
                                (Some(_), None) => false,  // Target specifies, source doesn't
                            }
                        })
                    }
                };

                element_assignable && shape_compatible
            }

            // Function subtyping (simplified - contravariant in params, covariant in return)
            (
                TypeAnnotation::Function { params: p1, return_type: r1 },
                TypeAnnotation::Function { params: p2, return_type: r2 }
            ) => {
                // Must have same number of parameters
                if p1.len() != p2.len() {
                    return false;
                }

                // Parameters: contravariant (other params must be assignable to self params)
                let params_compatible = p1.iter().zip(p2).all(|(param1, param2)| {
                    match (param1, param2) {
                        (None, _) | (_, None) => true,  // Untyped parameter is compatible
                        (Some(t1), Some(t2)) => t2.is_assignable_from(t1),  // Contravariant
                    }
                });

                // Return type: covariant (other return must be assignable to self return)
                let return_compatible = r1.is_assignable_from(r2);

                params_compatible && return_compatible
            }

            // No match
            _ => false,
        }
    }

    /// Get a human-readable string representation of this type
    pub fn to_string(&self) -> String {
        match self {
            TypeAnnotation::Number => "Number".to_string(),
            TypeAnnotation::Boolean => "Boolean".to_string(),
            TypeAnnotation::String => "String".to_string(),
            TypeAnnotation::Complex => "Complex".to_string(),
            TypeAnnotation::Vector => "Vector".to_string(),
            TypeAnnotation::Null => "null".to_string(),
            TypeAnnotation::Any => "Any".to_string(),

            TypeAnnotation::Tensor { element_type, shape } => {
                let elem_str = element_type.to_string();
                match shape {
                    None => format!("Tensor<{}>", elem_str),
                    Some(dims) => {
                        let dims_str = dims.iter()
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
                    let fields_str = fields.iter()
                        .map(|(name, (is_mut, ty))| {
                            if *is_mut {
                                format!("mut {}: {}", name, ty.to_string())
                            } else {
                                format!("{}: {}", name, ty.to_string())
                            }
                        })
                        .collect::<Vec<_>>()
                        .join(", ");
                    format!("{{{}}}", fields_str)
                }
            }

            TypeAnnotation::Function { params, return_type } => {
                let params_str = params.iter()
                    .map(|p| p.as_ref().map_or("Any".to_string(), |t| t.to_string()))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("Function<[{}], {}>", params_str, return_type.to_string())
            }

            TypeAnnotation::Union(types) => {
                types.iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<_>>()
                    .join(" | ")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_type_matching() {
        assert!(TypeAnnotation::Number.matches(&Value::Number(42.0)));
        assert!(!TypeAnnotation::Number.matches(&Value::String("hello".to_string())));

        assert!(TypeAnnotation::Boolean.matches(&Value::Boolean(true)));
        assert!(!TypeAnnotation::Boolean.matches(&Value::Number(1.0)));
    }

    #[test]
    fn test_union_type_matching() {
        let union = TypeAnnotation::Union(vec![
            TypeAnnotation::Number,
            TypeAnnotation::String,
        ]);

        assert!(union.matches(&Value::Number(42.0)));
        assert!(union.matches(&Value::String("hello".to_string())));
        assert!(!union.matches(&Value::Boolean(true)));
    }

    #[test]
    fn test_null_type_matching() {
        let optional_number = TypeAnnotation::Union(vec![
            TypeAnnotation::Number,
            TypeAnnotation::Null,
        ]);

        assert!(optional_number.matches(&Value::Number(42.0)));
        assert!(optional_number.matches(&Value::Null));
        assert!(!optional_number.matches(&Value::String("hello".to_string())));
    }

    #[test]
    fn test_any_type() {
        assert!(TypeAnnotation::Any.matches(&Value::Number(42.0)));
        assert!(TypeAnnotation::Any.matches(&Value::String("hello".to_string())));
        assert!(TypeAnnotation::Any.matches(&Value::Boolean(true)));
    }

    #[test]
    fn test_type_to_string() {
        assert_eq!(TypeAnnotation::Number.to_string(), "Number");
        assert_eq!(TypeAnnotation::Boolean.to_string(), "Boolean");

        let union = TypeAnnotation::Union(vec![
            TypeAnnotation::Number,
            TypeAnnotation::String,
        ]);
        assert_eq!(union.to_string(), "Number | String");
    }
}
