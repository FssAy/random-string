#[cfg(test)]
mod tests;

mod generator;

#[allow(unused_imports)]
pub use generator::*;

pub const DEFAULT_CHARSET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
