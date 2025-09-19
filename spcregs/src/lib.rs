mod bit_range;
mod register_descriptor;
mod register;
mod registers;
pub mod utils;

pub use bit_range::{BitRange, BitRangeKind, ByteOrder};
pub use register_descriptor::{RegisterDescriptor, RegisterDescriptorError};
pub use register::{Register, MAX_BIT_COUNT};
pub use registers::REGISTERS;

// Re-export externals
pub use funty::Unsigned;
pub use bitvec::mem::BitRegister;

#[cfg(test)]
mod unit_tests;

