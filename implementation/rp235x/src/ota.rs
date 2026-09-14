use core::cell::RefCell;

use capability::Http;
use embassy_boot_rp::{AlignedBuffer, BlockingFirmwareUpdater, FirmwareUpdaterConfig};
use embassy_net::Stack;
use embassy_rp::flash::{Blocking, Flash};
use embassy_rp::peripherals::FLASH;
use embassy_rp::Peri;
use embassy_sync::blocking_mutex::Mutex;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embedded_storage::nor_flash::ReadNorFlash;
use runtime_core::Ota;
use sha2::Sha256;
use ota::UpdateManifest;
use crate::Rp235xHttp;

const FLASH_SIZE: usize = 4 * 1024 * 1024;

#[derive(Debug)]
pub enum Rp235xOtaError {
    Http,
    Manifest,
    ManifestFieldTooLong,
    NoUpdateAvailable,
    FirmwareTooLarge,
    FirmwareSizeMismatch,
    FirmwareHashMismatch,
    InvalidVersion,
    Flash,
}

pub struct Rp235xOta<'d> {
    stack: Stack<'static>,
    flash: Mutex<NoopRawMutex, RefCell<Flash<'d, FLASH, Blocking, FLASH_SIZE>>>,
    manifest_url: &'static str,
    pending: Option<UpdateManifest>,
}

impl<'d> Rp235xOta<'d> {
    pub(crate) fn new(
        stack: Stack<'static>,
        flash: Peri<'d, FLASH>,
        manifest_url: &'static str,
    ) -> Self {
        Self {
            stack,
            flash: Mutex::new(RefCell::new(Flash::new_blocking(flash))),
            manifest_url,
            pending: None,
        }
    }

    pub fn confirm_boot(&mut self) -> Result<(), Rp235xOtaError> {
        let config = FirmwareUpdaterConfig::from_linkerfile_blocking(&self.flash, &self.flash);
        let mut aligned = AlignedBuffer([0; 1]);
        BlockingFirmwareUpdater::new(config, &mut aligned.0)
            .mark_booted()
            .map_err(|_| Rp235xOtaError::Flash)
    }

    pub fn pending_manifest(&self) -> Option<&UpdateManifest> {
        self.pending.as_ref()
    }

    fn parse_manifest(data: &[u8]) -> Result<UpdateManifest, Rp235xOtaError> {
        ota::parser::json(data).map_err(|_| Rp235xOtaError::Manifest)
    }

    fn decode_sha256(value: &str) -> Result<[u8; 32], Rp235xOtaError> {
        fn nibble(value: u8) -> Option<u8> {
            match value {
                b'0'..=b'9' => Some(value - b'0'),
                b'a'..=b'f' => Some(value - b'a' + 10),
                b'A'..=b'F' => Some(value - b'A' + 10),
                _ => None,
            }
        }

        let bytes = value.as_bytes();
        if bytes.len() != 64 {
            return Err(Rp235xOtaError::Manifest);
        }
        let mut decoded = [0u8; 32];
        for (index, output) in decoded.iter_mut().enumerate() {
            let high = nibble(bytes[index * 2]).ok_or(Rp235xOtaError::Manifest)?;
            let low = nibble(bytes[index * 2 + 1]).ok_or(Rp235xOtaError::Manifest)?;
            *output = high << 4 | low;
        }
        Ok(decoded)
    }

    fn version(value: &str) -> Result<(u32, u32, u32), Rp235xOtaError> {
        let value = value.strip_prefix('v').unwrap_or(value);
        let mut parts = value.split('.');
        let major = parts.next().and_then(|part| part.parse().ok()).ok_or(Rp235xOtaError::InvalidVersion)?;
        let minor = parts.next().and_then(|part| part.parse().ok()).ok_or(Rp235xOtaError::InvalidVersion)?;
        let patch = parts.next().and_then(|part| part.parse().ok()).ok_or(Rp235xOtaError::InvalidVersion)?;
        if parts.next().is_some() {
            return Err(Rp235xOtaError::InvalidVersion);
        }
        Ok((major, minor, patch))
    }
}

