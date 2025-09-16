use crate::RegisterDescriptor;
use std::collections::HashMap;
use std::sync::LazyLock;

pub mod cpu_x86_registers;
pub use cpu_x86_registers::X86_CPU_EFER;

pub struct Registers<'a> {
    registers: HashMap<&'a str, &'a RegisterDescriptor>,
}

impl<'a> Registers<'a> {
    pub fn new() -> Self {
        Self {
            registers: HashMap::new()
        }
    }

    pub fn insert(&mut self, name: &'a str, reg: &'a RegisterDescriptor) -> Option<&RegisterDescriptor> {
        self.registers.insert(name, reg)
    }
}

pub static REGISTERS: LazyLock<Registers> = LazyLock::new(|| {
    let mut map: Registers = Registers::new();
    map.insert(X86_CPU_EFER.name(), &X86_CPU_EFER);
    map
});
