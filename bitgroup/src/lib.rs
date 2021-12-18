use std::ops::RangeInclusive;
use std::fmt;
use std::marker::PhantomData;
use funty::Unsigned;
use bitvec::prelude::*;
use bitvec::mem::BitMemory;

static MAX_BITCOUNT: usize = u64::BITS as usize;
static BIT_RANGE_SEP: &str = ":";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ByteOrder {
    LittleEndian,
    BigEndian,
}

#[derive(Debug)]
pub struct BitSpan {
    span: RangeInclusive<usize>,
    kind: BitSpanKind,
    show_rsvd: bool,
    name: String,
    short: String,
    long: String,
}

impl BitSpan {
    pub fn new(
            span: RangeInclusive<usize>,
            kind: BitSpanKind,
            show_rsvd: bool,
            name: String,
            short: String,
            long: String) -> Self {
        Self { span, kind, show_rsvd, name, short, long }
    }
}

#[derive(Debug)]
pub enum BitSpanKind {
    Normal,
    ReservedMustBeZero,
    ReservedMustBeOne,
    ReservedUndefined,
    ReservedIgnored,
}

#[derive(Debug)]
pub struct BitGroup<T: Unsigned + BitMemory> {
    name: String,
    arch: String,
    device: String,
    desc: String,
    byte_order: ByteOrder,
    value: BitVec,
    bitspans: Vec<BitSpan>,
    phantom: PhantomData<T>,
}

enum BitSpanElement {
    Bits,
    Name,
    Short,
    Long,
}

impl<T: Unsigned + BitMemory> BitGroup<T> {
    pub fn new(
            name: String,
            arch: String,
            device: String,
            desc: String,
            byte_order: ByteOrder,
            bitspans: Vec<BitSpan>) -> Self {
        Self { name, arch, device, desc, byte_order, value: BitVec::new(), bitspans, phantom: PhantomData }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.desc
    }

    pub fn set_value(&mut self, value: T) {
        self.value.store(value);
    }

    #[inline(always)]
    fn bit_capacity(&self) -> usize {
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

    fn validate_bitspans(&self) -> Result<(), BitGroupError> {
        let total_bits = self.bit_capacity();
        let mut bitpos:Vec<_> = (0..total_bits).collect(); // Vector of valid bit positions to check overlap in bit ranges.
        for bitspan in &self.bitspans {
            if bitspan.span.is_empty() || *bitspan.span.end() >= total_bits {
                // The bit range is invalid (end() is inclusive)
                return Err(BitGroupError::InvalidBitRange);
            } else if bitspan.name.is_empty() {
               // The bit name is missing.
               return Err(BitGroupError::MissingBitName);
            } else if bitspan.short.is_empty() {
               // The bit description is missing.
               return Err(BitGroupError::MissingBitSpanDescription);
            } else {
                // Validate that bit ranges don't overlap.
                // We replace items in a vector (0..MAX_BITCOUNT) with poisoned values for
                // each range in the description. If the removed items contains a poisoned
                // value it implies some previous range already existed causing an overlap.
                // E.g. If MAX_BITCOUNT is 64, bitpos is (0..=63) and poison value is 64.
                let end = *bitspan.span.end() + 1;    // Exclusive bound.
                let start = *bitspan.span.start();    // Inclusive bound.
                let poison = vec![MAX_BITCOUNT; end - start];
                let removed:Vec<_> = bitpos.splice(start..end, poison).collect();
                if removed.iter().any(|&x| x == MAX_BITCOUNT) {
                    return Err(BitGroupError::OverlappingBitRange);
                }
            }
        }
        Ok(())
    }

    fn validate(&self) -> Result<(), BitGroupError> {
        if self.bit_capacity() > MAX_BITCOUNT {
            // The number of bits exceeds our maximum supported limit.
            Err(BitGroupError::InvalidBitCount)
        } else if self.bitspans.is_empty() {
           // None of the bits are described.
           Err(BitGroupError::MissingBitSpans)
        } else if self.bitspans.len() > MAX_BITCOUNT {
           // The number of bit descriptions exceeds our maximum bit count.
           Err(BitGroupError::InvalidBitCount)
        } else {
            // Validate the bit descriptions
            self.validate_bitspans()
        }
    }

    fn get_column_width(&self, element: BitSpanElement) -> usize {
        let mut col_len = 0;
        match element {
            BitSpanElement::Bits => {
                let mut has_ranges = false;
                for bitspan in &self.bitspans {
                    let bits_in_range = bitspan.span.end() + 1 - bitspan.span.start();
                    debug_assert!(bits_in_range >= 1);
                    if bits_in_range > 1 {
                        has_ranges = true;
                        break;
                    }
                }
                if has_ranges {
                    col_len = format!("{end}{sep}{start}", end=MAX_BITCOUNT, sep=BIT_RANGE_SEP, start=MAX_BITCOUNT).len();
                } else {
                    col_len = MAX_BITCOUNT.to_string().len();
                }
            }

            BitSpanElement::Name => {
                for bitspan in &self.bitspans {
                    col_len = std::cmp::max(col_len, bitspan.name.len());
                }
            }

            BitSpanElement::Short => {
                for bitspan in &self.bitspans {
                    col_len = std::cmp::max(col_len, bitspan.short.len());
                }
            }

            BitSpanElement::Long => {
                for bitspan in &self.bitspans {
                    col_len = std::cmp::max(col_len, bitspan.long.len());
                }
            }
        }
        col_len
    }
}

impl<T: Unsigned + BitMemory> fmt::Display for BitGroup<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Figure out column widths.
        static COL_SEP: &str = "  ";
        let name_column_width = self.get_column_width(BitSpanElement::Name);
        let short_column_width = self.get_column_width(BitSpanElement::Short);
        let bit_column_width = self.get_column_width(BitSpanElement::Bits);

