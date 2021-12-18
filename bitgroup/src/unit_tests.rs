use std::ops::RangeInclusive;
use super::{MAX_BITCOUNT, BitSpan, BitSpanKind, BitGroup, BitGroupError, ByteOrder};

#[test]
fn test_valid_bit_group() {
    let gen_bits: BitGroup<u64> = BitGroup::new(
        String::from("generic"),
        String::from("x86"),
        String::from("cpu"),
        String::from("description"),
        ByteOrder::LittleEndian,
        vec![
            BitSpan::new(
                RangeInclusive::new(0, 0),
                BitSpanKind::Normal,
                false,
                String::from("Gen 0"),
                String::from("Generic 0"),
                String::from("Generic Bit 0"),
            ),
            BitSpan::new(
                RangeInclusive::new(8, 8),
                BitSpanKind::Normal,
                false,
                String::from("Gen 1"),
                String::from("Generic 1"),
                String::from("Generic Bit 1"),
            ),
        ]);
    let res_fmt = gen_bits.validate();
    assert!(res_fmt.is_ok());
}

#[test]
fn test_invalid_bit_group() {
    let pair_invalid_64 = [
        //
        // Overlapping bit ranges (0..5) and (5..7)
        //
        (BitGroup::<u64>::new(
            String::from("generic"),
            String::from("x86"),
            String::from("cpu"),
            String::from("description"),
            ByteOrder::LittleEndian,
            vec![
                BitSpan::new(
                    RangeInclusive::new(0, 5),
                    BitSpanKind::Normal,
                    false,
                    String::from("Inv 0"),
                    String::from("Inv 0"),
                    String::from("Inv Bit 0"),
                ),
                BitSpan::new(
                    RangeInclusive::new(5, 7),
                    BitSpanKind::Normal,
                    false,
                    String::from("Inv 1"),
                    String::from("Inv 1"),
                    String::from("Inv Bit 1"),
                ),
            ],
        ),
        BitGroupError::OverlappingBitRange),

        //
        // Overlapping bit ranges (63..63) and (32..63)
        //
        (BitGroup::<u64>::new(
            String::from("generic"),
            String::from("x86"),
            String::from("cpu"),
            String::from("description"),
            ByteOrder::LittleEndian,
            vec![
                BitSpan::new(
                    RangeInclusive::new(63, 63),
                    BitSpanKind::Normal,
                    false,
                    String::from("Inv 1"),
                    String::from("Inv 1"),
                    String::from("Inv Bit 1"),
                ),
                BitSpan::new(
                    RangeInclusive::new(32, 63),
                    BitSpanKind::Normal,
                    false,
                    String::from("Inv 0"),
                    String::from("Inv 0"),
                    String::from("Inv Bit 0"),
                ),
            ],
        ),
        BitGroupError::OverlappingBitRange),

        //
        // Invalid bit range (1..0)
        //
        (BitGroup::<u64>::new(
            String::from("generic"),
            String::from("x86"),
            String::from("cpu"),
            String::from("description"),
            ByteOrder::LittleEndian,
            vec![
                BitSpan::new(
                    RangeInclusive::new(1, 0),
                    BitSpanKind::Normal,
                    false,
                    String::from("Inv 0"),
                    String::from("Inv 0"),
                    String::from("Inv Bit 0"),
                ),
            ],
        ),
        BitGroupError::InvalidBitRange),

        //
        // Invalid bit range (0..MAX_BITCOUNT)
        //
        (BitGroup::<u64>::new(
            String::from("generic"),
            String::from("x86"),
            String::from("cpu"),
            String::from("description"),
            ByteOrder::LittleEndian,
            vec![
                BitSpan::new(
                    RangeInclusive::new(0, MAX_BITCOUNT),
                    BitSpanKind::Normal,
                    false,
                    String::from("Inv 0"),
                    String::from("Inv 0"),
                    String::from("Inv Bit 0"),
                ),
            ],
        ),
        BitGroupError::InvalidBitRange),

        //
        // Invalid bit range (bit_count+1..bit_count+2)
        //
        (BitGroup::<u64>::new(
            String::from("generic"),
            String::from("x86"),
            String::from("cpu"),
            String::from("description"),
            ByteOrder::LittleEndian,
            vec![
                BitSpan::new(
                    RangeInclusive::new(64, 65),
                    BitSpanKind::Normal,
                    false,
                    String::from("Inv 0"),
                    String::from("Inv 0"),
                    String::from("Inv Bit 0"),
                ),
            ],
        ),
        BitGroupError::InvalidBitRange),

        //
        // Invalid bit range (bit_count+1..0)
        //
        (BitGroup::<u64>::new(
            String::from("generic"),
            String::from("x86"),
            String::from("cpu"),
            String::from("description"),
            ByteOrder::LittleEndian,
            vec![
                BitSpan::new(
                    RangeInclusive::new(64, 0),
                    BitSpanKind::Normal,
                    false,
                    String::from("Inv 0"),
                    String::from("Inv 0"),
                    String::from("Inv Bit 0"),
                ),
            ],
        ),
        BitGroupError::InvalidBitRange),
    ];

    for bg in &pair_invalid_64 {
        let res_fmt = bg.0.validate();
        assert!(res_fmt.is_err(), "{:?}", bg.0);
        assert_eq!(res_fmt.err().unwrap(), bg.1);
    }
}

