use sha2::{Sha256, Digest};

/// Size of SHA-256 checksum in bytes
pub const CHECKSUM_SIZE: usize = 32;

/// Calculate SHA-256 checksum of data
pub fn calculate_checksum(data: &[u8]) -> [u8; CHECKSUM_SIZE] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}

/// Verify checksum
pub fn verify_checksum(data: &[u8], expected: &[u8; CHECKSUM_SIZE]) -> bool {
    let computed = calculate_checksum(data);
    computed == *expected
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checksum_calculation() {
        let data = b"hello world";
        let checksum = calculate_checksum(data);
        assert_eq!(checksum.len(), CHECKSUM_SIZE);
    }

    #[test]
    fn test_checksum_verification() {
        let data = b"hello world";
        let checksum = calculate_checksum(data);
        assert!(verify_checksum(data, &checksum));
    }

    #[test]
    fn test_checksum_mismatch() {
        let data = b"hello world";
        let checksum = calculate_checksum(data);
        let wrong_data = b"goodbye world";
        assert!(!verify_checksum(wrong_data, &checksum));
    }

    #[test]
    fn test_checksum_deterministic() {
        let data = b"test data";
        let checksum1 = calculate_checksum(data);
        let checksum2 = calculate_checksum(data);
        assert_eq!(checksum1, checksum2);
    }
}
