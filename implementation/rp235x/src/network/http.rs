use embassy_net::dns::DnsSocket;
use embassy_net::Stack;
use embassy_net::tcp::client::{TcpClient, TcpClientState};
use embassy_rp::clocks::RoscRng;
use reqwless::client::{HttpClient, TlsConfig, TlsVerify};
use reqwless::request::Method;
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
        let tcp_state = TcpClientState::<1, 4096, 4096>::new();
        let tcp = TcpClient::new(self.stack, &tcp_state);

        let dns = DnsSocket::new(self.stack);

        let mut tls_read_buf = [0u8; 16 * 1024];
        let mut tls_write_buf = [0u8; 16 * 1024];

        let mut rng = RoscRng;

        let tls = TlsConfig::new(
            rng.next_u64(),
            &mut tls_read_buf,
            &mut tls_write_buf,
            TlsVerify::None,
        );

        let mut client =
            HttpClient::new_with_tls(&tcp, &dns, tls);

        let buf_start = buffer.as_ptr() as usize;

        let mut request = client
            .request(Method::GET, url)
            .await
            .map_err(|_| {
                log::error!("http: request creation failed");
                ()
            })?;

        let response = request
            .send(buffer)
            .await
            .map_err(|_| {
                log::error!("http: request send or response header read failed");
                ()
            })?;

        log::info!("http: response status {}", response.status.0);

        if response.status.is_redirection() {
            for (name, value) in response.headers() {
                if name.eq_ignore_ascii_case("location") {
                    match core::str::from_utf8(value) {
                        Ok(location) => log::info!("http: redirect location {}", location),
                        Err(_) => log::error!("http: redirect location is not UTF-8"),
                    }
                }
            }
        }

        if !response.status.is_successful() {
            log::error!("http: non-success response status {}", response.status.0);
            return Err(());
        }

        let body = response
            .body()
            .read_to_end()
            .await
            .map_err(|_| {
                log::error!("http: response body read failed");
                ()
            })?;

        let body_start = body.as_ptr() as usize;
        let body_len = body.len();

        let offset = body_start
            .checked_sub(buf_start)
            .ok_or_else(|| {
                log::error!("http: response body buffer is outside the supplied buffer");
                ()
            })?;

        if offset != 0 {
            buffer.copy_within(
                offset..offset + body_len,
                0,
            );
        }

        Ok(body_len)
    }
}
