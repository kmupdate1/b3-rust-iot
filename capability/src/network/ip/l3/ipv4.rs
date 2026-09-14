use core::net::Ipv4Addr;
use crate::NetworkDevice;

pub trait Ipv4NetworkDevice: NetworkDevice {
    fn ipv4_addr(&self) -> Option<Ipv4Addr>;
    async fn wait_ipv4_addr(&self) -> Result<Ipv4Addr, Self::Error>;
}
