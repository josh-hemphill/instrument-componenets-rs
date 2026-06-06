use instrument_core::error::Result;
use instrument_core::scpi::AsyncScpiSession;
use instrument_core::AsyncInstrumentSession;

/// Async DC power supply session view (IVI-inspired / SCPI :SOURce, :OUTPut).
pub struct AsyncDcPowerSupply {
    session: AsyncInstrumentSession,
}

impl AsyncDcPowerSupply {
    pub fn new(session: AsyncInstrumentSession) -> Self {
        Self { session }
    }

    pub fn session(&self) -> &AsyncInstrumentSession {
        &self.session
    }

    pub fn session_mut(&mut self) -> &mut AsyncInstrumentSession {
        &mut self.session
    }

    /// Sets output voltage in volts (SI) on the given channel (1-based).
    pub async fn set_voltage(&mut self, channel: u32, volts: f64) -> Result<()> {
        self.session
            .scpi_mut()
            .write(&format!(":SOUR{channel}:VOLT {volts}"))
            .await
    }

    /// Sets current limit in amps (SI) on the given channel.
    pub async fn set_current_limit(&mut self, channel: u32, amps: f64) -> Result<()> {
        self.session
            .scpi_mut()
            .write(&format!(":SOUR{channel}:CURR {amps}"))
            .await
    }

    /// Enables or disables output on the given channel.
    pub async fn output_enable(&mut self, channel: u32, enabled: bool) -> Result<()> {
        let state = if enabled { "ON" } else { "OFF" };
        self.session
            .scpi_mut()
            .write(&format!(":OUTP{channel} {state}"))
            .await
    }

    /// Reads measured output voltage in volts (SI).
    pub async fn read_voltage(&mut self, channel: u32) -> Result<f64> {
        let resp = self
            .session
            .scpi_mut()
            .query(&format!(":MEAS:VOLT? {channel}"))
            .await?;
        AsyncScpiSession::parse_f64(&resp)
    }

    /// Reads measured output current in amps (SI).
    pub async fn read_current(&mut self, channel: u32) -> Result<f64> {
        let resp = self
            .session
            .scpi_mut()
            .query(&format!(":MEAS:CURR? {channel}"))
            .await?;
        AsyncScpiSession::parse_f64(&resp)
    }
}
