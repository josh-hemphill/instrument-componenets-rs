use super::dialect_io;
use crate::classes::switch::parse_closed;
use instrument_core::error::Result;
use instrument_core::kind::InstrumentKind;
use instrument_core::scpi_commands;
use instrument_core::AsyncInstrumentSession;

/// Async switch / matrix session view (IVI-inspired / SCPI :ROUTe).
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

    fn dialect(&self) -> &'static instrument_core::DialectProfile {
        self.session.dialect_for(InstrumentKind::Switch)
    }

    fn cmd(&self, key: &str, vars: &[(&str, String)], fallback: String) -> String {
        dialect_io::try_formatted(self.dialect(), key, vars, fallback)
    }

    /// Formats a matrix path label for channels `ch1` and `ch2` (1-based).
    pub fn path_label(ch1: u32, ch2: u32) -> String {
        crate::classes::Switch::path_label(ch1, ch2)
    }

    /// Closes a route between two channels (1-based). IVI ClosePath equivalent.
    pub async fn close_route(&mut self, ch1: u32, ch2: u32) -> Result<()> {
        let cmd = self.cmd(
            "close_route",
            &[("ch1", ch1.to_string()), ("ch2", ch2.to_string())],
            scpi_commands::switch_close_route(ch1, ch2),
        );
        self.session.scpi_mut().write(&cmd).await
    }

    /// Opens a route between two channels (1-based).
    pub async fn open_route(&mut self, ch1: u32, ch2: u32) -> Result<()> {
        let cmd = self.cmd(
            "open_route",
            &[("ch1", ch1.to_string()), ("ch2", ch2.to_string())],
            scpi_commands::switch_open_route(ch1, ch2),
        );
        self.session.scpi_mut().write(&cmd).await
    }

    /// Returns whether a route is closed.
    pub async fn is_closed(&mut self, ch1: u32, ch2: u32) -> Result<bool> {
        let cmd = self.cmd(
            "is_closed",
            &[("ch1", ch1.to_string()), ("ch2", ch2.to_string())],
            scpi_commands::switch_is_closed(ch1, ch2),
        );
        let resp = self.session.scpi_mut().query(&cmd).await?;
        parse_closed(&resp)
    }

    /// Opens all routes.
    pub async fn open_all(&mut self) -> Result<()> {
        let cmd =
            dialect_io::try_command(self.dialect(), "open_all", scpi_commands::SWITCH_OPEN_ALL);
        self.session.scpi_mut().write(cmd).await
    }
}
