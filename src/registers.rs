use bitgroup::{BitGroup, BitSpan, BitSpanKind, ByteOrder};
use std::collections::HashMap;
use std::ops::RangeInclusive;

enum BitRegister {
    Reg64(BitGroup<u64>),
    Reg32(BitGroup<u32>),
}

pub struct Registers {
    registers: HashMap<String, BitRegister>,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            registers: HashMap::new()
        }
    }
}

