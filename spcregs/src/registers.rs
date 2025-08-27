use crate::RegisterDescriptor;
use std::collections::BTreeMap;

pub mod cpu_x86_registers;
pub use cpu_x86_registers::X86_CPU_EFER;

pub struct Registers<'a> {
    registers: BTreeMap<String, &'a RegisterDescriptor>,
}

impl<'a> Registers<'a> {
    pub fn new() -> Self {
        Self {
            registers: BTreeMap::new()
        }
    }

    pub fn insert(&mut self, name: &str, reg: &'a RegisterDescriptor) -> Option<&RegisterDescriptor> {
        self.registers.insert(name.to_string(), reg)
    }
}

