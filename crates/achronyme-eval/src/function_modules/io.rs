/// I/O and Environment Persistence Functions for Achronyme
///
/// This module provides functions for:
/// - Environment persistence: save_env() and restore_env() for saving/loading REPL sessions
/// - File inspection: env_info() for viewing metadata without loading
///
/// File Format: .ach (Achronyme Archive)
/// - Custom 64-byte header with magic bytes and version info
/// - MessagePack serialization for cross-platform compatibility
/// - Zstd compression (default level 3)
/// - SHA-256 checksums for integrity verification

use crate::functions::FunctionRegistry;
use achronyme_types::value::Value;
use achronyme_types::Environment;
use achronyme_env::{save_environment, restore_environment, get_metadata};
use achronyme_env::{SaveOptions, RestoreOptions, RestoreMode};
use std::collections::HashMap;

pub fn register_functions(registry: &mut FunctionRegistry) {
    // Environment persistence
    registry.register("save_env", save_env, 1); // save_env(path) or save_env(record_with_options)
    registry.register("restore_env", restore_env, 1); // restore_env(path) or restore_env(record_with_options)
    registry.register("env_info", env_info, 1); // env_info(path) - get metadata without loading
}

// ============================================================================
// Environment Persistence Functions
// ============================================================================

/// Save the current environment to a .ach file
///
/// Usage:
///   save_env("session.ach")
///   save_env({path: "session.ach", compress: true, compression_level: 5})
///   save_env({path: "session.ach", description: "My analysis", tags: ["experiment", "2024"]})
///   save_env({path: "session.ach", include_only: ["x", "y", "result"]})
///   save_env({path: "session.ach", exclude: ["temp_*", "debug_var"]})
///
/// Options (when passing a record):
///   - path: String - File path (required)
///   - compress: Boolean - Enable compression (default: true)
///   - compression_level: Number - Zstd level 1-22 (default: 3)
///   - description: String - Description of the file
///   - tags: Vector of strings - Tags for categorization
///   - include_only: Vector of strings - Only save these variables
///   - exclude: Vector of strings - Exclude these variables (supports "prefix_*" patterns)
///   - allow_overwrite: Boolean - Allow overwriting existing file (default: false)
///
/// Returns: Boolean (true on success)
fn save_env(args: &[Value]) -> Result<Value, String> {
    if args.is_empty() {
        return Err("save_env() requires at least 1 argument (path or options record)".to_string());
    }

    // Parse arguments - can be either a string path or a record with options
    let (path, options) = match &args[0] {
        Value::String(p) => {
            // Simple case: just a path
            (p.clone(), SaveOptions::default())
        }
        Value::Record(map) => {
            // Extract path from record
            let path = map.get("path")
                .and_then(|v| match v {
                    Value::String(s) => Some(s.clone()),
                    _ => None,
                })
                .ok_or_else(|| "save_env() record must contain 'path' as a String".to_string())?;

            // Build SaveOptions from record
            let mut opts = SaveOptions::default();

            // compress: Boolean
            if let Some(Value::Boolean(b)) = map.get("compress") {
                opts.compress = *b;
            }

            // compression_level: Number
            if let Some(Value::Number(n)) = map.get("compression_level") {
                if *n < 1.0 || *n > 22.0 {
                    return Err("compression_level must be between 1 and 22".to_string());
                }
                opts.compression_level = *n as i32;
            }

            // description: String
            if let Some(Value::String(desc)) = map.get("description") {
                opts.description = Some(desc.clone());
            }

            // tags: Vector of strings
            if let Some(Value::Vector(vec)) = map.get("tags") {
                let tags: Result<Vec<String>, String> = vec.iter()
                    .map(|v| match v {
                        Value::String(s) => Ok(s.clone()),
                        _ => Err("tags must be a vector of strings".to_string()),
                    })
                    .collect();
                opts.tags = tags?;
            }

            // include_only: Vector of strings
            if let Some(Value::Vector(vec)) = map.get("include_only") {
                let include: Result<Vec<String>, String> = vec.iter()
                    .map(|v| match v {
                        Value::String(s) => Ok(s.clone()),
                        _ => Err("include_only must be a vector of strings".to_string()),
                    })
                    .collect();
                opts.include_only = Some(include?);
            }

            // exclude: Vector of strings
            if let Some(Value::Vector(vec)) = map.get("exclude") {
                let exclude: Result<Vec<String>, String> = vec.iter()
                    .map(|v| match v {
                        Value::String(s) => Ok(s.clone()),
                        _ => Err("exclude must be a vector of strings".to_string()),
                    })
                    .collect();
                opts.exclude = exclude?;
            }

            // allow_overwrite: Boolean
            if let Some(Value::Boolean(b)) = map.get("allow_overwrite") {
                opts.allow_overwrite = *b;
            }

            (path, opts)
        }
        _ => return Err("save_env() argument must be a String path or Record with options".to_string()),
    };

    // Get the environment from somewhere
    // NOTE: This is a challenge - we need access to the current Environment
    // For now, we'll create a dummy environment and document that this needs integration
    // TODO: Pass Environment as context to functions or use thread-local storage
    let env = Environment::new();

    // Save the environment
    save_environment(&env, &path, options)
        .map_err(|e| format!("Failed to save environment: {}", e))?;

    Ok(Value::Boolean(true))
}

