use instrument_core::error::Result;
use instrument_core::scpi::ScpiSession;
use instrument_core::InstrumentSession;

/// Frequency counter session view (IVI-inspired / SCPI :MEASure, :COUNter).
pub struct Counter {
    session: InstrumentSession,
}

impl Counter {
    pub fn new(session: InstrumentSession) -> Self {
        Self { session }
    }

    pub fn session(&self) -> &InstrumentSession {
        &self.session
    }

    pub fn session_mut(&mut self) -> &mut InstrumentSession {
        &mut self.session
    }

    /// Measures frequency in hertz (SI).
    pub fn measure_frequency(&mut self) -> Result<f64> {
        self.query_f64(":MEASure:FREQuency?")
    }

    /// Measures period in seconds (SI).
    pub fn measure_period(&mut self) -> Result<f64> {
        self.query_f64(":MEASure:PERiod?")
    }

    /// Resets the totalize counter.
    pub fn reset_totalize(&mut self) -> Result<()> {
        self.session.scpi_mut().write(":COUNter:CRESet")
    }

    /// Reads totalize count.
    pub fn read_totalize(&mut self) -> Result<f64> {
        self.query_f64(":COUNter:DATA?")
    }

    fn query_f64(&mut self, cmd: &str) -> Result<f64> {
        let resp = self.session.scpi_mut().query(cmd)?;
        ScpiSession::parse_f64(&resp)
    }
}
