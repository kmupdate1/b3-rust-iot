#![no_std]
#![no_main]

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_time::Timer;
use embedded_alloc::LlffHeap;
use panic_probe as _;
use rp235x::{Pin10, Pin11, Pin12, Pin13, Pin14, Pin15};
use {defmt_rtt as _, panic_probe as _};

#[global_allocator]
static HEAP: LlffHeap = LlffHeap::empty();

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());

    let rs = Pin10::new(peripherals.PIN_10);
    let e  = Pin11::new(peripherals.PIN_11);
    let d4 = Pin12::new(peripherals.PIN_12);
    let d5 = Pin13::new(peripherals.PIN_13);
    let d6 = Pin14::new(peripherals.PIN_14);
    let d7 = Pin15::new(peripherals.PIN_15);

    let mut lcd = Lcd::new(
        rs.into_digital_output(),
        e.into_digital_output(),
        d4.into_digital_output(),
        d5.into_digital_output(),
        d6.into_digital_output(),
        d7.into_digital_output(),
    );

    lcd.init().await;

    loop {
        // Hello World
        // ON
        lcd.write_char(b'A').await;

        Timer::after_millis(3_000).await;

        // OFF
        lcd.write_char(b'B').await;

        Timer::after_millis(1_500).await;
    }
}

use capability::{DigitalOutput, IntoDigitalOutput};

struct Lcd<RS, E, D4, D5, D6, D7> {
    rs: RS,
    e: E,
    d4: D4,
    d5: D5,
    d6: D6,
    d7: D7,
}

impl<RS, E, D4, D5, D6, D7> Lcd<RS, E, D4, D5, D6, D7>
where
    RS: DigitalOutput,
    E: DigitalOutput,
    D4: DigitalOutput,
    D5: DigitalOutput,
    D6: DigitalOutput,
    D7: DigitalOutput,
{
    fn new(
        rs: RS,
        e: E,
        d4: D4,
        d5: D5,
        d6: D6,
        d7: D7,
    ) -> Self {
        Self { rs, e, d4, d5, d6, d7, }
    }

    async fn write_nibble(&mut self, value: u8) {
        if value & 0x01 != 0 {
            self.d4.set_high();
        } else {
            self.d4.set_low();
        }

        if value & 0x02 != 0 {
            self.d5.set_high();
        } else {
            self.d5.set_low();
        }

        if value & 0x04 != 0 {
            self.d6.set_high();
        } else {
            self.d6.set_low();
        }

        if value & 0x08 != 0 {
            self.d7.set_high();
        } else {
            self.d7.set_low();
        }

        self.e.set_high();
        Timer::after_micros(1).await;
        self.e.set_low();

        Timer::after_micros(50).await;
    }

    async fn write_byte(&mut self, value: u8, data: bool) {
        if data {
            self.rs.set_high();
        } else {
            self.rs.set_low();
        }

        self.write_nibble(value >> 4).await;
        self.write_nibble(value & 0x0f).await;
    }

    async fn init(&mut self) {
        self.rs.set_low();

        Timer::after_millis(50).await;

        self.write_nibble(0x03).await;
        Timer::after_millis(5).await;

        self.write_nibble(0x03).await;
        Timer::after_micros(150).await;

        self.write_nibble(0x03).await;
        Timer::after_micros(150).await;

        self.write_nibble(0x02).await;
    }

    async fn write_char(&mut self, c: u8) {
        self.write_byte(c, true).await;
    }
}
 