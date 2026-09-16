mod check;
mod error;
mod install;

pub use error::*;

use crate::{
    version, BootState, FirmwareSource, FirmwareVerifier, FirmwareWriter, ManifestSource,
    UpdateManifest,
};

const MANIFEST_BUFFER_SIZE: usize = 16 * 1024;
const DOWNLOAD_BUFFER_SIZE: usize = 4 * 1024;

pub struct Updater<Source, Target> {
    source: Source,
    target: Target,
    pending: Option<UpdateManifest>,
}

impl<Source, Target> Updater<Source, Target> {
    pub const fn new(source: Source, target: Target) -> Self {
        Self {
            source,
            target,
            pending: None,
        }
    }

    pub fn pending_manifest(&self) -> Option<&UpdateManifest> {
        self.pending.as_ref()
    }
}

impl<Source, Target> Updater<Source, Target>
where
    Source: ManifestSource
        + FirmwareSource<Error = <Source as ManifestSource>::Error>,
    Target: FirmwareWriter
        + FirmwareVerifier<Error = <Target as FirmwareWriter>::Error>
        + BootState<Error = <Target as FirmwareWriter>::Error>,
{
    pub fn confirm_boot(
        &mut self,
    ) -> Result<(), OtaError<<Source as ManifestSource>::Error, <Target as FirmwareWriter>::Error>> {
        self.target.confirm_boot().map_err(OtaError::Target)
    }

    pub async fn check(
        &mut self,
        current_version: &str,
    ) -> Result<bool, OtaError<<Source as ManifestSource>::Error, <Target as FirmwareWriter>::Error>> {
        let current = version::parse(current_version)
            .map_err(|_| OtaError::InvalidCurrentVersion)?;
        let mut buffer = [0; MANIFEST_BUFFER_SIZE];
        self.pending = check::for_update(&mut self.source, &current, &mut buffer).await?;
        Ok(self.pending.is_some())
    }

    pub async fn update(
        &mut self,
    ) -> Result<(), OtaError<<Source as ManifestSource>::Error, <Target as FirmwareWriter>::Error>> {
        let manifest = self.pending.as_ref().ok_or(OtaError::NoPendingUpdate)?;
        let mut buffer = [0; DOWNLOAD_BUFFER_SIZE];
        install::firmware(&mut self.source, &mut self.target, manifest, &mut buffer).await?;
        self.pending = None;
        Ok(())
    }

    pub async fn check_and_update(
        &mut self,
        current_version: &str,
    ) -> Result<UpdateOutcome, OtaError<<Source as ManifestSource>::Error, <Target as FirmwareWriter>::Error>> {
        if !self.check(current_version).await? {
            return Ok(UpdateOutcome::UpToDate);
        }
        self.update().await?;
        Ok(UpdateOutcome::ReadyToReboot)
    }
}