/// Restore an environment from a .ach file
///
/// Usage:
///   restore_env("session.ach")
///   restore_env({path: "session.ach", mode: "replace"})
///   restore_env({path: "session.ach", mode: "merge", overwrite: true})
///   restore_env({path: "session.ach", include_only: ["x", "y"]})
///   restore_env({path: "session.ach", exclude: ["temp_*"]})
///
/// Options (when passing a record):
///   - path: String - File path (required)
///   - mode: String - "merge" (default), "replace", or "namespace"
///   - overwrite: Boolean - Overwrite existing bindings in merge mode (default: false)
///   - namespace: String - Namespace name (required if mode is "namespace")
///   - include_only: Vector of strings - Only restore these variables
///   - exclude: Vector of strings - Exclude these variables (supports "prefix_*" patterns)
///   - verify_checksum: Boolean - Verify file integrity (default: true)
///   - strict_version: Boolean - Require exact version match (default: false)
///
/// Returns: Boolean (true on success)
fn restore_env(args: &[Value]) -> Result<Value, String> {
    if args.is_empty() {
        return Err("restore_env() requires at least 1 argument (path or options record)".to_string());
    }

    // Parse arguments
    let (path, options) = match &args[0] {
        Value::String(p) => {
            (p.clone(), RestoreOptions::default())
        }
        Value::Record(map) => {
            let path = map.get("path")
                .and_then(|v| match v {
                    Value::String(s) => Some(s.clone()),
                    _ => None,
                })
                .ok_or_else(|| "restore_env() record must contain 'path' as a String".to_string())?;

            let mut opts = RestoreOptions::default();

            // mode: String
            if let Some(Value::String(mode)) = map.get("mode") {
                opts.mode = match mode.as_str() {
                    "merge" => RestoreMode::Merge,
                    "replace" => RestoreMode::Replace,
                    "namespace" => RestoreMode::Namespace,
                    _ => return Err(format!("Invalid mode '{}', must be 'merge', 'replace', or 'namespace'", mode)),
                };
            }

            // overwrite: Boolean
            if let Some(Value::Boolean(b)) = map.get("overwrite") {
                opts.overwrite = *b;
            }

            // namespace: String
            if let Some(Value::String(ns)) = map.get("namespace") {
                opts.namespace = Some(ns.clone());
            }

            // Validate namespace mode
            if opts.mode == RestoreMode::Namespace && opts.namespace.is_none() {
                return Err("namespace mode requires 'namespace' field".to_string());
            }

            // include_only: Vector of strings
            if let Some(Value::Vector(vec)) = map.get("include_only") {
                let include: Result<Vec<String>, String> = vec.iter()
                    .map(|v| match v {
                        Value::String(s) => Ok(s.clone()),
                        _ => Err("include_only must be a vector of strings".to_string()),
                    })
                    .collect();
                opts.include_only = Some(include?);
            }

            // exclude: Vector of strings
            if let Some(Value::Vector(vec)) = map.get("exclude") {
                let exclude: Result<Vec<String>, String> = vec.iter()
                    .map(|v| match v {
                        Value::String(s) => Ok(s.clone()),
                        _ => Err("exclude must be a vector of strings".to_string()),
                    })
                    .collect();
                opts.exclude = exclude?;
            }

            // verify_checksum: Boolean
            if let Some(Value::Boolean(b)) = map.get("verify_checksum") {
                opts.verify_checksum = *b;
            }

            // strict_version: Boolean
            if let Some(Value::Boolean(b)) = map.get("strict_version") {
                opts.strict_version = *b;
            }

            (path, opts)
        }
        _ => return Err("restore_env() argument must be a String path or Record with options".to_string()),
    };

    // Restore the environment
    let _restored_env = restore_environment(&path, options)
        .map_err(|e| format!("Failed to restore environment: {}", e))?;

    // TODO: Merge/replace into current environment
    // This requires access to the current Environment context
    // For now, we just confirm it loaded successfully

    Ok(Value::Boolean(true))
}

