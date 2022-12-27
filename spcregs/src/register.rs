use crate::register_descriptor::RegisterDescriptor;
use crate::utils;
use funty::{Integral, Unsigned};
use bitvec::mem::BitMemory;
use std::fmt;

// The type that can hold the maximum value supported in a register
// and the maximum number of bits supported in a register.
pub type RegisterValue = u64;
pub static MAX_BIT_COUNT: usize = RegisterValue::BITS as usize;

pub struct Register<T: Unsigned + BitMemory> {
    value: Option<T>,
    descriptor: RegisterDescriptor,
}

impl<T: Unsigned + BitMemory> Register<T> {
    pub fn new(descriptor: RegisterDescriptor) -> Result<Self, RegisterError> {
        if Register::<T>::bit_capacity() >= descriptor.bit_count() {
            Ok(Self { value: None, descriptor })
        } else {
            Err(RegisterError::InvalidBitCount)
        }
    }

    pub fn set_value(&mut self, value: T) {
        self.value = Some(value);
    }

    pub fn clear_value(&mut self) {
        self.value = None;
    }

    #[inline(always)]
    const fn bit_capacity() -> usize {
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

impl<T: Unsigned + BitMemory> fmt::Display for Register<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.value.is_none() {
            write!(f, "{}", self.descriptor)
        } else {
            let res: Result<RegisterValue, _> = self.value.unwrap().try_into();
            match res {
                Err(_) => write!(f, "Couldn't convert register value"),
                Ok(val) => {



                    write!(f, "{}", utils::get_binary_string(val))
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum RegisterError {
    InvalidBitCount,
}

impl fmt::Display for RegisterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let err = match *self {
            RegisterError::InvalidBitCount => "register size insufficient to describe all its bits"
        };
        write!(f, "{}", err)
    }
}
