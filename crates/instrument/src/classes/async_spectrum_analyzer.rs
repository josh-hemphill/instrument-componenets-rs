use super::dialect_io;
use instrument_core::error::Result;
use instrument_core::kind::InstrumentKind;
use instrument_core::scpi::{parse_f64_csv, AsyncScpiSession};
use instrument_core::AsyncInstrumentSession;

/// Async spectrum analyzer session view.
pub struct AsyncSpectrumAnalyzer {
    session: AsyncInstrumentSession,
}

impl AsyncSpectrumAnalyzer {
    pub fn new(session: AsyncInstrumentSession) -> Self {
        Self { session }
    }

    pub fn session(&self) -> &AsyncInstrumentSession {
        &self.session
    }

    pub fn session_mut(&mut self) -> &mut AsyncInstrumentSession {
        &mut self.session
    }

    fn dialect(&self) -> &'static instrument_core::DialectProfile {
        self.session.dialect_for(InstrumentKind::SpectrumAnalyzer)
    }

    async fn write_dialect(&mut self, key: &str, vars: &[(&str, String)]) -> Result<()> {
        let cmd = dialect_io::formatted(self.dialect(), key, vars)?;
        self.session.scpi_mut().write(&cmd).await
    }

    async fn query_dialect(&mut self, key: &str) -> Result<String> {
        let cmd = dialect_io::command(self.dialect(), key)?;
        self.session.scpi_mut().query(cmd).await
    }

    /// Sets center frequency in hertz (SI).
    pub async fn set_center_frequency(&mut self, hz: f64) -> Result<()> {
        self.write_dialect("center_frequency", &[("hz", dialect_io::f64_text(hz))])
            .await
    }

    /// Sets frequency span in hertz (SI).
    pub async fn set_span(&mut self, hz: f64) -> Result<()> {
        self.write_dialect("span", &[("hz", dialect_io::f64_text(hz))])
            .await
    }

    /// Sets resolution bandwidth in hertz (SI).
    pub async fn set_rbw(&mut self, hz: f64) -> Result<()> {
        self.write_dialect("rbw", &[("hz", dialect_io::f64_text(hz))])
            .await
    }

    /// Sets video bandwidth in hertz (SI).
    pub async fn set_vbw(&mut self, hz: f64) -> Result<()> {
        self.write_dialect("vbw", &[("hz", dialect_io::f64_text(hz))])
            .await
    }

    /// Sets reference level in dBm.
    pub async fn set_ref_level(&mut self, dbm: f64) -> Result<()> {
        self.write_dialect("ref_level", &[("dbm", dialect_io::f64_text(dbm))])
            .await
    }

    /// Fetches TRACE1 as ASCII comma-separated amplitude samples.
    pub async fn fetch_trace_ascii(&mut self) -> Result<Vec<f64>> {
        parse_f64_csv(&self.query_dialect("trace_data").await?)
    }

    /// Moves marker to peak.
    pub async fn marker_peak(&mut self) -> Result<()> {
        let cmd = dialect_io::command(self.dialect(), "marker_peak")?;
        self.session.scpi_mut().write(cmd).await
    }

    /// Reads marker X (typically frequency in Hz).
    pub async fn marker_x(&mut self) -> Result<f64> {
        let resp = self.query_dialect("marker_x").await?;
        AsyncScpiSession::parse_f64(&resp)
    }

    /// Reads marker Y (typically amplitude in dBm).
    pub async fn marker_y(&mut self) -> Result<f64> {
        let resp = self.query_dialect("marker_y").await?;
        AsyncScpiSession::parse_f64(&resp)
    }

    /// Enables or disables continuous sweep.
    pub async fn sweep_continuous(&mut self, enabled: bool) -> Result<()> {
        let state = if enabled { "ON" } else { "OFF" };
        self.write_dialect("sweep_continuous", &[("state", state.into())])
            .await
    }

    /// Triggers a single sweep.
    pub async fn single_sweep(&mut self) -> Result<()> {
        let cmd = dialect_io::command(self.dialect(), "single_sweep")?;
        self.session.scpi_mut().write(cmd).await
    }

    /// Waits for operation complete (*OPC?).
    pub async fn wait_opc(&mut self) -> Result<()> {
        let _ = self.query_dialect("wait_opc").await?;
        Ok(())
    }

    /// Writes a dialect-formatted command (advanced).
    pub async fn write_dialect_command(
        &mut self,
        key: &str,
        vars: &[(&str, String)],
    ) -> Result<()> {
        self.write_dialect(key, vars).await
    }
}
