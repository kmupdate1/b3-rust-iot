use capability::l7::http::Http;
use ota::{FirmwareSource, ManifestSource};

use crate::Rp235xHttp;
use super::Rp235xOtaError;

pub struct Rp235xUpdateSource {
    http: Rp235xHttp,
    manifest_url: &'static str,
}

impl Rp235xUpdateSource {
    pub const fn new(http: Rp235xHttp, manifest_url: &'static str) -> Self {
        Self { http, manifest_url }
    }
}

impl ManifestSource for Rp235xUpdateSource {
    type Error = Rp235xOtaError;

    async fn read(&mut self, buffer: &mut [u8]) -> Result<usize, Self::Error> {
        self.http
            .get(self.manifest_url, buffer)
            .await
            .map_err(|_| Rp235xOtaError::Http)
    }
}

impl FirmwareSource for Rp235xUpdateSource {
    type Error = Rp235xOtaError;

    async fn read(
        &mut self,
        location: &str,
        offset: usize,
        buffer: &mut [u8],
    ) -> Result<usize, Self::Error> {
        if buffer.is_empty() { return Ok(0); }

        self.http
            .get_range(location, offset, offset + buffer.len() - 1, buffer)
            .await
            .map_err(|_| Rp235xOtaError::Http)
    }
}
