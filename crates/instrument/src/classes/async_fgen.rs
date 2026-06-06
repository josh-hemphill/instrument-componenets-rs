use super::fgen::Waveform;
use instrument_core::error::Result;
use instrument_core::scpi::AsyncScpiSession;
use instrument_core::AsyncInstrumentSession;

/// Async function / arbitrary waveform generator session view.
pub struct AsyncFunctionGenerator {
    session: AsyncInstrumentSession,
}

impl AsyncFunctionGenerator {
    pub fn new(session: AsyncInstrumentSession) -> Self {
        Self { session }
    }

    pub fn session(&self) -> &AsyncInstrumentSession {
        &self.session
    }

    pub fn session_mut(&mut self) -> &mut AsyncInstrumentSession {
        &mut self.session
    }

    /// Sets the output waveform.
    pub async fn set_waveform(&mut self, waveform: Waveform) -> Result<()> {
        self.session
            .scpi_mut()
            .write(&format!(":SOUR:FUNC {}", waveform.scpi_name()))
            .await
    }

    /// Sets output frequency in hertz (SI).
    pub async fn set_frequency(&mut self, hz: f64) -> Result<()> {
        self.session
            .scpi_mut()
            .write(&format!(":SOUR:FREQ {hz}"))
            .await
    }

    /// Sets peak-to-peak amplitude in volts (SI).
    pub async fn set_amplitude(&mut self, vpp: f64) -> Result<()> {
        self.session
            .scpi_mut()
            .write(&format!(":SOUR:VOLT {vpp}"))
            .await
    }

    /// Sets DC offset in volts (SI).
    pub async fn set_offset(&mut self, volts: f64) -> Result<()> {
        self.session
            .scpi_mut()
            .write(&format!(":SOUR:VOLT:OFFS {volts}"))
            .await
    }

    /// Enables or disables the output.
    pub async fn output_enable(&mut self, enabled: bool) -> Result<()> {
        let state = if enabled { "ON" } else { "OFF" };
        self.session
            .scpi_mut()
            .write(&format!(":OUTP {state}"))
            .await
    }

    /// Reads the configured frequency in hertz (SI).
    pub async fn read_frequency(&mut self) -> Result<f64> {
        let resp = self.session.scpi_mut().query(":SOUR:FREQ?").await?;
        AsyncScpiSession::parse_f64(&resp)
    }
}
