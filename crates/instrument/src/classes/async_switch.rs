use crate::classes::switch::parse_closed;
use instrument_core::error::Result;
use instrument_core::scpi_commands;
use instrument_core::AsyncInstrumentSession;

/// Async switch / matrix session view (IVI-inspired / SCPI :ROUTe).
///
/// Path model: routes are matrix channel pairs `(ch1, ch2)`. Use [`Switch::path_label`]
/// for a stable human-readable name; IVI "ClosePath" maps to [`close_route`].
pub struct AsyncSwitch {
    session: AsyncInstrumentSession,
}

impl AsyncSwitch {
    pub fn new(session: AsyncInstrumentSession) -> Self {
        Self { session }
    }

    pub fn session(&self) -> &AsyncInstrumentSession {
        &self.session
    }

    pub fn session_mut(&mut self) -> &mut AsyncInstrumentSession {
        &mut self.session
    }

    /// Formats a matrix path label for channels `ch1` and `ch2` (1-based).
    pub fn path_label(ch1: u32, ch2: u32) -> String {
        crate::classes::Switch::path_label(ch1, ch2)
    }

    /// Closes a route between two channels (1-based). IVI ClosePath equivalent.
    pub async fn close_route(&mut self, ch1: u32, ch2: u32) -> Result<()> {
        self.session
            .scpi_mut()
            .write(&scpi_commands::switch_close_route(ch1, ch2))
            .await
    }

    /// Opens a route between two channels (1-based).
    pub async fn open_route(&mut self, ch1: u32, ch2: u32) -> Result<()> {
        self.session
            .scpi_mut()
            .write(&scpi_commands::switch_open_route(ch1, ch2))
            .await
    }

    /// Returns whether a route is closed.
    pub async fn is_closed(&mut self, ch1: u32, ch2: u32) -> Result<bool> {
        let resp = self
            .session
            .scpi_mut()
            .query(&scpi_commands::switch_is_closed(ch1, ch2))
            .await?;
        parse_closed(&resp)
    }

    /// Opens all routes.
    pub async fn open_all(&mut self) -> Result<()> {
        self.session
            .scpi_mut()
            .write(scpi_commands::SWITCH_OPEN_ALL)
            .await
    }
}
