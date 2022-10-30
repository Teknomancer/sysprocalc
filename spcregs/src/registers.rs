use crate::BitGroup;
use std::collections::BTreeMap;
use std::ops::RangeInclusive;

mod cpu_x86_registers;

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

    pub fn set_value(&mut self, value: T) {
        self.value = Some(value);
    }

    pub fn clear_value(&mut self) {
        self.value = None;
    }

}

