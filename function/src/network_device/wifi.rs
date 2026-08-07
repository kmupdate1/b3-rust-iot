use crate::NetworkDevice;

pub trait Wifi: NetworkDevice {
    async fn connect(
        &mut self,
        ssid: &str,
        password: &str,
    ) -> Result<(), Self::Error>;
}
