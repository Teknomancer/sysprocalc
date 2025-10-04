use crate::RegisterDescriptor;
use std::collections::HashMap;
use std::sync::LazyLock;

mod x86;

pub struct RegisterMap<'a> {
    map: HashMap<&'a str, &'a RegisterDescriptor<'a>>,
}

impl<'a> Default for RegisterMap<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> RegisterMap<'a> {
    pub fn new() -> Self {
        let mut map: HashMap<&str, &RegisterDescriptor> = HashMap::with_capacity(REGISTERS.len());
        for desc in REGISTERS.iter() {
            map.insert(desc.name(), desc);
        }
        Self {
            // map: REGISTERS.iter().map(|k| (k.name(), *k)).collect() // I have no idea how many copies this might do
            map,
        }
    }

    pub fn insert(&mut self, name: &'a str, reg: &'a RegisterDescriptor) -> Option<&RegisterDescriptor<'_>> {
        self.map.insert(name, reg)
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn get(&self, name: &'a str) -> Option<&&RegisterDescriptor<'_>> {
        self.map.get(name)
    }
}

#[rustfmt::skip]
static REGISTERS: LazyLock<[&RegisterDescriptor; 2]> = LazyLock::new(|| {[
    &x86::CR0,
    &x86::EFER,
]});
