use crate::{Rp235xCyw43, Rp235xTcp, Rp235xUdp};
use cyw43::JoinOptions;
use capability::{NetworkDevice, Wifi};

pub struct Rp235xWifi<'a> {
    network: &'a Rp235xCyw43,
}

impl <'a> Rp235xWifi<'a> {
    pub fn new(network: &'a Rp235xCyw43) -> Self {
        Self { network }
    }
}

impl Rp235xWifi<'_> {
    pub fn tcp(&self) -> Rp235xTcp {
        Rp235xTcp::new(self.network.stack)
    }

    pub fn udp(&self) -> Rp235xUdp {
        Rp235xUdp::new(self.network.stack)
    }
}

impl NetworkDevice for Rp235xWifi<'_> {
    type Error = cyw43::JoinError;
}

impl Wifi for Rp235xWifi<'_> {
    async fn connect(&mut self, ssid: &str, password: &str) -> Result<(), Self::Error> {
        let mut control = self.network.control.lock().await;

        control
            .join(
                ssid,
                JoinOptions::new(password.as_bytes()),
            )
            .await?;

        drop(control);

        self.network.stack.wait_link_up().await;
        self.network.stack.wait_config_up().await;

        Ok(())
    }
}
