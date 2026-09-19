#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rp235xOtaError {
    Http,
    Flash,
    WriteChunkTooLarge,
}
