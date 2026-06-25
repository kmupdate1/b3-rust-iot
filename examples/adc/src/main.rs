#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::gpio;
use embassy_time::Timer;
use gpio::{Input, Level, Output, Pull};
use {defmt_rtt as _, panic_probe as _};

use hal_traits::analog_in::AnalogIn;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let gpio_26 = AnalogIn::read()
}
