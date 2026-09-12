use embassy_rp::i2c::{Async, Error, Instance, I2c as EmbassyI2c, InterruptHandler, SclPin, SdaPin, Config};
use embassy_rp::interrupt::typelevel::Binding;
use embassy_rp::Peri;
use capability::communication::I2c;

pub struct Rp235xI2c<'d, T: Instance> {
    i2c: EmbassyI2c<'d, T, Async>,
}

impl<'d, T: Instance> Rp235xI2c<'d, T> {
    pub fn new(
        i2c: Peri<'d, T>,
        scl: Peri<'d, impl SclPin<T>>,
        sda: Peri<'d, impl SdaPin<T>>,
        irq: impl Binding<T::Interrupt, InterruptHandler<T>>,
    ) -> Self {
        let i2c = EmbassyI2c::new_async(
            i2c, scl, sda, irq,
            Config::default(),
        );

        Self { i2c }
    }
}

impl<T: Instance> I2c for Rp235xI2c<'_, T> {
    type Error = Error;

    async fn write(&mut self, address: u8, data: &[u8]) -> Result<(), Self::Error> {
        self.i2c
            .write_async(address, data.iter().copied())
            .await
    }

    async fn read(&mut self, address: u8, data: &mut [u8]) -> Result<(), Self::Error> {
        self.i2c
            .read_async(address, data)
            .await
    }

    async fn write_read(&mut self, address: u8, write: &[u8], read: &mut [u8]) -> Result<(), Self::Error> {
        self.i2c
            .write_read_async(address, write.iter().copied(), read)
            .await
    }
}
