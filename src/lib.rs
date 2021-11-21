use std::error::Error;

pub mod common;
pub mod core;
pub mod store;
pub mod util;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;
