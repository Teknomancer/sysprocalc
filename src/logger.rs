use log::{Record, Level, Metadata, SetLoggerError, LevelFilter};

struct Logger;

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let mut level = record.level().to_string();
            level.make_ascii_lowercase();
            println!("[{}] {}", level, record.args());
        }
    }

    fn flush(&self) { }
}

static LOGGER: Logger = Logger;

pub fn init() -> Result<(), SetLoggerError> {
    // Levels are in the following order: Off, Error, Warn, Info, Debug, Trace.
    // Enabling a level enables levels prior to it.
    // We set the maximum level to Trace below
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Trace))
}

