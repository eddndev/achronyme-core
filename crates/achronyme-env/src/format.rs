use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Read, Write};
use crate::errors::{EnvError, Result};

/// Magic number for .ach files: "ACH\0"
pub const MAGIC: [u8; 4] = [0x41, 0x43, 0x48, 0x00];

/// Major version of the .ach format
pub const FORMAT_VERSION_MAJOR: u16 = 1;

/// Minor version of the .ach format
pub const FORMAT_VERSION_MINOR: u16 = 0;

/// Size of the header in bytes
pub const HEADER_SIZE: usize = 64;

/// Feature flags
pub mod flags {
    /// No special features
    pub const NONE: u32 = 0;

    /// File is compressed with Zstd
    pub const COMPRESSED: u32 = 1 << 0;

    /// File has extended metadata
    pub const HAS_METADATA: u32 = 1 << 1;

    /// File includes user-defined functions (future)
    pub const HAS_FUNCTIONS: u32 = 1 << 2;
}

/// Compression algorithms
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionType {
    None = 0,
    Zstd = 1,
}

impl CompressionType {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(CompressionType::None),
            1 => Some(CompressionType::Zstd),
            _ => None,
        }
    }
}

/// Header for .ach files (64 bytes total)
#[derive(Debug, Clone)]
pub struct AchHeader {
    /// Magic bytes: "ACH\0"
    pub magic: [u8; 4],

    /// Major version of format
    pub version_major: u16,

    /// Minor version of format
    pub version_minor: u16,

    /// Feature flags
    pub flags: u32,

    /// Unix timestamp of creation
    pub created_timestamp: u64,

    /// Achronyme version that created this file (padded to 16 bytes)
    pub achronyme_version: [u8; 16],

    /// Compression type
    pub compression: CompressionType,

    /// Reserved for future use (27 bytes to make total 64)
    pub reserved: [u8; 27],
}

impl AchHeader {
    /// Create a new header with default values
    pub fn new() -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Get version from Cargo.toml or use default
        let version_str = env!("CARGO_PKG_VERSION");
        let mut version_bytes = [0u8; 16];
        let bytes = version_str.as_bytes();
        let len = bytes.len().min(15); // Leave room for null terminator
        version_bytes[..len].copy_from_slice(&bytes[..len]);

        Self {
            magic: MAGIC,
            version_major: FORMAT_VERSION_MAJOR,
            version_minor: FORMAT_VERSION_MINOR,
            flags: flags::NONE,
            created_timestamp: timestamp,
            achronyme_version: version_bytes,
            compression: CompressionType::None,
            reserved: [0u8; 27],
        }
    }

    /// Write header to a writer
    pub fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(&self.magic)?;
        writer.write_u16::<LittleEndian>(self.version_major)?;
        writer.write_u16::<LittleEndian>(self.version_minor)?;
        writer.write_u32::<LittleEndian>(self.flags)?;
        writer.write_u64::<LittleEndian>(self.created_timestamp)?;
        writer.write_all(&self.achronyme_version)?;
        writer.write_u8(self.compression as u8)?;
        writer.write_all(&self.reserved)?;
        Ok(())
    }

    /// Read header from a reader
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let mut magic = [0u8; 4];
        reader.read_exact(&mut magic)?;

        if magic != MAGIC {
            return Err(EnvError::InvalidFormat(
                format!("Invalid magic bytes: expected {:?}, got {:?}", MAGIC, magic)
            ));
        }

        let version_major = reader.read_u16::<LittleEndian>()?;
        let version_minor = reader.read_u16::<LittleEndian>()?;
        let flags = reader.read_u32::<LittleEndian>()?;
        let created_timestamp = reader.read_u64::<LittleEndian>()?;

        let mut achronyme_version = [0u8; 16];
        reader.read_exact(&mut achronyme_version)?;

        let compression_byte = reader.read_u8()?;
        let compression = CompressionType::from_u8(compression_byte)
            .ok_or_else(|| EnvError::InvalidFormat(
                format!("Invalid compression type: {}", compression_byte)
            ))?;

        let mut reserved = [0u8; 27];
        reader.read_exact(&mut reserved)?;

        Ok(Self {
            magic,
            version_major,
            version_minor,
            flags,
            created_timestamp,
            achronyme_version,
            compression,
            reserved,
        })
    }

    /// Check if file is compressed
    pub fn is_compressed(&self) -> bool {
        self.flags & flags::COMPRESSED != 0
    }

    /// Set compression flag
    pub fn set_compressed(&mut self, compressed: bool) {
        if compressed {
            self.flags |= flags::COMPRESSED;
        } else {
            self.flags &= !flags::COMPRESSED;
        }
    }

    /// Get Achronyme version as string
    pub fn achronyme_version_string(&self) -> String {
        let end = self.achronyme_version
            .iter()
            .position(|&b| b == 0)
            .unwrap_or(16);
        String::from_utf8_lossy(&self.achronyme_version[..end]).to_string()
    }

    /// Verify version compatibility
    pub fn verify_version(&self, strict: bool) -> Result<()> {
        if strict && self.version_major != FORMAT_VERSION_MAJOR {
            return Err(EnvError::VersionMismatch {
                file_version: format!("{}.{}", self.version_major, self.version_minor),
                current_version: format!("{}.{}", FORMAT_VERSION_MAJOR, FORMAT_VERSION_MINOR),
            });
        }

        // For now, we only support version 1.x
        if self.version_major > FORMAT_VERSION_MAJOR {
            return Err(EnvError::VersionMismatch {
                file_version: format!("{}.{}", self.version_major, self.version_minor),
                current_version: format!("{}.{}", FORMAT_VERSION_MAJOR, FORMAT_VERSION_MINOR),
            });
        }

        Ok(())
    }
}

impl Default for AchHeader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_header_size() {
        let header = AchHeader::new();
        let mut buf = Vec::new();
        header.write(&mut buf).unwrap();
        assert_eq!(buf.len(), HEADER_SIZE);
    }

    #[test]
    fn test_header_roundtrip() {
        let header = AchHeader::new();
        let mut buf = Vec::new();
        header.write(&mut buf).unwrap();

        let mut cursor = Cursor::new(buf);
        let restored = AchHeader::read(&mut cursor).unwrap();

        assert_eq!(restored.magic, MAGIC);
        assert_eq!(restored.version_major, FORMAT_VERSION_MAJOR);
        assert_eq!(restored.version_minor, FORMAT_VERSION_MINOR);
    }

    #[test]
    fn test_invalid_magic() {
        let mut buf = vec![0xFF, 0xFF, 0xFF, 0xFF];
        buf.extend_from_slice(&[0u8; 60]);

        let mut cursor = Cursor::new(buf);
        let result = AchHeader::read(&mut cursor);

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), EnvError::InvalidFormat(_)));
    }

    #[test]
    fn test_compression_flag() {
        let mut header = AchHeader::new();
        assert!(!header.is_compressed());

        header.set_compressed(true);
        assert!(header.is_compressed());
        assert_eq!(header.flags & flags::COMPRESSED, flags::COMPRESSED);

        header.set_compressed(false);
        assert!(!header.is_compressed());
    }

    #[test]
    fn test_version_string() {
        let header = AchHeader::new();
        let version = header.achronyme_version_string();
        assert!(!version.is_empty());
    }
}
