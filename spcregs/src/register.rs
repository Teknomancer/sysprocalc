use crate::register_descriptor::{RegisterDescriptor, BitRangeElement};
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

                    let bit_count    = self.descriptor.bit_count();
                    let mut idx_last_bit = bit_count - 1;
                    let mut prev_padding = bit_count - 1;
                    assert!(bit_count > 0);

                    // First write out the binary bits seperated into groups of 4.
                    writeln!(f, " {}", utils::get_binary_string(val, Some(bit_count as u32)))?;

                    if let Some(bit_range_row) = self.descriptor.bit_ranges().last() {
                        write!(f, " ");
                        for bit_index in (0..bit_count).rev() {
                            if self.descriptor.has_bit(&bit_index) {
                                write!(f, "|");
                            } else {
                                write!(f, " ");
                            }
                            if bit_index % 4 == 0 {
                                write!(f, " ");
                            }
                        }
                        writeln!(f);
                    }

                    // Now iterate over each bit range.
                    for bit_range_row in self.descriptor.bit_ranges().into_iter() {
                        write!(f, " ");
                        let mut cur_bit = 0;
                        for idx_bit_col in (0..bit_count).rev() {
                            if idx_bit_col > *bit_range_row.span.start() {
                                let pad = if idx_bit_col % 4 == 0 { 2 } else { 1 };
                                write!(f, "{:width$}", " ", width = pad)?;
                            } else {
                                if idx_bit_col == *bit_range_row.span.start() {
                                    write!(f, "+");
                                    cur_bit = idx_bit_col;
                                } else if self.descriptor.has_bit(&idx_bit_col) {
                                    write!(f, "|");
                                } else {
                                    write!(f, "-");
                                }
                                if idx_bit_col > 0 && idx_bit_col % 4 == 0 {
                                    write!(f, " ");
                                }
                            }
                        }
                        let is_set_indicator = if val & ((1 as RegisterValue) << cur_bit) != 0 { " *" } else { "" };
                        writeln!(f, "--- {name:<namewidth$} ({bitnum:>bitwidth$}){bit_is_set}",
                                 name = bit_range_row.name,
                                 namewidth = self.descriptor.column_width(BitRangeElement::Name),
                                 bitnum = *bit_range_row.span.start(),
                                 bitwidth = self.descriptor.column_width(BitRangeElement::Bits),
                                 bit_is_set = is_set_indicator);
                    }
                    Ok(())
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

