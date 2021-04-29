//! This is the logging module for `cosmo-codi-d`.

use slog::{o, Drain, Level, LevelFilter, Logger};
use slog_async::Async;
use std::error::Error;

/// Initialisation routine for logging.
/// Derived from GitHub repo `leftwm/leftwm` (TODO: Make this a link)
#[allow(clippy::module_name_repetitions)]
#[must_use]
pub fn setup_logging(level: Level) -> Result<Logger, Errorr> {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = LevelFilter::new(drain, level).fuse();
    let drain = Async::new(drain).build().fuse();

    let log = Logger::root(drain, o!());

    trace!(log, "Logging initialised.");

    Ok(log)
}
