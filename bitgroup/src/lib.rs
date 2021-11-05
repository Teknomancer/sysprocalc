use std::ops::RangeInclusive;
use std::collections::HashSet;
use std::hash::Hash;
use std::fmt;
use std::convert::TryFrom;

static MAX_BITCOUNT: usize = 64;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ByteOrder {
    LittleEndian,
    BigEndian,
}

#[derive(Debug)]
pub struct BitSpan {
    spans: RangeInclusive<usize>,
    kind: BitSpanKind,
    show_rsvd: bool,
    name: String,
    short: String,
    long: String,
}

impl BitSpan {
    pub fn new(spans: RangeInclusive<usize>, kind: BitSpanKind, show_rsvd: bool, name: String, short: String, long: String) -> Self {
        BitSpan { spans, kind, show_rsvd, name, short, long }
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
pub struct BitGroup {
    name: String,
    arch: String,
    device: String,
    byte_order: ByteOrder,
    bit_count: usize,
    chunks: Vec<u8>,
    desc: Vec<BitSpan>,
}

impl BitGroup {
    pub fn new(
            name: String,
            arch: String,
            device: String,
            byte_order: ByteOrder,
            bit_count: usize,
            chunks: Vec<u8>,
            desc: Vec<BitSpan>) -> Self {
        BitGroup {
            name,
            arch,
            device,
            byte_order,
            bit_count,
            chunks,
            desc,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BitGroupError {
    MissingName,
    MissingArch,
    MissingDevice,
    UnknownArch,
    InvalidBitCount,
    InvalidChunkIndex,
    InvalidChunksLength,
    DuplicateChunkIndex,
    MissingDescription,
    InvalidBitRange,
    OverlappingBitRange,
    MissingBitName,
    MissingBitDescription,
}

impl fmt::Display for BitGroupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str_errkind = match self {
            BitGroupError::MissingName => "missing name",
            BitGroupError::MissingArch => "missing architecture",
            BitGroupError::MissingDevice => "missing device",
            BitGroupError::UnknownArch => "unknown architecture",
            BitGroupError::InvalidBitCount => "invalid number of bits",
            BitGroupError::InvalidChunkIndex => "invalid index in chunks'",
            BitGroupError::InvalidChunksLength => "invalid number of chunks",
            BitGroupError::DuplicateChunkIndex => "duplicate index in chunks",
            BitGroupError::MissingDescription => "missing description",
            BitGroupError::InvalidBitRange => "invalid bit range",
            BitGroupError::OverlappingBitRange => "overlapping bit range",
            BitGroupError::MissingBitName => "empty bit name",
            BitGroupError::MissingBitDescription => "missing bit description",
        };
        write!(f, "{}", str_errkind)
    }
}

fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

fn validate_bit_group_desc(bits: &BitGroup) -> Result<(), BitGroupError> {
    if bits.desc.len() > MAX_BITCOUNT as usize {
        // The number of bit descriptions exceeds our maximum bit count.
        Err(BitGroupError::InvalidBitCount)
    } else {
        let mut bitpos:Vec<_> = (0..MAX_BITCOUNT).collect(); // Vector of valid bit positions to check overlap in bit ranges.
        for desc in &bits.desc {
            if desc.spans.is_empty() || *desc.spans.end() >= bits.bit_count {
                // The bit range is invalid (end() is inclusive)
                return Err(BitGroupError::InvalidBitRange);
            } else if desc.name.is_empty() {
               // The bit name is missing.
               return Err(BitGroupError::MissingBitName);
            } else if desc.short.is_empty() {
               // The bit description is missing.
               return Err(BitGroupError::MissingBitDescription);
            } else {
                // Validate that bit ranges don't overlap.
                // We replace items in a vector (0..MAX_BITCOUNT) with poisoned values for
                // each range in the description. If the removed items contains a poisoned
                // value it implies some previous range already existed causing an overlap.
                // E.g. If MAX_BITCOUNT is 64, bitpos is (0..=63) and poison value is 64.
                let end = *desc.spans.end() + 1;    // Exclusive bound.
                let start = *desc.spans.start();    // Inclusive bound.
                let poison = vec![MAX_BITCOUNT; end - start];
                let removed:Vec<_> = bitpos.splice(start..end, poison).collect();
                if removed.iter().any(|&x| x == MAX_BITCOUNT) {
                    return Err(BitGroupError::OverlappingBitRange);
                }
            }
        }
        Ok(())
    }
}

fn validate_bit_group(bits: &BitGroup) -> Result<(), BitGroupError> {
    if bits.bit_count > MAX_BITCOUNT {
        // The number of bits exceeds our limit.
        Err(BitGroupError::InvalidBitCount)
    } else if bits.chunks.len() >= MAX_BITCOUNT as usize {
        // The chunks index array size exceeds the total number of bits.
        Err(BitGroupError::InvalidChunksLength)
    } else if !has_unique_elements(&bits.chunks) {
        // The chunks index array contains duplicate indices.
        Err(BitGroupError::DuplicateChunkIndex)
    } else if bits.desc.is_empty() {
       // None of the bits are described.
       Err(BitGroupError::MissingDescription)
    } else {
        // Validate the bit descriptions
        validate_bit_group_desc(bits)
    }
}

pub fn get_binary_string(val: u64) -> String {
    // Formats the number as binary digits with a space (from the right) for every 4 binary digits.
    let mut vec_bin: Vec<char> = Vec::with_capacity(82);
    let mut val = val;
    static BIN_DIGITS: [char; 2] = ['0', '1'];
    let num_digits = u64::MAX.count_ones() - val.leading_zeros();

    // Push first bit (to avoid extra branch in the loop for not pushing ' ' on 0th iteration).
    vec_bin.push(BIN_DIGITS[val.wrapping_rem(2) as usize]);
    val >>= 1;

    // Push remaining bits.
    for idx in 1..num_digits {
        if idx % 4 == 0 {
            vec_bin.push(' ');
        }
        vec_bin.push(BIN_DIGITS[val.wrapping_rem(2) as usize]);
        val >>= 1;
    }

    // Return the vector of binary-digit characters (after reversing for reading LTR) as string.
    vec_bin.iter().rev().collect::<String>()
}

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
        let mut str_bin_ruler = String::with_capacity(98);
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
            str_bin_ruler.push(' ');
            if idx % 4 == 0 {
                str_bin_ruler.push(' ');
            }
        }

        // Iterate over chunks of 8-bits and make the ruler
        for idx in (num_pad_bits..num_bits).rev().step_by(8) {
            str_bin_ruler.push_str(BIN_RULER[((idx + 1) >> 3) - 1]);
        }

        str_bin_ruler
    } else {
        "".to_string()
    }
}

pub fn fmt_bit_group(bits: &BitGroup) -> Result<String, BitGroupError> {
    validate_bit_group(bits)?;
    Ok("Validates Okay".to_string())
}


#[cfg(test)]
mod unit_tests;

