/// Writes a firmware artifact into an inactive update slot.
pub trait FirmwareWriter {
    type Error;

    fn capacity(&self) -> usize;

    /// Largest chunk accepted by one call to `write`.
    fn max_write_size(&self) -> usize;

    fn write(&mut self, offset: usize, data: &[u8]) -> Result<(), Self::Error>;
}