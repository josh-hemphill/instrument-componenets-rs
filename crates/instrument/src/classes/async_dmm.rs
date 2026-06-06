use instrument_core::error::Result;
use instrument_core::scpi::AsyncScpiSession;
use instrument_core::AsyncInstrumentSession;

/// Async digital multimeter session view (IVI-inspired / SCPI :MEASure).
pub struct AsyncDmm {
    session: AsyncInstrumentSession,
}

impl AsyncDmm {
    pub fn new(session: AsyncInstrumentSession) -> Self {
        Self { session }
    }

    pub fn session(&self) -> &AsyncInstrumentSession {
        &self.session
    }

    pub fn session_mut(&mut self) -> &mut AsyncInstrumentSession {
        &mut self.session
    }

    /// Measures DC voltage in volts (SI).
    pub async fn measure_voltage_dc(&mut self, range: Option<f64>) -> Result<f64> {
        let cmd = match range {
            Some(r) => format!(":MEAS:VOLT:DC? {r}"),
            None => ":MEAS:VOLT:DC?".into(),
        };
        self.query_f64(&cmd).await
    }

    /// Measures AC voltage in volts (SI).
    pub async fn measure_voltage_ac(&mut self, range: Option<f64>) -> Result<f64> {
        let cmd = match range {
            Some(r) => format!(":MEAS:VOLT:AC? {r}"),
            None => ":MEAS:VOLT:AC?".into(),
        };
        self.query_f64(&cmd).await
    }

    /// Measures DC current in amps (SI).
    pub async fn measure_current_dc(&mut self, range: Option<f64>) -> Result<f64> {
        let cmd = match range {
            Some(r) => format!(":MEAS:CURR:DC? {r}"),
            None => ":MEAS:CURR:DC?".into(),
        };
        self.query_f64(&cmd).await
    }

    /// Measures resistance in ohms (SI).
    pub async fn measure_resistance(&mut self, range: Option<f64>) -> Result<f64> {
        let cmd = match range {
            Some(r) => format!(":MEAS:RES? {r}"),
            None => ":MEAS:RES?".into(),
        };
        self.query_f64(&cmd).await
    }

    /// Configures DC voltage measurement for faster repeated reads.
    pub async fn configure_voltage_dc(
        &mut self,
        range: Option<f64>,
        resolution: Option<f64>,
    ) -> Result<()> {
        let mut cmd = String::from(":CONF:VOLT:DC");
        match (range, resolution) {
            (Some(r), Some(res)) => cmd = format!("{cmd} {r},{res}"),
            (Some(r), None) => cmd = format!("{cmd} {r}"),
            (None, Some(res)) => cmd = format!("{cmd} DEF,{res}"),
            (None, None) => {}
        }
        self.session.scpi_mut().write(&cmd).await
    }

    async fn query_f64(&mut self, cmd: &str) -> Result<f64> {
        let resp = self.session.scpi_mut().query(cmd).await?;
        AsyncScpiSession::parse_f64(&resp)
    }
}
