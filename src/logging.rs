//! This is the logging module for `codid`.

use std::sync::Mutex;

use slog::{o, Drain, Level, LevelFilter, Logger};

/// Initialisation routine for logging.
/// Derived from GitHub repo `leftwm/leftwm` (TODO: Make this a link)
#[allow(clippy::module_name_repetitions)]
pub fn setup_logging(level: Level) -> Option<Logger> {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = LevelFilter::new(drain, level).fuse();
    let drain = Mutex::new(drain).fuse();

    let log = Logger::root(drain, o!());

    trace!(log, "Logging initialised.");

    Some(log)
}
