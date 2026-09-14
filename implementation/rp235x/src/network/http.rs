use core::fmt::Write as _;
use embassy_net::dns::DnsSocket;
use embassy_net::Stack;
use embassy_net::tcp::client::{TcpClient, TcpClientState};
use embassy_rp::clocks::RoscRng;
use embedded_io_async::Read;
use heapless::String;
use reqwless::client::{HttpClient, TlsConfig, TlsVerify};
use reqwless::request::{Method, RequestBuilder};
use capability::Http;

pub struct Rp235xHttp {
    stack: Stack<'static>,
}

impl Rp235xHttp {
    pub(crate) fn new(stack: Stack<'static>) -> Self {
        Self { stack }
    }

    pub async fn get_range(
        &mut self,
        url: &str,
        start: usize,
        end: usize,
        buffer: &mut [u8],
    ) -> Result<usize, ()> {
        const MAX_REDIRECTS: usize = 3;
        const MAX_ATTEMPTS: usize = 5;

        if end < start || end - start + 1 > buffer.len() {
            log::error!("http: invalid range {}-{}", start, end);
            return Err(());
        }

        let expected_len = end - start + 1;
        let mut range = String::<64>::new();
        write!(&mut range, "bytes={}-{}", start, end).map_err(|_| ())?;

        for attempt in 1..=MAX_ATTEMPTS {
            let result = async {
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
                let mut client = HttpClient::new_with_tls(&tcp, &dns, tls);
                let mut current_url = String::<2048>::new();
                current_url.push_str(url).map_err(|_| ())?;

                for redirect_count in 0..=MAX_REDIRECTS {
                    let next_url = {
                        let headers = [
                            ("Range", range.as_str()),
                            ("Connection", "close"),
                        ];
                        let mut header_buffer = [0u8; 16 * 1024];
                        let request = client
                            .request(Method::GET, current_url.as_str())
                            .await
                            .map_err(|error| {
                                log::error!(
                                    "http: range request failed at {} (attempt {}): {:?}",
                                    start,
                                    attempt,
                                    error,
                                );
                            })?;
                        let mut request = request.headers(&headers);
                        let response = request.send(&mut header_buffer).await.map_err(|error| {
                            log::error!(
                                "http: range response failed at {} (attempt {}): {:?}",
                                start,
                                attempt,
                                error,
                            );
                        })?;

                        if response.status.0 == 206 {
                            if response.content_length != Some(expected_len) {
                                log::error!(
                                    "http: range size mismatch at {}: {:?} != {}",
                                    start,
                                    response.content_length,
                                    expected_len,
                                );
                                return Err(());
                            }

                            let mut reader = response.body().reader();
                            let mut received = 0usize;
                            while received < expected_len {
                                let read = reader.read(&mut buffer[received..expected_len]).await
                                    .map_err(|error| {
                                        log::error!(
                                            "http: range body failed at {} (attempt {}): {:?}",
                                            start + received,
                                            attempt,
                                            error,
                                        );
                                    })?;
                                if read == 0 {
                                    break;
                                }
                                received += read;
                            }

                            return if received == expected_len {
                                Ok(received)
                            } else {
                                log::error!(
                                    "http: incomplete range at {}: {} != {}",
                                    start,
                                    received,
                                    expected_len,
                                );
                                Err(())
                            };
                        }

                        if !response.status.is_redirection()
                            || redirect_count == MAX_REDIRECTS
                        {
                            log::error!(
                                "http: unexpected range response status {}",
                                response.status.0,
                            );
                            return Err(());
                        }

                        let mut redirect = String::<2048>::new();
                        for (name, value) in response.headers() {
                            if name.eq_ignore_ascii_case("location") {
                                let location = core::str::from_utf8(value).map_err(|_| ())?;
                                if !location.starts_with("https://") {
                                    log::error!("http: refusing non-HTTPS redirect");
                                    return Err(());
                                }
                                redirect.push_str(location).map_err(|_| ())?;
                                break;
                            }
                        }
                        if redirect.is_empty() {
                            log::error!("http: redirect response has no Location header");
                            return Err(());
                        }
                        redirect
                    };

                    current_url = next_url;
                }

                Err(())
            }
            .await;

            match result {
                Ok(len) => return Ok(len),
                Err(()) if attempt < MAX_ATTEMPTS => {
                    log::warn!(
                        "http: retrying range {}-{} ({}/{})",
                        start,
                        end,
                        attempt + 1,
                        MAX_ATTEMPTS,
                    );
                    embassy_time::Timer::after_secs(2).await;
                }
                Err(()) => return Err(()),
            }
        }

        Err(())
    }
}

impl Http for Rp235xHttp {
    type HttpError = ();

    async fn get(&mut self, url: &str, buffer: &mut [u8]) -> Result<usize, Self::HttpError> {
        const MAX_REDIRECTS: usize = 3;

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
        let mut client = HttpClient::new_with_tls(&tcp, &dns, tls);
        let mut current_url = String::<2048>::new();
        current_url.push_str(url).map_err(|_| {
            log::error!("http: URL is too long");
        })?;

        for redirect_count in 0..=MAX_REDIRECTS {
            let redirect_url = {
                let buf_start = buffer.as_ptr() as usize;
                let request = client
                    .request(Method::GET, current_url.as_str())
                    .await
                    .map_err(|error| {
                        log::error!("http: request creation failed: {:?}", error);
                    })?;
                let response = request.send(buffer).await.map_err(|error| {
                    log::error!(
                        "http: request send or response header read failed: {:?}",
                        error,
                    );
                })?;

                log::info!("http: response status {}", response.status.0);

                if response.status.is_successful() {
                    let body = response.body().read_to_end().await.map_err(|error| {
                        log::error!("http: response body read failed: {:?}", error);
                    })?;
                    let body_start = body.as_ptr() as usize;
                    let body_len = body.len();
                    let offset = body_start.checked_sub(buf_start).ok_or_else(|| {
                        log::error!("http: response body buffer is outside the supplied buffer");
                    })?;

                    if offset != 0 {
                        buffer.copy_within(offset..offset + body_len, 0);
                    }
                    return Ok(body_len);
                }

                if !response.status.is_redirection() {
                    log::error!("http: non-success response status {}", response.status.0);
                    return Err(());
                }
                if redirect_count == MAX_REDIRECTS {
                    log::error!("http: redirect limit exceeded");
                    return Err(());
                }

                let mut next_url = String::<2048>::new();
                for (name, value) in response.headers() {
                    if name.eq_ignore_ascii_case("location") {
                        let location = core::str::from_utf8(value).map_err(|_| {
                            log::error!("http: redirect location is not UTF-8");
                        })?;
                        if !location.starts_with("https://") {
                            log::error!("http: refusing non-HTTPS redirect");
                            return Err(());
                        }
                        next_url.push_str(location).map_err(|_| {
                            log::error!("http: redirect URL is too long");
                        })?;
                        log::info!("http: following redirect {}", redirect_count + 1);
                        break;
                    }
                }
                if next_url.is_empty() {
                    log::error!("http: redirect response has no Location header");
                    return Err(());
                }
                next_url
            };

            current_url = redirect_url;
        }

        Err(())
    }
}
