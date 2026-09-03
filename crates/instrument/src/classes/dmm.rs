use super::dialect_io;
use instrument_core::error::Result;
use instrument_core::kind::InstrumentKind;
use instrument_core::scpi::ScpiSession;
use instrument_core::scpi_commands;
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

    fn dialect(&self) -> &'static instrument_core::DialectProfile {
        self.session.dialect_for(InstrumentKind::Dmm)
    }

    fn cmd(&self, key: &str, vars: &[(&str, String)], fallback: String) -> String {
        dialect_io::try_formatted(self.dialect(), key, vars, fallback)
    }

    /// Measures DC voltage in volts (SI).
    pub fn measure_voltage_dc(&mut self, range: Option<f64>) -> Result<f64> {
        let cmd = self.cmd(
            "measure_voltage_dc",
            &dialect_io::range_vars(range),
            scpi_commands::dmm_measure_voltage_dc(range),
        );
        self.query_f64(&cmd)
    }

    /// Measures AC voltage in volts (SI).
    pub fn measure_voltage_ac(&mut self, range: Option<f64>) -> Result<f64> {
        let cmd = self.cmd(
            "measure_voltage_ac",
            &dialect_io::range_vars(range),
            scpi_commands::dmm_measure_voltage_ac(range),
        );
        self.query_f64(&cmd)
    }

    /// Measures DC current in amps (SI).
    pub fn measure_current_dc(&mut self, range: Option<f64>) -> Result<f64> {
        let cmd = self.cmd(
            "measure_current_dc",
            &dialect_io::range_vars(range),
            scpi_commands::dmm_measure_current_dc(range),
        );
        self.query_f64(&cmd)
    }

    /// Measures AC current in amps (SI).
    pub fn measure_current_ac(&mut self, range: Option<f64>) -> Result<f64> {
        let cmd = self.cmd(
            "measure_current_ac",
            &dialect_io::range_vars(range),
            scpi_commands::dmm_measure_current_ac(range),
        );
        self.query_f64(&cmd)
    }

    /// Measures 2-wire resistance in ohms (SI).
    pub fn measure_resistance(&mut self, range: Option<f64>) -> Result<f64> {
        self.measure_resistance_2wire(range)
    }

    /// Measures 2-wire resistance in ohms (SI).
    pub fn measure_resistance_2wire(&mut self, range: Option<f64>) -> Result<f64> {
        let cmd = self.cmd(
            "measure_resistance_2w",
            &dialect_io::range_vars(range),
            scpi_commands::dmm_measure_resistance_2wire(range),
        );
        self.query_f64(&cmd)
    }

    /// Measures 4-wire resistance in ohms (SI).
    pub fn measure_resistance_4wire(&mut self, range: Option<f64>) -> Result<f64> {
        let cmd = self.cmd(
            "measure_resistance_4w",
            &dialect_io::range_vars(range),
            scpi_commands::dmm_measure_resistance_4wire(range),
        );
        self.query_f64(&cmd)
    }

    /// Measures temperature (instrument units / °C when configured as Celsius).
    pub fn measure_temperature(&mut self, range: Option<f64>) -> Result<f64> {
        let cmd = self.cmd(
            "measure_temperature",
            &dialect_io::range_vars(range),
            scpi_commands::dmm_measure_temperature(range),
        );
        self.query_f64(&cmd)
    }

    /// Configures DC voltage measurement for faster repeated reads.
    pub fn configure_voltage_dc(
        &mut self,
        range: Option<f64>,
        resolution: Option<f64>,
    ) -> Result<()> {
        let cmd = self.cmd(
            "configure_voltage_dc",
            &dialect_io::range_resolution_vars(range, resolution),
            scpi_commands::dmm_configure_voltage_dc(range, resolution),
        );
        self.session.scpi_mut().write(&cmd)
    }

    /// Configures AC voltage measurement for faster repeated reads.
    pub fn configure_voltage_ac(
        &mut self,
        range: Option<f64>,
        resolution: Option<f64>,
    ) -> Result<()> {
        let cmd = self.cmd(
            "configure_voltage_ac",
            &dialect_io::range_resolution_vars(range, resolution),
            scpi_commands::dmm_configure_voltage_ac(range, resolution),
        );
        self.session.scpi_mut().write(&cmd)
    }

    /// Configures DC current measurement for faster repeated reads.
    pub fn configure_current_dc(
        &mut self,
        range: Option<f64>,
        resolution: Option<f64>,
    ) -> Result<()> {
        let cmd = self.cmd(
            "configure_current_dc",
            &dialect_io::range_resolution_vars(range, resolution),
            scpi_commands::dmm_configure_current_dc(range, resolution),
        );
        self.session.scpi_mut().write(&cmd)
    }

    /// Configures AC current measurement for faster repeated reads.
    pub fn configure_current_ac(
        &mut self,
        range: Option<f64>,
        resolution: Option<f64>,
    ) -> Result<()> {
        let cmd = self.cmd(
            "configure_current_ac",
            &dialect_io::range_resolution_vars(range, resolution),
            scpi_commands::dmm_configure_current_ac(range, resolution),
        );
        self.session.scpi_mut().write(&cmd)
    }

    /// Configures 2-wire resistance measurement for faster repeated reads.
    pub fn configure_resistance(
        &mut self,
        range: Option<f64>,
        resolution: Option<f64>,
    ) -> Result<()> {
        let cmd = self.cmd(
            "configure_resistance",
            &dialect_io::range_resolution_vars(range, resolution),
            scpi_commands::dmm_configure_resistance(range, resolution),
        );
        self.session.scpi_mut().write(&cmd)
    }

    /// Configures 4-wire resistance measurement for faster repeated reads.
    pub fn configure_resistance_4wire(
        &mut self,
        range: Option<f64>,
        resolution: Option<f64>,
    ) -> Result<()> {
        let cmd = self.cmd(
            "configure_resistance_4w",
            &dialect_io::range_resolution_vars(range, resolution),
            scpi_commands::dmm_configure_resistance_4wire(range, resolution),
        );
        self.session.scpi_mut().write(&cmd)
    }

    /// Initiates a measurement (INIT).
    pub fn initiate(&mut self) -> Result<()> {
        let cmd = dialect_io::try_command(self.dialect(), "initiate", scpi_commands::DMM_INITIATE);
        self.session.scpi_mut().write(cmd)
    }

    /// Fetches the last initiated measurement (FETC?).
    pub fn fetch(&mut self) -> Result<f64> {
        self.query_f64(dialect_io::try_command(
            self.dialect(),
            "fetch",
            scpi_commands::DMM_FETCH,
        ))
    }

    /// Reads a measurement immediately (READ?).
    pub fn read(&mut self) -> Result<f64> {
        self.query_f64(dialect_io::try_command(
            self.dialect(),
            "read",
            scpi_commands::DMM_READ,
        ))
    }

    /// Issues a software trigger (*TRG).
    pub fn software_trigger(&mut self) -> Result<()> {
        let cmd = dialect_io::try_command(
            self.dialect(),
            "software_trigger",
            scpi_commands::DMM_SOFTWARE_TRIGGER,
        );
        self.session.scpi_mut().write(cmd)
    }

    fn query_f64(&mut self, cmd: &str) -> Result<f64> {
        let resp = self.session.scpi_mut().query(cmd)?;
        ScpiSession::parse_f64(&resp)
    }
}
