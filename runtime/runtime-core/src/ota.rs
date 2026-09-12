pub trait Ota {
    type Error;

    async fn available(&mut self, current_v: &str) -> Result<bool, Self::Error>;
    async fn update(&mut self) -> Result<(), Self::Error>;
}
