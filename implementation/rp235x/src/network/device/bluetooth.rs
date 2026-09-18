pub mod host;
pub mod resources;

use capability::{Bluetooth, BluetoothError, NetworkDevice};
use cyw43::bluetooth::BtDriver;
use cyw43::Control;
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::mutex::Mutex;

pub struct Rp235xBluetooth<'a> {
    control: &'a Mutex<ThreadModeRawMutex, Control<'static>>,
    bluetooth: &'a Mutex<ThreadModeRawMutex, BtDriver<'static>>,
    enabled: bool,
}

impl<'a> Rp235xBluetooth<'a> {
    pub(crate) fn new(
        control: &'a Mutex<ThreadModeRawMutex, Control<'static>>,
        bluetooth: &'a Mutex<ThreadModeRawMutex, BtDriver<'static>>,
    ) -> Self {
        Self {
            control,
            bluetooth,
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

        let control = self.control.lock().await;
        let bluetooth = self.bluetooth.lock().await;

        drop(bluetooth);
        drop(control);

        self.enabled = true;

        Ok(())
    }

    async fn disable(&mut self) -> Result<(), Self::Error> {
        if !self.enabled {
            return Err(BluetoothError::NotEnabled);
        }

        let control = self.control.lock().await;
        let bluetooth = self.bluetooth.lock().await;

        drop(bluetooth);
        drop(control);

        self.enabled = false;

        Ok(())
    }
}
