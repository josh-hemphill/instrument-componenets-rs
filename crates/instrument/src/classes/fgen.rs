use instrument_core::error::Result;
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

    /// Sets the output waveform.
    pub fn set_waveform(&mut self, waveform: Waveform) -> Result<()> {
        self.session
            .scpi_mut()
            .write(&scpi_commands::fgen_set_waveform(waveform.scpi_name()))
    }

    /// Sets output frequency in hertz (SI).
    pub fn set_frequency(&mut self, hz: f64) -> Result<()> {
        self.session
            .scpi_mut()
            .write(&scpi_commands::fgen_set_frequency(hz))
    }

    /// Sets peak-to-peak amplitude in volts (SI).
    pub fn set_amplitude(&mut self, vpp: f64) -> Result<()> {
        self.session
            .scpi_mut()
            .write(&scpi_commands::fgen_set_amplitude(vpp))
    }

    /// Sets DC offset in volts (SI).
    pub fn set_offset(&mut self, volts: f64) -> Result<()> {
        self.session
            .scpi_mut()
            .write(&scpi_commands::fgen_set_offset(volts))
    }

    /// Sets square-wave duty cycle in percent.
    pub fn set_duty_cycle(&mut self, percent: f64) -> Result<()> {
        self.session
            .scpi_mut()
            .write(&scpi_commands::fgen_set_duty_cycle(percent))
    }

    /// Sets output load impedance in ohms (SI); use `INFinity` via vendor-specific call if needed.
    pub fn set_load(&mut self, ohms: f64) -> Result<()> {
        self.session
            .scpi_mut()
            .write(&scpi_commands::fgen_set_load(ohms))
    }

    /// Enables or disables the output.
    pub fn output_enable(&mut self, enabled: bool) -> Result<()> {
        let state = if enabled { "ON" } else { "OFF" };
        self.session
            .scpi_mut()
            .write(&scpi_commands::fgen_output_enable(state))
    }

    /// Sets burst cycle count.
    pub fn set_burst_count(&mut self, count: u32) -> Result<()> {
        self.session
            .scpi_mut()
            .write(&scpi_commands::fgen_burst_count(count))
    }

    /// Enables or disables burst mode.
    pub fn set_burst_state(&mut self, enabled: bool) -> Result<()> {
        let state = if enabled { "ON" } else { "OFF" };
        self.session
            .scpi_mut()
            .write(&scpi_commands::fgen_burst_state(state))
    }

    /// Sets burst trigger source (e.g. `IMM`, `EXT`, `BUS`).
    pub fn set_burst_trigger_source(&mut self, source: &str) -> Result<()> {
        self.session
            .scpi_mut()
            .write(&scpi_commands::fgen_burst_trigger(source))
    }

    /// Reads the configured frequency in hertz (SI).
    pub fn read_frequency(&mut self) -> Result<f64> {
        let resp = self
            .session
            .scpi_mut()
            .query(scpi_commands::FGEN_READ_FREQUENCY)?;
        ScpiSession::parse_f64(&resp)
    }
}
