//! This is the logging module for `codi-linuxd`.

use slog::{Drain, Level, LevelFilter, Logger, o};
use slog_async::Async;

/// Derived from GitHub repo `leftwm/leftwm` (TODO: Make this a link)
#[allow(clippy::module_name_repetitions)]
#[must_use]
pub fn setup_logging(level: Level) -> Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = LevelFilter::new(drain, level).fuse();
    let drain = Async::new(drain).build().fuse();

    let log = Logger::root(drain, o!("app" => "cosmo-codi-linuxd"));

    trace!(log, "Logging initialised.");

    log
}
