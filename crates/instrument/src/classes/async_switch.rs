use crate::classes::switch::parse_closed;
use instrument_core::error::Result;
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

    /// Closes a route between two channels (1-based).
    pub async fn close_route(&mut self, ch1: u32, ch2: u32) -> Result<()> {
        self.session
            .scpi_mut()
            .write(&format!(":ROUTe:CLOS (@({ch1},{ch2}))"))
            .await
    }

    /// Opens a route between two channels (1-based).
    pub async fn open_route(&mut self, ch1: u32, ch2: u32) -> Result<()> {
        self.session
            .scpi_mut()
            .write(&format!(":ROUTe:OPEN (@({ch1},{ch2}))"))
            .await
    }

    /// Returns whether a route is closed.
    pub async fn is_closed(&mut self, ch1: u32, ch2: u32) -> Result<bool> {
        let resp = self
            .session
            .scpi_mut()
            .query(&format!(":ROUTe:CLOS? (@({ch1},{ch2}))"))
            .await?;
        parse_closed(&resp)
    }

    /// Opens all routes.
    pub async fn open_all(&mut self) -> Result<()> {
        self.session.scpi_mut().write(":ROUTe:OPEN:ALL").await
    }
}
