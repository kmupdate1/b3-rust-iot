pub mod host;
pub mod resources;

use crate::Rp235xCyw43;
use capability::{Bluetooth, BluetoothError, NetworkDevice};

pub struct Rp235xBluetooth<'a> {
    network: &'a Rp235xCyw43,
    enabled: bool,
}

impl<'a> Rp235xBluetooth<'a> {
    pub fn new(network: &'a Rp235xCyw43) -> Self {
        Self {
            network,
            enabled: false,
        }
    }
}

impl<'a> NetworkDevice for Rp235xBluetooth<'a> {
    type Error = BluetoothError;
}

impl<'a> Bluetooth for Rp235xBluetooth<'a> {
    async fn enable(&mut self) -> Result<(), Self::Error> {
        if self.enabled { return Ok(()); }

        let mut control = self.network.control.lock().await;
        let mut bluetooth = self.network.bluetooth.lock().await;

        drop(bluetooth);
        drop(control);

        self.enabled = true;

        Ok(())
    }

    async fn disable(&mut self) -> Result<(), Self::Error> {
        if !self.enabled {
            return Err(BluetoothError::NotEnabled);
        }

        let mut control = self.network.control.lock().await;
        let mut bluetooth = self.network.bluetooth.lock().await;

        drop(bluetooth);
        drop(control);

        self.enabled = false;

        Ok(())
    }
}
