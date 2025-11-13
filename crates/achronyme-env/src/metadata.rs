use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Metadata for .ach files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    /// Who/what created this file
    pub created_by: String,

    /// When the file was created (ISO 8601)
    pub created_at: String,

    /// Platform (OS + architecture)
    pub platform: String,

    /// Number of variable bindings
    pub num_bindings: u32,

    /// Optional description
    #[serde(default)]
    pub description: Option<String>,

    /// Optional tags for categorization
    #[serde(default)]
    pub tags: Vec<String>,

    /// Custom metadata fields
    #[serde(default)]
    pub custom: HashMap<String, String>,

    /// List of binding names (for quick inspection)
    #[serde(default)]
    pub binding_names: Vec<String>,
}

impl Metadata {
    /// Create new metadata with defaults
    pub fn new() -> Self {
        let now: DateTime<Utc> = Utc::now();
        let platform = format!("{} {}", std::env::consts::OS, std::env::consts::ARCH);

        Self {
            created_by: format!("Achronyme v{}", env!("CARGO_PKG_VERSION")),
            created_at: now.to_rfc3339(),
            platform,
            num_bindings: 0,
            description: None,
            tags: Vec::new(),
            custom: HashMap::new(),
            binding_names: Vec::new(),
        }
    }

    /// Set description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Add tags
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    /// Add custom field
    pub fn with_custom(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.custom.insert(key.into(), value.into());
        self
    }

    /// Set binding names
    pub fn with_bindings(mut self, names: Vec<String>) -> Self {
        self.num_bindings = names.len() as u32;
        self.binding_names = names;
        self
    }
}

impl Default for Metadata {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata_creation() {
        let meta = Metadata::new();
        assert!(meta.created_by.contains("Achronyme"));
        assert!(!meta.created_at.is_empty());
        assert!(!meta.platform.is_empty());
    }

    #[test]
    fn test_metadata_builder() {
        let meta = Metadata::new()
            .with_description("Test workspace")
            .with_tags(vec!["experiment".into(), "test".into()])
            .with_custom("project", "my_project")
            .with_bindings(vec!["x".into(), "y".into()]);

        assert_eq!(meta.description, Some("Test workspace".into()));
        assert_eq!(meta.tags.len(), 2);
        assert_eq!(meta.custom.get("project"), Some(&"my_project".into()));
        assert_eq!(meta.num_bindings, 2);
    }

    #[test]
    fn test_metadata_serialization() {
        let meta = Metadata::new()
            .with_description("Test")
            .with_bindings(vec!["x".into()]);

        let encoded = rmp_serde::to_vec(&meta).unwrap();
        let decoded: Metadata = rmp_serde::from_slice(&encoded).unwrap();

        assert_eq!(meta.description, decoded.description);
        assert_eq!(meta.num_bindings, decoded.num_bindings);
    }
}
