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

