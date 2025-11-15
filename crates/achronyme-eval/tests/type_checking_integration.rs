//! Integration tests for type checking in the evaluator
//!
//! These tests verify that type annotations are enforced at runtime.
//! Tests are organized by category:
//! - basic: Basic let/mut type checking tests
//! - null: Null type and optional type tests
//! - function_params: Function parameter type checking tests
//! - assignment: Assignment type checking tests
//! - function_types: Function type annotation tests
//! - edge: Edge type annotation tests
//! - type_alias: Type alias tests

#[path = "type_checking_integration/assignment.rs"]
mod assignment;
#[path = "type_checking_integration/basic.rs"]
mod basic;
#[path = "type_checking_integration/edge.rs"]
mod edge;
#[path = "type_checking_integration/function_params.rs"]
mod function_params;
#[path = "type_checking_integration/function_types.rs"]
mod function_types;
#[path = "type_checking_integration/null.rs"]
mod null;
#[path = "type_checking_integration/type_alias.rs"]
mod type_alias;
