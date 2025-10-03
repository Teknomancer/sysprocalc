use crate::{BitRange, BitRangeKind, RegisterDescriptor, RegisterDescriptorError, MAX_BIT_COUNT, ByteOrder};
use std::ops::RangeInclusive;
use std::borrow::Cow;

#[test]
fn test_valid_register_descriptor() {
    let res = RegisterDescriptor::new(
        Cow::from("x86"),
        Cow::from("cpu"),
        Cow::from("generic"),
        Cow::from("description"),
        u64::BITS as usize,
        ByteOrder::LittleEndian,
        vec![
            BitRange::new(
                RangeInclusive::new(0, 0),
                BitRangeKind::Normal,
                true,
                Cow::from("Gen 0"),
                Cow::from("Generic 0"),
                Cow::from("Generic Bit 0"),
            ),
            BitRange::new(
                RangeInclusive::new(8, 8),
                BitRangeKind::Normal,
                true,
                Cow::from("Gen 1"),
                Cow::from("Generic 1"),
                Cow::from("Generic Bit 1"),
            ),
        ]
    );
    assert!(res.is_ok());
}

#[test]
fn test_invalid_register_descriptor() {
    // Overlapping bit ranges (0..5) and (5..7)
    let res = RegisterDescriptor::new(
        Cow::from("x86"),
        Cow::from("cpu"),
        Cow::from("generic"),
        Cow::from("description"),
        u64::BITS as usize,
        ByteOrder::LittleEndian,
        vec![
            BitRange::new(
                RangeInclusive::new(0, 5),
                BitRangeKind::Normal,
                true,
                Cow::from("Inv 0"),
                Cow::from("Inv 0"),
                Cow::from("Inv Bit 0"),
            ),
            BitRange::new(
                RangeInclusive::new(5, 7),
                BitRangeKind::Normal,
                true,
                Cow::from("Inv 1"),
                Cow::from("Inv 1"),
                Cow::from("Inv Bit 1"),
            ),
        ],
    );
    assert_eq!(res.unwrap_err(), RegisterDescriptorError::OverlappingBitRange);

    // Overlapping bit ranges (63..63) and (32..63)
    let res = RegisterDescriptor::new(
       Cow::from("x86"),
       Cow::from("cpu"),
       Cow::from("generic"),
       Cow::from("description"),
       u64::BITS as usize,
       ByteOrder::LittleEndian,
       vec![
           BitRange::new(
               RangeInclusive::new(63, 63),
               BitRangeKind::Normal,
               true,
               Cow::from("Inv 1"),
               Cow::from("Inv 1"),
               Cow::from("Inv Bit 1"),
           ),
           BitRange::new(
               RangeInclusive::new(32, 63),
               BitRangeKind::Normal,
               true,
               Cow::from("Inv 0"),
               Cow::from("Inv 0"),
               Cow::from("Inv Bit 0"),
           ),
       ],
    );
    assert_eq!(res.unwrap_err(), RegisterDescriptorError::OverlappingBitRange);

    // Invalid bit range (1..0)
    let res = RegisterDescriptor::new(
        Cow::from("x86"),
        Cow::from("cpu"),
        Cow::from("generic"),
        Cow::from("description"),
        u64::BITS as usize,
        ByteOrder::LittleEndian,
        vec![
            BitRange::new(
                RangeInclusive::new(1, 0),
                BitRangeKind::Normal,
                true,
                Cow::from("Inv 0"),
                Cow::from("Inv 0"),
                Cow::from("Inv Bit 0"),
            ),
        ],
    );
    assert_eq!(res.unwrap_err(), RegisterDescriptorError::InvalidBitRange);

    // Invalid bit range (0..MAX_BIT_COUNT)
    let res = RegisterDescriptor::new(
        Cow::from("x86"),
        Cow::from("cpu"),
        Cow::from("generic"),
        Cow::from("description"),
        u64::BITS as usize,
        ByteOrder::LittleEndian,
        vec![
            BitRange::new(
                RangeInclusive::new(0, MAX_BIT_COUNT),
                BitRangeKind::Normal,
                true,
                Cow::from("Inv 0"),
                Cow::from("Inv 0"),
                Cow::from("Inv Bit 0"),
            ),
        ],
    );
    assert_eq!(res.unwrap_err(), RegisterDescriptorError::InvalidBitRange);

    // Invalid bit range (bit_count+1..bit_count+2)
    let res = RegisterDescriptor::new(
        Cow::from("x86"),
        Cow::from("cpu"),
        Cow::from("generic"),
        Cow::from("description"),
        u64::BITS as usize,
        ByteOrder::LittleEndian,
        vec![
            BitRange::new(
                RangeInclusive::new(64, 65),
                BitRangeKind::Normal,
                true,
                Cow::from("Inv 0"),
                Cow::from("Inv 0"),
                Cow::from("Inv Bit 0"),
            ),
        ],
    );
    assert_eq!(res.unwrap_err(), RegisterDescriptorError::InvalidBitRange);

    // Invalid bit range (bit_count+1..0)
    let res = RegisterDescriptor::new(
        Cow::from("x86"),
        Cow::from("cpu"),
        Cow::from("generic"),
        Cow::from("description"),
        u64::BITS as usize,
        ByteOrder::LittleEndian,
        vec![
            BitRange::new(
                RangeInclusive::new(64, 0),
                BitRangeKind::Normal,
                true,
                Cow::from("Inv 0"),
                Cow::from("Inv 0"),
                Cow::from("Inv Bit 0"),
            ),
        ],
    );
    assert_eq!(res.unwrap_err(), RegisterDescriptorError::InvalidBitRange);

    // Invalid bit range order
    let res = RegisterDescriptor::new(
        Cow::from("x86"),
        Cow::from("cpu"),
        Cow::from("generic"),
        Cow::from("description"),
        u64::BITS as usize,
        ByteOrder::LittleEndian,
        vec![
            BitRange::new(
                RangeInclusive::new(2, 63),
                BitRangeKind::Normal,
                true,
                Cow::from("Inv 0"),
                Cow::from("Inv 0"),
                Cow::from("Inv Bit 0"),
            ),
            BitRange::new(
                RangeInclusive::new(0, 1),
                BitRangeKind::Normal,
                true,
                Cow::from("Inv 0"),
                Cow::from("Inv 0"),
                Cow::from("Inv Bit 0"),
            ),
        ],
    );
    assert_eq!(res.unwrap_err(), RegisterDescriptorError::InvalidBitRangeOrder);

    // Invalid bit range order
    let res = RegisterDescriptor::new(
        Cow::from("x86"),
        Cow::from("cpu"),
        Cow::from("generic"),
        Cow::from("description"),
        u64::BITS as usize,
        ByteOrder::LittleEndian,
        vec![
            BitRange::new(
                RangeInclusive::new(0, 0),
                BitRangeKind::Normal,
                true,
                Cow::from("Inv 0"),
                Cow::from("Inv 0"),
                Cow::from("Inv Bit 0"),
            ),
            BitRange::new(
                RangeInclusive::new(33, 63),
                BitRangeKind::Normal,
                true,
                Cow::from("Inv 0"),
                Cow::from("Inv 0"),
                Cow::from("Inv Bit 0"),
            ),
            BitRange::new(
                RangeInclusive::new(1, 32),
                BitRangeKind::Normal,
                true,
                Cow::from("Inv 0"),
                Cow::from("Inv 0"),
                Cow::from("Inv Bit 0"),
            ),
        ],
    );
    assert_eq!(res.unwrap_err(), RegisterDescriptorError::InvalidBitRangeOrder);

    // Missing bit range
    let res = RegisterDescriptor::new(
        Cow::from("x86"),
        Cow::from("cpu"),
        Cow::from("generic"),
        Cow::from("description"),
        u64::BITS as usize,
        ByteOrder::LittleEndian,
        vec![],
    );
    assert_eq!(res.unwrap_err(), RegisterDescriptorError::MissingBitRanges);

    // Missing arch
    let res = RegisterDescriptor::new(
        Cow::from(""),
        Cow::from("cpu"),
        Cow::from("generic"),
        Cow::from("description"),
        u64::BITS as usize,
        ByteOrder::LittleEndian,
        vec![
            BitRange::new(
                RangeInclusive::new(0, 5),
                BitRangeKind::Normal,
                true,
                Cow::from("Inv 0"),
                Cow::from("Inv 0"),
                Cow::from("Inv Bit 0"),
            ),
        ]
    );
    assert_eq!(res.unwrap_err(), RegisterDescriptorError::MissingArch);

    // Missing device
    let res = RegisterDescriptor::new(
        Cow::from("x86"),
        Cow::from(""),
        Cow::from("generic"),
        Cow::from("description"),
        u64::BITS as usize,
        ByteOrder::LittleEndian,
        vec![
            BitRange::new(
                RangeInclusive::new(0, 5),
                BitRangeKind::Normal,
                true,
                Cow::from("Inv 0"),
                Cow::from("Inv 0"),
                Cow::from("Inv Bit 0"),
            ),
        ]
    );
    assert_eq!(res.unwrap_err(), RegisterDescriptorError::MissingDevice);

    // Missing name
    let res = RegisterDescriptor::new(
        Cow::from("x86"),
        Cow::from("cpu"),
        Cow::from(""),
        Cow::from("description"),
        u64::BITS as usize,
        ByteOrder::LittleEndian,
        vec![
            BitRange::new(
                RangeInclusive::new(0, 5),
                BitRangeKind::Normal,
                true,
                Cow::from("Inv 0"),
                Cow::from("Inv 0"),
                Cow::from("Inv Bit 0"),
            ),
        ]
    );
    assert_eq!(res.unwrap_err(), RegisterDescriptorError::MissingName);

    // Missing description
    let res = RegisterDescriptor::new(
        Cow::from("x86"),
        Cow::from("cpu"),
        Cow::from("generic"),
        Cow::from(""),
        u64::BITS as usize,
        ByteOrder::LittleEndian,
        vec![
            BitRange::new(
                RangeInclusive::new(0, 5),
                BitRangeKind::Normal,
                true,
                Cow::from("Inv 0"),
                Cow::from("Inv 0"),
                Cow::from("Inv Bit 0"),
            ),
        ]
    );
    assert_eq!(res.unwrap_err(), RegisterDescriptorError::MissingDescription);
}

