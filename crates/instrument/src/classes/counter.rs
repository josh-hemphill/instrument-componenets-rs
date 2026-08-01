use instrument_core::error::Result;
use instrument_core::scpi::ScpiSession;
use instrument_core::scpi_commands;
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
        self.query_f64(scpi_commands::COUNTER_MEASURE_FREQUENCY)
    }

    /// Measures period in seconds (SI).
    pub fn measure_period(&mut self) -> Result<f64> {
        self.query_f64(scpi_commands::COUNTER_MEASURE_PERIOD)
    }

    /// Sets gate / aperture time in seconds (SI).
    pub fn set_gate_time(&mut self, seconds: f64) -> Result<()> {
        self.session
            .scpi_mut()
            .write(&scpi_commands::counter_gate_time(seconds))
    }

    /// Selects the measurement channel (1-based).
    pub fn select_channel(&mut self, channel: u32) -> Result<()> {
        self.session
            .scpi_mut()
            .write(&scpi_commands::counter_channel_select(channel))
    }

    /// Resets the totalize counter.
    pub fn reset_totalize(&mut self) -> Result<()> {
        self.session
            .scpi_mut()
            .write(scpi_commands::COUNTER_RESET_TOTALIZE)
    }

    /// Reads totalize count.
    pub fn read_totalize(&mut self) -> Result<f64> {
        self.query_f64(scpi_commands::COUNTER_READ_TOTALIZE)
    }

    fn query_f64(&mut self, cmd: &str) -> Result<f64> {
        let resp = self.session.scpi_mut().query(cmd)?;
        ScpiSession::parse_f64(&resp)
    }
}
