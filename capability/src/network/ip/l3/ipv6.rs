use core::net::Ipv6Addr;
use crate::NetworkDevice;

pub trait Ipv6NetworkDevice: NetworkDevice {
    fn ipv6_addr(&self) -> Option<Ipv6Addr>;

    async fn wait_ipv6_addr(&self) -> Result<Ipv6Addr, Self::Error>;
}
