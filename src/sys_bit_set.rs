use std::ops::Range;
use std::collections::HashSet;
use std::hash::Hash;
use std::fmt;

static MAX_BITCOUNT: u8 = 64;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ByteOrder {
    LittleEndian,
    BigEndian,
}

#[derive(Debug)]
pub struct SysBitSetDescription {
    spans: Range<u8>,
    kind: SysBitSetKind,
    name: String,
    short: String,
    long: String,
}

impl SysBitSetDescription {
    pub fn new(spans: Range<u8>, kind: SysBitSetKind, name: String, short: String, long: String) -> Self {
        SysBitSetDescription { spans, kind, name, short, long }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SysBitSetReserved {
    MustBeZero,
    MustBeOne,
    Undefined,
    Ignored,
}

#[derive(Debug)]
pub enum SysBitSetKind {
    Normal,
    Rsvd(SysBitSetReserved),
}

#[derive(Debug)]
pub struct SysBitSet {
    name: String,
    arch: String,
    device: String,
    byte_order: ByteOrder,
    bit_count: u8,
    chunks: Vec<u8>,
    rsvd: SysBitSetReserved,
    show_rsvd: bool,
    desc: Vec<SysBitSetDescription>,
}

impl SysBitSet {
    pub fn new(name: String, arch: String, device: String, byte_order: ByteOrder,
            bit_count: u8, chunks: Vec<u8>, desc: Vec<SysBitSetDescription>) -> Self {
        SysBitSet {
            name,
            arch,
            device,
            byte_order,
            bit_count,
            chunks,
            rsvd: SysBitSetReserved::MustBeZero,
            show_rsvd: false,
            desc,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SysBitSetError {
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

impl fmt::Display for SysBitSetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str_errkind = match self {
            SysBitSetError::MissingName => "missing name",
            SysBitSetError::MissingArch => "missing architecture",
            SysBitSetError::MissingDevice => "missing device",
            SysBitSetError::UnknownArch => "unknown architecture",
            SysBitSetError::InvalidBitCount => "invalid number of bits",
            SysBitSetError::InvalidChunkIndex => "invalid index in chunks'",
            SysBitSetError::InvalidChunksLength => "invalid number of chunks",
            SysBitSetError::DuplicateChunkIndex => "duplicate index in chunks",
            SysBitSetError::MissingDescription => "missing description",
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

fn validate_sys_bit_set(bits: &SysBitSet) -> Result<(), SysBitSetError> {
    if bits.bit_count > MAX_BITCOUNT {
        // The number of bits must exceeds our limit.
        Err(SysBitSetError::InvalidBitCount)
    } else if bits.chunks.len() >= MAX_BITCOUNT as usize {
        // The chunks index array size exceeds the total number of bits.
        Err(SysBitSetError::InvalidChunksLength)
    } else if !has_unique_elements(&bits.chunks) {
        // The chunks index array contains duplicate indices.
        Err(SysBitSetError::DuplicateChunkIndex)
    } else if bits.desc.is_empty() {
       // None of the bits are described.
       Err(SysBitSetError::MissingDescription)
    } else {
        Ok(())
    }
}

pub fn fmt_as_spaced_binary(val: u64) -> String {
    // Formats the number as binary digits with a space (from the right) for every 4 binary digits.
    let mut vec_bin: Vec<char> = Vec::with_capacity(82);
    let mut val = val;
    let chr_bin_digits = ['0', '1'];
    let mut num_digits = u64::MAX.count_ones() - val.leading_zeros();

    // Push first bit (to avoid extra branch in the loop for not pushing ' ' on 0th iteration).
    vec_bin.push(chr_bin_digits[val.wrapping_rem(2) as usize]);
    val >>= 1;

    // Push remaining bits.
    while num_digits > 1 {
        if (num_digits - 1) % 4 == 0 {
            vec_bin.push(' ');
        }
        vec_bin.push(chr_bin_digits[val.wrapping_rem(2) as usize]);
        val >>= 1;
        num_digits -= 1;
    }

    // Return the vector of binary-digit characters (after reversing for reading LTR) as string.
    vec_bin.iter().rev().collect::<String>()
}

pub fn fmt_binary_ruler(num_bits: u32) -> String {
    // Constructs a binary ruler (for every 8 bits) to ease visual counting of bits.
    // There might be a more efficient way to do this with Rust's string/vector
    // manipulation. But I can't be bothered now, just get something working.
    if num_bits >= 8 {
        let mut str_bin_ruler = String::with_capacity(98);
        let arr_ruler: [&str; 8] = [
            "|  7:0  |",
            "| 15:8  | ",
            "| 23:16 | ",
            "| 31:24 | ",
            "| 39:32 | ",
            "| 47:40 | ",
            "| 55:48 | ",
            "| 63:56 | ",
        ];

        // Ensure if we ever add 128-bit support this code will at least assert.
        debug_assert!(num_bits <= 64);

        // First we need to pad binary digits (with space) at the start when the
        // binary digit does not fall within a full chunk of 8-bits (in arr_ruler).
        // For e.g. "11 1111 1111", we need to pad the first 2 digits (plus 1 space)
        // from the left. We iterate below until we no longer need to pad digits.
        let mut pad_chars = 0;
        for idx in (0..num_bits as usize).rev() {
            if (idx + 1) % 8 != 0 {
                str_bin_ruler.push(' ');
                pad_chars += 1;
                if idx % 4 == 0 {
                    str_bin_ruler.push(' ');
                }
            }
            else {
                break;
            }
        }

        // Iterate over chunks of 8-bits and construct the ruler string.
        for idx in (pad_chars..num_bits as usize).rev().step_by(8) {
            str_bin_ruler.push_str(arr_ruler[((idx + 1) >> 3) - 1]);
        }

        str_bin_ruler
    } else {
        "".to_string()
    }
}

pub fn fmt_sys_bit_set(bits: &SysBitSet) -> Result<String, SysBitSetError> {
    validate_sys_bit_set(bits)?;
    Ok("Testing_Impl".to_string())
}

#[test]
fn test_valid_sys_bit_set() {
    let gen_bits = SysBitSet::new(
        "generic".to_owned(),
        "x86".to_owned(),
        "cpu".to_owned(),
        ByteOrder::LittleEndian,
        64, vec![],
        vec![
            SysBitSetDescription::new(
                Range { start: 0, end: 0 }, SysBitSetKind::Normal,
                "Gen 0".to_owned(),
                "Generic 0".to_owned(),
                "Generic 0 bit enable".to_owned(),
            ),
            SysBitSetDescription::new(
                Range { start: 8, end: 8 }, SysBitSetKind::Normal,
                "Gen 1".to_owned(),
                "Generic 1".to_owned(),
                "Generic 1 bit enable".to_owned(),
            ),
        ]);
    let res_fmt = validate_sys_bit_set(&gen_bits);
    assert!(res_fmt.is_ok());
}

#[test]
fn test_invalid_sys_bit_set() {
    let pair_invalid_bit_sets = [
        //
        // Invalid bit count
        //
        (SysBitSet::new(
            "generic".to_owned(),
            "x86".to_owned(),
            "cpu".to_owned(),
            ByteOrder::LittleEndian,
            128,
            vec![],
            vec![
                SysBitSetDescription::new(
                    Range { start: 0, end: 0 }, SysBitSetKind::Normal,
                    "Gen 0".to_owned(), "Gen 0".to_owned(), "Gen 0".to_owned(),
                ),
                SysBitSetDescription::new(
                    Range { start: 8, end: 8 }, SysBitSetKind::Normal,
                    "Gen 1".to_owned(), "Gen 1".to_owned(), "Gen 1".to_owned(),
                ),
            ],
        ),
        SysBitSetError::InvalidBitCount),

        //
        // TODO: Invalid chunk index
        //
    ];

    for bs in &pair_invalid_bit_sets {
        let res_fmt = validate_sys_bit_set(&bs.0);
        assert!(res_fmt.is_err(), "{:?}", bs.0);
        assert_eq!(res_fmt.err().unwrap(), bs.1);
    }
}

