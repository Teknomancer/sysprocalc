use spcregs::{BitRange, BitRangeKind, ByteOrder, Register, Unsigned, BitMemory, RegisterDescriptor};
use spceval::{Number, ExprError};

use rustyline::Editor;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use std::env;
use std::io::Write;
use std::ops::RangeInclusive;

#[cfg(debug_assertions)]
mod logger;

#[cfg(debug_assertions)]
static ERR_INIT_LOGGER: &str = "Error initializing logger:";

static USER_PROMPT: &str = "> ";
static BOOL_RADIX: &str = "Bool:";
static DEC_RADIX: &str = "Dec :";
static HEX_RADIX: &str = "Hex :";
static OCT_RADIX: &str = "Oct :";
static BIN_RADIX: &str = "Bin :";
static EXITING_APP: &str = "Exiting:";
static BITS_PLURAL: &str = "bits";
static BIT_SINGULAR: &str = "bit";

enum AppMode {
    Interactive,
    CommandLine,
}

struct SpcIo {
    stream: StandardStream,
    color: ColorChoice,
}

fn write_color(stream: &mut StandardStream, message: &str, col: Color, is_bold: bool) -> std::io::Result<()> {
    stream.set_color(ColorSpec::new()
          .set_fg(Some(col))
          .set_bold(is_bold))?;
    write!(stream, "{}", message)?;
    stream.reset()?;
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

fn write_result(spcio: &mut SpcIo, number: &Number) -> std::io::Result<()> {
    // Format as hex
    let str_hex_zfill = format!("{:#018x}", number.integer);
    let str_hex = format!("{:#x}", number.integer);

    // Format as octal
    let str_oct_zfill = format!("{:#022o}", number.integer);
    let str_oct = format!("{:#o}", number.integer);

    // Format as binary
    let str_bin_sfill = spcregs::utils::get_binary_string(number.integer, None);

    // Compute number of bits to make a binary ruler as well for writing the number of bits.
    let mut bit_count = u64::MAX.count_ones() - number.integer.leading_zeros();
    let str_bit_count;
    if bit_count < 2 {
        bit_count = 1; // Required because bin_digits gets computed as 0 when number.integer is 0.
        str_bit_count = BIT_SINGULAR;
    } else {
        str_bit_count = BITS_PLURAL;
    };

    let str_bool = if number.integer != 0 { "true" } else { "false" };

    // Write the formatted values
    write_color(&mut spcio.stream, BOOL_RADIX, Color::Cyan, true)?;
    writeln!(spcio.stream, " {:>24} (nat)", str_bool)?;
    write_color(&mut spcio.stream, DEC_RADIX, Color::Cyan, true)?;
    writeln!(spcio.stream, " {:>24} (u64)  {:>26} (f)", number.integer, number.float)?;
    write_color(&mut spcio.stream, HEX_RADIX, Color::Cyan, true)?;
    writeln!(spcio.stream, " {:>24} (u64)  {:>26} (n)", str_hex_zfill, str_hex)?;
    write_color(&mut spcio.stream, OCT_RADIX, Color::Cyan, true)?;
    writeln!(spcio.stream, " {:>24} (u64)  {:>26} (n)", str_oct_zfill, str_oct)?;
    write_color(&mut spcio.stream, BIN_RADIX, Color::Cyan, true)?;
    writeln!(spcio.stream, " {} ({} {})", str_bin_sfill, bit_count, str_bit_count)?;

    // Write the binary ruler if we have 8 or more bits.
    if bit_count >= 8 {
        let str_bin_ruler = spcregs::utils::get_binary_ruler_string(bit_count as u8);
        writeln!(spcio.stream, "      {}", str_bin_ruler)?;
    }

    // Write a blank line
    writeln!(spcio.stream)?;
    Ok(())
}

fn write_error(spcio: &mut SpcIo, str_expr: &str, opt_extra_padding: Option<usize>, err: ExprError, app_mode: AppMode)
    -> std::io::Result<()> {
    // Write the caret indicating where in the expression the error occurs in interactive mode.
    if let AppMode::Interactive = app_mode {
        let idx_char = byte_index_to_char_index(str_expr, err.index());
        let user_prompt_padding = USER_PROMPT.chars().count();

        // Calculate padding.
        let padding = if opt_extra_padding.is_some() {
            idx_char + user_prompt_padding + opt_extra_padding.unwrap()
        } else {
            idx_char + user_prompt_padding
        };

        // Pad the caret to the correct position and write the caret.
        write!(spcio.stream, "{:width$}", " ", width = padding)?;
        write_color(&mut spcio.stream, "^", Color::Red, true)?;
        writeln!(spcio.stream)?;

        // Passing width as 0 still produces 1 space, so do the padding here.
        write!(spcio.stream, "{:width$}", " ", width = user_prompt_padding)?;
    }

    // Write the error.
    write_color(&mut spcio.stream, "Error:", Color::Red, true)?;
    writeln!(spcio.stream, " {}", err)?;

    // Write a blank line
    writeln!(spcio.stream)?;
    Ok(())
}

fn evaluate_expr(str_expr: &str) -> Result<Number, ExprError>
{
    // Enable trace level logging while parsing and evaluating using spceval.
    #[cfg(debug_assertions)]
    log::set_max_level(log::LevelFilter::Trace);

    let res = spceval::evaluate(str_expr);

    // Disable logging.
    #[cfg(debug_assertions)]
    log::set_max_level(log::LevelFilter::Off);

    res
}

fn evaluate_input(spcio: &mut SpcIo, str_expr: &str, app_mode: AppMode) -> std::io::Result<()> {
    let mut tokens = str_expr.trim().splitn(2, ' ').fuse();
    let command = tokens.next();
    let args = tokens.next();
    let efer_cmd = "efer";
    match command {
        Some("q") | Some("quit") | Some("exit") => std::process::exit(0),
        Some("efer") => test_register(spcio, args, efer_cmd, AppMode::Interactive),
        Some(x) if x.is_empty() => Ok(()),

        // Use the original input expression given by the user rather
        // than the trimmed expression as it would mess up the error caret position.
        _ => evaluate_expr_and_write_result(spcio, str_expr, AppMode::Interactive),
    }
}

fn evaluate_expr_and_write_result(spcio: &mut SpcIo, str_expr: &str, app_mode: AppMode) -> std::io::Result<()> {
    match evaluate_expr(str_expr) {
        Ok(number) => write_result(spcio, &number),
        Err(e) => write_error(spcio, str_expr, None, e, app_mode),
    }
}

fn write_reg_desc_title<T: Unsigned + BitMemory>(spcio: &mut SpcIo, register: &Register<T>)  -> std::io::Result<()> {
    write!(spcio.stream, "{}.{} ", register.get_descriptor().device(), register.get_descriptor().arch())?;
    write_color(&mut spcio.stream, register.get_descriptor().name(), Color::Green, true)?;
    writeln!(spcio.stream, " ({})", register.get_descriptor().description())?;
    Ok(())
}

fn test_register(spcio: &mut SpcIo, opt_str_expr: Option<&str>, str_cmd: &str, app_mode: AppMode) -> std::io::Result<()> {
    let efer_descriptor = RegisterDescriptor::new(
        String::from("x86"),
        String::from("cpu"),
        String::from("EFER"),
        String::from("Extended Feature Register"),
        u32::BITS as usize,
        ByteOrder::LittleEndian,
        vec![
            BitRange::new(
                RangeInclusive::new(0, 0),
                BitRangeKind::Normal,
                true,
                String::from("SCE"),
                String::from("SysCall"),
                String::from("System Call Extensions"),
            ),
            BitRange::new(
                RangeInclusive::new(8, 8),
                BitRangeKind::Normal,
                true,
                String::from("LME"),
                String::from("Long mode enable"),
                String::from("Long mode enable"),
            ),
            BitRange::new(
                RangeInclusive::new(10, 10),
                BitRangeKind::Normal,
                true,
                String::from("LMA"),
                String::from("Long mode active"),
                String::from("Long mode active"),
            ),
            BitRange::new(
                RangeInclusive::new(11, 11),
                BitRangeKind::Normal,
                true,
                String::from("NXE"),
                String::from("No-execute enable"),
                String::from("No-execute enable"),
            ),
            BitRange::new(
                RangeInclusive::new(12, 12),
                BitRangeKind::Normal,
                true,
                String::from("SVME"),
                String::from("SVM enable"),
                String::from("Secure virtual machine enable (AMD)"),
            ),
            BitRange::new(
                RangeInclusive::new(13, 13),
                BitRangeKind::Normal,
                true,
                String::from("LMSL"),
                String::from("LMSL enable"),
                String::from("Long mode segment limit enable (AMD)"),
            ),
            BitRange::new(
                RangeInclusive::new(14, 14),
                BitRangeKind::Normal,
                true,
                String::from("FFXSR"),
                String::from("Fast FXSAVE/FXRSTOR"),
                String::from("Fast FXSAVE/FXRSTOR"),
            ),
        ]
    ).unwrap();


    match opt_str_expr {
        Some(str_expr) => {
            match evaluate_expr(str_expr) {
                Ok(number) => {
                    let mut efer: Register<u64> = Register::new(efer_descriptor).unwrap();
                    efer.set_value(number.integer);
                    write_reg_desc_title(spcio, &efer);
                    writeln!(spcio.stream, "{}", efer)?;
                }
                // The extra 1 below is for the space following the command.
                Err(e) => write_error(spcio, str_expr, Some(str_cmd.chars().count() + 1), e, app_mode)?,
            }
        }

        None => {
            //let efer: Register<u64> = Register::new(efer_descriptor).unwrap();
            //write_reg_desc_title(spcio, &efer);
            writeln!(spcio.stream, "{}", efer_descriptor)?;
        }
    }

    Ok(())
}

fn interactive_mode(spcio: &mut SpcIo) -> std::io::Result<()> {
    let editor_result = rustyline::DefaultEditor::new();
    if let Ok(mut editor) = editor_result {
        loop {
            let readline_result = editor.readline(USER_PROMPT);
            if let Ok(str_input) = readline_result {
                let input_expr = str_input.as_str();
                editor.add_history_entry(input_expr);
                evaluate_input(spcio, input_expr, AppMode::Interactive)?;
            } else {
                let mut stderr = SpcIo { stream: StandardStream::stderr(spcio.color), color: spcio.color };
                write_color(&mut stderr.stream, EXITING_APP, Color::Red, true)?;
                writeln!(stderr.stream, " {:?}", readline_result.err().unwrap())?;
                break;
            }
        }
        Ok(())
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::Other, "failed to create readline editor object"))
    }
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
    let color = if atty::is(atty::Stream::Stdout) {
        ColorChoice::Auto
    } else {
        ColorChoice::Never
    };

    let mut stdout = SpcIo { stream: StandardStream::stdout(color), color };
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        evaluate_input(&mut stdout, args.get(1).unwrap(), AppMode::CommandLine)
    } else {
        interactive_mode(&mut stdout)
    }
}