impl Ota for Rp235xOta<'_> {
    type Error = Rp235xOtaError;

    async fn available(&mut self, current_v: &str) -> Result<bool, Self::Error> {
        let mut buffer = [0u8; 16 * 1024];
        let mut http = Rp235xHttp::new(self.stack);
        let len = http.get(self.manifest_url, &mut buffer).await.map_err(|_| Rp235xOtaError::Http)?;
        let manifest = Self::parse_manifest(&buffer[..len])?;
        let available = (
            manifest.version.major,
            manifest.version.minor,
            manifest.version.patch,
        ) > Self::version(current_v)?;
        self.pending = available.then_some(manifest);
        Ok(available)
    }

    async fn update(&mut self) -> Result<(), Self::Error> {
        let manifest = self.pending.clone().ok_or(Rp235xOtaError::NoUpdateAvailable)?;
        if manifest.size == 0
            || manifest.sha256.len() != 64
            || !manifest.firmware_url.starts_with("https://")
        {
            return Err(Rp235xOtaError::Manifest);
        }
        let expected_hash = Self::decode_sha256(manifest.sha256.as_str())?;
        let config = FirmwareUpdaterConfig::from_linkerfile_blocking(&self.flash, &self.flash);
        if manifest.size as usize > config.dfu.capacity() {
            return Err(Rp235xOtaError::FirmwareTooLarge);
        }

        let mut aligned = AlignedBuffer([0; 1]);
        let mut updater = BlockingFirmwareUpdater::new(config, &mut aligned.0);
        ota_download::download_to_updater(
            self.stack,
            manifest.firmware_url.as_str(),
            manifest.size,
            &mut updater,
        ).await?;

        let mut actual_hash = [0u8; 32];
        let mut hash_buffer = [0u8; 4096];
        updater.hash::<Sha256>(manifest.size, &mut hash_buffer, &mut actual_hash)
            .map_err(|_| Rp235xOtaError::Flash)?;
        if actual_hash != expected_hash {
            return Err(Rp235xOtaError::FirmwareHashMismatch);
        }
        updater.mark_updated().map_err(|_| Rp235xOtaError::Flash)
    }
}

mod ota_download {
    use core::fmt::Write as _;

    use embassy_boot_rp::{AlignedBuffer, BlockingFirmwareUpdater};
    use embassy_net::dns::DnsSocket;
    use embassy_net::tcp::client::{TcpClient, TcpClientState};
    use embassy_net::Stack;
    use embassy_rp::clocks::RoscRng;
    use embassy_time::Timer;
    use embedded_io_async::Read;
    use embedded_storage::nor_flash::NorFlash;
    use heapless::String;
    use reqwless::client::{HttpClient, TlsConfig, TlsVerify};
    use reqwless::request::{Method, RequestBuilder};

    use super::Rp235xOtaError;

    const SEGMENT_SIZE: usize = 32 * 1024;
    const MAX_ATTEMPTS: usize = 5;
    const MAX_REDIRECTS: usize = 3;

