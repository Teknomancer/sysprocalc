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
        let mut registers: HashMap<&str, &RegisterDescriptor> = HashMap::with_capacity(REGISTERS.len());
        for desc in REGISTERS.iter() {
            registers.insert(desc.name(), desc);
        }
        Self {
            // registers: REGISTERS.iter().map(|k| (k.name(), *k)).collect() // I have no idea how many copies this might do
            registers: registers
        }
    }

    pub fn insert(&mut self, name: &'a str, reg: &'a RegisterDescriptor) -> Option<&RegisterDescriptor> {
        self.registers.insert(name, reg)
    }
}

// All new register descriptors should be added here
static REGISTERS: LazyLock<[&RegisterDescriptor; 1]> = LazyLock::new(|| { [
    &X86_CPU_EFER,
]});
