use sonic_rs::Result;

mod utils;

pub mod types;
pub use types::*;

pub fn from_str(s: &str) -> Result<ChatHistory> {
    sonic_rs::from_str(s)
}
