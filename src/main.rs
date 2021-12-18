use bitgroup::{BitGroup, BitSpan, BitSpanKind, ByteOrder};
use spceval::{Number, ExprError};
use rustyline::Editor;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use std::env;
use std::io::Write;
use std::ops::RangeInclusive;

mod registers;
#[cfg(debug_assertions)]
mod logger;

#[cfg(debug_assertions)]
static ERR_INIT_LOGGER: &str = "Error initializing logger:";

static USER_PROMPT: &str = "> ";
static DEC_RADIX: &str = "Dec:";
static HEX_RADIX: &str = "Hex:";
static OCT_RADIX: &str = "Oct:";
static BIN_RADIX: &str = "Bin:";
static EXITING_APP: &str = "Exiting:";
static BITS_PLURAL: &str = "bits";
static BIT_SINGULAR: &str = "bit";

enum AppMode {
    Interactive,
    NonInteractive,
}

fn write_color(stream: &mut StandardStream, message: &str, col: Color, is_intense: bool) -> std::io::Result<()> {
    stream.set_color(ColorSpec::new()
                     .set_fg(Some(col))
                     .set_intense(is_intense))?;
    write!(stream, "{}", message)?;
    stream.reset()?;
    Ok(())
}

fn print_result(stream: &mut StandardStream, number: &Number) -> std::io::Result<()> {
    // Format as hex
    let str_hex_zfill = format!("{:#018x}", number.integer);
    let str_hex = format!("{:#x}", number.integer);

    // Format as octal
    let str_oct_zfill = format!("{:#022o}", number.integer);
    let str_oct = format!("{:#o}", number.integer);

    // Format as binary
    let str_bin_sfill = bitgroup::get_binary_string(number.integer);
    // Compute number of bits (to make a binary ruler as well as display the number of bits).
    let mut bin_digits = u64::MAX.count_ones() - number.integer.leading_zeros();
    let str_bin_digits;
    if bin_digits < 2 {
        bin_digits = 1; // Required because bin_digits gets computed as 0 when number.integer is 0.
        str_bin_digits = BIT_SINGULAR;
    } else {
        str_bin_digits = BITS_PLURAL;
    };

    // Display the formatted values
    write_color(stream, DEC_RADIX, Color::Cyan, true)?;
    writeln!(stream, " {:>24} (u64)  {:>26} (f)", number.integer, number.float)?;
    write_color(stream, HEX_RADIX, Color::Cyan, true)?;
    writeln!(stream, " {:>24} (u64)  {:>26} (n)", str_hex_zfill, str_hex)?;
    write_color(stream, OCT_RADIX, Color::Cyan, true)?;
    writeln!(stream, " {:>24} (u64)  {:>26} (n)", str_oct_zfill, str_oct)?;
    write_color(stream, BIN_RADIX, Color::Cyan, true)?;
    writeln!(stream, " {} ({} {})", str_bin_sfill, bin_digits, str_bin_digits)?;

    // Display the binary ruler if we have more than 8 bits.
    if bin_digits >= 8 {
        let str_bin_ruler = bitgroup::get_binary_ruler_string(bin_digits as u8);
        writeln!(stream, "     {}", str_bin_ruler)?;
    }

    writeln!(stream)?;
    Ok(())
}

// Get a character index given a byte index in a string.
// This ensures the character index is always at a UTF-8 boundary.
fn byte_index_to_char_index(str_expr: &str, idx_byte: usize) -> usize {
    let mut idx_char = 0;
    for i in 0..idx_byte {
        if str_expr.is_char_boundary(i) {
            idx_char += 1;
        }
    }
    idx_char
}

fn print_error(stream: &mut StandardStream, str_expr: &str, err: ExprError, app_mode: AppMode) -> std::io::Result<()> {
    // Print the caret indicating where in the expression the error occurs in interactive mode.
    if let AppMode::Interactive = app_mode {
        let idx_char = byte_index_to_char_index(str_expr, err.index());
        write!(stream, "{:width$}", " ", width = idx_char + USER_PROMPT.len())?;
        write_color(stream, "^", Color::Red, true)?;
        writeln!(stream)?;

        // Passing width as 0 still produces 1 space, so do the padding here.
        write!(stream, "{:width$}", " ", width = USER_PROMPT.len())?;
    }
    // Print the error.
    write_color(stream, "Error:", Color::Red, true)?;
    writeln!(stream, " {}", err)?;
    writeln!(stream)?;

    Ok(())
}

