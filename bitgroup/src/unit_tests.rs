use crate::{BitRange, BitRangeKind, RegisterDescriptor, RegisterDescriptorError, Register, MAX_BIT_COUNT, ByteOrder};
use std::ops::RangeInclusive;

#[test]
fn test_valid_bit_group() {
    let gen_bits = RegisterDescriptor::new(
        String::from("x86"),
        String::from("cpu"),
        String::from("generic"),
        String::from("description"),
        u64::BITS as usize,
        ByteOrder::LittleEndian,
        vec![
            BitRange::new(
                RangeInclusive::new(0, 0),
                BitRangeKind::Normal,
                true,
                String::from("Gen 0"),
                String::from("Generic 0"),
                String::from("Generic Bit 0"),
            ),
            BitRange::new(
                RangeInclusive::new(8, 8),
                BitRangeKind::Normal,
                true,
                String::from("Gen 1"),
                String::from("Generic 1"),
                String::from("Generic Bit 1"),
            ),
        ]
    );
    let res_fmt = gen_bits.validate();
    assert!(res_fmt.is_ok());
}

#[test]
fn test_invalid_bit_group() {
    let pair_invalid_64 = [
        // Overlapping bit ranges (0..5) and (5..7)
        (RegisterDescriptor::new(
            String::from("x86"),
            String::from("cpu"),
            String::from("generic"),
            String::from("description"),
            u64::BITS as usize,
            ByteOrder::LittleEndian,
            vec![
                BitRange::new(
                    RangeInclusive::new(0, 5),
                    BitRangeKind::Normal,
                    true,
                    String::from("Inv 0"),
                    String::from("Inv 0"),
                    String::from("Inv Bit 0"),
                ),
                BitRange::new(
                    RangeInclusive::new(5, 7),
                    BitRangeKind::Normal,
                    true,
                    String::from("Inv 1"),
                    String::from("Inv 1"),
                    String::from("Inv Bit 1"),
                ),
            ],
        ),
        RegisterDescriptorError::OverlappingBitRange),

        // Overlapping bit ranges (63..63) and (32..63)
        (RegisterDescriptor::new(
            String::from("x86"),
            String::from("cpu"),
            String::from("generic"),
            String::from("description"),
            u64::BITS as usize,
            ByteOrder::LittleEndian,
            vec![
                BitRange::new(
                    RangeInclusive::new(63, 63),
                    BitRangeKind::Normal,
                    true,
                    String::from("Inv 1"),
                    String::from("Inv 1"),
                    String::from("Inv Bit 1"),
                ),
                BitRange::new(
                    RangeInclusive::new(32, 63),
                    BitRangeKind::Normal,
                    true,
                    String::from("Inv 0"),
                    String::from("Inv 0"),
                    String::from("Inv Bit 0"),
                ),
            ],
        ),
        RegisterDescriptorError::OverlappingBitRange),

        // Invalid bit range (1..0)
        (RegisterDescriptor::new(
            String::from("x86"),
            String::from("cpu"),
            String::from("generic"),
            String::from("description"),
            u64::BITS as usize,
            ByteOrder::LittleEndian,
            vec![
                BitRange::new(
                    RangeInclusive::new(1, 0),
                    BitRangeKind::Normal,
                    true,
                    String::from("Inv 0"),
                    String::from("Inv 0"),
                    String::from("Inv Bit 0"),
                ),
            ],
        ),
        RegisterDescriptorError::InvalidBitRange),

        // Invalid bit range (0..MAX_BIT_COUNT)
        (RegisterDescriptor::new(
            String::from("x86"),
            String::from("cpu"),
            String::from("generic"),
            String::from("description"),
            u64::BITS as usize,
            ByteOrder::LittleEndian,
            vec![
                BitRange::new(
                    RangeInclusive::new(0, MAX_BIT_COUNT),
                    BitRangeKind::Normal,
                    true,
                    String::from("Inv 0"),
                    String::from("Inv 0"),
                    String::from("Inv Bit 0"),
                ),
            ],
        ),
        RegisterDescriptorError::InvalidBitRange),

        // Invalid bit range (bit_count+1..bit_count+2)
        (RegisterDescriptor::new(
            String::from("x86"),
            String::from("cpu"),
            String::from("generic"),
            String::from("description"),
            u64::BITS as usize,
            ByteOrder::LittleEndian,
            vec![
                BitRange::new(
                    RangeInclusive::new(64, 65),
                    BitRangeKind::Normal,
                    true,
                    String::from("Inv 0"),
                    String::from("Inv 0"),
                    String::from("Inv Bit 0"),
                ),
            ],
        ),
        RegisterDescriptorError::InvalidBitRange),

        // Invalid bit range (bit_count+1..0)
        (RegisterDescriptor::new(
            String::from("x86"),
            String::from("cpu"),
            String::from("generic"),
            String::from("description"),
            u64::BITS as usize,
            ByteOrder::LittleEndian,
            vec![
                BitRange::new(
                    RangeInclusive::new(64, 0),
                    BitRangeKind::Normal,
                    true,
                    String::from("Inv 0"),
                    String::from("Inv 0"),
                    String::from("Inv Bit 0"),
                ),
            ],
        ),
        RegisterDescriptorError::InvalidBitRange),
    ];

    for bg in &pair_invalid_64 {
        let res_fmt = bg.0.validate();
        assert!(res_fmt.is_err(), "{:?}", bg.0);
        assert_eq!(res_fmt.err().unwrap(), bg.1);
    }
}

