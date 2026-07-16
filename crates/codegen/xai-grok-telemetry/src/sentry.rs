//! Privacy-build error-reporting stubs.
//!
//! Sentry is absent from the dependency graph. The public startup/shutdown
//! surface remains so callers do not need conditional compilation.

pub struct Config {
    pub client: &'static str,
    pub client_version: &'static str,
    pub release: &'static str,
    pub disabled: bool,
}

#[derive(Debug, Default)]
pub struct ClientInitGuard;

pub fn init(config: Config) -> ClientInitGuard {
    let _ = (
        config.client,
        config.client_version,
        config.release,
        config.disabled,
    );
    ClientInitGuard
}

pub fn flush_on_shutdown() {}
