use super::dialect_io;
use instrument_core::error::Result;
use instrument_core::kind::InstrumentKind;
use instrument_core::scpi::ScpiSession;
use instrument_core::scpi_commands;
use instrument_core::InstrumentSession;

/// Waveform shape for function generators.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Waveform {
    Sine,
    Square,
    Ramp,
    Pulse,
    Noise,
    Dc,
}

impl Waveform {
    pub(crate) fn scpi_name(self) -> &'static str {
        match self {
            Self::Sine => "SIN",
            Self::Square => "SQU",
            Self::Ramp => "RAMP",
            Self::Pulse => "PULS",
            Self::Noise => "NOIS",
            Self::Dc => "DC",
        }
    }
}

/// Function / arbitrary waveform generator session view.
pub struct FunctionGenerator {
    session: InstrumentSession,
}

impl FunctionGenerator {
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
        self.session.dialect_for(InstrumentKind::FunctionGenerator)
    }

    fn cmd(&self, key: &str, vars: &[(&str, String)], fallback: String) -> String {
        dialect_io::try_formatted(self.dialect(), key, vars, fallback)
    }

    /// Sets the output waveform.
    pub fn set_waveform(&mut self, waveform: Waveform) -> Result<()> {
        let name = waveform.scpi_name();
        let cmd = self.cmd(
            "set_waveform",
            &[("scpi_name", name.into())],
            scpi_commands::fgen_set_waveform(name),
        );
        self.session.scpi_mut().write(&cmd)
    }

    /// Sets output frequency in hertz (SI).
    pub fn set_frequency(&mut self, hz: f64) -> Result<()> {
        let cmd = self.cmd(
            "set_frequency",
            &[("hz", dialect_io::f64_text(hz))],
            scpi_commands::fgen_set_frequency(hz),
        );
        self.session.scpi_mut().write(&cmd)
    }

    /// Sets peak-to-peak amplitude in volts (SI).
    pub fn set_amplitude(&mut self, vpp: f64) -> Result<()> {
        let cmd = self.cmd(
            "set_amplitude",
            &[("vpp", dialect_io::f64_text(vpp))],
            scpi_commands::fgen_set_amplitude(vpp),
        );
        self.session.scpi_mut().write(&cmd)
    }

    /// Sets DC offset in volts (SI).
    pub fn set_offset(&mut self, volts: f64) -> Result<()> {
        let cmd = self.cmd(
            "set_offset",
            &[("volts", dialect_io::f64_text(volts))],
            scpi_commands::fgen_set_offset(volts),
        );
        self.session.scpi_mut().write(&cmd)
    }

    /// Sets square-wave duty cycle in percent.
    pub fn set_duty_cycle(&mut self, percent: f64) -> Result<()> {
        let cmd = self.cmd(
            "set_duty_cycle",
            &[("percent", dialect_io::f64_text(percent))],
            scpi_commands::fgen_set_duty_cycle(percent),
        );
        self.session.scpi_mut().write(&cmd)
    }

    /// Sets output load impedance in ohms (SI); use `INFinity` via vendor-specific call if needed.
    pub fn set_load(&mut self, ohms: f64) -> Result<()> {
        let cmd = self.cmd(
            "set_load",
            &[("ohms", dialect_io::f64_text(ohms))],
            scpi_commands::fgen_set_load(ohms),
        );
        self.session.scpi_mut().write(&cmd)
    }

    /// Enables or disables the output.
    pub fn output_enable(&mut self, enabled: bool) -> Result<()> {
        let state = if enabled { "ON" } else { "OFF" };
        let cmd = self.cmd(
            "output_enable",
            &[("state", state.into())],
            scpi_commands::fgen_output_enable(state),
        );
        self.session.scpi_mut().write(&cmd)
    }

    /// Sets burst cycle count.
    pub fn set_burst_count(&mut self, count: u32) -> Result<()> {
        let cmd = self.cmd(
            "burst_count",
            &[("count", count.to_string())],
            scpi_commands::fgen_burst_count(count),
        );
        self.session.scpi_mut().write(&cmd)
    }

    /// Enables or disables burst mode.
    pub fn set_burst_state(&mut self, enabled: bool) -> Result<()> {
        let state = if enabled { "ON" } else { "OFF" };
        let cmd = self.cmd(
            "burst_state",
            &[("state", state.into())],
            scpi_commands::fgen_burst_state(state),
        );
        self.session.scpi_mut().write(&cmd)
    }

    /// Sets burst trigger source (e.g. `IMM`, `EXT`, `BUS`).
    pub fn set_burst_trigger_source(&mut self, source: &str) -> Result<()> {
        let cmd = self.cmd(
            "burst_trigger",
            &[("source", source.into())],
            scpi_commands::fgen_burst_trigger(source),
        );
        self.session.scpi_mut().write(&cmd)
    }

    /// Reads the configured frequency in hertz (SI).
    pub fn read_frequency(&mut self) -> Result<f64> {
        let cmd = dialect_io::try_command(
            self.dialect(),
            "read_frequency",
            scpi_commands::FGEN_READ_FREQUENCY,
        );
        let resp = self.session.scpi_mut().query(cmd)?;
        ScpiSession::parse_f64(&resp)
    }
}
