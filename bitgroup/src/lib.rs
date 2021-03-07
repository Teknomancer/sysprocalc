use std::ops::Range;
use std::collections::HashSet;
use std::hash::Hash;
use std::fmt;
use std::convert::TryFrom;

static MAX_BITCOUNT: u8 = 64;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ByteOrder {
    LittleEndian,
    BigEndian,
}

#[derive(Debug)]
pub struct BitSpan {
    spans: Range<u8>,
    kind: BitSpanKind,
    show_rsvd: bool,
    name: String,
    short: String,
    long: String,
}

impl BitSpan {
    pub fn new(spans: Range<u8>, kind: BitSpanKind, show_rsvd: bool, name: String, short: String, long: String) -> Self {
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
    bit_count: u8,
    chunks: Vec<u8>,
    desc: Vec<BitSpan>,
}

impl BitGroup {
    pub fn new(
            name: String,
            arch: String,
            device: String,
            byte_order: ByteOrder,
            bit_count: u8,
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
        Ok(())
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
    Ok("Testing_Impl".to_string())
}

#[test]
fn test_valid_bit_group() {
    let gen_bits = BitGroup::new(
        String::from("generic"),
        String::from("x86"),
        String::from("cpu"),
        ByteOrder::LittleEndian,
        64, vec![],
        vec![
            BitSpan::new(
                Range { start: 0, end: 0 },
                BitSpanKind::Normal,
                false,
                String::from("Gen 0"),
                String::from("Generic 0"),
                String::from("Generic Bit 0"),
            ),
            BitSpan::new(
                Range { start: 8, end: 8 },
                BitSpanKind::Normal,
                false,
                String::from("Gen 1"),
                String::from("Generic 1"),
                String::from("Generic Bit 1"),
            ),
        ]);
    let res_fmt = validate_bit_group(&gen_bits);
    assert!(res_fmt.is_ok());
}

#[test]
fn test_invalid_bit_group() {
    let pair_invalid_bit_grps = [
        //
        // Invalid bit count
        //
        (BitGroup::new(
            String::from("generic"),
            String::from("x86"),
            String::from("cpu"),
            ByteOrder::LittleEndian,
            128,
            vec![],
            vec![
                BitSpan::new(
                    Range { start: 0, end: 0 },
                    BitSpanKind::Normal,
                    false,
                    String::from("Gen 0"),
                    String::from("Generic 0"),
                    String::from("Generic Bit 0"),
                ),
                BitSpan::new(
                    Range { start: 8, end: 8 },
                    BitSpanKind::Normal,
                    false,
                    String::from("Gen 1"),
                    String::from("Generic 1"),
                    String::from("Generic Bit 1"),
                ),
            ],
        ),
        BitGroupError::InvalidBitCount),

        //
        // TODO: Invalid chunk index
        //
    ];

    for bs in &pair_invalid_bit_grps {
        let res_fmt = validate_bit_group(&bs.0);
        assert!(res_fmt.is_err(), "{:?}", bs.0);
        assert_eq!(res_fmt.err().unwrap(), bs.1);
    }
}

