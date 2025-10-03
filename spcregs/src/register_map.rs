use crate::RegisterDescriptor;
use std::collections::HashMap;
use std::sync::LazyLock;

mod x86;

pub struct RegisterMap<'a> {
    map: HashMap<&'a str, &'a RegisterDescriptor<'a>>,
}

impl<'a> RegisterMap<'a> {
    pub fn new() -> Self {
        let mut map: HashMap<&str, &RegisterDescriptor> = HashMap::with_capacity(REGISTERS.len());
        for desc in REGISTERS.iter() {
            map.insert(desc.name(), desc);
        }
        Self {
            // map: REGISTERS.iter().map(|k| (k.name(), *k)).collect() // I have no idea how many copies this might do
            map
        }
    }

    pub fn insert(&mut self, name: &'a str, reg: &'a RegisterDescriptor) -> Option<&RegisterDescriptor> {
        self.map.insert(name, reg)
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn get(&self, name: &'a str) -> Option<&&RegisterDescriptor> {
        self.map.get(name)
    }
}

// All new register descriptors should be added here
static REGISTERS: LazyLock<[&RegisterDescriptor; 2]> = LazyLock::new(|| { [
    &x86::CR0,
    &x86::EFER,
]});
