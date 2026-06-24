use core::result::Result;

pub trait NetworkDevice {
    type Error: core::fmt::Debug;
    async fn send(&mut self, data: &[u8]) -> Result<(), Self::Error>;
    async fn receive(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error>;
}

pub trait Wifi: NetworkDevice {
    async fn connect(&mut self, ssid: &str, pwd: &str) -> Result<(), Self::Error>;
}
