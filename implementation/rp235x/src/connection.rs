use embassy_rp::adc::Channel;

pub struct Rp235xCon {
    channel: Channel<'static>,
}

impl Rp235xCon {
    pub(crate) fn new(channel: Channel<'static>) -> Self {
        Self { channel }
    }

    pub(crate) fn channel_mut(&mut self) -> &mut Channel<'static> { &mut self.channel }
}
