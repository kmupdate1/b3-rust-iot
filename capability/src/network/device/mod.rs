pub mod wifi;
pub mod bluetooth;
mod ethernet;

pub use bluetooth::*;
pub use ethernet::*;
pub use wifi::*;

pub trait NetworkDevice {
    type Error;
}