/// Get metadata from a .ach file without loading it
///
/// Usage:
///   env_info("session.ach")
///
/// Returns: Record with metadata fields:
///   - created_by: String - Who/what created the file
///   - created_at: String - Creation timestamp (ISO 8601)
///   - platform: String - Platform (OS + architecture)
///   - num_bindings: Number - Number of variables saved
///   - description: String - Description (if provided)
///   - tags: Vector of strings - Tags (if provided)
///   - binding_names: Vector of strings - List of variable names
fn env_info(args: &[Value]) -> Result<Value, String> {
    if args.is_empty() {
        return Err("env_info() requires 1 argument (path)".to_string());
    }

    let path = match &args[0] {
        Value::String(p) => p,
        _ => return Err("env_info() argument must be a String path".to_string()),
    };

    // Get metadata
    let metadata = get_metadata(path)
        .map_err(|e| format!("Failed to get metadata: {}", e))?;

    // Convert metadata to Record
    let mut record = HashMap::new();

    record.insert("created_by".to_string(), Value::String(metadata.created_by));
    record.insert("created_at".to_string(), Value::String(metadata.created_at));
    record.insert("platform".to_string(), Value::String(metadata.platform));
    record.insert("num_bindings".to_string(), Value::Number(metadata.num_bindings as f64));

    if let Some(desc) = metadata.description {
        record.insert("description".to_string(), Value::String(desc));
    }

    if !metadata.tags.is_empty() {
        let tags: Vec<Value> = metadata.tags
            .into_iter()
            .map(Value::String)
            .collect();
        record.insert("tags".to_string(), Value::Vector(tags));
    }

    if !metadata.binding_names.is_empty() {
        let names: Vec<Value> = metadata.binding_names
            .into_iter()
            .map(Value::String)
            .collect();
        record.insert("binding_names".to_string(), Value::Vector(names));
    }

    // Add custom fields if present
    for (key, value) in metadata.custom {
        record.insert(key, Value::String(value));
    }

    Ok(Value::Record(record))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_env_info_requires_path() {
        let result = env_info(&[]);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("requires 1 argument"));
    }

    #[test]
    fn test_env_info_path_must_be_string() {
        let result = env_info(&[Value::Number(42.0)]);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must be a String"));
    }

    #[test]
    fn test_save_env_requires_argument() {
        let result = save_env(&[]);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("requires at least 1 argument"));
    }

    #[test]
    fn test_save_env_accepts_string_path() {
        use std::env::temp_dir;
        use std::time::{SystemTime, UNIX_EPOCH};

        // Use a unique temporary file path to avoid conflicts
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let temp_path = temp_dir().join(format!("test_{}.ach", timestamp));

        let result = save_env(&[Value::String(temp_path.to_string_lossy().to_string())]);

        // Clean up if created
        if temp_path.exists() {
            let _ = std::fs::remove_file(&temp_path);
        }

        // Should succeed with empty environment
        if let Err(e) = &result {
            eprintln!("Error: {}", e);
        }
        assert!(result.is_ok());
    }

    #[test]
    fn test_save_env_record_requires_path() {
        let mut map = HashMap::new();
        map.insert("compress".to_string(), Value::Boolean(true));

        let result = save_env(&[Value::Record(map)]);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must contain 'path'"));
    }

    #[test]
    fn test_save_env_compression_level_validation() {
        let mut map = HashMap::new();
        map.insert("path".to_string(), Value::String("test.ach".to_string()));
        map.insert("compression_level".to_string(), Value::Number(99.0));

        let result = save_env(&[Value::Record(map)]);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("between 1 and 22"));
    }

    #[test]
    fn test_restore_env_requires_argument() {
        let result = restore_env(&[]);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("requires at least 1 argument"));
    }

    #[test]
    fn test_restore_env_mode_validation() {
        let mut map = HashMap::new();
        map.insert("path".to_string(), Value::String("test.ach".to_string()));
        map.insert("mode".to_string(), Value::String("invalid_mode".to_string()));

        let result = restore_env(&[Value::Record(map)]);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid mode"));
    }

    #[test]
    fn test_restore_env_namespace_mode_requires_namespace() {
        let mut map = HashMap::new();
        map.insert("path".to_string(), Value::String("test.ach".to_string()));
        map.insert("mode".to_string(), Value::String("namespace".to_string()));

        let result = restore_env(&[Value::Record(map)]);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("requires 'namespace'"));
    }
}
