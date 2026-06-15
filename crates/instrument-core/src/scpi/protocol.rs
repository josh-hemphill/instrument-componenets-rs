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

/// Parses comma-separated numeric SCPI responses (e.g. waveform ASCII data).
pub fn parse_f64_csv(response: &str) -> Result<Vec<f64>> {
    let mut values = Vec::new();
    for part in response.split(',') {
        let trimmed = part.trim();
        if trimmed.is_empty() {
            continue;
        }
        values.push(
            trimmed
                .parse()
                .map_err(|_| Error::Parse(format!("expected number, got '{trimmed}'")))?,
        );
    }
    Ok(values)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_f64_csv_handles_spaces_and_trailing_comma() {
        let values = parse_f64_csv(" 1.0, 2.5 ,3.0,").unwrap();
        assert_eq!(values, vec![1.0, 2.5, 3.0]);
    }
}
