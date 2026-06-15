use crate::classes::{Counter, DcPowerSupply, Dmm, FunctionGenerator, Oscilloscope, Switch};
use instrument_core::connect::ConnectOptions;
use instrument_core::diagnostics::{CommsObserver, DeviceHealth, Diagnostics};
use instrument_core::error::Result;
use instrument_core::kind::InstrumentKind;
use instrument_core::session::{
    ensure_kind_supported, InstrumentSession, SessionOpener, SessionPool,
};
use instrument_core::DiscoveredDevice;
use std::sync::{Arc, Mutex};

/// Handle to a catalog device — cheap to clone, opens sessions on demand.
#[derive(Clone)]
pub struct DeviceRef {
    pub(crate) device: DiscoveredDevice,
    pub(crate) opener: Arc<dyn SessionOpener>,
    pub(crate) connect_options: ConnectOptions,
    pub(crate) health: Arc<Mutex<DeviceHealth>>,
    pub(crate) observer: Option<Arc<dyn CommsObserver>>,
}

impl DeviceRef {
    pub fn discovered(&self) -> &DiscoveredDevice {
        &self.device
    }

    pub fn address(&self) -> &instrument_core::ResourceAddress {
        &self.device.address
    }

    pub fn supported_kinds(&self) -> &[InstrumentKind] {
        &self.device.supported_kinds
    }

    pub fn connect_options(&self) -> &ConnectOptions {
        &self.connect_options
    }

    pub fn health(&self) -> DeviceHealth {
        self.health.lock().unwrap().clone()
    }

    pub fn with_connect_options(mut self, opts: ConnectOptions) -> Self {
        self.connect_options = opts;
        self
    }

    fn diagnostics(&self) -> Diagnostics {
        let mut diag = Diagnostics::new(&self.device.address.raw).with_health(self.health.clone());
        if let Some(observer) = &self.observer {
            diag = diag.with_observer(observer.clone());
        }
        diag
    }

    /// Opens a new independent session to this device.
    pub fn open_session(&self) -> Result<InstrumentSession> {
        let transport = self
            .opener
            .open(&self.device.address, &self.connect_options)?;
        InstrumentSession::new_with_diagnostics(
            self.device.address.clone(),
            transport,
            self.connect_options.clone(),
            self.device.identity.clone(),
            Some(self.diagnostics()),
        )
    }

    pub fn session_pool(&self) -> Result<SessionPool> {
        Ok(SessionPool::new(self.open_session()?))
    }

    pub fn open_dmm(&self) -> Result<Dmm> {
        ensure_kind_supported(
            &self.device.address,
            InstrumentKind::Dmm,
            &self.device.supported_kinds,
        )?;
        Ok(Dmm::new(self.open_session()?))
    }

    pub fn open_dc_power_supply(&self) -> Result<DcPowerSupply> {
        ensure_kind_supported(
            &self.device.address,
            InstrumentKind::DcPowerSupply,
            &self.device.supported_kinds,
        )?;
        Ok(DcPowerSupply::new(self.open_session()?))
    }

    pub fn open_function_generator(&self) -> Result<FunctionGenerator> {
        ensure_kind_supported(
            &self.device.address,
            InstrumentKind::FunctionGenerator,
            &self.device.supported_kinds,
        )?;
        Ok(FunctionGenerator::new(self.open_session()?))
    }

    pub fn open_oscilloscope(&self) -> Result<Oscilloscope> {
        ensure_kind_supported(
            &self.device.address,
            InstrumentKind::Oscilloscope,
            &self.device.supported_kinds,
        )?;
        Ok(Oscilloscope::new(self.open_session()?))
    }

    pub fn open_switch(&self) -> Result<Switch> {
        ensure_kind_supported(
            &self.device.address,
            InstrumentKind::Switch,
            &self.device.supported_kinds,
        )?;
        Ok(Switch::new(self.open_session()?))
    }

    pub fn open_counter(&self) -> Result<Counter> {
        ensure_kind_supported(
            &self.device.address,
            InstrumentKind::Counter,
            &self.device.supported_kinds,
        )?;
        Ok(Counter::new(self.open_session()?))
    }

    pub fn open_untyped(&self) -> Result<InstrumentSession> {
        self.open_session()
    }
}
