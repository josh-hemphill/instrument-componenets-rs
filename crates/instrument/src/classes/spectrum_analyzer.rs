use instrument_core::dialect::resolve_dialect;
use instrument_core::error::{Error, Result};
use instrument_core::kind::InstrumentKind;
use instrument_core::scpi::{parse_f64_csv, ScpiSession};
use instrument_core::scpi_commands;
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
        let identity = self.session.identity();
        resolve_dialect(
            InstrumentKind::SpectrumAnalyzer,
            identity.manufacturer.as_deref(),
            identity.model.as_deref(),
        )
    }

    fn write_dialect(&mut self, key: &str, vars: &[(&str, String)]) -> Result<()> {
        let cmd = self
            .dialect()
            .format_command(key, vars)
            .ok_or_else(|| Error::Unsupported("spectrum analyzer dialect missing command"))?;
        self.session.scpi_mut().write(&cmd)
    }

    fn query_dialect(&mut self, key: &str) -> Result<String> {
        let cmd = self
            .dialect()
            .command(key)
            .ok_or_else(|| Error::Unsupported("spectrum analyzer dialect missing command"))?;
        self.session.scpi_mut().query(cmd)
    }

    /// Sets center frequency in hertz (SI).
    pub fn set_center_frequency(&mut self, hz: f64) -> Result<()> {
        self.session
            .scpi_mut()
            .write(&scpi_commands::specan_center_frequency(hz))
    }

    /// Sets frequency span in hertz (SI).
    pub fn set_span(&mut self, hz: f64) -> Result<()> {
        self.session
            .scpi_mut()
            .write(&scpi_commands::specan_span(hz))
    }

    /// Sets resolution bandwidth in hertz (SI).
    pub fn set_rbw(&mut self, hz: f64) -> Result<()> {
        self.session
            .scpi_mut()
            .write(&scpi_commands::specan_rbw(hz))
    }

    /// Sets video bandwidth in hertz (SI).
    pub fn set_vbw(&mut self, hz: f64) -> Result<()> {
        self.session
            .scpi_mut()
            .write(&scpi_commands::specan_vbw(hz))
    }

    /// Sets reference level in dBm.
    pub fn set_ref_level(&mut self, dbm: f64) -> Result<()> {
        self.session
            .scpi_mut()
            .write(&scpi_commands::specan_ref_level(dbm))
    }

    /// Fetches TRACE1 as ASCII comma-separated amplitude samples.
    pub fn fetch_trace_ascii(&mut self) -> Result<Vec<f64>> {
        let resp = match self.query_dialect("trace_data") {
            Ok(r) => r,
            Err(_) => self
                .session
                .scpi_mut()
                .query(scpi_commands::SPECAN_TRACE_DATA)?,
        };
        parse_f64_csv(&resp)
    }

    /// Moves marker to peak.
    pub fn marker_peak(&mut self) -> Result<()> {
        let cmd = self
            .dialect()
            .command("marker_peak")
            .unwrap_or(scpi_commands::SPECAN_MARKER_PEAK);
        self.session.scpi_mut().write(cmd)
    }

    /// Reads marker X (typically frequency in Hz).
    pub fn marker_x(&mut self) -> Result<f64> {
        let cmd = self
            .dialect()
            .command("marker_x")
            .unwrap_or(scpi_commands::SPECAN_MARKER_X);
        let resp = self.session.scpi_mut().query(cmd)?;
        ScpiSession::parse_f64(&resp)
    }

    /// Reads marker Y (typically amplitude in dBm).
    pub fn marker_y(&mut self) -> Result<f64> {
        let cmd = self
            .dialect()
            .command("marker_y")
            .unwrap_or(scpi_commands::SPECAN_MARKER_Y);
        let resp = self.session.scpi_mut().query(cmd)?;
        ScpiSession::parse_f64(&resp)
    }

    /// Enables or disables continuous sweep.
    pub fn sweep_continuous(&mut self, enabled: bool) -> Result<()> {
        let state = if enabled { "ON" } else { "OFF" };
        self.session
            .scpi_mut()
            .write(&scpi_commands::specan_sweep_continuous(state))
    }

    /// Triggers a single sweep.
    pub fn single_sweep(&mut self) -> Result<()> {
        let cmd = self
            .dialect()
            .command("single_sweep")
            .unwrap_or(scpi_commands::SPECAN_SINGLE_SWEEP);
        self.session.scpi_mut().write(cmd)
    }

    /// Waits for operation complete (*OPC?).
    pub fn wait_opc(&mut self) -> Result<()> {
        let cmd = self
            .dialect()
            .command("wait_opc")
            .unwrap_or(scpi_commands::SPECAN_WAIT_OPC);
        let _ = self.session.scpi_mut().query(cmd)?;
        Ok(())
    }

    /// Writes a dialect-formatted command (advanced).
    pub fn write_dialect_command(&mut self, key: &str, vars: &[(&str, String)]) -> Result<()> {
        self.write_dialect(key, vars)
    }
}
