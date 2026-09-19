use crate::ManifestError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OtaError<SourceError, TargetError> {
    Source(SourceError),
    Target(TargetError),
    Manifest(ManifestError),
    InvalidCurrentVersion,
    NoPendingUpdate,
    InvalidWriteSize,
    FirmwareTooLarge { size: usize, capacity: usize },
    FirmwareSizeMismatch { expected: usize, actual: usize },
    FirmwareHashMismatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateOutcome {
    UpToDate,
    ReadyToReboot,
}
