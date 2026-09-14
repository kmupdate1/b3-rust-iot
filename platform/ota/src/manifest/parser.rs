use crate::{ManifestError, UpdateManifest};
use serde::Deserialize;

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

    let size = wire
        .size
        .ok_or(ManifestError::MissingSize)?;

    let sha256 = wire
        .sha256
        .ok_or(ManifestError::MissingSha256)?;

    UpdateManifest::new(
        wire.version,
        firmware_url,
        size,
        sha256,
    )
}
