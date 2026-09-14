pub mod parser;

use heapless::String;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub pre: Option<String<32>>,
    pub build: Option<String<32>>,
}
