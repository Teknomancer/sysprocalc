use std::ops::RangeInclusive;
use super::{MAX_BITCOUNT, validate_bit_group, BitSpan, BitSpanKind, BitGroup, BitGroupError, ByteOrder};

#[test]
fn test_valid_bit_group() {
    let gen_bits = BitGroup::new(
        String::from("generic"),
        String::from("x86"),
        String::from("cpu"),
        String::from("description"),
        ByteOrder::LittleEndian,
        MAX_BITCOUNT,
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
    let res_fmt = validate_bit_group(&gen_bits);
    assert!(res_fmt.is_ok());
}

#[test]
fn test_invalid_bit_group() {
    let pair_invalid_bit_grps = [
        //
        // Invalid bit count (MAX_BITCOUNT+1)
        //
        (BitGroup::new(
            String::from("generic"),
            String::from("x86"),
            String::from("cpu"),
            String::from("description"),
            ByteOrder::LittleEndian,
            MAX_BITCOUNT + 1,
            vec![
                BitSpan::new(
                    RangeInclusive::new(0, 0),
                    BitSpanKind::Normal,
                    false,
                    String::from("Inv 0"),
                    String::from("Inv 0"),
                    String::from("Inv Bit 0"),
                ),
            ],
        ),
        BitGroupError::InvalidBitCount),

        //
        // Overlapping bit ranges (0..5) and (5..7)
        //
        (BitGroup::new(
            String::from("generic"),
            String::from("x86"),
            String::from("cpu"),
            String::from("description"),
            ByteOrder::LittleEndian,
            MAX_BITCOUNT,
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
        (BitGroup::new(
            String::from("generic"),
            String::from("x86"),
            String::from("cpu"),
            String::from("description"),
            ByteOrder::LittleEndian,
            MAX_BITCOUNT,
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
        (BitGroup::new(
            String::from("generic"),
            String::from("x86"),
            String::from("cpu"),
            String::from("description"),
            ByteOrder::LittleEndian,
            MAX_BITCOUNT,
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
        (BitGroup::new(
            String::from("generic"),
            String::from("x86"),
            String::from("cpu"),
            String::from("description"),
            ByteOrder::LittleEndian,
            MAX_BITCOUNT,
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
        // Invalid bit range (bit_count+1..bit_count+1)
        //
        (BitGroup::new(
            String::from("generic"),
            String::from("x86"),
            String::from("cpu"),
            String::from("description"),
            ByteOrder::LittleEndian,
            32,
            vec![
                BitSpan::new(
                    RangeInclusive::new(33, 33),
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
        (BitGroup::new(
            String::from("generic"),
            String::from("x86"),
            String::from("cpu"),
            String::from("description"),
            ByteOrder::LittleEndian,
            32,
            vec![
                BitSpan::new(
                    RangeInclusive::new(32, 0),
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

    for bs in &pair_invalid_bit_grps {
        let res_fmt = validate_bit_group(&bs.0);
        assert!(res_fmt.is_err(), "{:?}", bs.0);
        assert_eq!(res_fmt.err().unwrap(), bs.1);
    }
}

