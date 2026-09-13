pub trait BootState {
    type Error;
    
    fn confirm_boot(&mut self) -> Result<(), Self::Error>;
    
    fn mark_updated(&mut self) -> Result<(), Self::Error>;
}
