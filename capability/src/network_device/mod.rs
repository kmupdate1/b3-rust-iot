pub mod wifi;
pub mod bluetooth;

use core::net::{Ipv4Addr, Ipv6Addr};
pub use wifi::*;
pub use bluetooth::*;

pub trait NetworkDevice {
    type Error;
}

pub trait IpNetworkDevice: NetworkDevice {
    fn ipv4_addr(&self) -> Option<Ipv4Addr>;
    fn ipv6_addr(&self) -> Option<Ipv6Addr>;
}
