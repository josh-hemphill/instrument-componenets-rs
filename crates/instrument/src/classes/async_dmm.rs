use instrument_core::error::Result;
use instrument_core::scpi::AsyncScpiSession;
use instrument_core::scpi_commands;
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
        let cmd = scpi_commands::dmm_measure_voltage_dc(range);
        self.query_f64(&cmd).await
    }

    /// Measures AC voltage in volts (SI).
    pub async fn measure_voltage_ac(&mut self, range: Option<f64>) -> Result<f64> {
        let cmd = scpi_commands::dmm_measure_voltage_ac(range);
        self.query_f64(&cmd).await
    }

    /// Measures DC current in amps (SI).
    pub async fn measure_current_dc(&mut self, range: Option<f64>) -> Result<f64> {
        let cmd = scpi_commands::dmm_measure_current_dc(range);
        self.query_f64(&cmd).await
    }

    /// Measures AC current in amps (SI).
    pub async fn measure_current_ac(&mut self, range: Option<f64>) -> Result<f64> {
        let cmd = scpi_commands::dmm_measure_current_ac(range);
        self.query_f64(&cmd).await
    }

    /// Measures 2-wire resistance in ohms (SI).
    pub async fn measure_resistance(&mut self, range: Option<f64>) -> Result<f64> {
        self.measure_resistance_2wire(range).await
    }

    /// Measures 2-wire resistance in ohms (SI).
    pub async fn measure_resistance_2wire(&mut self, range: Option<f64>) -> Result<f64> {
        let cmd = scpi_commands::dmm_measure_resistance_2wire(range);
        self.query_f64(&cmd).await
    }

    /// Measures 4-wire resistance in ohms (SI).
    pub async fn measure_resistance_4wire(&mut self, range: Option<f64>) -> Result<f64> {
        let cmd = scpi_commands::dmm_measure_resistance_4wire(range);
        self.query_f64(&cmd).await
    }

    /// Measures temperature (instrument units / °C when configured as Celsius).
    pub async fn measure_temperature(&mut self, range: Option<f64>) -> Result<f64> {
        let cmd = scpi_commands::dmm_measure_temperature(range);
        self.query_f64(&cmd).await
    }

    /// Configures DC voltage measurement for faster repeated reads.
    pub async fn configure_voltage_dc(
        &mut self,
        range: Option<f64>,
        resolution: Option<f64>,
    ) -> Result<()> {
        let cmd = scpi_commands::dmm_configure_voltage_dc(range, resolution);
        self.session.scpi_mut().write(&cmd).await
    }

    /// Configures AC voltage measurement for faster repeated reads.
    pub async fn configure_voltage_ac(
        &mut self,
        range: Option<f64>,
        resolution: Option<f64>,
    ) -> Result<()> {
        let cmd = scpi_commands::dmm_configure_voltage_ac(range, resolution);
        self.session.scpi_mut().write(&cmd).await
    }

    /// Configures DC current measurement for faster repeated reads.
    pub async fn configure_current_dc(
        &mut self,
        range: Option<f64>,
        resolution: Option<f64>,
    ) -> Result<()> {
        let cmd = scpi_commands::dmm_configure_current_dc(range, resolution);
        self.session.scpi_mut().write(&cmd).await
    }

    /// Configures AC current measurement for faster repeated reads.
    pub async fn configure_current_ac(
        &mut self,
        range: Option<f64>,
        resolution: Option<f64>,
    ) -> Result<()> {
        let cmd = scpi_commands::dmm_configure_current_ac(range, resolution);
        self.session.scpi_mut().write(&cmd).await
    }

    /// Configures 2-wire resistance measurement for faster repeated reads.
    pub async fn configure_resistance(
        &mut self,
        range: Option<f64>,
        resolution: Option<f64>,
    ) -> Result<()> {
        let cmd = scpi_commands::dmm_configure_resistance(range, resolution);
        self.session.scpi_mut().write(&cmd).await
    }

    /// Configures 4-wire resistance measurement for faster repeated reads.
    pub async fn configure_resistance_4wire(
        &mut self,
        range: Option<f64>,
        resolution: Option<f64>,
    ) -> Result<()> {
        let cmd = scpi_commands::dmm_configure_resistance_4wire(range, resolution);
        self.session.scpi_mut().write(&cmd).await
    }

    /// Initiates a measurement (INIT).
    pub async fn initiate(&mut self) -> Result<()> {
        self.session
            .scpi_mut()
            .write(scpi_commands::DMM_INITIATE)
            .await
    }

    /// Fetches the last initiated measurement (FETC?).
    pub async fn fetch(&mut self) -> Result<f64> {
        self.query_f64(scpi_commands::DMM_FETCH).await
    }

    /// Reads a measurement immediately (READ?).
    pub async fn read(&mut self) -> Result<f64> {
        self.query_f64(scpi_commands::DMM_READ).await
    }

    /// Issues a software trigger (*TRG).
    pub async fn software_trigger(&mut self) -> Result<()> {
        self.session
            .scpi_mut()
            .write(scpi_commands::DMM_SOFTWARE_TRIGGER)
            .await
    }

    async fn query_f64(&mut self, cmd: &str) -> Result<f64> {
        let resp = self.session.scpi_mut().query(cmd).await?;
        AsyncScpiSession::parse_f64(&resp)
    }
}
