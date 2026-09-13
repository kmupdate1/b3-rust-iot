pub trait FirmwareWriter {
    type Error;
    
    fn capacity(&self) -> usize;
    
    fn write(
        &mut self,
        offset: usize,
        data: &[u8],
    ) -> Result<(), Self::Error>;
}
