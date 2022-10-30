use crate::register_descriptor::RegisterDescriptor;
use funty::{Integral, Unsigned};
use bitvec::mem::BitMemory;
use std::fmt;

pub struct Register<T: Unsigned + BitMemory> {
    value: Option<T>,
    descriptor: RegisterDescriptor,
}

impl<T: Unsigned + BitMemory> Register<T> {
    pub fn new(descriptor: RegisterDescriptor) -> Result<Self, RegisterError> {
        if Register::<T>::bit_capacity() < descriptor.bit_count() {
            Err(RegisterError::InvalidBitCount)
        } else {
            Ok(Self { value: None, descriptor })
        }
    }

    pub fn set(&mut self, value: Option<T>) {
        self.value = value;
    }

    #[inline(always)]
    fn bit_capacity() -> usize {
        // T::BITS is ambiguous because of multiple definitions caused by using
        // funty 2.0.x and funty 1.2.0. The latter is used by bitvec 0.22.x,
        // see https://github.com/myrrlyn/funty/issues/3

        // I strongly prefer using the latest version of crates whenever possible.
        // I don't want to include an older funty version as it also means using
        // older names such as "IsUnsigned" vs the newer "Unsigned" etc.
        // To resolve this problem, we must fully qualify the type to its trait
        // ("Integral") which only exists in the newer funty 2.0.x version.
        <T as Integral>::BITS as usize
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum RegisterError {
    InvalidBitCount,
}

impl fmt::Display for RegisterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let err = match *self {
            RegisterError::InvalidBitCount => "invalid bit count"
        };
        write!(f, "{}", err)
    }
}

