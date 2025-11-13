/// End-to-end tests for environment persistence
///
/// These tests verify that save_env and restore_env work correctly
/// with real environment data.

use achronyme_eval::Evaluator;
use achronyme_types::value::Value;
use std::env::temp_dir;
use std::time::{SystemTime, UNIX_EPOCH};

fn eval(source: &str) -> Result<Value, String> {
    let mut evaluator = Evaluator::new();
    evaluator.eval_str(source)
}

#[test]
fn test_save_and_restore_complete_workflow() {
    use std::fs;

    // Create unique temp file
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let temp_path = temp_dir().join(format!("test_workflow_{}.ach", timestamp));
    let path_str = temp_path.to_string_lossy().to_string();

    // Create first evaluator and define some variables
    let mut eval1 = Evaluator::new();
    let _ = eval1.eval_str("let x = 42");
    let _ = eval1.eval_str("let name = \"Achronyme\"");
    let _ = eval1.eval_str("let pi_approx = 3.14159");
    let _ = eval1.eval_str("let numbers = [1, 2, 3, 4, 5]");

    // Save the environment
    let save_cmd = format!("save_env(\"{}\")", path_str.replace("\\", "\\\\"));
    let save_result = eval1.eval_str(&save_cmd);
    assert!(save_result.is_ok(), "save_env should succeed: {:?}", save_result.err());

    // Verify file was created
    assert!(temp_path.exists(), "File should be created");

    // Create second evaluator and restore
    let mut eval2 = Evaluator::new();
    let restore_cmd = format!("restore_env(\"{}\")", path_str.replace("\\", "\\\\"));
    let restore_result = eval2.eval_str(&restore_cmd);
    assert!(restore_result.is_ok(), "restore_env should succeed: {:?}", restore_result.err());

    // Verify variables were restored
    assert_eq!(eval2.eval_str("x").unwrap(), Value::Number(42.0));
    assert_eq!(eval2.eval_str("name").unwrap(), Value::String("Achronyme".to_string()));
    assert_eq!(eval2.eval_str("pi_approx").unwrap(), Value::Number(3.14159));

    // Verify vector was restored (may be Vector or Tensor)
    match eval2.eval_str("numbers").unwrap() {
        Value::Vector(v) => {
            assert_eq!(v.len(), 5);
            assert_eq!(v[0], Value::Number(1.0));
            assert_eq!(v[4], Value::Number(5.0));
        }
        Value::Tensor(t) => {
            assert_eq!(t.data().len(), 5);
            assert_eq!(t.data()[0], 1.0);
            assert_eq!(t.data()[4], 5.0);
        }
        other => panic!("Expected vector or tensor, got {:?}", other),
    }

    // Clean up
    let _ = fs::remove_file(&temp_path);
}

#[test]
fn test_save_env_with_compression_options() {
    use std::fs;

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let temp_path = temp_dir().join(format!("test_compression_{}.ach", timestamp));
    let path_str = temp_path.to_string_lossy().to_string();

    let mut evaluator = Evaluator::new();
    let _ = evaluator.eval_str("let data = [1, 2, 3, 4, 5]");

    // Save with specific compression settings
    let save_cmd = format!(
        "save_env({{path: \"{}\", compress: true, compression_level: 5, allow_overwrite: true}})",
        path_str.replace("\\", "\\\\")
    );
    let result = evaluator.eval_str(&save_cmd);
    assert!(result.is_ok(), "save_env with options should succeed: {:?}", result.err());

    assert!(temp_path.exists(), "Compressed file should be created");

    // Clean up
    let _ = fs::remove_file(&temp_path);
}

