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
        Self {
            map: REGISTERS.iter().map(|k| (k.name(), *k)).collect()
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
static REGISTERS: LazyLock<[&RegisterDescriptor; 3]> = LazyLock::new(|| {[
    &x86::CR0,
    &x86::CR4,
    &x86::EFER,
]});

// Can't we push this to the consumer?
pub static REGISTERMAP: LazyLock<RegisterMap> = LazyLock::new(|| RegisterMap::new());
