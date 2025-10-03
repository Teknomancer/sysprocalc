use std::ops::RangeInclusive;
use std::borrow::Cow;
use serde::Deserialize;

#[derive(Copy, Clone, Deserialize, Debug, PartialEq)]
pub enum ByteOrder {
    LittleEndian,
    BigEndian,
}

#[derive(Deserialize, Debug, PartialEq)]
pub enum BitRangeKind {
    Normal,
    ReservedMustBeZero,
    ReservedMustBeOne,
    ReservedUndefined,
    ReservedIgnored,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(bound(deserialize = "'de: 'static"))]
pub struct BitRange<'a> {
    #[serde(flatten)]
    pub span: RangeInclusive<usize>,
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
        span: RangeInclusive<usize>,
        kind: BitRangeKind,
        show: bool,
        name: Cow<'a, str>,
        short: Cow<'a, str>,
        long: Cow<'a, str>
    ) -> Self {
        Self { span, kind, show, name, short, long }
    }
}
