use crate::error::{Error, Result};

/// Cached instrument capabilities discovered at runtime.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct SessionCapabilities {
    pub syst_err: Option<bool>,
    pub opc: Option<bool>,
}

/// Appends the terminator when missing from a command string.
pub(crate) fn normalize_command(command: &str, terminator: &str) -> String {
    let mut payload = command.to_string();
    if !payload.ends_with(terminator) {
        payload.push_str(terminator);
    }
    payload
}

/// Returns the maximum write attempts for a command.
pub(crate) fn max_write_attempts(idempotent: bool, retries: u32) -> u32 {
    if idempotent {
        retries + 1
    } else {
        1
    }
}

/// Parses a numeric SCPI response.
pub fn parse_f64(response: &str) -> Result<f64> {
    response
        .trim()
        .parse()
        .map_err(|_| Error::Parse(format!("expected number, got '{response}'")))
}
