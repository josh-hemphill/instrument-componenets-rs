use crate::classes::scope::{parse_preamble_x_increment, VoltageTrace};
use instrument_core::error::Result;
use instrument_core::scpi::{parse_f64_csv, AsyncScpiSession};
use instrument_core::AsyncInstrumentSession;

/// Async oscilloscope session view (IVI-inspired / SCPI :TIMebase, :CHANnel, :WAVeform).
pub struct AsyncOscilloscope {
    session: AsyncInstrumentSession,
}

impl AsyncOscilloscope {
    pub fn new(session: AsyncInstrumentSession) -> Self {
        Self { session }
    }

    pub fn session(&self) -> &AsyncInstrumentSession {
        &self.session
    }

    pub fn session_mut(&mut self) -> &mut AsyncInstrumentSession {
        &mut self.session
    }

    /// Sets horizontal timebase scale in seconds per division (SI).
    pub async fn set_timebase_scale(&mut self, seconds_per_div: f64) -> Result<()> {
        self.session
            .scpi_mut()
            .write(&format!(":TIMebase:SCALe {seconds_per_div}"))
            .await
    }

    /// Reads horizontal timebase scale in seconds per division (SI).
    pub async fn read_timebase_scale(&mut self) -> Result<f64> {
        let resp = self.session.scpi_mut().query(":TIMebase:SCALe?").await?;
        AsyncScpiSession::parse_f64(&resp)
    }

    /// Sets vertical scale for a channel in volts per division (SI).
    pub async fn set_channel_scale(&mut self, channel: u32, volts_per_div: f64) -> Result<()> {
        self.session
            .scpi_mut()
            .write(&format!(":CHANnel{channel}:SCALe {volts_per_div}"))
            .await
    }

    /// Starts acquisition.
    pub async fn run(&mut self) -> Result<()> {
        self.session.scpi_mut().write(":RUN").await
    }

    /// Stops acquisition.
    pub async fn stop(&mut self) -> Result<()> {
        self.session.scpi_mut().write(":STOP").await
    }

    /// Captures a voltage trace from a channel (1-based).
    pub async fn capture_voltage_trace(&mut self, channel: u32) -> Result<VoltageTrace> {
        let scpi = self.session.scpi_mut();
        scpi.write(&format!(":WAVeform:SOURce CHAN{channel}"))
            .await?;
        scpi.write(":WAVeform:FORMat ASCii").await?;
        let sample_interval_s = match scpi.query(":WAVeform:PREamble?").await {
            Ok(preamble) => parse_preamble_x_increment(&preamble).unwrap_or(0.0),
            Err(_) => 0.0,
        };
        let data = scpi.query(":WAVeform:DATA?").await?;
        let samples = parse_f64_csv(&data)?;
        Ok(VoltageTrace {
            samples,
            sample_interval_s,
        })
    }
}