        let mut out = String::from("");
        if self.value.is_empty() {
            for bitspan in self.bitspans.iter().rev() {
                let end = bitspan.span.end();
                let start = bitspan.span.start();
                let bitpos = if end == start {
                    format!("{}", start)
                } else {
                    format!("{end}{sep}{start}", end = end, sep = BIT_RANGE_SEP, start = start)
                };
                out = format!(
                    "{}{bit:>bit_width$}{sep0}{name:>name_width$}{sep1}{short:<short_width$}{sep2}{long}\n",
                    out,
                    bit = bitpos,
                    bit_width = bit_column_width,
                    sep0 = COL_SEP,
                    name = bitspan.name, name_width = name_column_width,
                    sep1 = COL_SEP,
                    short = bitspan.short, short_width = short_column_width,
                    sep2 = COL_SEP,
                    long = bitspan.long,
                    );
            }
        } else {
            // See https://docs.rs/crate/bitvec/latest for access patterns.
            // We ideally want to index a group of bits.
            todo!();
        }
        write!(f, "{}", out)
    }
}

#[derive(Debug, PartialEq)]
pub enum BitGroupError {
    MissingName,
    MissingArch,
    MissingDevice,
    UnknownArch,
    InvalidBitCount,
    MissingBitSpans,
    InvalidBitRange,
    OverlappingBitRange,
    MissingBitName,
    MissingBitSpanDescription,
}

impl fmt::Display for BitGroupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let err = match *self {
            BitGroupError::MissingName => "missing name",
            BitGroupError::MissingArch => "missing architecture",
            BitGroupError::MissingDevice => "missing device",
            BitGroupError::UnknownArch => "unknown architecture",
            BitGroupError::InvalidBitCount => "invalid number of bits",
            BitGroupError::MissingBitSpans => "missing bit spans",
            BitGroupError::InvalidBitRange => "invalid bit range",
            BitGroupError::OverlappingBitRange => "overlapping bit range",
            BitGroupError::MissingBitName => "empty bit name",
            BitGroupError::MissingBitSpanDescription => "missing bit description",
        };
        write!(f, "{}", err)
    }
}

// Figure out a better place to put this static function
pub fn get_binary_string(val: u64) -> String {
    // Formats the number as binary digits with a space (from the right) for every 4 binary digits.
    static BIN_DIGITS: [char; 2] = ['0', '1'];
    let mut bin_out: Vec<char> = Vec::with_capacity(88);
    let mut val = val;
    let num_digits = u64::MAX.count_ones() - val.leading_zeros();

    // Push first bit (to avoid extra branch in the loop for not pushing ' ' on 0th iteration).
    bin_out.push(BIN_DIGITS[val.wrapping_rem(2) as usize]);
    val >>= 1;

    // Push remaining bits.
    for idx in 1..num_digits {
        if idx % 4 == 0 {
            bin_out.push(' ');
        }
        bin_out.push(BIN_DIGITS[val.wrapping_rem(2) as usize]);
        val >>= 1;
    }

    // Return the vector of binary-digit characters (after reversing for reading LTR) as string.
    bin_out.iter().rev().collect::<String>()
}

// Figure out a better place to put this static function
pub fn get_binary_ruler_string(num_bits: u8) -> String {
    // Makes a binary ruler (for every 8 bits) to ease visual counting of bits.
    // There might be a more efficient way to do this with Rust's string/vector
    // manipulation. But I can't be bothered now, just get something working.

    // First convert the u32 to usize since we need a usize to index into an array.
    // This is will panic if the conversion fails (on architectures where usize
    // is insufficient to hold 32 bits). Panic is better than failing in weird ways.
    let num_bits = usize::try_from(num_bits).unwrap();

    // Ensure if we ever add 128-bit support this code will at least assert.
    debug_assert!(num_bits <= 64);

    if num_bits >= 8 {
        let mut ruler_out = String::with_capacity(98);
        static BIN_RULER: [&str; 8] = [
            "|  7:0  |",
            "| 15:8  | ",
            "| 23:16 | ",
            "| 31:24 | ",
            "| 39:32 | ",
            "| 47:40 | ",
            "| 55:48 | ",
            "| 63:56 | ",
        ];

        // First we need to pad with spaces at the start for those binary digits
        // that do not fall within a chunk of 8-bits (see BIN_RULER).
        // For e.g. "10 1111 1111", we need to pad the first 2 digits (and 1 space)
        // from the left. We iterate below until we no longer need to pad with spaces
        // prior to the start of the ruler.
        // TODO: I'm sure this can be optimized, but no time now.
        let num_pad_bits = num_bits % 8;
        for idx in 0..num_pad_bits {
            ruler_out.push(' ');
            if idx % 4 == 0 {
                ruler_out.push(' ');
            }
        }

        // Iterate over chunks of 8-bits and make the ruler
        for idx in (num_pad_bits..num_bits).rev().step_by(8) {
            ruler_out.push_str(BIN_RULER[((idx + 1) >> 3) - 1]);
        }

        ruler_out
    } else {
        "".to_string()
    }
}

#[cfg(test)]
mod unit_tests;

