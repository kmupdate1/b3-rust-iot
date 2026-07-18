use embassy_rp::config::Config;
use rp235x::adc::AdcDriver;

fn main() {
    let gpio26 = AdcDriver::init(embassy_rp::init(Config::config()));
}
