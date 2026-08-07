#![no_std]
#![no_main]

use core::default::Default;
use defmt::*;
use defmt::export::bool;
use defmt_rtt as _;
use panic_probe as _;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use field_edge::{composePin0, composePin1};
use log::warn;
use function::{AnalogReader, Wifi, WifiConfig};
use rp235x::analog::{AdcPin26, Rp235xAdc};
use rp235x::{Pico2wWifiResources, Pin0, Pin1, Pin26, Rp235xWifi};

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());

    let gpio0 = Pin0::new(peripherals.PIN_0);
    let gpio1 = Pin1::new(peripherals.PIN_1);
    let mut blue = composePin0(gpio0);
    let mut red = composePin1(gpio1);

    blue.start();
    Timer::after(Duration::from_secs(1)).await;
    blue.stop();

    let wifi_resources = Pico2wWifiResources::new(
        peripherals.PIN_23,
        peripherals.PIN_25,
        peripherals.PIN_24,
        peripherals.PIN_29,
        peripherals.PIO0,
        peripherals.DMA_CH0,
        peripherals.DMA_CH1,
    );

    red.start();
    Timer::after(Duration::from_secs(1)).await;
    let wifi_config = WifiConfig {
        hostname: "b3c-field-edge-0001",
        ssid: "Buffalo-2G-8F20",
        password: "hrtsedgmndi6c",
    };
    let mut wifi = Rp235xWifi::builder(wifi_resources, spawner, wifi_config).await;
    red.stop();

    match wifi
        .connect("Buffalo-2G-8F20", "hrtsedgmndi6c")
        .await
    {
        Ok(_) => {
            for _i in 1..3 {
                blue.start();
                Timer::after(Duration::from_secs(1)).await;
                blue.stop();
                Timer::after(Duration::from_secs(1)).await;
            }
        },
        Err(_) => {
            for _i in 0..5 {
                red.start();
                Timer::after(Duration::from_secs(1)).await;
                red.stop();
                Timer::after(Duration::from_secs(1)).await;
            }
        },
    }

    let mut adc = Rp235xAdc::new(peripherals.ADC);
    let mut channel = AdcPin26::new(Pin26::new(peripherals.PIN_26));

    let mut rx_buffer = [0u8; 1024];
    let mut tx_buffer = [0u8; 1024];
    let tcp = wifi.tcp();

    loop {
        let mut connection = match tcp
            .connect(
                "192.168.11.3", 23065,
                &mut rx_buffer, &mut tx_buffer,
            )
            .await
        {
            Ok(t) => t,
            Err(_) => {
                red.start();
                Timer::after_secs(3).await;
                red.stop();
                continue;
            }
        };

        for _i in 0..5 {
            blue.start();
            red.start();
            Timer::after(Duration::from_millis(100)).await;
            blue.stop();
            red.stop();
            Timer::after(Duration::from_millis(100)).await;
        }

        blue.start();
        red.start();
        Timer::after(Duration::from_secs(7)).await;
        blue.stop();
        red.stop();
        Timer::after(Duration::from_secs(3)).await;

        loop {
            match adc.read(&mut channel).await {
                Ok(raw) => {
                    // 一瞬だけ点灯
                    blue.start();

                    let mut buffer = [0u8; 32];
                    let len = encode_observation(1, 1, raw as f64, &mut buffer);

                    match connection.send(&buffer[..len]).await {
                        Ok(_) => {
                            blue.stop();
                        },
                        Err(_) => {
                            blue.stop();
                            for _ in 0..3 {
                                red.start();
                                Timer::after(Duration::from_millis(100)).await;
                                red.stop();
                                Timer::after(Duration::from_millis(100)).await;
                            }

                            Timer::after(Duration::from_secs(1)).await;
                            break;
                        },
                    }

                    Timer::after(Duration::from_secs(10)).await;
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
}

fn encode_observation(
    equipment_id: u32,
    channel_id: u32,
    value: f64,
    buf: &mut [u8; 32],
) -> usize {
    let mut i = 0;

    // field 1: uint32 equipment_id
    // tag = (1 << 3) | 0 = 0x08
    buf[i] = 0x08;
    i += 1;
    i += encode_varint(equipment_id as u64, &mut buf[i..]);

    // field 2: uint32 channel_id
    // tag = (2 << 3) | 0 = 0x10
    buf[i] = 0x10;
    i += 1;
    i += encode_varint(channel_id as u64, &mut buf[i..]);

    // field 3: double value
    // tag = (3 << 3) | 1 = 0x19
    buf[i] = 0x19;
    i += 1;

    let bytes = value.to_bits().to_le_bytes();
    buf[i..i + 8].copy_from_slice(&bytes);
    i += 8;

    i
}

fn encode_varint(mut value: u64, buf: &mut [u8]) -> usize {
    let mut i = 0;

    loop {
        if value < 0x80 {
            buf[i] = value as u8;
            return i + 1;
        }

        buf[i] = (value as u8 & 0x7f) | 0x80;
        value >>= 7;
        i += 1;
    }
}
