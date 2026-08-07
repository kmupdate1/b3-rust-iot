use embassy_rp::Peri;
use embassy_rp::peripherals::*;

macro_rules! define_pins {
    ($($pin:ident => $peri:ident),* $(,)?) => {$(
        pub struct $pin<'d> { inner: Peri<'d, $peri> }
    
        impl<'d> $pin<'d> {
            pub fn new(pin: Peri<'d, $peri>) -> Self {
                Self { inner: pin }
            }

            pub(crate) fn into_inner(self) -> Peri<'d, $peri> { self.inner }
        }
    )*};
}

define_pins!(
    Pin0 => PIN_0,
    Pin1 => PIN_1,
    Pin2 => PIN_2,
    Pin3 => PIN_3,
    Pin4 => PIN_4,
    Pin5 => PIN_5,
    Pin6 => PIN_6,
    Pin7 => PIN_7,
    Pin8 => PIN_8,
    Pin9 => PIN_9,
    Pin10 => PIN_10,
    Pin11 => PIN_11,
    Pin12 => PIN_12,
    Pin13 => PIN_13,
    Pin14 => PIN_14,
    Pin15 => PIN_15,
    Pin16 => PIN_16,
    Pin17 => PIN_17,
    Pin18 => PIN_18,
    Pin19 => PIN_19,
    Pin20 => PIN_20,
    Pin21 => PIN_21,
    Pin22 => PIN_22,
    Pin23 => PIN_23,
    Pin24 => PIN_24,
    Pin25 => PIN_25,
    Pin26 => PIN_26,
    Pin27 => PIN_27,
    Pin28 => PIN_28,
    Pin29 => PIN_29,
);
