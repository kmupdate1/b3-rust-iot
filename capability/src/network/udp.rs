use alloc::string::String;

pub trait Udp {
    type Error;
    
    async fn bind(&mut self, port: u16) -> Result<(), Self::Error>;
    async fn send(&mut self, data: &[u8], addr: &str, port: u16) -> Result<(), Self::Error>;
    async fn receive(&mut self, buffer: &mut [u8]) -> Result<(usize, String, u16), Self::Error>;
}
