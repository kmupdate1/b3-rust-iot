use core::cell::RefCell;

use embassy_boot_rp::{AlignedBuffer, BlockingFirmwareUpdater, FirmwareUpdaterConfig};
use embassy_rp::flash::{Blocking, Flash};
use embassy_rp::peripherals::FLASH;
use embassy_rp::Peri;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::blocking_mutex::Mutex;
use embedded_storage::nor_flash::ReadNorFlash;
use ota::{BootState, FirmwareDigest, FirmwareVerifier, FirmwareWriter};
use sha2::Sha256;

use super::Rp235xOtaError;

const FLASH_SIZE: usize = 4 * 1024 * 1024;
const MAX_WRITE_SIZE: usize = 4 * 1024;

pub struct Rp235xFirmwareTarget<'d> {
    flash: Mutex<NoopRawMutex, RefCell<Flash<'d, FLASH, Blocking, FLASH_SIZE>>>,
}

impl<'d> Rp235xFirmwareTarget<'d> {
    pub fn new(flash: Peri<'d, FLASH>) -> Self {
        Self {
            flash: Mutex::new(RefCell::new(Flash::new_blocking(flash))),
        }
    }

}

impl FirmwareWriter for Rp235xFirmwareTarget<'_> {
    type Error = Rp235xOtaError;

    fn capacity(&self) -> usize {
        FirmwareUpdaterConfig::from_linkerfile_blocking(&self.flash, &self.flash)
            .dfu
            .capacity()
    }

    fn max_write_size(&self) -> usize {
        MAX_WRITE_SIZE
    }

    fn write(&mut self, offset: usize, data: &[u8]) -> Result<(), Self::Error> {
        if data.len() > MAX_WRITE_SIZE {
            return Err(Rp235xOtaError::WriteChunkTooLarge);
        }

        let config = FirmwareUpdaterConfig::from_linkerfile_blocking(&self.flash, &self.flash);
        let mut state = AlignedBuffer([0; 1]);
        let mut data_buffer = AlignedBuffer([0; MAX_WRITE_SIZE]);
        data_buffer.0[..data.len()].copy_from_slice(data);
        BlockingFirmwareUpdater::new(config, &mut state.0)
            .write_firmware(offset, &data_buffer.0[..data.len()])
            .map_err(|_| Rp235xOtaError::Flash)
    }
}

impl FirmwareVerifier for Rp235xFirmwareTarget<'_> {
    type Error = Rp235xOtaError;

    fn digest(&mut self, size: usize) -> Result<FirmwareDigest, Self::Error> {
        let config = FirmwareUpdaterConfig::from_linkerfile_blocking(&self.flash, &self.flash);
        let mut state = AlignedBuffer([0; 1]);
        let mut hash_buffer = [0; MAX_WRITE_SIZE];
        let mut digest = [0; 32];
        BlockingFirmwareUpdater::new(config, &mut state.0)
            .hash::<Sha256>(size as u32, &mut hash_buffer, &mut digest)
            .map_err(|_| Rp235xOtaError::Flash)?;
        Ok(FirmwareDigest::from_bytes(digest))
    }
}

impl BootState for Rp235xFirmwareTarget<'_> {
    type Error = Rp235xOtaError;

    fn confirm_boot(&mut self) -> Result<(), Self::Error> {
        let config = FirmwareUpdaterConfig::from_linkerfile_blocking(&self.flash, &self.flash);
        let mut state = AlignedBuffer([0; 1]);
        BlockingFirmwareUpdater::new(config, &mut state.0)
            .mark_booted()
            .map_err(|_| Rp235xOtaError::Flash)
    }

    fn mark_updated(&mut self) -> Result<(), Self::Error> {
        let config = FirmwareUpdaterConfig::from_linkerfile_blocking(&self.flash, &self.flash);
        let mut state = AlignedBuffer([0; 1]);
        BlockingFirmwareUpdater::new(config, &mut state.0)
            .mark_updated()
            .map_err(|_| Rp235xOtaError::Flash)
    }
}