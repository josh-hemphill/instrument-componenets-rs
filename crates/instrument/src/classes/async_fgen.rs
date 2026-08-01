use super::fgen::Waveform;
use instrument_core::error::Result;
use instrument_core::scpi::AsyncScpiSession;
use instrument_core::scpi_commands;
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
            .write(&scpi_commands::fgen_set_waveform(waveform.scpi_name()))
            .await
    }

    /// Sets output frequency in hertz (SI).
    pub async fn set_frequency(&mut self, hz: f64) -> Result<()> {
        self.session
            .scpi_mut()
            .write(&scpi_commands::fgen_set_frequency(hz))
            .await
    }

    /// Sets peak-to-peak amplitude in volts (SI).
    pub async fn set_amplitude(&mut self, vpp: f64) -> Result<()> {
        self.session
            .scpi_mut()
            .write(&scpi_commands::fgen_set_amplitude(vpp))
            .await
    }

    /// Sets DC offset in volts (SI).
    pub async fn set_offset(&mut self, volts: f64) -> Result<()> {
        self.session
            .scpi_mut()
            .write(&scpi_commands::fgen_set_offset(volts))
            .await
    }

    /// Sets square-wave duty cycle in percent.
    pub async fn set_duty_cycle(&mut self, percent: f64) -> Result<()> {
        self.session
            .scpi_mut()
            .write(&scpi_commands::fgen_set_duty_cycle(percent))
            .await
    }

    /// Sets output load impedance in ohms (SI).
    pub async fn set_load(&mut self, ohms: f64) -> Result<()> {
        self.session
            .scpi_mut()
            .write(&scpi_commands::fgen_set_load(ohms))
            .await
    }

    /// Enables or disables the output.
    pub async fn output_enable(&mut self, enabled: bool) -> Result<()> {
        let state = if enabled { "ON" } else { "OFF" };
        self.session
            .scpi_mut()
            .write(&scpi_commands::fgen_output_enable(state))
            .await
    }

    /// Sets burst cycle count.
    pub async fn set_burst_count(&mut self, count: u32) -> Result<()> {
        self.session
            .scpi_mut()
            .write(&scpi_commands::fgen_burst_count(count))
            .await
    }

    /// Enables or disables burst mode.
    pub async fn set_burst_state(&mut self, enabled: bool) -> Result<()> {
        let state = if enabled { "ON" } else { "OFF" };
        self.session
            .scpi_mut()
            .write(&scpi_commands::fgen_burst_state(state))
            .await
    }

    /// Sets burst trigger source (e.g. `IMM`, `EXT`, `BUS`).
    pub async fn set_burst_trigger_source(&mut self, source: &str) -> Result<()> {
        self.session
            .scpi_mut()
            .write(&scpi_commands::fgen_burst_trigger(source))
            .await
    }

    /// Reads the configured frequency in hertz (SI).
    pub async fn read_frequency(&mut self) -> Result<f64> {
        let resp = self
            .session
            .scpi_mut()
            .query(scpi_commands::FGEN_READ_FREQUENCY)
            .await?;
        AsyncScpiSession::parse_f64(&resp)
    }
}
