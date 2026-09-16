use heapless::String;
use serde::Deserialize;

use crate::{version, FirmwareDigest, ManifestError, UpdateManifest};

#[derive(Deserialize)]
struct WireManifest<'a> {
    version: &'a str,
    firmware_url: Option<&'a str>,
    size: Option<u32>,
    sha256: Option<&'a str>,
}

pub fn json(data: &[u8]) -> Result<UpdateManifest, ManifestError> {
    let (wire, _) = serde_json_core::from_slice::<WireManifest<'_>>(data)
        .map_err(|_| ManifestError::InvalidManifest)?;
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

    let mut location = String::new();
    location
        .push_str(firmware_url)
        .map_err(|_| ManifestError::FieldTooLong)?;

    Ok(UpdateManifest {
        version: version::parse(wire.version)?,
        firmware_url: location,
        size,
        digest: FirmwareDigest::from_hex(sha256)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_manifest_into_domain_values() {
        let manifest = json(
            br#"{"version":"v1.2.3","firmware_url":"https://example.test/fw.bin","size":42,"sha256":"0000000000000000000000000000000000000000000000000000000000000000"}"#,
        )
        .unwrap();

        assert_eq!(manifest.version.major, 1);
        assert_eq!(manifest.size, 42);
        assert_eq!(manifest.digest.as_bytes(), &[0; 32]);
    }
}