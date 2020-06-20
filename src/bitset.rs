use std::ops::Range;
use std::collections::HashSet;
use std::hash::Hash;

static MAX_BITCOUNT: u8 = 64;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ByteOrder {
    LittleEndian,
    BigEndian,
}

pub struct BitSetDescription {
    spans: Range<u8>,
    kind: BitSetKind,
    name: String,
    short: String,
    long: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BitSetReserved {
    MustBeZero,
    MustBeOne,
    Undefined,
    Ignored,
}

pub enum BitSetKind {
    Normal,
    Rsvd(BitSetReserved),
}

pub struct BitSet {
    name: String,
    arch: String,
    device: String,
    byte_order: ByteOrder,
    bit_count: u8,
    chunks: Vec<u8>,
    rsvd: BitSetReserved,
    show_rsvd: bool,
    desc: Vec<BitSetDescription>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BitSetError {
    MissingName,
    MissingArch,
    MissingDevice,
    UnknownArch,
    InvalidBitCount,
    InvalidChunkIndex,
    InvalidChunksLength,
    DuplicateChunkIndex,
}

fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

fn validate_bitset(bits: &BitSet) -> Result<(), BitSetError> {
    if bits.bit_count > MAX_BITCOUNT {
        // The number of bits must exceeds our limit.
        Err(BitSetError::InvalidBitCount)
    } else if bits.chunks.len() >= MAX_BITCOUNT as usize {
        // The chunks index array size exceeds the total number of bits.
        Err(BitSetError::InvalidChunksLength)
    } else if !has_unique_elements(&bits.chunks) {
        // The chunks index array contains duplicate indices.
        return Err(BitSetError::DuplicateChunkIndex)
    } else {
        Ok(())
    }
}

pub fn format_bitset(bits: &BitSet) -> Result<String, BitSetError> {
    validate_bitset(bits)?;
    Err(BitSetError::MissingName)
}

