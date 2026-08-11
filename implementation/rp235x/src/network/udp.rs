use embassy_net::Stack;

pub struct Rp235xUdp {
    stack: Stack<'static>
}

impl Rp235xUdp {
    pub(crate) fn new(stack: Stack<'static>) -> Self {
        Self { stack }
    }
}
