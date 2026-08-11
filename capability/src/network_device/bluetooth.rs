use crate::NetworkDevice;

pub trait Bluetooth: NetworkDevice {
    async fn enable(&mut self) -> Result<(), Self::Error>;
    async fn disable(&mut self) -> Result<(), Self::Error>;
}

#[derive(Debug)]
pub enum BluetoothError {
    NotEnabled,
    AlreadyEnabled,
}
