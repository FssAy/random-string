#[cfg(test)]
mod tests;

mod generator;

#[cfg(feature = "charsets")]
pub mod charsets;

#[allow(unused_imports)]
pub use generator::*;
