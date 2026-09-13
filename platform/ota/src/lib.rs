#![no_std]

mod manifest;
mod manifest_source;
mod firmware_downloader;
mod firmware_writer;
mod firmware_verifier;
mod boot_state;

pub use manifest::*;
pub use manifest_source::*;
pub use firmware_downloader::*;
pub use firmware_writer::*;
pub use firmware_verifier::*;
pub use boot_state::*;

pub trait Ota {
    type Error;

    async fn update(&mut self) -> Result<(), Self::Error>;
}
