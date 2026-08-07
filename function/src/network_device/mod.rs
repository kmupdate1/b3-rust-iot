pub mod wifi;

pub use wifi::*;

pub trait NetworkDevice {
    type Error;
}
