use crate::classes::scope::{parse_preamble_x_increment, VoltageTrace};
use instrument_core::error::Result;
use instrument_core::scpi::{parse_f64_csv, AsyncScpiSession};
use instrument_core::scpi_commands;
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
            .write(&scpi_commands::scope_set_timebase_scale(seconds_per_div))
            .await
    }

    /// Reads horizontal timebase scale in seconds per division (SI).
    pub async fn read_timebase_scale(&mut self) -> Result<f64> {
        let resp = self
            .session
            .scpi_mut()
            .query(scpi_commands::SCOPE_READ_TIMEBASE_SCALE)
            .await?;
        AsyncScpiSession::parse_f64(&resp)
    }

    /// Sets vertical scale for a channel in volts per division (SI).
    pub async fn set_channel_scale(&mut self, channel: u32, volts_per_div: f64) -> Result<()> {
        self.session
            .scpi_mut()
            .write(&scpi_commands::scope_set_channel_scale(channel, volts_per_div))
            .await
    }

    /// Enables or disables channel display.
    pub async fn set_channel_display(&mut self, channel: u32, enabled: bool) -> Result<()> {
        let state = if enabled { "ON" } else { "OFF" };
        self.session
            .scpi_mut()
            .write(&scpi_commands::scope_channel_display(channel, state))
            .await
    }

    /// Sets channel coupling (`DC`, `AC`, or `GND`).
    pub async fn set_channel_coupling(&mut self, channel: u32, coupling: &str) -> Result<()> {
        self.session
            .scpi_mut()
            .write(&scpi_commands::scope_channel_coupling(channel, coupling))
            .await
    }

    /// Sets edge trigger source (e.g. `CHAN1`, `EXT`).
    pub async fn set_trigger_source(&mut self, source: &str) -> Result<()> {
        self.session
            .scpi_mut()
            .write(&scpi_commands::scope_trigger_source(source))
            .await
    }

    /// Sets edge trigger level in volts (SI).
    pub async fn set_trigger_level(&mut self, volts: f64) -> Result<()> {
        self.session
            .scpi_mut()
            .write(&scpi_commands::scope_trigger_level(volts))
            .await
    }

    /// Sets edge trigger slope (`POS` or `NEG`).
    pub async fn set_trigger_slope(&mut self, slope: &str) -> Result<()> {
        self.session
            .scpi_mut()
            .write(&scpi_commands::scope_trigger_slope(slope))
            .await
    }

    /// Starts acquisition.
    pub async fn run(&mut self) -> Result<()> {
        self.session.scpi_mut().write(scpi_commands::SCOPE_RUN).await
    }

    /// Stops acquisition.
    pub async fn stop(&mut self) -> Result<()> {
        self.session
            .scpi_mut()
            .write(scpi_commands::SCOPE_STOP)
            .await
    }

    /// Arms a single acquisition.
    pub async fn single(&mut self) -> Result<()> {
        self.session
            .scpi_mut()
            .write(scpi_commands::SCOPE_SINGLE)
            .await
    }

    /// Measures peak-to-peak voltage on a channel (SI volts).
    pub async fn measure_vpp(&mut self, channel: u32) -> Result<f64> {
        let resp = self
            .session
            .scpi_mut()
            .query(&scpi_commands::scope_measure_vpp(channel))
            .await?;
        AsyncScpiSession::parse_f64(&resp)
    }

    /// Measures frequency on a channel (SI hertz).
    pub async fn measure_frequency(&mut self, channel: u32) -> Result<f64> {
        let resp = self
            .session
            .scpi_mut()
            .query(&scpi_commands::scope_measure_frequency(channel))
            .await?;
        AsyncScpiSession::parse_f64(&resp)
    }

    /// Captures a voltage trace from a channel (1-based).
    pub async fn capture_voltage_trace(&mut self, channel: u32) -> Result<VoltageTrace> {
        let scpi = self.session.scpi_mut();
        scpi.write(&scpi_commands::scope_set_waveform_source(channel))
            .await?;
        scpi.write(scpi_commands::SCOPE_WAVEFORM_FORMAT_ASCII)
            .await?;
        let sample_interval_s = match scpi.query(scpi_commands::SCOPE_WAVEFORM_PREAMBLE).await {
            Ok(preamble) => parse_preamble_x_increment(&preamble).unwrap_or(0.0),
            Err(_) => 0.0,
        };
        let data = scpi.query(scpi_commands::SCOPE_WAVEFORM_DATA).await?;
        let samples = parse_f64_csv(&data)?;
        Ok(VoltageTrace {
            samples,
            sample_interval_s,
        })
    }
}
