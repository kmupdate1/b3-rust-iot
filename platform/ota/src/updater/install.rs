use crate::{
    BootState, FirmwareSource, FirmwareStorage, FirmwareVerifier, FirmwareWriter, ManifestSource,
    OtaError, UpdateManifest,
};

const PROGRESS_LOG_INTERVAL: usize = 64 * 1024;

pub(super) async fn firmware<Source, Target>(
    source: &mut Source,
    target: &mut Target,
    manifest: &UpdateManifest,
    buffer: &mut [u8],
) -> Result<(), OtaError<<Source as ManifestSource>::Error, <Target as FirmwareStorage>::Error>>
where
    Source: ManifestSource
        + FirmwareSource<Error = <Source as ManifestSource>::Error>,
    Target: FirmwareStorage + BootState<Error = <Target as FirmwareStorage>::Error>,
{
    let size = manifest.size as usize;

    log::info!("updater/install: firmware: firmware size: {} KiB", (size + 1023) / 1024);

    let mut writer = target.writer().map_err(OtaError::Target)?;
    let capacity = writer.capacity();
    if size > capacity {
        return Err(OtaError::FirmwareTooLarge { size, capacity });
    }

    let write_size = core::cmp::min(buffer.len(), writer.max_write_size());
    if write_size == 0 {
        return Err(OtaError::InvalidWriteSize);
    }

    log::info!("updater/install: firmware: writing {} bytes", write_size);

    let mut offset = 0;
    let mut next_progress = PROGRESS_LOG_INTERVAL;

    while offset < size {
        let expected = core::cmp::min(write_size, size - offset);
        let received = FirmwareSource::read(
            source,
            manifest.firmware_url.as_str(),
            offset,
            &mut buffer[..expected],
        )
        .await
        .map_err(OtaError::Source)?;
        if received != expected {
            return Err(OtaError::FirmwareSizeMismatch {
                expected,
                actual: received,
            });
        }
        writer
            .write(offset, &buffer[..received])
            .map_err(OtaError::Target)?;

        offset += received;

        if offset >= next_progress || offset == size {
            let percent = offset * 100 / size;

            log::info!(
                "updater/install: downloading: {} / {} KiB ({}%)",
                offset / 1024,
                (size + 1023) / 1024,
                percent
            );

            next_progress += PROGRESS_LOG_INTERVAL;
        }
    }

    let digest = writer.digest(size).map_err(OtaError::Target)?;
    if digest != manifest.digest {
        return Err(OtaError::FirmwareHashMismatch);
    }
    drop(writer);
    target.mark_updated().map_err(OtaError::Target)
}