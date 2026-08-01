use instrument_core::error::Result;
use instrument_core::scpi::ScpiSession;
use instrument_core::scpi_commands;
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

    /// Configures units, auto-range/average, optional correction frequency and offset.
    pub fn configure_measurement(
        &mut self,
        unit: PowerUnit,
        auto_range: bool,
        auto_average: bool,
        correction_freq_hz: Option<f64>,
        offset_db: Option<f64>,
    ) -> Result<()> {
        let scpi = self.session.scpi_mut();
        scpi.write(&scpi_commands::pwrmeter_unit(unit.scpi_name()))?;
        scpi.write(&scpi_commands::pwrmeter_auto_range(if auto_range {
            "ON"
        } else {
            "OFF"
        }))?;
        scpi.write(&scpi_commands::pwrmeter_auto_average(if auto_average {
            "ON"
        } else {
            "OFF"
        }))?;
        if let Some(hz) = correction_freq_hz {
            scpi.write(&scpi_commands::pwrmeter_correction_frequency(hz))?;
        }
        if let Some(db) = offset_db {
            scpi.write(&scpi_commands::pwrmeter_offset(db))?;
        }
        Ok(())
    }

    /// Initiates a measurement (INIT).
    pub fn initiate(&mut self) -> Result<()> {
        self.session
            .scpi_mut()
            .write(scpi_commands::PWRMETER_INITIATE)
    }

    /// Fetches the last initiated measurement (FETC?).
    pub fn fetch(&mut self) -> Result<f64> {
        let resp = self
            .session
            .scpi_mut()
            .query(scpi_commands::PWRMETER_FETCH)?;
        ScpiSession::parse_f64(&resp)
    }

    /// Reads a measurement immediately (READ?).
    pub fn read(&mut self) -> Result<f64> {
        let resp = self.session.scpi_mut().query(scpi_commands::PWRMETER_READ)?;
        ScpiSession::parse_f64(&resp)
    }
}
