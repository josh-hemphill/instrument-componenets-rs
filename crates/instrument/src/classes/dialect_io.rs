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
///
/// Templates that still contain `{ident}` placeholders are rejected so a
/// no-arg path cannot emit braces (C# `DialectCommand.Try` already did this).
pub(crate) fn try_command<'a>(profile: &DialectProfile, key: &str, fallback: &'a str) -> &'a str {
    match profile.command(key) {
        Some(cmd) if !has_unreplaced_placeholder(cmd) => cmd,
        _ => fallback,
    }
}

/// Dialect-formatted command, or `fallback` when the template is missing or unusable.
///
/// Use the dialect when every placeholder in the template can be filled.
/// Extra supplied vars that the template does not mention are ignored when the
/// template has placeholders (vendor `{range}` still runs if `{resolution}` is
/// also passed). A constant template cannot represent any supplied var, so
/// those calls fall back (ranged DMM measure keeps the range).
pub(crate) fn try_formatted(
    profile: &DialectProfile,
    key: &str,
    vars: &[(&str, String)],
    fallback: String,
) -> String {
    let Some(template) = profile.command(key) else {
        return fallback;
    };
    let extras = vars
        .iter()
        .any(|(name, _)| !template.contains(&format!("{{{name}}}")));
    if extras && !has_unreplaced_placeholder(template) {
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
    use instrument_core::dialect::{resolve_dialect, DialectProfile};
    use instrument_core::kind::InstrumentKind;

    fn test_profile(commands: &'static [(&'static str, &'static str)]) -> DialectProfile {
        DialectProfile {
            id: "test",
            kind: InstrumentKind::Dmm,
            manufacturer_glob: "*",
            model_glob: "*",
            channels: 1,
            commands,
        }
    }

    #[test]
    fn try_command_uses_dialect_then_fallback() {
        let dmm = resolve_dialect(InstrumentKind::Dmm, None, None);
        assert_eq!(try_command(dmm, "initiate", "FALLBACK"), "INIT");
        assert_eq!(try_command(dmm, "missing", "FALLBACK"), "FALLBACK");
    }

    #[test]
    fn try_command_falls_back_on_leftover_placeholders() {
        let profile = test_profile(&[("read_frequency", ":SOUR{channel}:FREQ?")]);
        assert_eq!(
            try_command(&profile, "read_frequency", "FALLBACK"),
            "FALLBACK"
        );
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
    fn try_formatted_unescapes_quoted_scpi() {
        let counter = resolve_dialect(InstrumentKind::Counter, None, None);
        let cmd = try_formatted(
            counter,
            "channel_select",
            &[("channel", "1".into())],
            "FALLBACK".into(),
        );
        assert_eq!(cmd, ":SENSe:FUNCtion:ON \"FREQ 1\"");
    }

    #[test]
    fn try_formatted_falls_back_on_leftover_placeholders() {
        let fgen = resolve_dialect(InstrumentKind::FunctionGenerator, None, None);
        let cmd = try_formatted(fgen, "set_waveform", &[], "FALLBACK".into());
        assert_eq!(cmd, "FALLBACK");
    }

    #[test]
    fn try_formatted_ignores_extra_optional_vars() {
        let profile = test_profile(&[("configure_voltage_dc", ":CONF:VOLT:DC {range}")]);
        let cmd = try_formatted(
            &profile,
            "configure_voltage_dc",
            &[("range", "10".into()), ("resolution", "0.001".into())],
            "FALLBACK".into(),
        );
        assert_eq!(cmd, ":CONF:VOLT:DC 10");
    }
}
