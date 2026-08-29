//! Looks up SCPI strings from the resolved dialect profile.
use instrument_core::dialect::DialectProfile;
use instrument_core::error::{Error, Result};

pub(crate) fn command(profile: &DialectProfile, key: &str) -> Result<&'static str> {
    profile
        .command(key)
        .ok_or(Error::Unsupported("dialect missing command"))
}

pub(crate) fn formatted(
    profile: &DialectProfile,
    key: &str,
    vars: &[(&str, String)],
) -> Result<String> {
    profile
        .format_command(key, vars)
        .ok_or(Error::Unsupported("dialect missing command"))
}

pub(crate) fn f64_text(value: f64) -> String {
    format!("{value}")
}
