use instrument_core::error::{Error, Result};
use instrument_core::scpi_commands;
use instrument_core::InstrumentSession;

/// Switch / matrix session view (IVI-inspired / SCPI :ROUTe).
///
/// Path model: routes are matrix channel pairs `(ch1, ch2)`. Use [`path_label`]
/// for a stable human-readable name; IVI "ClosePath" maps to [`close_route`].
pub struct Switch {
    session: InstrumentSession,
}

impl Switch {
    pub fn new(session: InstrumentSession) -> Self {
        Self { session }
    }

    pub fn session(&self) -> &InstrumentSession {
        &self.session
    }

    pub fn session_mut(&mut self) -> &mut InstrumentSession {
        &mut self.session
    }

    /// Formats a matrix path label for channels `ch1` and `ch2` (1-based).
    ///
    /// Equivalent naming to IVI `ClosePath` / `OpenPath` path strings.
    pub fn path_label(ch1: u32, ch2: u32) -> String {
        format!("(@({ch1},{ch2}))")
    }

    /// Closes a route between two channels (1-based). IVI ClosePath equivalent.
    pub fn close_route(&mut self, ch1: u32, ch2: u32) -> Result<()> {
        self.session
            .scpi_mut()
            .write(&scpi_commands::switch_close_route(ch1, ch2))
    }

    /// Opens a route between two channels (1-based). IVI OpenPath equivalent.
    pub fn open_route(&mut self, ch1: u32, ch2: u32) -> Result<()> {
        self.session
            .scpi_mut()
            .write(&scpi_commands::switch_open_route(ch1, ch2))
    }

    /// Returns whether a route is closed.
    pub fn is_closed(&mut self, ch1: u32, ch2: u32) -> Result<bool> {
        let resp = self
            .session
            .scpi_mut()
            .query(&scpi_commands::switch_is_closed(ch1, ch2))?;
        parse_closed(&resp)
    }

    /// Opens all routes.
    pub fn open_all(&mut self) -> Result<()> {
        self.session
            .scpi_mut()
            .write(scpi_commands::SWITCH_OPEN_ALL)
    }
}

pub(crate) fn parse_closed(response: &str) -> Result<bool> {
    let trimmed = response.trim();
    match trimmed.to_ascii_uppercase().as_str() {
        "1" | "ON" | "CLOSED" => Ok(true),
        "0" | "OFF" | "OPEN" => Ok(false),
        _ => Err(Error::Parse(format!(
            "expected route state, got '{response}'"
        ))),
    }
}
