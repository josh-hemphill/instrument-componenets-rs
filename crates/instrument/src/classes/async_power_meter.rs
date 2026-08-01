use super::power_meter::PowerUnit;
use instrument_core::error::Result;
use instrument_core::scpi::AsyncScpiSession;
use instrument_core::scpi_commands;
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

    /// Configures units, auto-range/average, optional correction frequency and offset.
    pub async fn configure_measurement(
        &mut self,
        unit: PowerUnit,
        auto_range: bool,
        auto_average: bool,
        correction_freq_hz: Option<f64>,
        offset_db: Option<f64>,
    ) -> Result<()> {
        let scpi = self.session.scpi_mut();
        scpi.write(&scpi_commands::pwrmeter_unit(unit.scpi_name()))
            .await?;
        scpi.write(&scpi_commands::pwrmeter_auto_range(if auto_range {
            "ON"
        } else {
            "OFF"
        }))
        .await?;
        scpi.write(&scpi_commands::pwrmeter_auto_average(if auto_average {
            "ON"
        } else {
            "OFF"
        }))
        .await?;
        if let Some(hz) = correction_freq_hz {
            scpi.write(&scpi_commands::pwrmeter_correction_frequency(hz))
                .await?;
        }
        if let Some(db) = offset_db {
            scpi.write(&scpi_commands::pwrmeter_offset(db)).await?;
        }
        Ok(())
    }

    /// Initiates a measurement (INIT).
    pub async fn initiate(&mut self) -> Result<()> {
        self.session
            .scpi_mut()
            .write(scpi_commands::PWRMETER_INITIATE)
            .await
    }

    /// Fetches the last initiated measurement (FETC?).
    pub async fn fetch(&mut self) -> Result<f64> {
        let resp = self
            .session
            .scpi_mut()
            .query(scpi_commands::PWRMETER_FETCH)
            .await?;
        AsyncScpiSession::parse_f64(&resp)
    }

    /// Reads a measurement immediately (READ?).
    pub async fn read(&mut self) -> Result<f64> {
        let resp = self
            .session
            .scpi_mut()
            .query(scpi_commands::PWRMETER_READ)
            .await?;
        AsyncScpiSession::parse_f64(&resp)
    }
}
