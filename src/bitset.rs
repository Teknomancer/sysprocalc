use std::ops::Range;

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
}

pub fn format_bitset(bits: &BitSet) -> Result<String, BitSetError> {
    Err(BitSetError::MissingName)
}

