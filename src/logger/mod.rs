use chrono::Local;
use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};

use crate::config::Config;

struct AppLogger;

impl log::Log for AppLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("[{} - {}] {}", Local::now(), record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

static LOGGER: AppLogger = AppLogger;

pub fn init(cfg: &Config) -> Result<(), SetLoggerError> {
    let max_level = if cfg.debug { LevelFilter::Debug } else { LevelFilter::Info };
    
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(max_level))
}