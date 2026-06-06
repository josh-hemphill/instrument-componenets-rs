use instrument_core::error::Result;
use instrument_core::scpi::ScpiSession;
use instrument_core::InstrumentSession;

/// Digital multimeter session view (IVI-inspired / SCPI :MEASure).
pub struct Dmm {
    session: InstrumentSession,
}

impl Dmm {
    pub fn new(session: InstrumentSession) -> Self {
        Self { session }
    }

    pub fn from_session(_session: &InstrumentSession) -> Result<Self> {
        // Shared session pattern requires caller to manage locking via SessionPool
        Err(instrument_core::Error::Unsupported(
            "use SessionPool for shared session views",
        ))
    }

    pub fn session(&self) -> &InstrumentSession {
        &self.session
    }

    pub fn session_mut(&mut self) -> &mut InstrumentSession {
        &mut self.session
    }

    /// Measures DC voltage in volts (SI).
    pub fn measure_voltage_dc(&mut self, range: Option<f64>) -> Result<f64> {
        let cmd = match range {
            Some(r) => format!(":MEAS:VOLT:DC? {r}"),
            None => ":MEAS:VOLT:DC?".into(),
        };
        self.query_f64(&cmd)
    }

    /// Measures AC voltage in volts (SI).
    pub fn measure_voltage_ac(&mut self, range: Option<f64>) -> Result<f64> {
        let cmd = match range {
            Some(r) => format!(":MEAS:VOLT:AC? {r}"),
            None => ":MEAS:VOLT:AC?".into(),
        };
        self.query_f64(&cmd)
    }

    /// Measures DC current in amps (SI).
    pub fn measure_current_dc(&mut self, range: Option<f64>) -> Result<f64> {
        let cmd = match range {
            Some(r) => format!(":MEAS:CURR:DC? {r}"),
            None => ":MEAS:CURR:DC?".into(),
        };
        self.query_f64(&cmd)
    }

    /// Measures resistance in ohms (SI).
    pub fn measure_resistance(&mut self, range: Option<f64>) -> Result<f64> {
        let cmd = match range {
            Some(r) => format!(":MEAS:RES? {r}"),
            None => ":MEAS:RES?".into(),
        };
        self.query_f64(&cmd)
    }

    /// Configures DC voltage measurement for faster repeated reads.
    pub fn configure_voltage_dc(
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
        self.session.scpi_mut().write(&cmd)
    }

    fn query_f64(&mut self, cmd: &str) -> Result<f64> {
        let resp = self.session.scpi_mut().query(cmd)?;
        ScpiSession::parse_f64(&resp)
    }
}
