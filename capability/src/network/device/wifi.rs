use crate::device::NetworkDevice;

pub trait Wifi: NetworkDevice {
    async fn connect(
        &mut self,
        ssid: &str,
        password: &str,
    ) -> Result<(), Self::Error>;

    async fn disconnect(&mut self) -> Result<(), Self::Error>;
}

#[derive(Debug)]
pub enum WifiError {}

pub struct WifiCredentials<'a> {
    pub ssid: &'a str,
    pub password: &'a str,
}
