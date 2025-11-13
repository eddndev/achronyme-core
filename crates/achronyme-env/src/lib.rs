//! Environment persistence and serialization for Achronyme
//!
//! This crate provides functionality to save and restore Achronyme runtime environments
//! to/from `.ach` (Achronyme Archive) files.
//!
//! # Format
//!
//! The `.ach` format uses a hybrid approach:
//! - Custom 64-byte header for version control and metadata offsets
//! - MessagePack serialization for the actual data
//! - Optional Zstandard compression
//! - SHA-256 checksum for integrity verification
//!
//! # Example
//!
//! ```rust,no_run
//! use achronyme_env::{save_environment, restore_environment, SaveOptions};
//! use achronyme_types::Environment;
//! use std::path::Path;
//!
//! let env = Environment::new();
//! // ... populate environment ...
//!
//! // Save
//! save_environment(&env, Path::new("workspace.ach"), SaveOptions::default()).unwrap();
//!
//! // Restore
//! let restored = restore_environment(Path::new("workspace.ach"), Default::default()).unwrap();
//! ```

pub mod errors;
pub mod format;
pub mod metadata;
pub mod persist;
pub mod serialize;
pub mod checksum;

// Re-exports
pub use errors::{EnvError, Result};
pub use format::{AchHeader, MAGIC, FORMAT_VERSION_MAJOR, FORMAT_VERSION_MINOR};
pub use metadata::Metadata;
pub use persist::{save_environment, restore_environment, get_metadata, SaveOptions, RestoreOptions, RestoreMode};
pub use serialize::{serialize_value, deserialize_value};
