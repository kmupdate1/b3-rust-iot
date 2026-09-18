use crate::FirmwareDigest;

/// Reads the staged image back and calculates its digest.
pub trait FirmwareVerifier {
    type Error;

    fn digest(&mut self, size: usize) -> Result<FirmwareDigest, Self::Error>;
}
