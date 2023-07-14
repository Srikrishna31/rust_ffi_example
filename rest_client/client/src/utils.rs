use chrono::Local;
use fern;
use log::LevelFilter;
use std::sync::Once;

use crate::errors::*;

/// Initialize the global logger and log to `rest_client.log`.
///
/// Note that this is an idempotent function, so you can call it as many times as you want and logging
/// will only be initialized the first time.
#[no_mangle]
pub extern "C" fn initialize_logging() {
    static INITIALIZE: Once = Once::new();
    INITIALIZE.call_once(|| {
        fern::Dispatch::new()
            .format(|out, message, record| {
                out.finish(format_args!(
                    "{} {:7} ({}#{}): {}{}",
                    Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                    record.level(),
                    record.module_path().or_else(|| Option::from("")).unwrap(),
                    record.line().or_else(|| Option::from(0)).unwrap(),
                    message,
                    if cfg!(windows) { "\r" } else { "" }
                ))
            })
            .level(LevelFilter::Debug)
            .chain(fern::log_file("rest_client.log").unwrap())
            .apply()
            .unwrap();
    });
}

pub fn backtrace(e: &Error) {
    error!("Error: {}", e);

    for cause in e.iter().skip(1) {
        warn!("\tCaused By: {}", cause);
    }
}
