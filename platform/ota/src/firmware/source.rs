/// Provides random-access reads from a firmware artifact.
pub trait FirmwareSource {
    type Error;

    async fn read(
        &mut self,
        location: &str,
        offset: usize,
        buffer: &mut [u8],
    ) -> Result<usize, Self::Error>;
}