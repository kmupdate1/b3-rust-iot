use embassy_net::Stack;
use capability::Http;

pub struct Rp235xHttp {
    stack: Stack<'static>,
}

impl Rp235xHttp {
    pub(crate) fn new(
        stack: Stack<'static>,
    ) -> Self {
        Self { stack }
    }
}

impl Http for Rp235xHttp {
    type HttpError = ();

    async fn get(&mut self, url: &str, buffer: &mut [u8]) -> Result<usize, Self::HttpError> {
        todo!("not yet implemented")
    }
}
