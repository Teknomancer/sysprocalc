use std::ops::RangeInclusive;

#[derive(Debug)]
pub enum ByteOrder {
    LittleEndian,
    BigEndian,
}

#[derive(Debug)]
pub struct BitRange {
    pub span: RangeInclusive<usize>,
    pub kind: BitRangeKind,
    pub show: bool,
    pub name: String,
    pub short: String,
    pub long: String,
}

impl BitRange {
    pub fn new(
        span: RangeInclusive<usize>,
        kind: BitRangeKind,
        show: bool,
        name: String,
        short: String,
        long: String
    ) -> Self {
        Self { span, kind, show, name, short, long }
    }
}

#[derive(Debug)]
pub enum BitRangeKind {
    Normal,
    ReservedMustBeZero,
    ReservedMustBeOne,
    ReservedUndefined,
    ReservedIgnored,
}

