use core::cell::RefCell;

use embassy_boot_rp::{AlignedBuffer, BlockingFirmwareUpdater, FirmwareUpdaterConfig};
use embassy_rp::flash::{Blocking, Flash};
use embassy_rp::peripherals::FLASH;
use embassy_rp::Peri;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::blocking_mutex::Mutex;
use embedded_storage::nor_flash::ReadNorFlash;
use ota::UpdateManifest;
use runtime_core::Ota;
use sha2::Sha256;
use capability::l7::http::Http;
use crate::Rp235xHttp;

const FLASH_SIZE: usize = 4 * 1024 * 1024;
const MANIFEST_BUFFER_SIZE: usize = 16 * 1024;
const DOWNLOAD_CHUNK_SIZE: usize = 4 * 1024;

#[derive(Debug)]
pub enum Rp235xOtaError {
    Http,
    Manifest,
    NoUpdateAvailable,
    FirmwareTooLarge,
    FirmwareSizeMismatch,
    FirmwareHashMismatch,
    InvalidVersion,
    Flash,
}

pub struct Rp235xUpdateSource {
    http: Rp235xHttp,
}

impl Rp235xUpdateSource {
    pub fn new(http: Rp235xHttp) -> Self {
        Self { http }
    }

    async fn manifest(
        &mut self,
        url: &str,
        buffer: &mut [u8],
    ) -> Result<UpdateManifest, Rp235xOtaError> {
        let len = self
            .http
            .get(url, buffer)
            .await
            .map_err(|_| Rp235xOtaError::Http)?;

        ota::parser::json(&buffer[..len]).map_err(|_| Rp235xOtaError::Manifest)
    }

    async fn firmware_chunk(
        &mut self,
        url: &str,
        offset: usize,
        buffer: &mut [u8],
    ) -> Result<usize, Rp235xOtaError> {
        let end = offset + buffer.len() - 1;
        self.http
            .get_range(url, offset, end, buffer)
            .await
            .map_err(|_| Rp235xOtaError::Http)
    }
}

pub struct Rp235xFirmwareStorage<'d> {
    flash: Mutex<NoopRawMutex, RefCell<Flash<'d, FLASH, Blocking, FLASH_SIZE>>>,
}

impl<'d> Rp235xFirmwareStorage<'d> {
    pub fn new(flash: Peri<'d, FLASH>) -> Self {
        Self {
            flash: Mutex::new(RefCell::new(Flash::new_blocking(flash))),
        }
    }

    pub fn confirm_boot(&mut self) -> Result<(), Rp235xOtaError> {
        let config = FirmwareUpdaterConfig::from_linkerfile_blocking(&self.flash, &self.flash);
        let mut aligned = AlignedBuffer([0; 1]);

        BlockingFirmwareUpdater::new(config, &mut aligned.0)
            .mark_booted()
            .map_err(|_| Rp235xOtaError::Flash)
    }
}

pub struct Rp235xOta<'d> {
    source: Rp235xUpdateSource,
    storage: Rp235xFirmwareStorage<'d>,
    manifest_url: &'static str,
    pending: Option<UpdateManifest>,
}

impl<'d> Rp235xOta<'d> {
    pub fn new(
        source: Rp235xUpdateSource,
        storage: Rp235xFirmwareStorage<'d>,
        manifest_url: &'static str,
    ) -> Self {
        Self {
            source,
            storage,
            manifest_url,
            pending: None,
        }
    }

    pub fn confirm_boot(&mut self) -> Result<(), Rp235xOtaError> {
        self.storage.confirm_boot()
    }

    pub fn pending_manifest(&self) -> Option<&UpdateManifest> {
        self.pending.as_ref()
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
        let major = parts
            .next()
            .and_then(|part| part.parse().ok())
            .ok_or(Rp235xOtaError::InvalidVersion)?;
        let minor = parts
            .next()
            .and_then(|part| part.parse().ok())
            .ok_or(Rp235xOtaError::InvalidVersion)?;
        let patch = parts
            .next()
            .and_then(|part| part.parse().ok())
            .ok_or(Rp235xOtaError::InvalidVersion)?;

        if parts.next().is_some() {
            return Err(Rp235xOtaError::InvalidVersion);
        }

        Ok((major, minor, patch))
    }
}

impl Ota for Rp235xOta<'_> {
    type Error = Rp235xOtaError;

    async fn available(&mut self, current_v: &str) -> Result<bool, Self::Error> {
        let mut buffer = [0u8; MANIFEST_BUFFER_SIZE];
        let manifest = self
            .source
            .manifest(self.manifest_url, &mut buffer)
            .await?;

        let available = (
            manifest.version.major,
            manifest.version.minor,
            manifest.version.patch,
        ) > Self::version(current_v)?;

        self.pending = available.then_some(manifest);
        Ok(available)
    }

    async fn update(&mut self) -> Result<(), Self::Error> {
        let manifest = self
            .pending
            .clone()
            .ok_or(Rp235xOtaError::NoUpdateAvailable)?;

        if manifest.size == 0
            || manifest.sha256.len() != 64
            || !manifest.firmware_url.starts_with("https://")
        {
            return Err(Rp235xOtaError::Manifest);
        }

        let expected_hash = Self::decode_sha256(manifest.sha256.as_str())?;
        let config = FirmwareUpdaterConfig::from_linkerfile_blocking(
            &self.storage.flash,
            &self.storage.flash,
        );

        if manifest.size as usize > config.dfu.capacity() {
            return Err(Rp235xOtaError::FirmwareTooLarge);
        }

        let mut aligned = AlignedBuffer([0; 1]);
        let mut updater = BlockingFirmwareUpdater::new(config, &mut aligned.0);
        let expected_size = manifest.size as usize;
        let mut offset = 0usize;
        let mut chunk = AlignedBuffer([0u8; DOWNLOAD_CHUNK_SIZE]);

        while offset < expected_size {
            let chunk_len = core::cmp::min(DOWNLOAD_CHUNK_SIZE, expected_size - offset);
            let received = self
                .source
                .firmware_chunk(
                    manifest.firmware_url.as_str(),
                    offset,
                    &mut chunk.0[..chunk_len],
                )
                .await?;

            if received != chunk_len {
                return Err(Rp235xOtaError::FirmwareSizeMismatch);
            }

            updater
                .write_firmware(offset, &chunk.0[..received])
                .map_err(|_| Rp235xOtaError::Flash)?;

            offset += received;
            log::info!("updater: downloaded {} / {} bytes", offset, expected_size);
        }

        let mut actual_hash = [0u8; 32];
        let mut hash_buffer = [0u8; 4096];
        updater
            .hash::<Sha256>(manifest.size, &mut hash_buffer, &mut actual_hash)
            .map_err(|_| Rp235xOtaError::Flash)?;

        if actual_hash != expected_hash {
            return Err(Rp235xOtaError::FirmwareHashMismatch);
        }

        updater
            .mark_updated()
            .map_err(|_| Rp235xOtaError::Flash)
    }
}
