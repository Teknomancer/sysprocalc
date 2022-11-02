mod bit_range;
mod register_descriptor;
mod register;
pub mod utils;

pub use bit_range::{BitRange, BitRangeKind, ByteOrder };
pub use register_descriptor::{RegisterDescriptor, RegisterDescriptorError };
pub use register::{Register, MAX_BIT_COUNT};

#[cfg(test)]
mod unit_tests;

