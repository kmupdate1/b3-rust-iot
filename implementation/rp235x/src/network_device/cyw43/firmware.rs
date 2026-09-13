use cyw43::bluetooth::BtDriver;
use cyw43::{aligned_bytes, Control, Cyw43439, NetDriver, Runner, SpiBus};
use cyw43_pio::PioSpi;
use embassy_rp::gpio::Output;
use embassy_rp::peripherals::PIO0;
use static_cell::StaticCell;
use crate::cyw43::bus::Bus;

static STATE: StaticCell<cyw43::State> = StaticCell::new();

pub(super) struct Firmware {
    pub net_device: NetDriver<'static>,
    pub bluetooth: BtDriver<'static>,
    pub control: Control<'static>,
    pub runner: Runner<
        'static,
        SpiBus<Output<'static>, PioSpi<'static, PIO0, 0>>,
        Cyw43439,
    >,
}

pub(super) async fn init(bus: Bus) -> Firmware {
    let fw = aligned_bytes!("../../../firmware/43439A0.bin");
    let btfw = aligned_bytes!("../../../firmware/43439A0_btfw.bin");
    let clm = aligned_bytes!("../../../firmware/43439A0_clm.bin");
    let nvram = aligned_bytes!("../../../firmware/nvram_rp2040.bin");

    let state = STATE.init(cyw43::State::new());

    let (net_device, bluetooth, mut control, runner) =
        cyw43::new_with_bluetooth(
            state,
            bus.pwr,
            bus.spi,
            fw,
            btfw,
            nvram,
        ).await;

    control
        .init(clm)
        .await;

    control
        .set_power_management(cyw43::PowerManagementMode::Performance)
        .await;

    Firmware {
        net_device,
        bluetooth,
        control,
        runner,
    }
}
