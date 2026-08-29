use super::dialect_io;
use super::power_meter::PowerUnit;
use instrument_core::error::Result;
use instrument_core::kind::InstrumentKind;
use instrument_core::scpi::AsyncScpiSession;
use instrument_core::AsyncInstrumentSession;

/// Async RF / microwave power meter session view.
pub struct AsyncPowerMeter {
    session: AsyncInstrumentSession,
}

impl AsyncPowerMeter {
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
        self.session.dialect_for(InstrumentKind::PowerMeter)
    }

    async fn write_dialect(&mut self, key: &str, vars: &[(&str, String)]) -> Result<()> {
        let cmd = dialect_io::formatted(self.dialect(), key, vars)?;
        self.session.scpi_mut().write(&cmd).await
    }

    async fn query_dialect(&mut self, key: &str) -> Result<String> {
        let cmd = dialect_io::command(self.dialect(), key)?;
        self.session.scpi_mut().query(cmd).await
    }

    /// Configures units, auto-range/average, optional correction frequency and offset.
    pub async fn configure_measurement(
        &mut self,
        unit: PowerUnit,
        auto_range: bool,
        auto_average: bool,
        correction_freq_hz: Option<f64>,
        offset_db: Option<f64>,
    ) -> Result<()> {
        let on_off = |enabled: bool| if enabled { "ON" } else { "OFF" };
        self.write_dialect("unit", &[("unit", unit.scpi_name().into())])
            .await?;
        self.write_dialect("auto_range", &[("state", on_off(auto_range).into())])
            .await?;
        self.write_dialect("auto_average", &[("state", on_off(auto_average).into())])
            .await?;
        if let Some(hz) = correction_freq_hz {
            self.write_dialect("correction_frequency", &[("hz", dialect_io::f64_text(hz))])
                .await?;
        }
        if let Some(db) = offset_db {
            self.write_dialect("offset", &[("db", dialect_io::f64_text(db))])
                .await?;
        }
        Ok(())
    }

    /// Initiates a measurement (INIT).
    pub async fn initiate(&mut self) -> Result<()> {
        let cmd = dialect_io::command(self.dialect(), "initiate")?;
        self.session.scpi_mut().write(cmd).await
    }

    /// Fetches the last initiated measurement (FETC?).
    pub async fn fetch(&mut self) -> Result<f64> {
        AsyncScpiSession::parse_f64(&self.query_dialect("fetch").await?)
    }

    /// Reads a measurement immediately (READ?).
    pub async fn read(&mut self) -> Result<f64> {
        AsyncScpiSession::parse_f64(&self.query_dialect("read").await?)
    }
}
