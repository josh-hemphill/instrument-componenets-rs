use crate::classes::{
    AsyncCounter, AsyncDcPowerSupply, AsyncDmm, AsyncFunctionGenerator, AsyncOscilloscope,
    AsyncSwitch,
};
use instrument_core::async_session::AsyncSessionOpener;
use instrument_core::connect::ConnectOptions;
use instrument_core::diagnostics::{CommsObserver, DeviceHealth, Diagnostics};
use instrument_core::error::Result;
use instrument_core::kind::InstrumentKind;
use instrument_core::session::ensure_kind_supported;
use instrument_core::{AsyncInstrumentSession, AsyncSessionPool, DiscoveredDevice};
use std::sync::{Arc, Mutex};

/// Async handle to a catalog device — cheap to clone, opens sessions on demand.
#[derive(Clone)]
pub struct AsyncDeviceRef {
    pub(crate) device: DiscoveredDevice,
    pub(crate) opener: Arc<dyn AsyncSessionOpener>,
    pub(crate) connect_options: ConnectOptions,
    pub(crate) health: Arc<Mutex<DeviceHealth>>,
    pub(crate) observer: Option<Arc<dyn CommsObserver>>,
}

impl AsyncDeviceRef {
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

    /// Opens a new independent async session to this device.
    pub async fn open_session(&self) -> Result<AsyncInstrumentSession> {
        let transport = self
            .opener
            .open(&self.device.address, &self.connect_options)
            .await?;
        AsyncInstrumentSession::new_with_diagnostics(
            self.device.address.clone(),
            transport,
            self.connect_options.clone(),
            self.device.identity.clone(),
            Some(self.diagnostics()),
        )
        .await
    }

    pub async fn session_pool(&self) -> Result<AsyncSessionPool> {
        Ok(AsyncSessionPool::new(self.open_session().await?))
    }

    pub async fn open_dmm(&self) -> Result<AsyncDmm> {
        ensure_kind_supported(
            &self.device.address,
            InstrumentKind::Dmm,
            &self.device.supported_kinds,
        )?;
        Ok(AsyncDmm::new(self.open_session().await?))
    }

    pub async fn open_dc_power_supply(&self) -> Result<AsyncDcPowerSupply> {
        ensure_kind_supported(
            &self.device.address,
            InstrumentKind::DcPowerSupply,
            &self.device.supported_kinds,
        )?;
        Ok(AsyncDcPowerSupply::new(self.open_session().await?))
    }

    pub async fn open_function_generator(&self) -> Result<AsyncFunctionGenerator> {
        ensure_kind_supported(
            &self.device.address,
            InstrumentKind::FunctionGenerator,
            &self.device.supported_kinds,
        )?;
        Ok(AsyncFunctionGenerator::new(self.open_session().await?))
    }

    pub async fn open_oscilloscope(&self) -> Result<AsyncOscilloscope> {
        ensure_kind_supported(
            &self.device.address,
            InstrumentKind::Oscilloscope,
            &self.device.supported_kinds,
        )?;
        Ok(AsyncOscilloscope::new(self.open_session().await?))
    }

    pub async fn open_switch(&self) -> Result<AsyncSwitch> {
        ensure_kind_supported(
            &self.device.address,
            InstrumentKind::Switch,
            &self.device.supported_kinds,
        )?;
        Ok(AsyncSwitch::new(self.open_session().await?))
    }

    pub async fn open_counter(&self) -> Result<AsyncCounter> {
        ensure_kind_supported(
            &self.device.address,
            InstrumentKind::Counter,
            &self.device.supported_kinds,
        )?;
        Ok(AsyncCounter::new(self.open_session().await?))
    }

    pub async fn open_untyped(&self) -> Result<AsyncInstrumentSession> {
        self.open_session().await
    }
}
