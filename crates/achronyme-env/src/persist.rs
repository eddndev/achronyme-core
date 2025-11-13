use std::path::Path;
use std::io::{BufWriter, BufReader, Write, Read};
use std::fs::File;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use achronyme_types::Environment;
use crate::errors::{EnvError, Result};
use crate::format::{AchHeader, CompressionType};
use crate::metadata::Metadata;
use crate::serialize::SerializedValue;
use crate::checksum::{calculate_checksum, verify_checksum, CHECKSUM_SIZE};

/// Options for saving environment
#[derive(Debug, Clone)]
pub struct SaveOptions {
    /// Enable compression (Zstd)
    pub compress: bool,

    /// Compression level (1-22, default 3)
    pub compression_level: i32,

    /// Description of the file
    pub description: Option<String>,

    /// Tags for categorization
    pub tags: Vec<String>,

    /// Only include these bindings (None = all)
    pub include_only: Option<Vec<String>>,

    /// Exclude these bindings
    pub exclude: Vec<String>,

    /// Allow overwriting existing file
    pub allow_overwrite: bool,
}

impl Default for SaveOptions {
    fn default() -> Self {
        Self {
            compress: true,
            compression_level: 3,
            description: None,
            tags: Vec::new(),
            include_only: None,
            exclude: Vec::new(),
            allow_overwrite: false,
        }
    }
}

/// Mode for restoring environment
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RestoreMode {
    /// Merge with current environment (default)
    Merge,
    /// Replace current environment completely
    Replace,
    /// Load into namespace
    Namespace,
}

/// Options for restoring environment
#[derive(Debug, Clone)]
pub struct RestoreOptions {
    /// Restore mode
    pub mode: RestoreMode,

    /// Overwrite existing bindings (for Merge mode)
    pub overwrite: bool,

    /// Namespace name (for Namespace mode)
    pub namespace: Option<String>,

    /// Only restore these bindings (None = all)
    pub include_only: Option<Vec<String>>,

    /// Exclude these bindings
    pub exclude: Vec<String>,

    /// Verify checksum
    pub verify_checksum: bool,

    /// Strict version checking
    pub strict_version: bool,
}

impl Default for RestoreOptions {
    fn default() -> Self {
        Self {
            mode: RestoreMode::Merge,
            overwrite: false,
            namespace: None,
            include_only: None,
            exclude: Vec::new(),
            verify_checksum: true,
            strict_version: false,
        }
    }
}

/// Internal structure for .ach file body
#[derive(Debug, Serialize, Deserialize)]
struct AchBody {
    metadata: Metadata,
    bindings: HashMap<String, SerializedValue>,
}

/// Save environment to .ach file
pub fn save_environment(
    env: &Environment,
    path: impl AsRef<Path>,
    options: SaveOptions,
) -> Result<()> {
    let path = path.as_ref();

    // Check if file exists and overwrite is not allowed
    if path.exists() && !options.allow_overwrite {
        return Err(EnvError::Custom(
            format!("File already exists: {:?}. Use allow_overwrite option.", path)
        ));
    }

    // Collect bindings to save
    let mut bindings_to_save = HashMap::new();
    let mut binding_names = Vec::new();

    // Get snapshot of all variables in environment
    let snapshot = env.snapshot();

    for (name, value) in snapshot.iter() {
        // Apply filters
        if let Some(ref include_only) = options.include_only {
            if !include_only.contains(&name.to_string()) {
                continue;
            }
        }

        if options.exclude.iter().any(|pattern| {
            // Simple pattern matching: exact match or starts_with for "prefix_*"
            if pattern.ends_with('*') {
                let prefix = &pattern[..pattern.len() - 1];
                name.starts_with(prefix)
            } else {
                name == pattern
            }
        }) {
            continue;
        }

        // Serialize value
        let serialized = SerializedValue::from_value(value);

        // Warn about unsupported values
        if matches!(serialized, SerializedValue::Unsupported { .. }) {
            eprintln!("Warning: Variable '{}' is not serializable and will be skipped", name);
            continue;
        }

        bindings_to_save.insert(name.clone(), serialized);
        binding_names.push(name.clone());
    }

    // Sort binding names for consistent output
    binding_names.sort();

    // Create metadata
    let mut metadata = Metadata::new()
        .with_bindings(binding_names);

    if let Some(desc) = options.description {
        metadata = metadata.with_description(desc);
    }

    if !options.tags.is_empty() {
        metadata = metadata.with_tags(options.tags);
    }

    // Create body
    let body = AchBody {
        metadata,
        bindings: bindings_to_save,
    };

    // Serialize body to MessagePack
    let body_bytes = rmp_serde::to_vec(&body)?;

    // Compress if requested
    let (final_bytes, compression) = if options.compress {
        let compressed = zstd::encode_all(&body_bytes[..], options.compression_level)
            .map_err(|e| EnvError::Compression(e.to_string()))?;
        (compressed, CompressionType::Zstd)
    } else {
        (body_bytes, CompressionType::None)
    };

    // Calculate checksum
    let checksum = calculate_checksum(&final_bytes);

    // Create header
    let mut header = AchHeader::new();
    header.compression = compression;
    if options.compress {
        header.set_compressed(true);
    }

    // Write to file
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    // Write header
    header.write(&mut writer)?;

    // Write body
    writer.write_all(&final_bytes)?;

    // Write checksum
    writer.write_all(&checksum)?;

    writer.flush()?;

    Ok(())
}

