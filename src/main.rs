use bitgroup::*;
use termcolor::*;
use rustyline::Editor;
use std::env;
use std::io::Write;
use std::ops::Range;

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

fn write_color(stream: &mut StandardStream, s: &str, col: Color, is_intense: bool) -> std::io::Result<()> {
    stream.set_color(ColorSpec::new()
                     .set_fg(Some(col))
                     .set_intense(is_intense))?;
    write!(stream, "{}", s)?;
    stream.reset()?;
    Ok(())
}

fn print_result_num(stream: &mut StandardStream, number: &spceval::Number) -> std::io::Result<()> {
    // Format as hex
    let str_hex_zfill = format!("{:#016x}", number.integer);
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

fn print_error(stream: &mut StandardStream, str_expr: &str, err: spceval::ExprError, app_mode: AppMode) -> std::io::Result<()> {
    // Print the caret indicating where in the expression the error occurs in interactive mode.
    if let AppMode::Interactive = app_mode {
        let idx_char = byte_index_to_char_index(str_expr, err.idx_expr);
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

#[inline(always)]
fn parse_and_eval_expr_internal(stream: &mut StandardStream, str_expr: &str, app_mode: AppMode) -> std::io::Result<()> {
    match spceval::parse(&str_expr) {
        Ok(mut expr_ctx) => {
            match spceval::evaluate(&mut expr_ctx) {
                Ok(expr_result) => {
                    match expr_result {
                        spceval::ExprResult::Number(n) => print_result_num(stream, &n)?,
                        spceval::ExprResult::Command(c) => println!("Result: {}", c),
                    }
                }
                Err(e) => print_error(stream, str_expr, e, app_mode)?,
            }
        }
        Err(e) => print_error(stream, str_expr, e, app_mode)?,
    }
    Ok(())
}

fn parse_and_eval_expr(stream: &mut StandardStream, str_expr: &str, app_mode: AppMode) -> std::io::Result<()> {
    // Enable trace level logging while parsing and evaluating using spceval.
    #[cfg(debug_assertions)]
    log::set_max_level(log::LevelFilter::Trace);

    parse_and_eval_expr_internal(stream, str_expr, app_mode)?;

    // Disable logging.
    #[cfg(debug_assertions)]
    log::set_max_level(log::LevelFilter::Off);
    Ok(())
}

fn test_bitgroup_desc(stream: &mut StandardStream) -> std::io::Result<()> {
    let efer_bit_span = vec![
        BitSpan::new(
            Range { start: 0, end: 0 },
            BitSpanKind::Normal,
            false,
            String::from("SCE"),
            String::from("SysCall Ext."),
            String::from("System Call Extensions"),
        ),
    ];
    let efer_bits = BitGroup::new(
        String::from("EFER"),
        String::from("x86"),
        String::from("cpu"),
        ByteOrder::LittleEndian,
        64,
        vec![],
        efer_bit_span
    );
    let res_fmt = bitgroup::fmt_bit_group(&efer_bits);
    match res_fmt {
        Ok(v) => writeln!(stream, "{}", v)?,
        Err(e) => writeln!(stream, "Error: {}", e)?,
    };

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
    let args: Vec<String> = env::args().collect();

    // Command-line mode, evaluate and quit.
    if args.len() > 1 {
        parse_and_eval_expr(&mut stdout, args.get(1).unwrap(), AppMode::NonInteractive)?;
        return Ok(());
    }

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
                    "efer" => {
                        test_bitgroup_desc(&mut stdout)?;
                        continue;
                    }
                    _ => (),
                }
                parse_and_eval_expr(&mut stdout, str_expr, AppMode::Interactive)?;
            }
        } else {
            let mut stderr = StandardStream::stderr(color_choice);
            write_color(&mut stderr, EXITING_APP, Color::Red, true)?;
            writeln!(&mut stderr, " {:?}", res_readline.err().unwrap())?;
            return Ok(());
        }
    }
}

