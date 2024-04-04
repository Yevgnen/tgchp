use std::io::Read;

use serde_json::Result;

mod utils;

pub mod types;
pub use types::*;

pub fn from_str(s: &str) -> Result<ChatHistory> {
    serde_json::from_str(s)
}

pub fn from_reader<R>(reader: R) -> Result<ChatHistory>
where
    R: Read,
{
    serde_json::from_reader(reader)
}
