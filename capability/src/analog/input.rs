pub trait AnalogChannel {}

pub trait AnalogReader<C>
where
    C: AnalogChannel,
{
    type Error;

    async fn read(&mut self, channel: &mut C) -> Result<u16, Self::Error>;
}
