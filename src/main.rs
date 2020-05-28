// SysProCalc - System Programmer's Calculator, Library.
// Copyright (C) 2020 Ramshankar (aka Teknomancer)
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

extern crate spceval;
use std::io::{self, Write};
use std::collections::VecDeque;

#[cfg(debug_assertions)]
mod logger;

fn print_number_result(number: &spceval::Number) {
    // Format as hex
    let str_hex_zfill = format!("{:#016x}", number.integer);
    let str_hex = format!("{:#x}", number.integer);

    // Format as octal
    let str_oct_zfill = format!("{:#022o}", number.integer);
    let str_oct = format!("{:#o}", number.integer);

    // Format as binary
    // We want a space (from the right) for every 4 binary digits.
    let str_bin = format!("{:b}", number.integer);
    let len_str_bin = str_bin.len();
    let mut queue_bin: VecDeque<char> = VecDeque::with_capacity(128);
    for (idx, chr) in str_bin.chars().rev().enumerate() {
        if idx > 0 && idx % 4 == 0 {
            queue_bin.push_front(' ');
        }
        queue_bin.push_front(chr);
    }
    let str_bin_sfill = queue_bin.iter().collect::<String>();

    // Display the formatted strings
    println!("Dec: {:>24} (u64)  {:>26} (n)", number.integer, number.float);
    println!("Hex: {:>24} (u64)  {:>26} (n)", str_hex_zfill, str_hex);
    println!("Oct: {:>24} (u64)  {:>26} (n)", str_oct_zfill, str_oct);
    println!("Bin: {} ({} bits)", str_bin_sfill, len_str_bin);

    // We want a binary ruler (for every 8 bits) to ease visual counting of bits.
    // There might be a more efficient way to do this with Rust's string/vector
    // manipulation. But I can't be bothered now, just get something working.
    if len_str_bin >= 8 {
        let mut str_bin_ruler = String::with_capacity(96);
        let arr_ruler: [&str; 8] = [
            "|  7:0  |",
            "| 15:8  | ",
            "| 23:16 | ",
            "| 31:24 | ",
            "| 39:32 | ",
            "| 47:40 | ",
            "| 55:48 | ",
            "| 63:56 | ",
        ];

        // Ensure if we ever add 128-bit support this code will at least assert.
        debug_assert!(len_str_bin <= 64);

        // Construct the binary ruler
        // First we need to pad binary digits (with space) at the start when the
        // binary digit does not fall within a full chunk of 8-bits (in arr_ruler).
        // For e.g. "11 1111 1111", we need to pad the first 2 digits (plus 1 space)
        // from the left. We iterate below until we no longer need to pad digits.
        let mut pad_chars = 0;
        for idx in (0..len_str_bin).rev() {
            if (idx + 1) % 8 != 0 {
                str_bin_ruler.push(' ');
                pad_chars += 1;
                if idx % 4 == 0 {
                    str_bin_ruler.push(' ');
                }
            }
            else {
                break;
            }
        }

        // Next iterate over chunks of 8-bits and construct the ruler string.
        for idx in (pad_chars..len_str_bin).rev().step_by(8) {
            str_bin_ruler.push_str(arr_ruler[((idx + 1) >> 3) - 1]);
        }

        // Display the binary ruler.
        println!("     {}", str_bin_ruler);
    }
}

fn main() -> std::io::Result<()> {
    // Initialize the logger only on debug builds
    #[cfg(debug_assertions)]
    if let Err(e) = logger::init() {
        println!("error initializing logger: {:?}", e)
    }

    let mut stdout = io::stdout();
    loop {
        // print! macro has buffered behavior. Therefore, manually write and flush stdout so we
        // have a prompt on the same line that the user can input text.
        const BSTR_PROMPT: &[u8; 2] = b"> ";
        stdout.write_all(BSTR_PROMPT)?;
        stdout.flush()?;

        let mut str_input = String::new();
        if let Err(e) = io::stdin().read_line(&mut str_input) {
            println!("Error: {}", e);
            return Err(e)
        }

        // Get a slice to the input string after trimming trailing newlines.
        // Needs to work on Windows (CR/LF), Linux (LF) and macOS (CR).
        let str_expr = str_input.trim_end_matches(&['\r', '\n'][..]);

        // Handle application commands.
        match str_expr {
            "q" | "quit" | "exit" => return Ok(()),
            _ => (),
        }

        // If there's no expression, don't bother trying to evaluate it.
        // Otherwise, the evaluator will return a missing expression error.
        if !str_expr.is_empty() {
            // Parse the expression.
            let res_parse = spceval::parse(&str_expr);
            if let Err(e) = res_parse {
                println!("{:width$}^", " ", width = e.idx_expr + BSTR_PROMPT.len());
                println!("{:width$}Error: {errdesc}", " ", width = BSTR_PROMPT.len(), errdesc = e);
                continue;
            }

            // Evaluate the parsed expression.
            let mut expr_ctx = res_parse.unwrap();
            let res_eval = spceval::evaluate(&mut expr_ctx);
            if let Err(e) = res_eval {
                println!("{:width$}^", " ", width = e.idx_expr + BSTR_PROMPT.len());
                println!("{:width$}Error: {errdesc}", " ", width = BSTR_PROMPT.len(), errdesc = e);
                continue;
            }

            // Handle the result of the expression evaluation.
            let res_expr = res_eval.unwrap();
            match res_expr {
                spceval::ExprResult::Number(n) => print_number_result(&n),
                spceval::ExprResult::Command(c) => println!("Result: {}", c),
            }

            // An empty line so the output doesn't look too cramped with the next input.
            println!("");
        }
    }
}

