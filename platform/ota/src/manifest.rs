use heapless::String;
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct  UpdateManifest {
    pub version: String<32>,
    pub firmware_url: String<512>,
    pub size: u32,
    pub sha256: String<64>,
}

#[derive(Deserialize)]
struct WireManifest<'a> {
    version: &'a str,
    firmware_url: Option<&'a str>,
    size: Option<u32>,
    sha256: Option<&'a str>,
}
