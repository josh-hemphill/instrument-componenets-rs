use super::dialect_io;
use instrument_core::error::Result;
use instrument_core::kind::InstrumentKind;
use instrument_core::scpi::{parse_f64_csv, ScpiSession};
use instrument_core::InstrumentSession;

/// Spectrum analyzer session view.
pub struct SpectrumAnalyzer {
    session: InstrumentSession,
}

impl SpectrumAnalyzer {
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
        self.session.dialect_for(InstrumentKind::SpectrumAnalyzer)
    }

    fn write_dialect(&mut self, key: &str, vars: &[(&str, String)]) -> Result<()> {
        let cmd = dialect_io::formatted(self.dialect(), key, vars)?;
        self.session.scpi_mut().write(&cmd)
    }

    fn query_dialect(&mut self, key: &str) -> Result<String> {
        let cmd = dialect_io::command(self.dialect(), key)?;
        self.session.scpi_mut().query(cmd)
    }

    /// Sets center frequency in hertz (SI).
    pub fn set_center_frequency(&mut self, hz: f64) -> Result<()> {
        self.write_dialect("center_frequency", &[("hz", dialect_io::f64_text(hz))])
    }

    /// Sets frequency span in hertz (SI).
    pub fn set_span(&mut self, hz: f64) -> Result<()> {
        self.write_dialect("span", &[("hz", dialect_io::f64_text(hz))])
    }

    /// Sets resolution bandwidth in hertz (SI).
    pub fn set_rbw(&mut self, hz: f64) -> Result<()> {
        self.write_dialect("rbw", &[("hz", dialect_io::f64_text(hz))])
    }

    /// Sets video bandwidth in hertz (SI).
    pub fn set_vbw(&mut self, hz: f64) -> Result<()> {
        self.write_dialect("vbw", &[("hz", dialect_io::f64_text(hz))])
    }

    /// Sets reference level in dBm.
    pub fn set_ref_level(&mut self, dbm: f64) -> Result<()> {
        self.write_dialect("ref_level", &[("dbm", dialect_io::f64_text(dbm))])
    }

    /// Fetches TRACE1 as ASCII comma-separated amplitude samples.
    pub fn fetch_trace_ascii(&mut self) -> Result<Vec<f64>> {
        parse_f64_csv(&self.query_dialect("trace_data")?)
    }

    /// Moves marker to peak.
    pub fn marker_peak(&mut self) -> Result<()> {
        let cmd = dialect_io::command(self.dialect(), "marker_peak")?;
        self.session.scpi_mut().write(cmd)
    }

    /// Reads marker X (typically frequency in Hz).
    pub fn marker_x(&mut self) -> Result<f64> {
        let resp = self.query_dialect("marker_x")?;
        ScpiSession::parse_f64(&resp)
    }

    /// Reads marker Y (typically amplitude in dBm).
    pub fn marker_y(&mut self) -> Result<f64> {
        let resp = self.query_dialect("marker_y")?;
        ScpiSession::parse_f64(&resp)
    }

    /// Enables or disables continuous sweep.
    pub fn sweep_continuous(&mut self, enabled: bool) -> Result<()> {
        let state = if enabled { "ON" } else { "OFF" };
        self.write_dialect("sweep_continuous", &[("state", state.into())])
    }

    /// Triggers a single sweep.
    pub fn single_sweep(&mut self) -> Result<()> {
        let cmd = dialect_io::command(self.dialect(), "single_sweep")?;
        self.session.scpi_mut().write(cmd)
    }

    /// Waits for operation complete (*OPC?).
    pub fn wait_opc(&mut self) -> Result<()> {
        let _ = self.query_dialect("wait_opc")?;
        Ok(())
    }

    /// Writes a dialect-formatted command (advanced).
    pub fn write_dialect_command(&mut self, key: &str, vars: &[(&str, String)]) -> Result<()> {
        self.write_dialect(key, vars)
    }
}
