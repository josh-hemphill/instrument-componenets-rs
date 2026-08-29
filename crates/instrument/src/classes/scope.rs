use instrument_core::error::Result;
use instrument_core::scpi::{parse_f64_csv, ScpiSession};
use instrument_core::scpi_commands;
use instrument_core::InstrumentSession;

/// Captured voltage waveform samples in SI units.
#[derive(Debug, Clone, PartialEq)]
pub struct VoltageTrace {
    /// Sample values in volts.
    pub samples: Vec<f64>,
    /// Time between samples in seconds (0.0 when preamble unavailable).
    pub sample_interval_s: f64,
}

/// Oscilloscope session view (IVI-inspired / SCPI :TIMebase, :CHANnel, :WAVeform).
pub struct Oscilloscope {
    session: InstrumentSession,
}

impl Oscilloscope {
    pub fn new(session: InstrumentSession) -> Self {
        Self { session }
    }

    pub fn session(&self) -> &InstrumentSession {
        &self.session
    }

    pub fn session_mut(&mut self) -> &mut InstrumentSession {
        &mut self.session
    }

    /// Sets horizontal timebase scale in seconds per division (SI).
    pub fn set_timebase_scale(&mut self, seconds_per_div: f64) -> Result<()> {
        self.session
            .scpi_mut()
            .write(&scpi_commands::scope_set_timebase_scale(seconds_per_div))
    }

    /// Reads horizontal timebase scale in seconds per division (SI).
    pub fn read_timebase_scale(&mut self) -> Result<f64> {
        let resp = self
            .session
            .scpi_mut()
            .query(scpi_commands::SCOPE_READ_TIMEBASE_SCALE)?;
        ScpiSession::parse_f64(&resp)
    }

    /// Sets vertical scale for a channel in volts per division (SI).
    pub fn set_channel_scale(&mut self, channel: u32, volts_per_div: f64) -> Result<()> {
        self.session
            .scpi_mut()
            .write(&scpi_commands::scope_set_channel_scale(
                channel,
                volts_per_div,
            ))
    }

    /// Enables or disables channel display.
    pub fn set_channel_display(&mut self, channel: u32, enabled: bool) -> Result<()> {
        let state = if enabled { "ON" } else { "OFF" };
        self.session
            .scpi_mut()
            .write(&scpi_commands::scope_channel_display(channel, state))
    }

    /// Sets channel coupling (`DC`, `AC`, or `GND`).
    pub fn set_channel_coupling(&mut self, channel: u32, coupling: &str) -> Result<()> {
        self.session
            .scpi_mut()
            .write(&scpi_commands::scope_channel_coupling(channel, coupling))
    }

    /// Sets edge trigger source (e.g. `CHAN1`, `EXT`).
    pub fn set_trigger_source(&mut self, source: &str) -> Result<()> {
        self.session
            .scpi_mut()
            .write(&scpi_commands::scope_trigger_source(source))
    }

    /// Sets edge trigger level in volts (SI).
    pub fn set_trigger_level(&mut self, volts: f64) -> Result<()> {
        self.session
            .scpi_mut()
            .write(&scpi_commands::scope_trigger_level(volts))
    }

    /// Sets edge trigger slope (`POS` or `NEG`).
    pub fn set_trigger_slope(&mut self, slope: &str) -> Result<()> {
        self.session
            .scpi_mut()
            .write(&scpi_commands::scope_trigger_slope(slope))
    }

    /// Starts acquisition.
    pub fn run(&mut self) -> Result<()> {
        self.session.scpi_mut().write(scpi_commands::SCOPE_RUN)
    }

    /// Stops acquisition.
    pub fn stop(&mut self) -> Result<()> {
        self.session.scpi_mut().write(scpi_commands::SCOPE_STOP)
    }

    /// Arms a single acquisition.
    pub fn single(&mut self) -> Result<()> {
        self.session.scpi_mut().write(scpi_commands::SCOPE_SINGLE)
    }

    /// Measures peak-to-peak voltage on a channel (SI volts).
    pub fn measure_vpp(&mut self, channel: u32) -> Result<f64> {
        let resp = self
            .session
            .scpi_mut()
            .query(&scpi_commands::scope_measure_vpp(channel))?;
        ScpiSession::parse_f64(&resp)
    }

    /// Measures frequency on a channel (SI hertz).
    pub fn measure_frequency(&mut self, channel: u32) -> Result<f64> {
        let resp = self
            .session
            .scpi_mut()
            .query(&scpi_commands::scope_measure_frequency(channel))?;
        ScpiSession::parse_f64(&resp)
    }

    /// Captures a voltage trace from a channel (1-based).
    pub fn capture_voltage_trace(&mut self, channel: u32) -> Result<VoltageTrace> {
        let scpi = self.session.scpi_mut();
        scpi.write(&scpi_commands::scope_set_waveform_source(channel))?;
        scpi.write(scpi_commands::SCOPE_WAVEFORM_FORMAT_ASCII)?;
        let sample_interval_s = scpi
            .query(scpi_commands::SCOPE_WAVEFORM_PREAMBLE)
            .ok()
            .and_then(|preamble| parse_preamble_x_increment(&preamble))
            .unwrap_or(0.0);
        let data = scpi.query(scpi_commands::SCOPE_WAVEFORM_DATA)?;
        let samples = parse_f64_csv(&data)?;
        Ok(VoltageTrace {
            samples,
            sample_interval_s,
        })
    }
}

pub(crate) fn parse_preamble_x_increment(preamble: &str) -> Option<f64> {
    let fields: Vec<&str> = preamble.split(',').map(str::trim).collect();
    fields.get(4).and_then(|v| v.parse().ok())
}
