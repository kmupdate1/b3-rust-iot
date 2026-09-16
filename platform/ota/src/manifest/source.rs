/// Provides the raw manifest document. Parsing belongs to the OTA platform.
pub trait ManifestSource {
    type Error;

    async fn read(&mut self, buffer: &mut [u8]) -> Result<usize, Self::Error>;
}