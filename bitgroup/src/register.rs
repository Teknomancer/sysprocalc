use crate::register_descriptor::RegisterDescriptor;
use funty::Unsigned;
use bitvec::mem::BitMemory;
use std::fmt;

pub struct Register<T: Unsigned + BitMemory> {
    value: Option<T>,
    descriptor: RegisterDescriptor,
}

impl<T: Unsigned + BitMemory> Register<T> {
    pub fn new(descriptor: RegisterDescriptor) -> Result<Self, RegisterError> {
        if Register::<T>::bit_capacity() < descriptor.bit_count {
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
        // Cannot use T::BITS due to multiple definitions caused by
        // funty 2.0.0 vs funty 1.2.0 (latter required by bitvec 0.22.x).
        // I don't want to include funty 1.2.0 as it also has other changes
        // to names such as using the older "IsUnsigned" vs the newer "Unsigned".
        // I prefer to use the latest version where possible despite bitvec
        // internally still using funty 1.2.0. Since "BITS" is the only conflict
        // for now, this is manageable.
        // See https://github.com/myrrlyn/funty/issues/3

        // T::BITS as usize
        std::mem::size_of::<T>() * 8
    }
}

#[derive(Debug, PartialEq)]
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

