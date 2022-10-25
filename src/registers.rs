use bitgroup::{BitGroup, BitSpan, BitSpanKind, ByteOrder};
use std::collections::BTreeMap;
use std::ops::RangeInclusive;

enum BitRegister<'a> {
    Reg64(BitGroup<'a, u64>),
    Reg32(BitGroup<'a, u32>),
}

pub struct Registers<'a> {
    registers: BTreeMap<String, BitRegister<'a>>,
}

impl<'a> Registers<'a> {
    pub fn new() -> Self {
        Self {
            registers: BTreeMap::new()
        }
    }
}

