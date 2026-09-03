use super::dialect_io;
use instrument_core::error::Result;
use instrument_core::kind::InstrumentKind;
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

    fn dialect(&self) -> &'static instrument_core::DialectProfile {
        self.session.dialect_for(InstrumentKind::Oscilloscope)
    }

    fn cmd(&self, key: &str, vars: &[(&str, String)], fallback: String) -> String {
        dialect_io::try_formatted(self.dialect(), key, vars, fallback)
    }

    /// Sets horizontal timebase scale in seconds per division (SI).
    pub fn set_timebase_scale(&mut self, seconds_per_div: f64) -> Result<()> {
        let cmd = self.cmd(
            "set_timebase_scale",
            &[("seconds_per_div", dialect_io::f64_text(seconds_per_div))],
            scpi_commands::scope_set_timebase_scale(seconds_per_div),
        );
        self.session.scpi_mut().write(&cmd)
    }

    /// Reads horizontal timebase scale in seconds per division (SI).
    pub fn read_timebase_scale(&mut self) -> Result<f64> {
        let cmd = dialect_io::try_command(
            self.dialect(),
            "read_timebase_scale",
            scpi_commands::SCOPE_READ_TIMEBASE_SCALE,
        );
        let resp = self.session.scpi_mut().query(cmd)?;
        ScpiSession::parse_f64(&resp)
    }

    /// Sets vertical scale for a channel in volts per division (SI).
    pub fn set_channel_scale(&mut self, channel: u32, volts_per_div: f64) -> Result<()> {
        let cmd = self.cmd(
            "set_channel_scale",
            &[
                ("channel", channel.to_string()),
                ("volts_per_div", dialect_io::f64_text(volts_per_div)),
            ],
            scpi_commands::scope_set_channel_scale(channel, volts_per_div),
        );
        self.session.scpi_mut().write(&cmd)
    }

    /// Enables or disables channel display.
    pub fn set_channel_display(&mut self, channel: u32, enabled: bool) -> Result<()> {
        let state = if enabled { "ON" } else { "OFF" };
        let cmd = self.cmd(
            "channel_display",
            &[("channel", channel.to_string()), ("state", state.into())],
            scpi_commands::scope_channel_display(channel, state),
        );
        self.session.scpi_mut().write(&cmd)
    }

    /// Sets channel coupling (`DC`, `AC`, or `GND`).
    pub fn set_channel_coupling(&mut self, channel: u32, coupling: &str) -> Result<()> {
        let cmd = self.cmd(
            "channel_coupling",
            &[
                ("channel", channel.to_string()),
                ("coupling", coupling.into()),
            ],
            scpi_commands::scope_channel_coupling(channel, coupling),
        );
        self.session.scpi_mut().write(&cmd)
    }

    /// Sets edge trigger source (e.g. `CHAN1`, `EXT`).
    pub fn set_trigger_source(&mut self, source: &str) -> Result<()> {
        let cmd = self.cmd(
            "trigger_source",
            &[("source", source.into())],
            scpi_commands::scope_trigger_source(source),
        );
        self.session.scpi_mut().write(&cmd)
    }

    /// Sets edge trigger level in volts (SI).
    pub fn set_trigger_level(&mut self, volts: f64) -> Result<()> {
        let cmd = self.cmd(
            "trigger_level",
            &[("volts", dialect_io::f64_text(volts))],
            scpi_commands::scope_trigger_level(volts),
        );
        self.session.scpi_mut().write(&cmd)
    }

    /// Sets edge trigger slope (`POS` or `NEG`).
    pub fn set_trigger_slope(&mut self, slope: &str) -> Result<()> {
        let cmd = self.cmd(
            "trigger_slope",
            &[("slope", slope.into())],
            scpi_commands::scope_trigger_slope(slope),
        );
        self.session.scpi_mut().write(&cmd)
    }

    /// Starts acquisition.
    pub fn run(&mut self) -> Result<()> {
        let cmd = dialect_io::try_command(self.dialect(), "run", scpi_commands::SCOPE_RUN);
        self.session.scpi_mut().write(cmd)
    }

    /// Stops acquisition.
    pub fn stop(&mut self) -> Result<()> {
        let cmd = dialect_io::try_command(self.dialect(), "stop", scpi_commands::SCOPE_STOP);
        self.session.scpi_mut().write(cmd)
    }

    /// Arms a single acquisition.
    pub fn single(&mut self) -> Result<()> {
        let cmd = dialect_io::try_command(self.dialect(), "single", scpi_commands::SCOPE_SINGLE);
        self.session.scpi_mut().write(cmd)
    }

    /// Measures peak-to-peak voltage on a channel (SI volts).
    pub fn measure_vpp(&mut self, channel: u32) -> Result<f64> {
        let cmd = self.cmd(
            "measure_vpp",
            &[("channel", channel.to_string())],
            scpi_commands::scope_measure_vpp(channel),
        );
        let resp = self.session.scpi_mut().query(&cmd)?;
        ScpiSession::parse_f64(&resp)
    }

    /// Measures frequency on a channel (SI hertz).
    pub fn measure_frequency(&mut self, channel: u32) -> Result<f64> {
        let cmd = self.cmd(
            "measure_frequency",
            &[("channel", channel.to_string())],
            scpi_commands::scope_measure_frequency(channel),
        );
        let resp = self.session.scpi_mut().query(&cmd)?;
        ScpiSession::parse_f64(&resp)
    }

    /// Captures a voltage trace from a channel (1-based). ASCII only; binary `#N` is deferred.
    pub fn capture_voltage_trace(&mut self, channel: u32) -> Result<VoltageTrace> {
        let source = self.cmd(
            "waveform_source",
            &[("channel", channel.to_string())],
            scpi_commands::scope_set_waveform_source(channel),
        );
        let format = dialect_io::try_command(
            self.dialect(),
            "waveform_format_ascii",
            scpi_commands::SCOPE_WAVEFORM_FORMAT_ASCII,
        );
        let preamble_cmd = dialect_io::try_command(
            self.dialect(),
            "waveform_preamble",
            scpi_commands::SCOPE_WAVEFORM_PREAMBLE,
        );
        let data_cmd = dialect_io::try_command(
            self.dialect(),
            "waveform_data",
            scpi_commands::SCOPE_WAVEFORM_DATA,
        );
        let scpi = self.session.scpi_mut();
        scpi.write(&source)?;
        scpi.write(format)?;
        let sample_interval_s = scpi
            .query(preamble_cmd)
            .ok()
            .and_then(|preamble| parse_preamble_x_increment(&preamble))
            .unwrap_or(0.0);
        let data = scpi.query(data_cmd)?;
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
