use crate::{
    BootState, FirmwareSource, FirmwareStorage, FirmwareVerifier, FirmwareWriter, ManifestSource,
    OtaError, UpdateManifest,
};

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
    let mut writer = target.writer().map_err(OtaError::Target)?;
    let capacity = writer.capacity();
    if size > capacity {
        return Err(OtaError::FirmwareTooLarge { size, capacity });
    }

    let write_size = core::cmp::min(buffer.len(), writer.max_write_size());
    if write_size == 0 {
        return Err(OtaError::InvalidWriteSize);
    }
    let mut offset = 0;
    while offset < size {
        let expected = core::cmp::min(write_size, size - offset);
        let received = source
            .read(
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
    }

    let digest = writer.digest(size).map_err(OtaError::Target)?;
    if digest != manifest.digest {
        return Err(OtaError::FirmwareHashMismatch);
    }
    drop(writer);
    target.mark_updated().map_err(OtaError::Target)
}