/// Restore environment from .ach file
pub fn restore_environment(
    path: impl AsRef<Path>,
    options: RestoreOptions,
) -> Result<Environment> {
    let path = path.as_ref();

    // Open file
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    // Read header
    let header = AchHeader::read(&mut reader)?;

    // Verify version
    header.verify_version(options.strict_version)?;

    // Read body
    let mut body_bytes = Vec::new();
    reader.read_to_end(&mut body_bytes)?;

    // Extract checksum (last 32 bytes)
    if body_bytes.len() < CHECKSUM_SIZE {
        return Err(EnvError::InvalidFormat(
            "File too small to contain checksum".to_string()
        ));
    }

    let data_end = body_bytes.len() - CHECKSUM_SIZE;
    let data_bytes = &body_bytes[..data_end];
    let checksum_bytes: &[u8; CHECKSUM_SIZE] = body_bytes[data_end..].try_into()
        .map_err(|_| EnvError::InvalidFormat("Invalid checksum size".to_string()))?;

    // Verify checksum
    if options.verify_checksum {
        if !verify_checksum(data_bytes, checksum_bytes) {
            return Err(EnvError::ChecksumMismatch);
        }
    }

    // Decompress if needed
    let decompressed = if header.is_compressed() {
        zstd::decode_all(data_bytes)
            .map_err(|e| EnvError::Decompression(e.to_string()))?
    } else {
        data_bytes.to_vec()
    };

    // Deserialize body
    let body: AchBody = rmp_serde::from_slice(&decompressed)?;

    // Create environment
    let mut env = Environment::new();

    // Apply bindings based on mode and filters
    for (name, serialized_value) in body.bindings {
        // Apply filters
        if let Some(ref include_only) = options.include_only {
            if !include_only.contains(&name) {
                continue;
            }
        }

        if options.exclude.iter().any(|pattern| {
            if pattern.ends_with('*') {
                let prefix = &pattern[..pattern.len() - 1];
                name.starts_with(prefix)
            } else {
                &name == pattern
            }
        }) {
            continue;
        }

        // Deserialize value
        match serialized_value.to_value() {
            Ok(value) => {
                if let Err(e) = env.define(name.clone(), value) {
                    eprintln!("Warning: Failed to define '{}': {}", name, e);
                }
            }
            Err(e) => {
                eprintln!("Warning: Failed to restore '{}': {}", name, e);
            }
        }
    }

    Ok(env)
}

