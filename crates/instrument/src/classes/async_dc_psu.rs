use instrument_core::dialect::resolve_dialect;
use instrument_core::error::Result;
use instrument_core::kind::InstrumentKind;
use instrument_core::scpi::AsyncScpiSession;
use instrument_core::scpi_commands;
use instrument_core::AsyncInstrumentSession;

use super::dc_psu::parse_on_off;

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

    /// Returns channel count from the resolved dialect profile (default 1).
    pub fn channel_count(&self) -> u32 {
        let identity = self.session.identity();
        resolve_dialect(
            InstrumentKind::DcPowerSupply,
            identity.manufacturer.as_deref(),
            identity.model.as_deref(),
        )
        .channels
        .max(1)
    }

    /// Sets output voltage in volts (SI) on the given channel (1-based).
    pub async fn set_voltage(&mut self, channel: u32, volts: f64) -> Result<()> {
        self.session
            .scpi_mut()
            .write(&scpi_commands::psu_set_voltage(channel, volts))
            .await
    }

    /// Sets current limit in amps (SI) on the given channel.
    pub async fn set_current_limit(&mut self, channel: u32, amps: f64) -> Result<()> {
        self.session
            .scpi_mut()
            .write(&scpi_commands::psu_set_current_limit(channel, amps))
            .await
    }

    /// Enables or disables output on the given channel.
    pub async fn output_enable(&mut self, channel: u32, enabled: bool) -> Result<()> {
        let state = if enabled { "ON" } else { "OFF" };
        self.session
            .scpi_mut()
            .write(&scpi_commands::psu_output_enable(channel, state))
            .await
    }

    /// Queries whether output is enabled on the given channel.
    pub async fn output_state_query(&mut self, channel: u32) -> Result<bool> {
        let resp = self
            .session
            .scpi_mut()
            .query(&scpi_commands::psu_output_state_query(channel))
            .await?;
        parse_on_off(&resp)
    }

    /// Sets over-voltage protection level in volts (SI).
    pub async fn ovp_level(&mut self, channel: u32, volts: f64) -> Result<()> {
        self.session
            .scpi_mut()
            .write(&scpi_commands::psu_ovp_level(channel, volts))
            .await
    }

    /// Enables or disables over-voltage protection.
    pub async fn ovp_enable(&mut self, channel: u32, enabled: bool) -> Result<()> {
        let state = if enabled { "ON" } else { "OFF" };
        self.session
            .scpi_mut()
            .write(&scpi_commands::psu_ovp_enable(channel, state))
            .await
    }

    /// Queries whether over-voltage protection is enabled.
    pub async fn ovp_query(&mut self, channel: u32) -> Result<bool> {
        let resp = self
            .session
            .scpi_mut()
            .query(&scpi_commands::psu_ovp_query(channel))
            .await?;
        parse_on_off(&resp)
    }

    /// Enables or disables remote sense on the given channel.
    pub async fn sense_enable(&mut self, channel: u32, enabled: bool) -> Result<()> {
        let state = if enabled { "ON" } else { "OFF" };
        self.session
            .scpi_mut()
            .write(&scpi_commands::psu_sense_enable(channel, state))
            .await
    }

    /// Reads measured output voltage in volts (SI).
    pub async fn read_voltage(&mut self, channel: u32) -> Result<f64> {
        let resp = self
            .session
            .scpi_mut()
            .query(&scpi_commands::psu_read_voltage(channel))
            .await?;
        AsyncScpiSession::parse_f64(&resp)
    }

    /// Reads measured output current in amps (SI).
    pub async fn read_current(&mut self, channel: u32) -> Result<f64> {
        let resp = self
            .session
            .scpi_mut()
            .query(&scpi_commands::psu_read_current(channel))
            .await?;
        AsyncScpiSession::parse_f64(&resp)
    }
}
