use std::ops::RangeInclusive;
use serde::Deserialize;

#[derive(Copy, Clone, Deserialize, Debug, PartialEq)]
pub enum ByteOrder {
    LittleEndian,
    BigEndian,
}

#[derive(Deserialize, Debug)]
pub enum BitRangeKind {
    Normal,
    ReservedMustBeZero,
    ReservedMustBeOne,
    ReservedUndefined,
    ReservedIgnored,
}

#[derive(Deserialize, Debug)]
pub struct BitRange {
    #[serde(flatten)]
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

