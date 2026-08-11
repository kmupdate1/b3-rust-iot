pub mod wifi;
pub mod bluetooth;

pub use wifi::*;
pub use bluetooth::*;

pub trait NetworkDevice {
    type Error;
}
