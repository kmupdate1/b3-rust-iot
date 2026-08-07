use cyw43::{aligned_bytes, JoinOptions};
use cyw43_pio::{PioSpi, DEFAULT_CLOCK_DIVIDER};
use embassy_executor::Spawner;
use embassy_net::StackResources;
use embassy_rp::{bind_interrupts, dma};
use embassy_rp::clocks::RoscRng;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::peripherals::{DMA_CH0, DMA_CH1, PIO0};
use embassy_rp::pio::{InterruptHandler, Pio};
use static_cell::StaticCell;
use function::{NetworkDevice, Wifi};
use crate::Pico2wWifiResources;

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

pub struct Rp235xWifi {
    control: cyw43::Control<'static>,
    stack: embassy_net::Stack<'static>,
}

impl Rp235xWifi {
    pub async fn new(
        resources: Pico2wWifiResources<'static>,
        spawner: Spawner,
    ) -> Self {
        let mut rng = RoscRng;
        let seed = rng.next_u64();

        let fw = aligned_bytes!("../../firmware/43439A0.bin");
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

        let (net_device, mut control, runner) =
            cyw43::new(state, pwr, spi, fw, nvram).await;

        spawner.spawn(cyw43_task(runner).unwrap());
        control.init(clm).await;

        control
            .set_power_management(cyw43::PowerManagementMode::PowerSave)
            .await;

        let mut dhcp_config = embassy_net::DhcpConfig::default();
        dhcp_config.hostname = Some(
            heapless::String::try_from("b3c-field-edge-0001").unwrap()
        );

        let config = embassy_net::Config::dhcpv4(dhcp_config);

        let (stack, net_runner) = embassy_net::new(
            net_device,
            config,
            RESOURCES.init(StackResources::new()),
            seed,
        );

        spawner.spawn(net_task(net_runner).unwrap());

        Self { control, stack }
    }
}

impl NetworkDevice for Rp235xWifi {
    type Error = cyw43::JoinError;
}

impl Wifi for Rp235xWifi {
    async fn connect(&mut self, ssid: &str, password: &str) -> Result<(), Self::Error> {
        self.control
            .join(
                ssid,
                JoinOptions::new(password.as_bytes()),
            )
            .await?;

        self.stack.wait_link_up().await;
        self.stack.wait_config_up().await;

        Ok(())
    }
}
