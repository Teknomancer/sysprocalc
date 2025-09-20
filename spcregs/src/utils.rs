pub fn get_binary_string(val: u64, fixed_bit_width: Option<u32>) -> String {
    // Formats the number as binary digits with a space (from the right) for every 4 binary digits.
    static BIN_DIGITS: [char; 2] = ['0', '1'];
    let mut bin_out: Vec<char> = Vec::with_capacity(88);
    let mut val = val;

    let bit_width = match fixed_bit_width {
        Some(fixed_bit_width) if fixed_bit_width <= u64::BITS
            => fixed_bit_width,
        _   => u64::MAX.count_ones() - val.leading_zeros(),
    };

    // Push first bit (to avoid extra branch in the loop for not pushing ' ' on 0th iteration).
    bin_out.push(BIN_DIGITS[val.wrapping_rem(2) as usize]);
    val >>= 1;

    // Push remaining bits.
    for idx in 1..bit_width {
        if idx % 4 == 0 {
            bin_out.push(' ');
        }
        bin_out.push(BIN_DIGITS[val.wrapping_rem(2) as usize]);
        val >>= 1;
    }

    // Return the vector of binary-digit characters (after reversing for reading LTR) as string.
    bin_out.iter().rev().collect::<String>()
}

pub fn get_binary_ruler_string(bit_count: u8) -> String {
    // Makes a binary ruler (for every 8 bits) to ease visual counting of bits.
    // There might be a more efficient way to do this with Rust's string/vector
    // manipulation. But I can't be bothered now, just get something working.

    // First convert the u32 to usize since we need a usize to index into an array.
    // This is will panic if the conversion fails (on architectures where usize
    // is insufficient to hold 32 bits). Panic is better than failing in weird ways.
    let bit_count = usize::from(bit_count);

    // Ensure if we ever add 128-bit support this code will at least assert.
    debug_assert!(bit_count <= 64);

    if bit_count >= 8 {
        let mut ruler_out = String::with_capacity(88);
        static BIN_RULER: [&str; 8] = [
            "|  7:0  |",
            "| 15:8  | ",
            "| 23:16 | ",
            "| 31:24 | ",
            "| 39:32 | ",
            "| 47:40 | ",
            "| 55:48 | ",
            "| 63:56 | ",
        ];

        // First we need to pad with spaces at the start for those binary digits
        // that do not fall within a chunk of 8-bits (see BIN_RULER).
        // For e.g. "10 1111 1111", we need to pad the first 2 digits (and 1 space)
        // from the left. We iterate below until we no longer need to pad with spaces
        // prior to the start of the ruler.
        // TODO: I'm sure this can be optimized, but no time now.
        let num_pad_bits = bit_count % 8;
        for idx in 0..num_pad_bits {
            ruler_out.push(' ');
            if idx % 4 == 0 {
                ruler_out.push(' ');
            }
        }

        // Iterate over chunks of 8-bits and make the ruler
        for idx in (num_pad_bits..bit_count).rev().step_by(8) {
            ruler_out.push_str(BIN_RULER[((idx + 1) >> 3) - 1]);
        }

        ruler_out
    } else {
        "".to_string()
    }
}

