use crate::{Rp235xHttp, Rp235xTcp, Rp235xUdp};
use capability::{Ipv4NetworkDevice, Ipv6NetworkDevice, NetworkDevice, Wifi};
use core::net::{Ipv4Addr, Ipv6Addr};
use cyw43::{Control, JoinOptions};
use embassy_net::Stack;
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_time::Timer;

pub struct Rp235xWifi<'a> {
    control: &'a Mutex<ThreadModeRawMutex, Control<'static>>,
    stack: Stack<'static>,
}

impl <'a> Rp235xWifi<'a> {
    pub(crate) fn new(
        control: &'a Mutex<ThreadModeRawMutex, Control<'static>>,
        stack: Stack<'static>,
    ) -> Self {
        Self { control, stack }
    }
}

impl Rp235xWifi<'_> {
    pub fn http(&self) -> Rp235xHttp {
        Rp235xHttp::new(self.stack)
    }

    pub fn tcp(&self) -> Rp235xTcp {
        Rp235xTcp::new(self.stack)
    }

    pub fn udp(&self) -> Rp235xUdp {
        Rp235xUdp::new(self.stack)
    }
}

impl NetworkDevice for Rp235xWifi<'_> {
    type Error = cyw43::JoinError;
}

impl Ipv4NetworkDevice for Rp235xWifi<'_> {
    fn ipv4_addr(&self) -> Option<Ipv4Addr> {
        let addr = self.stack.config_v4()?.address.address();

        Some(Ipv4Addr::from(addr.octets()))
    }

    async fn wait_ipv4_addr(&self) -> Result<Ipv4Addr, Self::Error> {
        self.stack.wait_config_up().await;

        loop {
            if let Some(addr) = self.ipv4_addr() {
                return Ok(addr);
            }

            Timer::after_millis(100).await;
        }
    }
}

impl Ipv6NetworkDevice for Rp235xWifi<'_> {
    fn ipv6_addr(&self) -> Option<Ipv6Addr> {
        let addr = self.stack.config_v6()?.address.address();

        Some(Ipv6Addr::from(addr.octets()))
    }

    async fn wait_ipv6_addr(&self) -> Result<Ipv6Addr, Self::Error> {
        loop {
            if let Some(addr) = self.ipv6_addr() {
                return Ok(addr);
            }

            Timer::after_millis(100).await;
        }
    }
}

impl Wifi for Rp235xWifi<'_> {
    async fn connect(&mut self, ssid: &str, password: &str) -> Result<(), Self::Error> {
        let mut control = self.control.lock().await;

        control
            .join(
                ssid,
                JoinOptions::new(password.as_bytes()),
            )
            .await?;

        drop(control);

        self.stack.wait_link_up().await;

        Ok(())
    }

    async fn disconnect(&mut self) -> Result<(), Self::Error> {
        let mut control = self.control.lock().await;

        control.leave().await;

        drop(control);

        self.stack.wait_config_down().await;
        self.stack.wait_link_down().await;

        Ok(())
    }
}
