/*
use interface::network_device::NetworkDevice;

pub struct WifiDriver {

}

pub enum WifiError {
    ConnectionLost,
    Timeout,
    InvalidPacket,
    HardwareFailure(u32),
}

impl NetworkDevice for WifiDriver {
    type Error = WifiError;

    async fn send(&mut self, data: &[u8]) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn receive(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        Ok(0)
    }
}
*/
