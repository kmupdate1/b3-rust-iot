use cyw43::NetDriver;
use embassy_net::{Config, Runner, Stack, StackResources};
use static_cell::StaticCell;

static RESOURCES: StaticCell<StackResources<3>> = StaticCell::new();

#[embassy_executor::task]
pub(crate) async fn net_task(
    mut runner: Runner<
        'static,
        NetDriver<'static>,
    >,
) -> ! { runner.run().await; }

pub(crate) fn init(
    net_device: NetDriver<'static>,
    seed: u64,
) -> (
    Stack<'static>,
    Runner<'static, NetDriver<'static>>,
) {
    let config = Config::dhcpv4(Default::default());

    embassy_net::new(
        net_device,
        config,
        RESOURCES.init(StackResources::new()),
        seed,
    )
}
