use crate::FirmwareVerifier;

/// Opens a stateful write session for an inactive update slot.
pub trait FirmwareStorage {
    type Error;
    type Writer<'a>: FirmwareWriter<Error = Self::Error>
        + FirmwareVerifier<Error = Self::Error>
    where
        Self: 'a;

    fn writer(&mut self) -> Result<Self::Writer<'_>, Self::Error>;
}

/// A single stateful firmware write session.
pub trait FirmwareWriter {
    type Error;

    fn capacity(&self) -> usize;

    /// Largest chunk accepted by one call to `write`.
    fn max_write_size(&self) -> usize;

    fn write(&mut self, offset: usize, data: &[u8]) -> Result<(), Self::Error>;
}