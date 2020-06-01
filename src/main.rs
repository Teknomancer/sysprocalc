use spceval::*;
use termcolor::*;
use rustyline::Editor;
use std::env;
use std::io::Write;
use std::collections::VecDeque;

#[cfg(debug_assertions)]
mod logger;

#[cfg(debug_assertions)]
const ERR_INIT_LOGGER: &str = "Error initializing logger:";

const USER_PROMPT: &str = "> ";
const DEC_RADIX: &str = "Dec:";
const HEX_RADIX: &str = "Hex:";
const OCT_RADIX: &str = "Oct:";
const BIN_RADIX: &str = "Bin:";
const EXITING_APP: &str = "Exiting:";
const BITS_PLURAL: &str = "bits";
const BIT_SINGULAR: &str = "bit";

fn write_color(stream: &mut StandardStream, s: &str, col: Color, is_intense: bool) -> std::io::Result<()> {
    stream.set_color(ColorSpec::new()
                     .set_fg(Some(col))
                     .set_intense(is_intense))?;
    write!(stream, "{}", s)?;
    stream.reset()?;
    Ok(())
}

fn print_number_result(stream: &mut StandardStream, number: &spceval::Number) -> std::io::Result<()> {
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
    write_color(stream, DEC_RADIX, Color::Cyan, true)?;
    writeln!(stream, " {:>24} (u64)  {:>26} (f)", number.integer, number.float)?;
    write_color(stream, HEX_RADIX, Color::Cyan, true)?;
    writeln!(stream, " {:>24} (u64)  {:>26} (n)", str_hex_zfill, str_hex)?;
    write_color(stream, OCT_RADIX, Color::Cyan, true)?;
    writeln!(stream, " {:>24} (u64)  {:>26} (n)", str_oct_zfill, str_oct)?;
    write_color(stream, BIN_RADIX, Color::Cyan, true)?;
    let str_bits = if len_str_bin > 1 { BITS_PLURAL } else { BIT_SINGULAR };
    writeln!(stream, " {} ({} {})", str_bin_sfill, len_str_bin, str_bits)?;

    // Construct a binary ruler (for every 8 bits) to ease visual counting of bits.
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

        // Iterate over chunks of 8-bits and construct the ruler string.
        for idx in (pad_chars..len_str_bin).rev().step_by(8) {
            str_bin_ruler.push_str(arr_ruler[((idx + 1) >> 3) - 1]);
        }

        // Display the binary ruler.
        writeln!(stream, "     {}", str_bin_ruler)?;
    }

    Ok(())
}

fn print_error(stream: &mut StandardStream, err: ExprError) -> std::io::Result<()> {
    write!(stream, "{:width$}", " ", width = err.idx_expr + USER_PROMPT.len())?;
    write_color(stream, "^", Color::Red, true)?;
    writeln!(stream)?;

    write!(stream, "{:width$}", " ", width = USER_PROMPT.len())?;
    write_color(stream, "Error:", Color::Red, true)?;
    writeln!(stream, " {}", err)?;
    writeln!(stream)?;

    Ok(())
}

fn parse_and_eval_expr_internal(stream: &mut StandardStream, str_expr: &str) -> std::io::Result<()> {
    let res_parse = spceval::parse(&str_expr);
    if let Err(e) = res_parse {
        print_error(stream, e)?;
        return Ok(());
    }

    let mut expr_ctx = res_parse.unwrap();
    let res_eval = spceval::evaluate(&mut expr_ctx);
    if let Err(e) = res_eval {
        print_error(stream, e)?;
        return Ok(());
    }

    let res_expr = res_eval.unwrap();
    match res_expr {
        spceval::ExprResult::Number(n) => print_number_result(stream, &n)?,
        spceval::ExprResult::Command(c) => println!("Result: {}", c),
    }

    writeln!(stream)?;
    Ok(())
}

#[inline(always)]
fn parse_and_eval_expr(stream: &mut StandardStream, str_expr: &str) -> std::io::Result<()> {
    // Enable trace level logging while parsing and evaluating using spceval.
    #[cfg(debug_assertions)]
    log::set_max_level(log::LevelFilter::Trace);

    parse_and_eval_expr_internal(stream, str_expr)?;

    // Disable logging.
    #[cfg(debug_assertions)]
    log::set_max_level(log::LevelFilter::Off);
    Ok(())
}

fn main() -> std::io::Result<()> {
    // Create a logger but keep logging disabled to shut up rustyline's logging.
    // Need to find a way to disable rustyline's logger at compile time...
    #[cfg(debug_assertions)]
    if let Err(e) = logger::init(log::LevelFilter::Off) {
        println!("{} {:?}", ERR_INIT_LOGGER, e);
    }

    // Detect presence of a terminal to determine use of color output.
    // Later allow forcing a color choice via command-line arguments.
    let color_choice = if atty::is(atty::Stream::Stdout) {
        ColorChoice::Auto
    } else {
        ColorChoice::Never
    };

    let mut stdout = StandardStream::stdout(color_choice);
    let args : Vec<String> = env::args().collect();

    // Command-line mode, evaluate and quit.
    if args.len() > 1 {
        parse_and_eval_expr(&mut stdout, args.get(1).unwrap())?;
        Ok(())
    } else {
        // Interactive mode.
        let mut line_editor = Editor::<()>::new();
        loop {
            let res_readline = line_editor.readline(USER_PROMPT);
            if let Ok(str_input) = res_readline {
                let str_expr = str_input.as_str();
                line_editor.add_history_entry(str_expr);

                if !str_expr.is_empty() {
                    match str_expr {
                        "q" | "quit" | "exit" => return Ok(()),
                        _ => (),
                    }
                    parse_and_eval_expr(&mut stdout, str_expr)?;
                }
            } else {
                let mut stderr = StandardStream::stderr(color_choice);
                write_color(&mut stderr, EXITING_APP, Color::Red, true)?;
                writeln!(&mut stderr, " {:?}", res_readline.err().unwrap())?;
                return Ok(());
            }
        }
    }
}

