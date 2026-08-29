use super::dialect_io;
use instrument_core::error::Result;
use instrument_core::kind::InstrumentKind;
use instrument_core::scpi::ScpiSession;
use instrument_core::InstrumentSession;

/// Power meter display / readout unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerUnit {
    Watt,
    Dbm,
}

impl PowerUnit {
    pub(crate) fn scpi_name(self) -> &'static str {
        match self {
            Self::Watt => "W",
            Self::Dbm => "DBM",
        }
    }
}

/// RF / microwave power meter session view.
pub struct PowerMeter {
    session: InstrumentSession,
}

impl PowerMeter {
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
        self.session.dialect_for(InstrumentKind::PowerMeter)
    }

    fn write_dialect(&mut self, key: &str, vars: &[(&str, String)]) -> Result<()> {
        let cmd = dialect_io::formatted(self.dialect(), key, vars)?;
        self.session.scpi_mut().write(&cmd)
    }

    fn query_dialect(&mut self, key: &str) -> Result<String> {
        let cmd = dialect_io::command(self.dialect(), key)?;
        self.session.scpi_mut().query(cmd)
    }

    /// Configures units, auto-range/average, optional correction frequency and offset.
    pub fn configure_measurement(
        &mut self,
        unit: PowerUnit,
        auto_range: bool,
        auto_average: bool,
        correction_freq_hz: Option<f64>,
        offset_db: Option<f64>,
    ) -> Result<()> {
        let on_off = |enabled: bool| if enabled { "ON" } else { "OFF" };
        self.write_dialect("unit", &[("unit", unit.scpi_name().into())])?;
        self.write_dialect("auto_range", &[("state", on_off(auto_range).into())])?;
        self.write_dialect("auto_average", &[("state", on_off(auto_average).into())])?;
        if let Some(hz) = correction_freq_hz {
            self.write_dialect("correction_frequency", &[("hz", dialect_io::f64_text(hz))])?;
        }
        if let Some(db) = offset_db {
            self.write_dialect("offset", &[("db", dialect_io::f64_text(db))])?;
        }
        Ok(())
    }

    /// Initiates a measurement (INIT).
    pub fn initiate(&mut self) -> Result<()> {
        let cmd = dialect_io::command(self.dialect(), "initiate")?;
        self.session.scpi_mut().write(cmd)
    }

    /// Fetches the last initiated measurement (FETC?).
    pub fn fetch(&mut self) -> Result<f64> {
        ScpiSession::parse_f64(&self.query_dialect("fetch")?)
    }

    /// Reads a measurement immediately (READ?).
    pub fn read(&mut self) -> Result<f64> {
        ScpiSession::parse_f64(&self.query_dialect("read")?)
    }
}
