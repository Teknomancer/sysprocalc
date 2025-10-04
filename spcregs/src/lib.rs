mod bit_range;
mod register;
mod register_descriptor;
mod register_map;
pub mod utils;

pub use bit_range::{BitRange, BitRangeKind, BitSpan, ByteOrder};
pub use register::{MAX_BIT_COUNT, Register};
pub use register_descriptor::{RegisterDescriptor, RegisterDescriptorError};
pub use register_map::RegisterMap;

// Re-export externals
pub use bitvec::mem::BitRegister;
pub use funty::Unsigned;

#[cfg(test)]
mod unit_tests;
