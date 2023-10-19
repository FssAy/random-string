#[cfg(test)]
mod tests;

mod generator;

#[allow(unused_imports)]
pub use generator::*;

pub const default_charset: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
