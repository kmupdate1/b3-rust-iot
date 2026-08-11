use crate::NetworkDevice;

pub trait Wifi: NetworkDevice {
    async fn connect(
        &mut self,
        ssid: &str,
        password: &str,
    ) -> Result<(), Self::Error>;
}

#[derive(Debug)]
pub enum WifiError {}

pub struct WifiConfig<'a> {
    pub hostname: &'a str,
    pub ssid: &'a str,
    pub password: &'a str,
}