fn evaluate_expr(stream: &mut StandardStream, str_expr: &str, app_mode: AppMode) -> std::io::Result<()> {
    // Enable trace level logging while parsing and evaluating using spceval.
    #[cfg(debug_assertions)]
    log::set_max_level(log::LevelFilter::Trace);

    match spceval::evaluate(str_expr) {
        Ok(number) => print_result(stream, &number)?,
        Err(e) => print_error(stream, str_expr, e, app_mode)?,
    }

    // Disable logging.
    #[cfg(debug_assertions)]
    log::set_max_level(log::LevelFilter::Off);

    Ok(())
}

fn test_bitgroup_desc(stream: &mut StandardStream) -> std::io::Result<()> {
    let efer_bitspans = vec![
        BitSpan::new(
            RangeInclusive::new(0, 0),
            BitSpanKind::Normal,
            false,
            String::from("SCE"),
            String::from("SysCall Ext."),
            String::from("System Call Extensions"),
        ),
        BitSpan::new(
            RangeInclusive::new(1, 1),
            BitSpanKind::Normal,
            false,
            String::from("LME"),
            String::from("Long mode enable"),
            String::from("Long mode enable"),
        ),
        BitSpan::new(
            RangeInclusive::new(10, 10),
            BitSpanKind::Normal,
            false,
            String::from("LMA"),
            String::from("Long mode active"),
            String::from("Long mode active"),
        ),
        BitSpan::new(
            RangeInclusive::new(11, 11),
            BitSpanKind::Normal,
            false,
            String::from("NXE"),
            String::from("No-execute enable"),
            String::from("No-execute enable"),
        ),
        BitSpan::new(
            RangeInclusive::new(12, 12),
            BitSpanKind::Normal,
            false,
            String::from("SVME"),
            String::from("SVM enable"),
            String::from("Secure virtual machine enable (AMD)"),
        ),
        BitSpan::new(
            RangeInclusive::new(13, 13),
            BitSpanKind::Normal,
            false,
            String::from("LMSL"),
            String::from("LMSL enable"),
            String::from("Long mode segment limit enable (AMD)"),
        ),
        BitSpan::new(
            RangeInclusive::new(14, 14),
            BitSpanKind::Normal,
            false,
            String::from("FFXSR"),
            String::from("Fast FXSAVE/FXRSTOR"),
            String::from("Fast FXSAVE/FXRSTOR"),
        ),
    ];
    let efer: BitGroup<u64> = BitGroup::new(
        String::from("EFER"),
        String::from("x86"),
        String::from("cpu"),
        String::from("Extended Feature Register"),
        ByteOrder::LittleEndian,
        efer_bitspans
    );
    write_color(stream, efer.name(), Color::Cyan, true)?;
    writeln!(stream, " ({})", efer.description())?;
    writeln!(stream, "{}", efer)?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    // Create a logger but keep logging disabled to shut up rustyline's logging.
    // Need to find a way to disable rustyline's logger at compile time...
    #[cfg(debug_assertions)]
    {
        if let Err(e) = logger::init(log::LevelFilter::Off) {
            println!("{} {:?}", ERR_INIT_LOGGER, e);
        }
    }

    // Detect presence of a terminal to determine use of color output.
    // Later allow forcing a color choice via command-line arguments.
    let color_choice = if atty::is(atty::Stream::Stdout) {
        ColorChoice::Auto
    } else {
        ColorChoice::Never
    };

    let mut stdout = StandardStream::stdout(color_choice);
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // Command-line mode.
        evaluate_expr(&mut stdout, args.get(1).unwrap(), AppMode::NonInteractive)?;
    } else {
        // Interactive mode.
        let mut editor = Editor::<()>::new();
        loop {
            let res_line = editor.readline(USER_PROMPT);
            if let Ok(str_input) = res_line {
                let input_expr = str_input.as_str();
                editor.add_history_entry(input_expr);

                let input_expr_trimmed = input_expr.trim();
                if !input_expr_trimmed.is_empty() {
                    match input_expr_trimmed {
                        "q" | "quit" | "exit" => break,
                        "efer" => {
                            test_bitgroup_desc(&mut stdout)?;
                            continue;
                        }
                        _ => (),
                    }
                    // Use the original input expression given by the user rather
                    // than the trimmed expression as it would mess up the error caret.
                    evaluate_expr(&mut stdout, input_expr, AppMode::Interactive)?;
                }
            } else {
                let mut stderr = StandardStream::stderr(color_choice);
                write_color(&mut stderr, EXITING_APP, Color::Red, true)?;
                writeln!(&mut stderr, " {:?}", res_line.err().unwrap())?;
                break;
            }
        }
    }

    Ok(())
}

