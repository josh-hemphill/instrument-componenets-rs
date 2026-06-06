use instrument_core::error::Result;
use instrument_core::scpi::ScpiSession;
use instrument_core::InstrumentSession;

/// DC power supply session view (IVI-inspired / SCPI :SOURce, :OUTPut).
pub struct DcPowerSupply {
    session: InstrumentSession,
}

impl DcPowerSupply {
    pub fn new(session: InstrumentSession) -> Self {
        Self { session }
    }

    pub fn session(&self) -> &InstrumentSession {
        &self.session
    }

    pub fn session_mut(&mut self) -> &mut InstrumentSession {
        &mut self.session
    }

    /// Sets output voltage in volts (SI) on the given channel (1-based).
    pub fn set_voltage(&mut self, channel: u32, volts: f64) -> Result<()> {
        self.session
            .scpi_mut()
            .write(&format!(":SOUR{channel}:VOLT {volts}"))
    }

    /// Sets current limit in amps (SI) on the given channel.
    pub fn set_current_limit(&mut self, channel: u32, amps: f64) -> Result<()> {
        self.session
            .scpi_mut()
            .write(&format!(":SOUR{channel}:CURR {amps}"))
    }

    /// Enables or disables output on the given channel.
    pub fn output_enable(&mut self, channel: u32, enabled: bool) -> Result<()> {
        let state = if enabled { "ON" } else { "OFF" };
        self.session
            .scpi_mut()
            .write(&format!(":OUTP{channel} {state}"))
    }

    /// Reads measured output voltage in volts (SI).
    pub fn read_voltage(&mut self, channel: u32) -> Result<f64> {
        let resp = self
            .session
            .scpi_mut()
            .query(&format!(":MEAS:VOLT? {channel}"))?;
        ScpiSession::parse_f64(&resp)
    }

    /// Reads measured output current in amps (SI).
    pub fn read_current(&mut self, channel: u32) -> Result<f64> {
        let resp = self
            .session
            .scpi_mut()
            .query(&format!(":MEAS:CURR? {channel}"))?;
        ScpiSession::parse_f64(&resp)
    }
}
