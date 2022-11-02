use crate::bit_range::{BitRange, ByteOrder};
use crate::register::MAX_BIT_COUNT;

use std::fmt;

static BIT_RANGE_SEP: &str = ":";

#[derive(Debug)]
pub struct RegisterDescriptor {
    arch: String,
    device: String,
    name: String,
    desc: String,
    bit_count: usize,
    byte_order: ByteOrder,
    bit_ranges: Vec<BitRange>,
}

#[derive(Debug)]
enum BitRangeElement {
    Bits,
    Name,
    Short,
    Long,
}

impl RegisterDescriptor {
    pub fn new(
        arch: String,
        device: String,
        name: String,
        desc: String,
        bit_count: usize,
        byte_order: ByteOrder,
        bit_ranges: Vec<BitRange>
    ) -> Result<Self, RegisterDescriptorError> {
        // Check that the bit ranges isn't empty.
        if bit_ranges.is_empty() {
           return Err(RegisterDescriptorError::MissingBitRanges)
        }

        // Check if the number of bits in the register is within supported limits.
        if bit_count > MAX_BIT_COUNT {
            return Err(RegisterDescriptorError::InvalidBitCount);
        }

        let mut bitpos:Vec<_> = (0..bit_count).collect(); // Vector of valid bit positions to check overlap in bit ranges.
        for bit_range in &bit_ranges {
            if bit_range.name.is_empty() {
               return Err(RegisterDescriptorError::MissingBitName);
            }

            if bit_range.short.is_empty() {
               return Err(RegisterDescriptorError::MissingBitRangeDescription);
            }

            // Check if the bit range is within supported limits (note: end() is inclusive bound)
            if bit_range.span.is_empty() || *bit_range.span.end() >= bit_count {
                return Err(RegisterDescriptorError::InvalidBitRange);
            }

            // Check that bit ranges don't overlap.
            // We replace items in a vector (0..MAX_BIT_COUNT) with poisoned values for
            // each range in the description. If the removed items contains a poisoned
            // value it implies some previous range already existed causing an overlap.
            // E.g. If MAX_BIT_COUNT is 64, bitpos is (0..=63) and poison value is 64.
            let end = *bit_range.span.end() + 1;    // Exclusive bound.
            let start = *bit_range.span.start();    // Inclusive bound.
            let poison = vec![MAX_BIT_COUNT; end - start];
            let removed:Vec<_> = bitpos.splice(start..end, poison).collect();
            if removed.iter().any(|&x| x == MAX_BIT_COUNT) {
                return Err(RegisterDescriptorError::OverlappingBitRange);
            }
        }

        Ok(Self { name, arch, device, desc, bit_count, byte_order, bit_ranges })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.desc
    }

    pub fn device(&self) -> &str {
        &self.device
    }

    pub fn arch(&self) -> &str {
        &self.arch
    }

    pub fn bit_count(&self) -> usize {
        self.bit_count
    }

    fn column_width(&self, element: BitRangeElement) -> usize {
        let mut col_len = 0;
        match element {
            BitRangeElement::Bits => {
                let mut has_ranges = false;
                for bit_range in &self.bit_ranges {
                    let bits_in_range = bit_range.span.end() + 1 - bit_range.span.start();
                    debug_assert!(bits_in_range >= 1);
                    if bits_in_range > 1 {
                        has_ranges = true;
                        break;
                    }
                }
                if has_ranges {
                    col_len = format!("{end}{sep}{start}", end=MAX_BIT_COUNT, sep=BIT_RANGE_SEP, start=MAX_BIT_COUNT).len();
                } else {
                    col_len = MAX_BIT_COUNT.to_string().len();
                }
            }

            BitRangeElement::Name => {
                for bit_range in &self.bit_ranges {
                    col_len = std::cmp::max(col_len, bit_range.name.len());
                }
            }

            BitRangeElement::Short => {
                for bit_range in &self.bit_ranges {
                    col_len = std::cmp::max(col_len, bit_range.short.len());
                }
            }

            BitRangeElement::Long => {
                for bit_range in &self.bit_ranges {
                    col_len = std::cmp::max(col_len, bit_range.long.len());
                }
            }
        }
        col_len
    }
}

impl fmt::Display for RegisterDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Figure out column widths.
        static COL_SEP: &str = "  ";
        let name_width = self.column_width(BitRangeElement::Name);
        let short_width = self.column_width(BitRangeElement::Short);
        let bits_width = self.column_width(BitRangeElement::Bits);

        // Format the bit ranges
        let mut out = String::from("");
        for bit_range in self.bit_ranges.iter().rev() {
            let end = bit_range.span.end();
            let start = bit_range.span.start();
            let bitpos = if end == start {
                format!("{}", start)
            } else {
                format!("{end}{sep}{start}", end = end, sep = BIT_RANGE_SEP, start = start)
            };
            out = format!(
                "{}{bit:>bits_w$}{sep0}{name:>name_w$}{sep1}{short:<short_w$}{sep2}{long}\n",
                out,
                bit = bitpos,
                bits_w = bits_width,
                sep0 = COL_SEP,
                name = bit_range.name, name_w = name_width,
                sep1 = COL_SEP,
                short = bit_range.short, short_w = short_width,
                sep2 = COL_SEP,
                long = bit_range.long
            );
        }
        write!(f, "{}", out)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum RegisterDescriptorError {
    InvalidBitCount,
    InvalidBitRange,
    MissingArch,
    MissingBitName,
    MissingBitRanges,
    MissingBitRangeDescription,
    MissingDevice,
    MissingName,
    OverlappingBitRange,
    UnknownArch,
}

impl fmt::Display for RegisterDescriptorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let err = match *self {
            RegisterDescriptorError::InvalidBitCount => "invalid bit count",
            RegisterDescriptorError::InvalidBitRange => "invalid bit range",
            RegisterDescriptorError::MissingArch => "missing architecture",
            RegisterDescriptorError::MissingBitName => "empty bit name",
            RegisterDescriptorError::MissingBitRanges => "missing bit ranges",
            RegisterDescriptorError::MissingBitRangeDescription => "missing bit description",
            RegisterDescriptorError::MissingDevice => "missing device",
            RegisterDescriptorError::MissingName => "missing name",
            RegisterDescriptorError::OverlappingBitRange => "overlapping bit range",
            RegisterDescriptorError::UnknownArch => "unknown architecture",
        };
        write!(f, "{}", err)
    }
}

