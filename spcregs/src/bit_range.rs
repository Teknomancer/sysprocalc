use serde::Deserialize;
use std::borrow::Cow;

#[derive(Deserialize, Debug, PartialEq, Copy, Clone, Ord, PartialOrd, Eq)]
pub enum ByteOrder {
    LittleEndian,
    BigEndian,
}

#[zerovec::make_ule(BitRangeKindULE)]
#[derive(Deserialize, Debug, PartialEq, Copy, Clone, Ord, PartialOrd, Eq)]
#[repr(u8)]
pub enum BitRangeKind {
    Normal = 0,
    ReservedMustBeZero = 1,
    ReservedMustBeOne = 2,
    ReservedUndefined = 3,
    ReservedIgnored = 4,
}

// We roll our own RangeInclusive style struct here because
// we cannot implement a trait for a struct that lives outside our crate
// and zerovec does not implement the necessary traits for RangeInclusive
#[zerovec::make_ule(ULE)]
#[derive(Deserialize, Debug, PartialEq, Copy, Clone, Ord, PartialOrd, Eq)]
pub struct BitSpan {
    pub first: u16,
    pub last: u16,
}

impl BitSpan {
    pub fn new(first: u16, last: u16) -> Self {
        Self { first, last }
    }

    pub fn contains(&self, item: &u16) -> bool {
        item >= &self.first && item <= &self.last
    }

    pub fn is_empty(&self) -> bool {
        self.last < self.first
    }
}

impl<'z> zerofrom::ZeroFrom<'z, BitRange<'_>> for BitRange<'z> {
    fn zero_from(other: &'z BitRange<'_>) -> Self {
        BitRange {
            span: other.span,
            kind: other.kind,
            show: other.show,
            name: Cow::Borrowed(&other.name),
            short: Cow::Borrowed(&other.short),
            long: Cow::Borrowed(&other.long),
        }
    }
}

#[zerovec::make_varule(BitRangeULE)]
#[zerovec::derive(Deserialize)]
#[derive(Deserialize, Debug, PartialEq, Clone, Ord, PartialOrd, Eq)]
pub struct BitRange<'a> {
    #[serde(flatten)]
    pub span: BitSpan,
    pub kind: BitRangeKind,
    pub show: bool,
    #[serde(borrow)]
    pub name: Cow<'a, str>,
    #[serde(borrow)]
    pub short: Cow<'a, str>,
    #[serde(borrow)]
    pub long: Cow<'a, str>,
}

impl<'a> BitRange<'a> {
    pub fn new(
        span: BitSpan,
        kind: BitRangeKind,
        show: bool,
        name: Cow<'a, str>,
        short: Cow<'a, str>,
        long: Cow<'a, str>,
    ) -> Self {
        Self { span, kind, show, name, short, long }
    }
}
