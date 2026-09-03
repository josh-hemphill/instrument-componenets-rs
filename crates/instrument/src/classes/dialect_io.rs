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

pub(crate) fn range_vars(range: Option<f64>) -> Vec<(&'static str, String)> {
    match range {
        Some(r) => vec![("range", f64_text(r))],
        None => Vec::new(),
    }
}

pub(crate) fn range_resolution_vars(
    range: Option<f64>,
    resolution: Option<f64>,
) -> Vec<(&'static str, String)> {
    let mut vars = Vec::new();
    if let Some(r) = range {
        vars.push(("range", f64_text(r)));
    }
    if let Some(r) = resolution {
        vars.push(("resolution", f64_text(r)));
    }
    vars
}

/// Dialect command, or `fallback` when the profile has no template for `key`.
pub(crate) fn try_command<'a>(profile: &DialectProfile, key: &str, fallback: &'a str) -> &'a str {
    match profile.command(key) {
        Some(cmd) => cmd,
        None => fallback,
    }
}

/// Dialect-formatted command, or `fallback` when the template is missing or cannot take `vars`.
pub(crate) fn try_formatted(
    profile: &DialectProfile,
    key: &str,
    vars: &[(&str, String)],
    fallback: String,
) -> String {
    let Some(template) = profile.command(key) else {
        return fallback;
    };
    if vars
        .iter()
        .any(|(name, _)| !template.contains(&format!("{{{name}}}")))
    {
        return fallback;
    }
    let Some(formatted) = profile.format_command(key, vars) else {
        return fallback;
    };
    if has_unreplaced_placeholder(&formatted) {
        return fallback;
    }
    formatted
}

fn has_unreplaced_placeholder(s: &str) -> bool {
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] != b'{' {
            i += 1;
            continue;
        }
        let start = i + 1;
        let mut j = start;
        while j < bytes.len() && (bytes[j].is_ascii_alphanumeric() || bytes[j] == b'_') {
            j += 1;
        }
        if j > start && j < bytes.len() && bytes[j] == b'}' {
            return true;
        }
        i += 1;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use instrument_core::dialect::resolve_dialect;
    use instrument_core::kind::InstrumentKind;

    #[test]
    fn try_command_uses_dialect_then_fallback() {
        let dmm = resolve_dialect(InstrumentKind::Dmm, None, None);
        assert_eq!(try_command(dmm, "initiate", "FALLBACK"), "INIT");
        assert_eq!(try_command(dmm, "missing", "FALLBACK"), "FALLBACK");
    }

    #[test]
    fn try_formatted_falls_back_when_template_cannot_take_vars() {
        let dmm = resolve_dialect(InstrumentKind::Dmm, None, None);
        let fallback = ":MEAS:VOLT:DC? 10".to_string();
        let with_range = try_formatted(
            dmm,
            "measure_voltage_dc",
            &[("range", "10".into())],
            fallback.clone(),
        );
        assert_eq!(with_range, fallback);
        let bare = try_formatted(dmm, "measure_voltage_dc", &[], ":MEAS:VOLT:DC?".into());
        assert_eq!(bare, ":MEAS:VOLT:DC?");
    }

    #[test]
    fn try_formatted_fills_dialect_placeholders() {
        let psu = resolve_dialect(InstrumentKind::DcPowerSupply, None, None);
        let cmd = try_formatted(
            psu,
            "set_voltage",
            &[("channel", "1".into()), ("volts", "3.3".into())],
            "FALLBACK".into(),
        );
        assert_eq!(cmd, ":SOUR1:VOLT 3.3");
    }

    #[test]
    fn try_formatted_falls_back_on_leftover_placeholders() {
        let fgen = resolve_dialect(InstrumentKind::FunctionGenerator, None, None);
        let cmd = try_formatted(fgen, "set_waveform", &[], "FALLBACK".into());
        assert_eq!(cmd, "FALLBACK");
    }
}
