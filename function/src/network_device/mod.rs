pub trait NetworkDevice {
    type Error;

    async fn connect(&mut self) -> Result<(), Self::Error>;
}