    pub async fn download_to_updater<DFU, STATE>(
        stack: Stack<'static>,
        url: &str,
        expected_size: u32,
        updater: &mut BlockingFirmwareUpdater<'_, DFU, STATE>,
    ) -> Result<(), Rp235xOtaError>
    where
        DFU: NorFlash,
        STATE: NorFlash,
    {
        let tcp_state = TcpClientState::<1, 4096, 4096>::new();
        let tcp = TcpClient::new(stack, &tcp_state);
        let dns = DnsSocket::new(stack);
        let mut tls_read_buffer = [0u8; 16 * 1024];
        let mut tls_write_buffer = [0u8; 16 * 1024];
        let mut rng = RoscRng;
        let tls = TlsConfig::new(
            rng.next_u64(),
            &mut tls_read_buffer,
            &mut tls_write_buffer,
            TlsVerify::None,
        );
        let mut client = HttpClient::new_with_tls(&tcp, &dns, tls);

        let expected_size = expected_size as usize;
        let mut offset = 0usize;

        while offset < expected_size {
            let segment_start = offset;
            let segment_end = core::cmp::min(segment_start + SEGMENT_SIZE, expected_size) - 1;
            let segment_len = segment_end - segment_start + 1;
            let mut attempt = 1usize;

            loop {
                let result: Result<usize, Rp235xOtaError> = async {
                    let mut current_url = String::<2048>::new();
                    current_url
                        .push_str(url)
                        .map_err(|_| Rp235xOtaError::ManifestFieldTooLong)?;

                    let mut range = String::<64>::new();
                    write!(&mut range, "bytes={}-{}", segment_start, segment_end)
                        .map_err(|_| Rp235xOtaError::Http)?;

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
                                        "updater: firmware request failed at {} (attempt {}): {:?}",
                                        segment_start,
                                        attempt,
                                        error,
                                    );
                                    Rp235xOtaError::Http
                                })?;
                            let mut request = request.headers(&headers);
                            let response = request
                                .send(&mut header_buffer)
                                .await
                                .map_err(|error| {
                                    log::error!(
                                        "updater: firmware response failed at {} (attempt {}): {:?}",
                                        segment_start,
                                        attempt,
                                        error,
                                    );
                                    Rp235xOtaError::Http
                                })?;

                            if response.status.0 == 206 {
                                if response.content_length != Some(segment_len) {
                                    return Err(Rp235xOtaError::FirmwareSizeMismatch);
                                }

                                let mut reader = response.body().reader();
                                let mut chunk = AlignedBuffer([0u8; 4096]);
                                let mut received = 0usize;

                                loop {
                                    let read = reader
                                        .read(&mut chunk.0)
                                        .await
                                        .map_err(|error| {
                                            log::error!(
                                                "updater: firmware body failed at {} (attempt {}): {:?}",
                                                segment_start + received,
                                                attempt,
                                                error,
                                            );
                                            Rp235xOtaError::Http
                                        })?;

                                    if read == 0 {
                                        break;
                                    }
                                    if received + read > segment_len {
                                        return Err(Rp235xOtaError::FirmwareSizeMismatch);
                                    }

                                    updater
                                        .write_firmware(
                                            segment_start + received,
                                            &chunk.0[..read],
                                        )
                                        .map_err(|_| Rp235xOtaError::Flash)?;
                                    received += read;
                                }

                                return if received == segment_len {
                                    Ok(received)
                                } else {
                                    Err(Rp235xOtaError::FirmwareSizeMismatch)
                                };
                            }

                            if !response.status.is_redirection()
                                || redirect_count == MAX_REDIRECTS
                            {
                                log::error!(
                                    "updater: unexpected firmware response status {}",
                                    response.status.0,
                                );
                                return Err(Rp235xOtaError::Http);
                            }

                            let mut redirect = String::<2048>::new();
                            for (name, value) in response.headers() {
                                if name.eq_ignore_ascii_case("location") {
                                    let value = core::str::from_utf8(value)
                                        .map_err(|_| Rp235xOtaError::Http)?;
                                    if !value.starts_with("https://") {
                                        return Err(Rp235xOtaError::Http);
                                    }
                                    redirect
                                        .push_str(value)
                                        .map_err(|_| {
                                            Rp235xOtaError::ManifestFieldTooLong
                                        })?;
                                    break;
                                }
                            }

                            if redirect.is_empty() {
                                return Err(Rp235xOtaError::Http);
                            }
                            redirect
                        };

                        current_url = next_url;
                    }

                    Err(Rp235xOtaError::Http)
                }
                .await;

                match result {
                    Ok(received) => {
                        offset += received;
                        log::info!(
                            "updater: downloaded {} / {} bytes",
                            offset,
                            expected_size,
                        );
                        break;
                    }
                    Err(Rp235xOtaError::Http) if attempt < MAX_ATTEMPTS => {
                        log::warn!(
                            "updater: retrying range {}-{} ({}/{})",
                            segment_start,
                            segment_end,
                            attempt + 1,
                            MAX_ATTEMPTS,
                        );
                        attempt += 1;
                        Timer::after_secs(2).await;
                    }
                    Err(error) => return Err(error),
                }
            }
        }

        Ok(())
    }
}
