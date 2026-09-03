use super::dialect_io;
use super::fgen::Waveform;
use instrument_core::error::Result;
use instrument_core::kind::InstrumentKind;
use instrument_core::scpi::AsyncScpiSession;
use instrument_core::scpi_commands;
use instrument_core::AsyncInstrumentSession;

/// Async function / arbitrary waveform generator session view.
pub struct AsyncFunctionGenerator {
    session: AsyncInstrumentSession,
}

impl AsyncFunctionGenerator {
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
        self.session.dialect_for(InstrumentKind::FunctionGenerator)
    }

    fn cmd(&self, key: &str, vars: &[(&str, String)], fallback: String) -> String {
        dialect_io::try_formatted(self.dialect(), key, vars, fallback)
    }

    /// Sets the output waveform.
    pub async fn set_waveform(&mut self, waveform: Waveform) -> Result<()> {
        let name = waveform.scpi_name();
        let cmd = self.cmd(
            "set_waveform",
            &[("scpi_name", name.into())],
            scpi_commands::fgen_set_waveform(name),
        );
        self.session.scpi_mut().write(&cmd).await
    }

    /// Sets output frequency in hertz (SI).
    pub async fn set_frequency(&mut self, hz: f64) -> Result<()> {
        let cmd = self.cmd(
            "set_frequency",
            &[("hz", dialect_io::f64_text(hz))],
            scpi_commands::fgen_set_frequency(hz),
        );
        self.session.scpi_mut().write(&cmd).await
    }

    /// Sets peak-to-peak amplitude in volts (SI).
    pub async fn set_amplitude(&mut self, vpp: f64) -> Result<()> {
        let cmd = self.cmd(
            "set_amplitude",
            &[("vpp", dialect_io::f64_text(vpp))],
            scpi_commands::fgen_set_amplitude(vpp),
        );
        self.session.scpi_mut().write(&cmd).await
    }

    /// Sets DC offset in volts (SI).
    pub async fn set_offset(&mut self, volts: f64) -> Result<()> {
        let cmd = self.cmd(
            "set_offset",
            &[("volts", dialect_io::f64_text(volts))],
            scpi_commands::fgen_set_offset(volts),
        );
        self.session.scpi_mut().write(&cmd).await
    }

    /// Sets square-wave duty cycle in percent.
    pub async fn set_duty_cycle(&mut self, percent: f64) -> Result<()> {
        let cmd = self.cmd(
            "set_duty_cycle",
            &[("percent", dialect_io::f64_text(percent))],
            scpi_commands::fgen_set_duty_cycle(percent),
        );
        self.session.scpi_mut().write(&cmd).await
    }

    /// Sets output load impedance in ohms (SI).
    pub async fn set_load(&mut self, ohms: f64) -> Result<()> {
        let cmd = self.cmd(
            "set_load",
            &[("ohms", dialect_io::f64_text(ohms))],
            scpi_commands::fgen_set_load(ohms),
        );
        self.session.scpi_mut().write(&cmd).await
    }

    /// Enables or disables the output.
    pub async fn output_enable(&mut self, enabled: bool) -> Result<()> {
        let state = if enabled { "ON" } else { "OFF" };
        let cmd = self.cmd(
            "output_enable",
            &[("state", state.into())],
            scpi_commands::fgen_output_enable(state),
        );
        self.session.scpi_mut().write(&cmd).await
    }

    /// Sets burst cycle count.
    pub async fn set_burst_count(&mut self, count: u32) -> Result<()> {
        let cmd = self.cmd(
            "burst_count",
            &[("count", count.to_string())],
            scpi_commands::fgen_burst_count(count),
        );
        self.session.scpi_mut().write(&cmd).await
    }

    /// Enables or disables burst mode.
    pub async fn set_burst_state(&mut self, enabled: bool) -> Result<()> {
        let state = if enabled { "ON" } else { "OFF" };
        let cmd = self.cmd(
            "burst_state",
            &[("state", state.into())],
            scpi_commands::fgen_burst_state(state),
        );
        self.session.scpi_mut().write(&cmd).await
    }

    /// Sets burst trigger source (e.g. `IMM`, `EXT`, `BUS`).
    pub async fn set_burst_trigger_source(&mut self, source: &str) -> Result<()> {
        let cmd = self.cmd(
            "burst_trigger",
            &[("source", source.into())],
            scpi_commands::fgen_burst_trigger(source),
        );
        self.session.scpi_mut().write(&cmd).await
    }

    /// Reads the configured frequency in hertz (SI).
    pub async fn read_frequency(&mut self) -> Result<f64> {
        let cmd = dialect_io::try_command(
            self.dialect(),
            "read_frequency",
            scpi_commands::FGEN_READ_FREQUENCY,
        );
        let resp = self.session.scpi_mut().query(cmd).await?;
        AsyncScpiSession::parse_f64(&resp)
    }
}
