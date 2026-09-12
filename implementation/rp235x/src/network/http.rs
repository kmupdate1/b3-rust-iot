use embassy_net::dns::DnsSocket;
use embassy_net::Stack;
use embassy_net::tcp::client::{TcpClient, TcpClientState};
use reqwless::client::HttpClient;
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
        let tcp_state = TcpClientState::<1, 4069, 4096>::new();
        let tcp = TcpClient::new(
            self.stack,
            &tcp_state,
        );

        let dns = DnsSocket::new(self.stack);

        let mut client = HttpClient::new(&tcp, &dns);

        todo!()
    }
}
