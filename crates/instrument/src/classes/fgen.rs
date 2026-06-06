use instrument_core::error::Result;
use instrument_core::scpi::ScpiSession;
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
            .write(&format!(":SOUR:FUNC {}", waveform.scpi_name()))
    }

    /// Sets output frequency in hertz (SI).
    pub fn set_frequency(&mut self, hz: f64) -> Result<()> {
        self.session.scpi_mut().write(&format!(":SOUR:FREQ {hz}"))
    }

    /// Sets peak-to-peak amplitude in volts (SI).
    pub fn set_amplitude(&mut self, vpp: f64) -> Result<()> {
        self.session.scpi_mut().write(&format!(":SOUR:VOLT {vpp}"))
    }

    /// Sets DC offset in volts (SI).
    pub fn set_offset(&mut self, volts: f64) -> Result<()> {
        self.session
            .scpi_mut()
            .write(&format!(":SOUR:VOLT:OFFS {volts}"))
    }

    /// Enables or disables the output.
    pub fn output_enable(&mut self, enabled: bool) -> Result<()> {
        let state = if enabled { "ON" } else { "OFF" };
        self.session.scpi_mut().write(&format!(":OUTP {state}"))
    }

    /// Reads the configured frequency in hertz (SI).
    pub fn read_frequency(&mut self) -> Result<f64> {
        let resp = self.session.scpi_mut().query(":SOUR:FREQ?")?;
        ScpiSession::parse_f64(&resp)
    }
}
