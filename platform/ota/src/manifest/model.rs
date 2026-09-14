use heapless::String;
use crate::version::Version;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateManifest {
    pub version: Version,
    pub firmware_url: String<512>,
    pub size: u32,
    pub sha256: String<64>,
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
