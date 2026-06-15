use instrument_core::error::Result;
use instrument_core::scpi::AsyncScpiSession;
use instrument_core::AsyncInstrumentSession;

/// Async frequency counter session view (IVI-inspired / SCPI :MEASure, :COUNter).
pub struct AsyncCounter {
    session: AsyncInstrumentSession,
}

impl AsyncCounter {
    pub fn new(session: AsyncInstrumentSession) -> Self {
        Self { session }
    }

    pub fn session(&self) -> &AsyncInstrumentSession {
        &self.session
    }

    pub fn session_mut(&mut self) -> &mut AsyncInstrumentSession {
        &mut self.session
    }

    /// Measures frequency in hertz (SI).
    pub async fn measure_frequency(&mut self) -> Result<f64> {
        self.query_f64(":MEASure:FREQuency?").await
    }

    /// Measures period in seconds (SI).
    pub async fn measure_period(&mut self) -> Result<f64> {
        self.query_f64(":MEASure:PERiod?").await
    }

    /// Resets the totalize counter.
    pub async fn reset_totalize(&mut self) -> Result<()> {
        self.session.scpi_mut().write(":COUNter:CRESet").await
    }

    /// Reads totalize count.
    pub async fn read_totalize(&mut self) -> Result<f64> {
        self.query_f64(":COUNter:DATA?").await
    }

    async fn query_f64(&mut self, cmd: &str) -> Result<f64> {
        let resp = self.session.scpi_mut().query(cmd).await?;
        AsyncScpiSession::parse_f64(&resp)
    }
}
