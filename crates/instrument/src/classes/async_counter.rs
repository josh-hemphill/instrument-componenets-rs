use instrument_core::error::Result;
use instrument_core::scpi::AsyncScpiSession;
use instrument_core::scpi_commands;
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
        self.query_f64(scpi_commands::COUNTER_MEASURE_FREQUENCY)
            .await
    }

    /// Measures period in seconds (SI).
    pub async fn measure_period(&mut self) -> Result<f64> {
        self.query_f64(scpi_commands::COUNTER_MEASURE_PERIOD).await
    }

    /// Sets gate / aperture time in seconds (SI).
    pub async fn set_gate_time(&mut self, seconds: f64) -> Result<()> {
        self.session
            .scpi_mut()
            .write(&scpi_commands::counter_gate_time(seconds))
            .await
    }

    /// Selects the measurement channel (1-based).
    pub async fn select_channel(&mut self, channel: u32) -> Result<()> {
        self.session
            .scpi_mut()
            .write(&scpi_commands::counter_channel_select(channel))
            .await
    }

    /// Resets the totalize counter.
    pub async fn reset_totalize(&mut self) -> Result<()> {
        self.session
            .scpi_mut()
            .write(scpi_commands::COUNTER_RESET_TOTALIZE)
            .await
    }

    /// Reads totalize count.
    pub async fn read_totalize(&mut self) -> Result<f64> {
        self.query_f64(scpi_commands::COUNTER_READ_TOTALIZE).await
    }

    async fn query_f64(&mut self, cmd: &str) -> Result<f64> {
        let resp = self.session.scpi_mut().query(cmd).await?;
        AsyncScpiSession::parse_f64(&resp)
    }
}
