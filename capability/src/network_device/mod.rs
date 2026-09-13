pub mod wifi;
pub mod bluetooth;

use core::net::{Ipv4Addr, Ipv6Addr};
pub use wifi::*;
pub use bluetooth::*;

pub trait NetworkDevice {
    type Error;
}

pub trait Ipv4NetworkDevice: NetworkDevice {
    fn ipv4_addr(&self) -> Option<Ipv4Addr>;
    async fn wait_ipv4_addr(&self) -> Result<Ipv4Addr, Self::Error>;
}

pub trait Ipv6NetworkDevice: NetworkDevice {
    fn ipv6_addr(&self) -> Option<Ipv6Addr>;
    
    async fn wait_ipv6_addr(&self) -> Result<Ipv6Addr, Self::Error>;
}
