use heapless::String;
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateManifest {
    pub version: String<32>,
    pub firmware_url: String<512>,
    pub size: u32,
    pub sha256: String<64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManifestError {
    InvalidJson,
    MissingFirmwareUrl,
    MissingSize,
    MissingSha256,
    FieldTooLong,
    InvalidFirmwareUrl,
    InvalidFirmwareSize,
    InvalidSha256,
    InvalidVersion,
}

#[derive(Deserialize)]
struct WireManifest<'a> {
    version: &'a str,
    firmware_url: Option<&'a str>,
    size: Option<u32>,
    sha256: Option<&'a str>,
}

impl UpdateManifest {
    pub fn from_json(data: &[u8]) -> Result<Self, ManifestError> {
        let (wire, _) = serde_json_core::from_slice::<WireManifest<'_>>(data)
            .map_err(|_| ManifestError::InvalidJson)?;

        let firmware_url = wire
            .firmware_url
            .ok_or(ManifestError::MissingFirmwareUrl)?;
        let size = wire.size.ok_or(ManifestError::MissingSize)?;
        let sha256 = wire.sha256.ok_or(ManifestError::MissingSha256)?;

        if !firmware_url.starts_with("https://") {
            return Err(ManifestError::InvalidFirmwareUrl);
        }
        if size == 0 {
            return Err(ManifestError::InvalidFirmwareSize);
        }
        if sha256.len() != 64 || !sha256.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            return Err(ManifestError::InvalidSha256);
        }

        // Validate the version before storing it so every UpdateManifest has
        // the same version semantics as update selection.
        parse_version(wire.version)?;

        let mut version = String::new();
        version
            .push_str(wire.version)
            .map_err(|_| ManifestError::FieldTooLong)?;

        let mut firmware_url_value = String::new();
        firmware_url_value
            .push_str(firmware_url)
            .map_err(|_| ManifestError::FieldTooLong)?;

        let mut sha256_value = String::new();
        sha256_value
            .push_str(sha256)
            .map_err(|_| ManifestError::FieldTooLong)?;

        Ok(Self {
            version,
            firmware_url: firmware_url_value,
            size,
            sha256: sha256_value,
        })
    }

    pub fn is_newer_than(&self, current_version: &str) -> Result<bool, ManifestError> {
        Ok(parse_version(self.version.as_str())? > parse_version(current_version)?)
    }
}

fn parse_version(value: &str) -> Result<(u32, u32, u32), ManifestError> {
    let value = value.strip_prefix('v').unwrap_or(value);
    let mut parts = value.split('.');

    let major = parse_version_part(parts.next())?;
    let minor = parse_version_part(parts.next())?;
    let patch = parse_version_part(parts.next())?;

    if parts.next().is_some() {
        return Err(ManifestError::InvalidVersion);
    }

    Ok((major, minor, patch))
}

fn parse_version_part(part: Option<&str>) -> Result<u32, ManifestError> {
    part.and_then(|part| part.parse().ok())
        .ok_or(ManifestError::InvalidVersion)
}

#[cfg(test)]
mod tests {
    use super::{ManifestError, UpdateManifest};

    const SHA256: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

    #[test]
    fn parses_a_complete_manifest() {
        let data = br#"{
            "version": "v0.1.2",
            "firmware_url": "https://updates.example/firmware.bin",
            "size": 713564,
            "sha256": "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
        }"#;

        let manifest = UpdateManifest::from_json(data).unwrap();

        assert_eq!(manifest.version.as_str(), "v0.1.2");
        assert_eq!(manifest.size, 713564);
        assert_eq!(manifest.sha256.as_str(), SHA256);
    }

    #[test]
    fn compares_numeric_version_components() {
        let data = br#"{
            "version": "0.10.0",
            "firmware_url": "https://updates.example/firmware.bin",
            "size": 1,
            "sha256": "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
        }"#;
        let manifest = UpdateManifest::from_json(data).unwrap();

        assert!(manifest.is_newer_than("0.9.9").unwrap());
        assert!(!manifest.is_newer_than("0.10.0").unwrap());
    }

    #[test]
    fn rejects_incomplete_or_unsafe_manifests() {
        let missing_hash = br#"{
            "version": "0.1.0",
            "firmware_url": "https://updates.example/firmware.bin",
            "size": 1
        }"#;
        assert_eq!(
            UpdateManifest::from_json(missing_hash),
            Err(ManifestError::MissingSha256),
        );

        let insecure_url = br#"{
            "version": "0.1.0",
            "firmware_url": "http://updates.example/firmware.bin",
            "size": 1,
            "sha256": "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
        }"#;
        assert_eq!(
            UpdateManifest::from_json(insecure_url),
            Err(ManifestError::InvalidFirmwareUrl),
        );
    }
}
