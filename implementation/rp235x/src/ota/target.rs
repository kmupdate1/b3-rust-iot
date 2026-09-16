use core::cell::RefCell;

use embassy_boot_rp::{AlignedBuffer, BlockingFirmwareUpdater, FirmwareUpdaterConfig};
use embassy_embedded_hal::flash::partition::BlockingPartition;
use embassy_rp::flash::{Blocking, Flash};
use embassy_rp::peripherals::FLASH;
use embassy_rp::Peri;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::blocking_mutex::Mutex;
use embedded_storage::nor_flash::ReadNorFlash;
use ota::{BootState, FirmwareDigest, FirmwareStorage, FirmwareVerifier, FirmwareWriter};
use sha2::Sha256;

use super::Rp235xOtaError;

const FLASH_SIZE: usize = 4 * 1024 * 1024;
const MAX_WRITE_SIZE: usize = 4 * 1024;

type RpFlash<'d> = Flash<'d, FLASH, Blocking, FLASH_SIZE>;
type RpPartition<'a, 'd> = BlockingPartition<'a, NoopRawMutex, RpFlash<'d>>;

pub struct Rp235xFirmwareTarget<'d> {
    flash: Mutex<NoopRawMutex, RefCell<RpFlash<'d>>>,
    state: AlignedBuffer<1>,
}

pub struct Rp235xFirmwareWriter<'a, 'd> {
    updater: BlockingFirmwareUpdater<'a, RpPartition<'a, 'd>, RpPartition<'a, 'd>>,
    capacity: usize,
}

impl<'d> Rp235xFirmwareTarget<'d> {
    pub fn new(flash: Peri<'d, FLASH>) -> Self {
        Self {
            flash: Mutex::new(RefCell::new(Flash::new_blocking(flash))),
            state: AlignedBuffer([0; 1]),
        }
    }
}

impl<'d> FirmwareStorage for Rp235xFirmwareTarget<'d> {
    type Error = Rp235xOtaError;
    type Writer<'a> = Rp235xFirmwareWriter<'a, 'd> where Self: 'a;

    fn writer(&mut self) -> Result<Self::Writer<'_>, Self::Error> {
        let config = FirmwareUpdaterConfig::from_linkerfile_blocking(&self.flash, &self.flash);
        let capacity = config.dfu.capacity();
        Ok(Rp235xFirmwareWriter {
            updater: BlockingFirmwareUpdater::new(config, &mut self.state.0),
            capacity,
        })
    }
}

impl FirmwareWriter for Rp235xFirmwareWriter<'_, '_> {
    type Error = Rp235xOtaError;

    fn capacity(&self) -> usize {
        self.capacity
    }

    fn max_write_size(&self) -> usize {
        MAX_WRITE_SIZE
    }

    fn write(&mut self, offset: usize, data: &[u8]) -> Result<(), Self::Error> {
        if data.len() > MAX_WRITE_SIZE {
            return Err(Rp235xOtaError::WriteChunkTooLarge);
        }

        let mut aligned = AlignedBuffer([0; MAX_WRITE_SIZE]);
        aligned.0[..data.len()].copy_from_slice(data);
        self.updater
            .write_firmware(offset, &aligned.0[..data.len()])
            .map_err(|_| Rp235xOtaError::Flash)
    }
}

impl FirmwareVerifier for Rp235xFirmwareWriter<'_, '_> {
    type Error = Rp235xOtaError;

    fn digest(&mut self, size: usize) -> Result<FirmwareDigest, Self::Error> {
        let mut hash_buffer = [0; MAX_WRITE_SIZE];
        let mut digest = [0; 32];
        self.updater
            .hash::<Sha256>(size as u32, &mut hash_buffer, &mut digest)
            .map_err(|_| Rp235xOtaError::Flash)?;
        Ok(FirmwareDigest::from_bytes(digest))
    }
}

impl BootState for Rp235xFirmwareTarget<'_> {
    type Error = Rp235xOtaError;

    fn confirm_boot(&mut self) -> Result<(), Self::Error> {
        let config = FirmwareUpdaterConfig::from_linkerfile_blocking(&self.flash, &self.flash);
        BlockingFirmwareUpdater::new(config, &mut self.state.0)
            .mark_booted()
            .map_err(|_| Rp235xOtaError::Flash)
    }

    fn mark_updated(&mut self) -> Result<(), Self::Error> {
        let config = FirmwareUpdaterConfig::from_linkerfile_blocking(&self.flash, &self.flash);
        BlockingFirmwareUpdater::new(config, &mut self.state.0)
            .mark_updated()
            .map_err(|_| Rp235xOtaError::Flash)
    }
}