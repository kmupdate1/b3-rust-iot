#![no_std]
#![no_main]

use core::default::Default;
use defmt::*;
use defmt_rtt as _;
use panic_probe as _;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use field_edge::{composePin0, composePin1};
use log::warn;
use function::AnalogReader;
use rp235x::analog::{AdcPin26, Rp235xAdc};
use rp235x::{Pin0, Pin1, Pin26};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());

    let gpio0 = Pin0::new(peripherals.PIN_0);
    let gpio1 = Pin1::new(peripherals.PIN_1);
    let mut blue = composePin0(gpio0);
    let mut red = composePin1(gpio1);

    let mut adc = Rp235xAdc::new(peripherals.ADC);
    let mut channel = AdcPin26::new(Pin26::new(peripherals.PIN_26));

    loop {
        match adc.read(&mut channel).await {
            Ok(raw) => {
                // 一瞬だけ点灯
                blue.start();
                Timer::after(Duration::from_millis(10)).await;

                blue.stop();

                // ADC値そのものを消灯時間(ms)にする
                Timer::after(Duration::from_millis(raw as u64)).await;
            },
            Err(_) => {
                warn!("ADC read failed");
                red.start();
            },
        }

        Timer::after(Duration::from_secs(1)).await;
        red.stop();
    }
}