/// Get metadata from .ach file without loading the entire file
pub fn get_metadata(path: impl AsRef<Path>) -> Result<Metadata> {
    let path = path.as_ref();

    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    // Read header
    let header = AchHeader::read(&mut reader)?;

    // Read body
    let mut body_bytes = Vec::new();
    reader.read_to_end(&mut body_bytes)?;

    // Extract data (without checksum)
    if body_bytes.len() < CHECKSUM_SIZE {
        return Err(EnvError::InvalidFormat("File too small".to_string()));
    }

    let data_end = body_bytes.len() - CHECKSUM_SIZE;
    let data_bytes = &body_bytes[..data_end];

    // Decompress if needed
    let decompressed = if header.is_compressed() {
        zstd::decode_all(data_bytes)
            .map_err(|e| EnvError::Decompression(e.to_string()))?
    } else {
        data_bytes.to_vec()
    };

    // Deserialize body (we only need metadata)
    let body: AchBody = rmp_serde::from_slice(&decompressed)?;

    Ok(body.metadata)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use achronyme_types::value::Value;

    #[test]
    fn test_ach_body_serialization() {
        // Test just the AchBody struct serialization
        let mut bindings = HashMap::new();
        bindings.insert(
            "x".to_string(),
            SerializedValue::Number(42.0)
        );

        let metadata = Metadata::new()
            .with_description("Test")
            .with_bindings(vec!["x".to_string()]);

        let body = AchBody {
            metadata,
            bindings,
        };

        // Test serialization
        let encoded = rmp_serde::to_vec(&body).unwrap();
        eprintln!("Encoded {} bytes", encoded.len());

        // Test deserialization
        let decoded: AchBody = rmp_serde::from_slice(&encoded).unwrap();
        assert_eq!(decoded.bindings.len(), 1);
        assert_eq!(decoded.metadata.num_bindings, 1);
    }

    #[test]
    fn test_save_and_restore_basic() {
        let mut env = Environment::new();
        env.define("x".to_string(), Value::Number(42.0)).unwrap();
        env.define("name".to_string(), Value::String("test".to_string())).unwrap();

        let temp = NamedTempFile::new().unwrap();
        let path = temp.path();

        // Save
        save_environment(&env, path, SaveOptions {
            allow_overwrite: true,
            ..Default::default()
        }).unwrap();

        // Restore
        let restored = restore_environment(path, Default::default()).unwrap();

        assert_eq!(restored.get("x").unwrap(), Value::Number(42.0));
        assert_eq!(restored.get("name").unwrap(), Value::String("test".to_string()));
    }

    #[test]
    fn test_save_without_compression() {
        let mut env = Environment::new();
        env.define("x".to_string(), Value::Number(1.0)).unwrap();

        let temp = NamedTempFile::new().unwrap();
        let path = temp.path();

        save_environment(&env, path, SaveOptions {
            compress: false,
            allow_overwrite: true,
            ..Default::default()
        }).unwrap();

        let restored = restore_environment(path, Default::default()).unwrap();
        assert_eq!(restored.get("x").unwrap(), Value::Number(1.0));
    }

    #[test]
    fn test_save_with_filters() {
        let mut env = Environment::new();
        env.define("keep_this".to_string(), Value::Number(1.0)).unwrap();
        env.define("skip_this".to_string(), Value::Number(2.0)).unwrap();
        env.define("temp_var".to_string(), Value::Number(3.0)).unwrap();

        let temp = NamedTempFile::new().unwrap();
        let path = temp.path();

        save_environment(&env, path, SaveOptions {
            include_only: Some(vec!["keep_this".to_string()]),
            allow_overwrite: true,
            ..Default::default()
        }).unwrap();

        let restored = restore_environment(path, Default::default()).unwrap();
        assert_eq!(restored.get("keep_this").unwrap(), Value::Number(1.0));
        assert!(restored.get("skip_this").is_err());
    }

    #[test]
    fn test_get_metadata() {
        let mut env = Environment::new();
        env.define("x".to_string(), Value::Number(1.0)).unwrap();
        env.define("y".to_string(), Value::Number(2.0)).unwrap();

        let temp = NamedTempFile::new().unwrap();
        let path = temp.path();

        save_environment(&env, path, SaveOptions {
            description: Some("Test file".to_string()),
            tags: vec!["test".to_string()],
            allow_overwrite: true,
            ..Default::default()
        }).unwrap();

        let metadata = get_metadata(path).unwrap();
        assert_eq!(metadata.num_bindings, 2);
        assert_eq!(metadata.description, Some("Test file".to_string()));
        assert!(metadata.tags.contains(&"test".to_string()));
    }

    #[test]
    fn test_checksum_verification() {
        let mut env = Environment::new();
        env.define("x".to_string(), Value::Number(1.0)).unwrap();

        let temp = NamedTempFile::new().unwrap();
        let path = temp.path();

        save_environment(&env, path, SaveOptions {
            allow_overwrite: true,
            ..Default::default()
        }).unwrap();

        // Corrupt the file
        let mut data = std::fs::read(path).unwrap();
        data[100] ^= 0xFF; // Flip some bits
        std::fs::write(path, data).unwrap();

        // Should fail with checksum mismatch
        let result = restore_environment(path, Default::default());
        assert!(matches!(result, Err(EnvError::ChecksumMismatch)));
    }
}
