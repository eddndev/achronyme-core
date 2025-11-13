use thiserror::Error;

/// Result type for environment operations
pub type Result<T> = std::result::Result<T, EnvError>;

/// Errors that can occur during environment persistence operations
#[derive(Error, Debug)]
pub enum EnvError {
    /// I/O error
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// Deserialization error
    #[error("Deserialization error: {0}")]
    Deserialization(String),

    /// Invalid file format
    #[error("Invalid .ach file: {0}")]
    InvalidFormat(String),

    /// Version mismatch
    #[error("Version mismatch: file version {file_version}, current version {current_version}")]
    VersionMismatch {
        file_version: String,
        current_version: String,
    },

    /// Checksum verification failed
    #[error("Checksum verification failed")]
    ChecksumMismatch,

    /// Compression error
    #[error("Compression error: {0}")]
    Compression(String),

    /// Decompression error
    #[error("Decompression error: {0}")]
    Decompression(String),

    /// Binding not found
    #[error("Binding not found: {0}")]
    BindingNotFound(String),

    /// Invalid option
    #[error("Invalid option: {0}")]
    InvalidOption(String),

    /// Custom error
    #[error("{0}")]
    Custom(String),
}

impl From<rmp_serde::encode::Error> for EnvError {
    fn from(err: rmp_serde::encode::Error) -> Self {
        EnvError::Serialization(err.to_string())
    }
}

impl From<rmp_serde::decode::Error> for EnvError {
    fn from(err: rmp_serde::decode::Error) -> Self {
        EnvError::Deserialization(err.to_string())
    }
}
