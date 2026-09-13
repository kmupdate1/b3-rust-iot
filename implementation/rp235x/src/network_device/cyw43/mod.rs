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
use crate::Pico2wCyw43Resources;

pub struct Rp235xCyw43 {
    pub(crate) control: Mutex<ThreadModeRawMutex, Control<'static>>,
    pub(crate) stack: Stack<'static>,
    pub(crate) bluetooth: Mutex<ThreadModeRawMutex, BtDriver<'static>>,
}

impl Rp235xCyw43 {
    pub async fn new(
        resources: Pico2wCyw43Resources<'static>,
        spawner: Spawner,
    ) -> Self {
        let mut rng = RoscRng;
        let seed = rng.next_u64();

        let bus = bus::init(resources);
        let fw = firmware::init(bus).await;

        spawner
            .spawn(runner::cyw43_task(fw.runner).unwrap());

        let (stack, net_runner) =
        network::init(fw.net_device, seed);

        spawner
            .spawn(network::net_task(net_runner).unwrap());

        Self {
            control: Mutex::new(fw.control),
            stack,
            bluetooth: Mutex::new(fw.bluetooth),
        }
    }
}
