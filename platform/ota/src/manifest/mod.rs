mod model;
mod source;
pub mod parser;

use heapless::String;
pub use model::*;
pub use source::*;
use crate::version;

impl UpdateManifest {
    fn new(
        version: &str,
        firmware_url: &str,
        size: u32,
        sha256: &str,
    ) -> Result<Self, ManifestError> {
        if !firmware_url.starts_with("https://") {
            return Err(ManifestError::InvalidFirmwareUrl);
        }
        if size == 0 {
            return Err(ManifestError::InvalidFirmwareSize);
        }
        if sha256.len() != 64 || !sha256.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            return Err(ManifestError::InvalidSha256);
        }

        let mut firmware_url_value = String::new();
        firmware_url_value
            .push_str(firmware_url)
            .map_err(|_| ManifestError::FieldTooLong)?;

        let mut sha256_value = String::new();
        sha256_value
            .push_str(sha256)
            .map_err(|_| ManifestError::FieldTooLong)?;

        Ok(Self {
            version: version::parser::parse(version)?,
            firmware_url: firmware_url_value,
            size,
            sha256: sha256_value,
        })
    }
}
