mod bus;
mod firmware;
mod network;
mod runner;

use cyw43::bluetooth::BtDriver;
use cyw43::Control;
use embassy_executor::Spawner;
use embassy_net::Stack;
use embassy_rp::clocks::RoscRng;
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::mutex::Mutex;
use crate::{Pico2wCyw43Resources, Rp235xBluetooth};
use crate::wifi::Rp235xWifi;

pub struct Rp235xCyw43 {
    control: Mutex<ThreadModeRawMutex, Control<'static>>,
    stack: Stack<'static>,
    bluetooth: Mutex<ThreadModeRawMutex, BtDriver<'static>>,
}

impl Rp235xCyw43 {
    pub async fn new(
        resources: Pico2wCyw43Resources<'static>,
        spawner: Spawner,
    ) -> Self {
        let mut rng = RoscRng;
        let seed = rng.next_u64();

        log::info!("Cyw43: firmware initializing, seed is {}", seed);
        let bus = bus::init(resources);
        let mut fw = firmware::init(bus).await;

        log::info!("Cyw43: spawning runner");
        spawner
            .spawn(runner::cyw43_task(fw.runner).unwrap());

        log::info!("Cyw43: configuring control");
        firmware::control(&mut fw.control).await;

        log::info!("Cyw43: network initializing");
        let (stack, net_runner) =
            network::init(fw.net_device, seed);

        log::info!("Cyw43: spawning network runner");
        spawner
            .spawn(network::net_task(net_runner).unwrap());

        log::info!("Cyw43: ready");
        Self {
            control: Mutex::new(fw.control),
            stack,
            bluetooth: Mutex::new(fw.bluetooth),
        }
    }

    pub fn wifi(&self) -> Rp235xWifi<'_> {
        Rp235xWifi::new(
            &self.control,
            self.stack,
        )
    }

    pub fn bluetooth(&self) -> Rp235xBluetooth<'_> {
        Rp235xBluetooth::new(
            &self.control,
            &self.bluetooth,
        )
    }
}
