/// Controls the persistent boot state shared with the bootloader.
pub trait BootState {
    type Error;

    /// Confirms that the currently running tentative image is healthy.
    fn confirm_boot(&mut self) -> Result<(), Self::Error>;

    /// Selects the staged image for the next boot.
    fn mark_updated(&mut self) -> Result<(), Self::Error>;
}