#[test]
fn test_env_info_metadata() {
    use std::fs;

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let temp_path = temp_dir().join(format!("test_metadata_{}.ach", timestamp));
    let path_str = temp_path.to_string_lossy().to_string();

    let mut evaluator = Evaluator::new();
    let _ = evaluator.eval_str("let test_var = 123");

    // Save with description and tags
    let save_cmd = format!(
        "save_env({{path: \"{}\", description: \"Test file\", tags: [\"test\", \"demo\"], allow_overwrite: true}})",
        path_str.replace("\\", "\\\\")
    );
    evaluator.eval_str(&save_cmd).unwrap();

    // Get metadata
    let info_cmd = format!("env_info(\"{}\")", path_str.replace("\\", "\\\\"));
    let info_result = evaluator.eval_str(&info_cmd);
    assert!(info_result.is_ok(), "env_info should succeed");

    match info_result.unwrap() {
        Value::Record(map) => {
            // Check that metadata fields exist
            assert!(map.contains_key("created_by"));
            assert!(map.contains_key("created_at"));
            assert!(map.contains_key("platform"));
            assert!(map.contains_key("num_bindings"));
            assert!(map.contains_key("description"));

            // Verify description
            if let Some(Value::String(desc)) = map.get("description") {
                assert_eq!(desc, "Test file");
            } else {
                panic!("Description should be a string");
            }

            // Verify num_bindings
            if let Some(Value::Number(n)) = map.get("num_bindings") {
                assert_eq!(*n, 1.0); // Only test_var was saved
            } else {
                panic!("num_bindings should be a number");
            }
        }
        _ => panic!("env_info should return a record"),
    }

    // Clean up
    let _ = fs::remove_file(&temp_path);
}

#[test]
fn test_restore_with_merge_mode() {
    use std::fs;

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let temp_path = temp_dir().join(format!("test_merge_{}.ach", timestamp));
    let path_str = temp_path.to_string_lossy().to_string();

    // Save environment with x=10
    let mut eval1 = Evaluator::new();
    let _ = eval1.eval_str("let x = 10");
    let _ = eval1.eval_str("let y = 20");
    let save_cmd = format!("save_env(\"{}\")", path_str.replace("\\", "\\\\"));
    eval1.eval_str(&save_cmd).unwrap();

    // Create new environment with x=99 and z=30
    let mut eval2 = Evaluator::new();
    let _ = eval2.eval_str("let x = 99");
    let _ = eval2.eval_str("let z = 30");

    // Restore with merge mode (default), should not overwrite x
    let restore_cmd = format!("restore_env(\"{}\")", path_str.replace("\\", "\\\\"));
    eval2.eval_str(&restore_cmd).unwrap();

    // x should still be 99 (not overwritten)
    assert_eq!(eval2.eval_str("x").unwrap(), Value::Number(99.0));
    // y should be restored
    assert_eq!(eval2.eval_str("y").unwrap(), Value::Number(20.0));
    // z should still exist
    assert_eq!(eval2.eval_str("z").unwrap(), Value::Number(30.0));

    // Clean up
    let _ = fs::remove_file(&temp_path);
}

#[test]
fn test_restore_with_overwrite() {
    use std::fs;

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let temp_path = temp_dir().join(format!("test_overwrite_{}.ach", timestamp));
    let path_str = temp_path.to_string_lossy().to_string();

    // Save environment with x=10
    let mut eval1 = Evaluator::new();
    let _ = eval1.eval_str("let x = 10");
    let save_cmd = format!("save_env(\"{}\")", path_str.replace("\\", "\\\\"));
    eval1.eval_str(&save_cmd).unwrap();

    // Create new environment with x=99
    let mut eval2 = Evaluator::new();
    let _ = eval2.eval_str("let x = 99");

    // Restore with overwrite enabled
    let restore_cmd = format!(
        "restore_env({{path: \"{}\", overwrite: true}})",
        path_str.replace("\\", "\\\\")
    );
    eval2.eval_str(&restore_cmd).unwrap();

    // x should now be 10 (overwritten from file)
    assert_eq!(eval2.eval_str("x").unwrap(), Value::Number(10.0));

    // Clean up
    let _ = fs::remove_file(&temp_path);
}
