use embassy_rp::config::Config;
use hal_rp235x::adc::AdcDriver;
use hal_rp235x::pins::Gpio26;

fn main() {
    let gpio26 = AdcDriver::init(embassy_rp::init(Config::config()));
}
