pub trait Http {
    type HttpError;
    async fn get(&mut self, url: &str, buffer: &mut [u8]) -> Result<usize, Self::HttpError>;
}
