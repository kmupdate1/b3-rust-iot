#![no_std]

mod manifest;
mod firmware;
mod activation;
mod version;

pub use manifest::*;
pub use firmware::*;
pub use activation::*;

pub trait Ota {
    type Error;

    async fn update(&mut self) -> Result<(), Self::Error>;
}
