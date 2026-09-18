use heapless::String;

use crate::Version;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FirmwareDigest([u8; 32]);

impl FirmwareDigest {
    pub const fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    pub(crate) fn from_hex(value: &str) -> Result<Self, ManifestError> {
        fn nibble(value: u8) -> Option<u8> {
            match value {
                b'0'..=b'9' => Some(value - b'0'),
                b'a'..=b'f' => Some(value - b'a' + 10),
                b'A'..=b'F' => Some(value - b'A' + 10),
                _ => None,
            }
        }

        let encoded = value.as_bytes();
        if encoded.len() != 64 {
            return Err(ManifestError::InvalidSha256);
        }

        let mut bytes = [0; 32];
        for (index, byte) in bytes.iter_mut().enumerate() {
            let high = nibble(encoded[index * 2]).ok_or(ManifestError::InvalidSha256)?;
            let low = nibble(encoded[index * 2 + 1]).ok_or(ManifestError::InvalidSha256)?;
            *byte = high << 4 | low;
        }

        Ok(Self(bytes))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateManifest {
    pub version: Version,
    pub firmware_url: String<512>,
    pub size: u32,
    pub digest: FirmwareDigest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManifestError {
    MissingFirmwareUrl,
    MissingSize,
    MissingSha256,
    FieldTooLong,
    InvalidManifest,
    InvalidFirmwareUrl,
    InvalidFirmwareSize,
    InvalidSha256,
    InvalidVersion,
}
