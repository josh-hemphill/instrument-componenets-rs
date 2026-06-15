use instrument_core::error::Result;
use instrument_core::scpi::{parse_f64_csv, ScpiSession};
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
            .write(&format!(":TIMebase:SCALe {seconds_per_div}"))
    }

    /// Reads horizontal timebase scale in seconds per division (SI).
    pub fn read_timebase_scale(&mut self) -> Result<f64> {
        let resp = self.session.scpi_mut().query(":TIMebase:SCALe?")?;
        ScpiSession::parse_f64(&resp)
    }

    /// Sets vertical scale for a channel in volts per division (SI).
    pub fn set_channel_scale(&mut self, channel: u32, volts_per_div: f64) -> Result<()> {
        self.session.scpi_mut().write(&format!(
            ":CHANnel{channel}:SCALe {volts_per_div}"
        ))
    }

    /// Starts acquisition.
    pub fn run(&mut self) -> Result<()> {
        self.session.scpi_mut().write(":RUN")
    }

    /// Stops acquisition.
    pub fn stop(&mut self) -> Result<()> {
        self.session.scpi_mut().write(":STOP")
    }

    /// Captures a voltage trace from a channel (1-based).
    pub fn capture_voltage_trace(&mut self, channel: u32) -> Result<VoltageTrace> {
        let scpi = self.session.scpi_mut();
        scpi.write(&format!(":WAVeform:SOURce CHAN{channel}"))?;
        scpi.write(":WAVeform:FORMat ASCii")?;
        let sample_interval_s = scpi
            .query(":WAVeform:PREamble?")
            .ok()
            .and_then(|preamble| parse_preamble_x_increment(&preamble))
            .unwrap_or(0.0);
        let data = scpi.query(":WAVeform:DATA?")?;
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
