use crate::Pico2wCyw43Resources;
use cyw43::bluetooth::BtDriver;
use cyw43::{aligned_bytes, Control};
use cyw43_pio::{PioSpi, DEFAULT_CLOCK_DIVIDER};
use embassy_executor::Spawner;
use embassy_net::{DhcpConfig, Stack, StackResources};
use embassy_rp::clocks::RoscRng;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::peripherals::{DMA_CH0, DMA_CH1, PIO0};
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_rp::{bind_interrupts, dma};
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::mutex::Mutex;
use static_cell::StaticCell;

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
    DMA_IRQ_0 => dma::InterruptHandler<DMA_CH0>, dma::InterruptHandler<DMA_CH1>;
});

static STATE: StaticCell<cyw43::State> = StaticCell::new();
static RESOURCES: StaticCell<StackResources<3>> = StaticCell::new();

#[embassy_executor::task]
async fn cyw43_task(
    runner: cyw43::Runner<
        'static,
        cyw43::SpiBus<Output<'static>, PioSpi<'static, PIO0, 0>>,
        cyw43::Cyw43439,
    >,
) -> ! {
    runner.run().await;
}

#[embassy_executor::task]
async fn net_task(
    mut runner: embassy_net::Runner<
        'static,
        cyw43::NetDriver<'static>,
    >,
) -> ! {
    runner.run().await;
}

pub struct Rp235xCyw43 {
    pub(crate) control: Mutex<ThreadModeRawMutex, Control<'static>>,
    pub(crate) stack: Stack<'static>,
    pub(crate) bluetooth: Mutex<ThreadModeRawMutex, BtDriver<'static>>,
}

impl Rp235xCyw43 {
    pub async fn builder(
        resources: Pico2wCyw43Resources<'static>,
        spawner: Spawner,
    ) -> Self {
        let mut rng = RoscRng;
        let seed = rng.next_u64();

        let fw = aligned_bytes!("../../firmware/43439A0.bin");
        let btfw = aligned_bytes!("../../firmware/43439A0_btfw.bin");
        let clm = aligned_bytes!("../../firmware/43439A0_clm.bin");
        let nvram = aligned_bytes!("../../firmware/nvram_rp2040.bin");

        let pwr = Output::new(resources.pwr, Level::Low);
        let cs = Output::new(resources.cs, Level::High);

        let mut pio = Pio::new(resources.pio, Irqs);

        let spi = PioSpi::new(
            &mut pio.common,
            pio.sm0,
            DEFAULT_CLOCK_DIVIDER,
            pio.irq0,
            cs,
            resources.dio,
            resources.clk,
            dma::Channel::new(resources.dma0, Irqs),
            dma::Channel::new(resources.dma1, Irqs),
        );

        let state = STATE.init(cyw43::State::new());

        let (net_device, bluetooth, mut control, runner) =
            cyw43::new_with_bluetooth(state, pwr, spi, fw, btfw, nvram).await;

        spawner.spawn(cyw43_task(runner).unwrap());

        control.init(clm).await;

        control
            .set_power_management(cyw43::PowerManagementMode::PowerSave)
            .await;

        let config = embassy_net::Config::dhcpv4(
            DhcpConfig::default()
        );

        let (stack, net_runner) = embassy_net::new(
            net_device,
            config,
            RESOURCES.init(StackResources::new()),
            seed,
        );

        spawner.spawn(net_task(net_runner).unwrap());

        Self {
            control: Mutex::new(control),
            stack,
            bluetooth: Mutex::new(bluetooth),
        }
    }
}
