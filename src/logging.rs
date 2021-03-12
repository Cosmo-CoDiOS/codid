//! This is the logging module for `codi-linuxd`.

use slog::{o, Drain, Level, LevelFilter, Logger};
use slog_async::Async;
#[cfg(any(feature = "sailfish", feature = "ubports", feature = "gemian"))]
use slog_journald::JournaldDrain;

/// Derived from GitHub repo `leftwm/leftwm` (TODO: Make this a link)
#[allow(clippy::module_name_repetitions)]
#[must_use]
pub fn setup_logging(level: Level) -> Logger {
    #[cfg(any(feature = "sailfish", feature = "ubports", feature = "gemian"))]
    let journald_drain = JournaldDrain.ignore_res().fuse();

    #[cfg(any(feature = "slog-journald", feature = "slog-term"))]
    let decorator = slog_term::TermDecorator::new().build();

    let term_drain = slog_term::FullFormat::new(decorator).build().fuse();

    #[cfg(any(feature = "sailfish", feature = "ubports", feature = "gemian"))]
    let drain = slog::Duplicate(term_drain, journald_drain).fuse();

    let drain = Async::new(drain).build().fuse();

    let drain = LevelFilter::new(drain, level).fuse();

    let log = Logger::root(drain, o!("app" => "cosmo-codi-linuxd"));

    trace!(log, "Logging initialised.");

    log
}
