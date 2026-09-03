use super::dialect_io;
use instrument_core::error::Result;
use instrument_core::kind::InstrumentKind;
use instrument_core::scpi::ScpiSession;
use instrument_core::scpi_commands;
use instrument_core::InstrumentSession;

/// Frequency counter session view (IVI-inspired / SCPI :MEASure, :COUNter).
pub struct Counter {
    session: InstrumentSession,
}

impl Counter {
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
        self.session.dialect_for(InstrumentKind::Counter)
    }

    fn cmd(&self, key: &str, vars: &[(&str, String)], fallback: String) -> String {
        dialect_io::try_formatted(self.dialect(), key, vars, fallback)
    }

    /// Measures frequency in hertz (SI).
    pub fn measure_frequency(&mut self) -> Result<f64> {
        self.query_f64(dialect_io::try_command(
            self.dialect(),
            "measure_frequency",
            scpi_commands::COUNTER_MEASURE_FREQUENCY,
        ))
    }

    /// Measures period in seconds (SI).
    pub fn measure_period(&mut self) -> Result<f64> {
        self.query_f64(dialect_io::try_command(
            self.dialect(),
            "measure_period",
            scpi_commands::COUNTER_MEASURE_PERIOD,
        ))
    }

    /// Sets gate / aperture time in seconds (SI).
    pub fn set_gate_time(&mut self, seconds: f64) -> Result<()> {
        let cmd = self.cmd(
            "gate_time",
            &[("seconds", dialect_io::f64_text(seconds))],
            scpi_commands::counter_gate_time(seconds),
        );
        self.session.scpi_mut().write(&cmd)
    }

    /// Selects the measurement channel (1-based).
    pub fn select_channel(&mut self, channel: u32) -> Result<()> {
        let cmd = self.cmd(
            "channel_select",
            &[("channel", channel.to_string())],
            scpi_commands::counter_channel_select(channel),
        );
        self.session.scpi_mut().write(&cmd)
    }

    /// Resets the totalize counter.
    pub fn reset_totalize(&mut self) -> Result<()> {
        let cmd = dialect_io::try_command(
            self.dialect(),
            "reset_totalize",
            scpi_commands::COUNTER_RESET_TOTALIZE,
        );
        self.session.scpi_mut().write(cmd)
    }

    /// Reads totalize count.
    pub fn read_totalize(&mut self) -> Result<f64> {
        self.query_f64(dialect_io::try_command(
            self.dialect(),
            "read_totalize",
            scpi_commands::COUNTER_READ_TOTALIZE,
        ))
    }

    fn query_f64(&mut self, cmd: &str) -> Result<f64> {
        let resp = self.session.scpi_mut().query(cmd)?;
        ScpiSession::parse_f64(&resp)
    }
}
