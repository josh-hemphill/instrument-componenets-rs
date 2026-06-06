use crate::scpi::ScpiSession;
use std::time::Duration;

pub const PROBE_TIMEOUT: Duration = Duration::from_millis(800);

pub const DMM_READONLY_COMMANDS: &[&str] = &[":SENS:FUNC?", "SENS:FUNC?", ":FUNC?", "FUNC?"];

pub const PSU_READONLY_COMMANDS: &[&str] = &[":OUTP? 1", "OUTP? 1", ":OUTP?", "OUTP?"];

pub const FGEN_READONLY_COMMANDS: &[&str] = &[":SOUR:FUNC?", "SOUR:FUNC?"];

pub const DMM_ACQUISITION_COMMANDS: &[&str] = &[":MEAS:VOLT:DC?", "MEAS:VOLT:DC?"];

/// Returns true when any probe command succeeds on the session.
pub fn probe_any(session: &mut ScpiSession, commands: &[&str], timeout: Duration) -> bool {
    commands
        .iter()
        .any(|cmd| session.query_with_timeout(cmd, timeout).is_ok())
}
