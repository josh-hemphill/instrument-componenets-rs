use crate::classes::{DcPowerSupply, Dmm, FunctionGenerator};
use crate::device::DeviceRef;
use crate::mock_backend::MockSessionOpener;
use instrument_core::address::ResourceAddress;
use instrument_core::classifier::{classify_from_address, merge_classifications};
use instrument_core::connect::ConnectOptions;
use instrument_core::diagnostics::{CommsObserver, DeviceHealth};
use instrument_core::error::Result;
use instrument_core::identity::DeviceId;
use instrument_core::kind::InstrumentKind;
use instrument_core::mock::{mock_address, ScriptedFixture};
use instrument_core::registry::ModelRegistry;
use instrument_core::session::SessionOpener;
use instrument_core::DiscoveredDevice;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Catalog of discovered or injected devices.
pub struct DeviceCatalog {
    opener: Arc<dyn SessionOpener>,
    devices: Vec<DiscoveredDevice>,
    by_address: HashMap<String, usize>,
    by_device_id: HashMap<String, usize>,
    connect_options: ConnectOptions,
    health_registry: HashMap<String, Arc<Mutex<DeviceHealth>>>,
    observer: Option<Arc<dyn CommsObserver>>,
}

impl DeviceCatalog {
    pub fn from_devices(opener: Arc<dyn SessionOpener>, devices: Vec<DiscoveredDevice>) -> Self {
        Self::from_devices_with_observer(opener, devices, None)
    }

    pub fn from_devices_with_observer(
        opener: Arc<dyn SessionOpener>,
        devices: Vec<DiscoveredDevice>,
        observer: Option<Arc<dyn CommsObserver>>,
    ) -> Self {
        let mut by_address = HashMap::new();
        let mut by_device_id = HashMap::new();
        let mut health_registry = HashMap::new();

        for (idx, dev) in devices.iter().enumerate() {
            by_address.insert(dev.address.raw.clone(), idx);
            by_device_id.insert(dev.device_id().0.clone(), idx);
            health_registry.insert(
                dev.address.raw.clone(),
                Arc::new(Mutex::new(DeviceHealth::default())),
            );
        }

        Self {
            opener,
            devices,
            by_address,
            by_device_id,
            connect_options: ConnectOptions::default(),
            health_registry,
            observer,
        }
    }

    pub fn from_fixture(address: &str, fixture: ScriptedFixture) -> Result<Self> {
        let addr = if address.starts_with("mock://") {
            ResourceAddress::parse(address)?
        } else {
            mock_address(address)?
        };

        let idn = fixture.idn().clone();
        let kinds: Vec<_> = fixture.kinds().to_vec();
        let transport = fixture.into_transport();
        let opener = MockSessionOpener::new();
        opener.register(&addr.raw, transport);

        let (identity, layer1) = classify_from_address(&addr, &ModelRegistry::embedded());
        let mut id = identity;
        id.manufacturer = Some(idn.manufacturer.clone());
        id.model = Some(idn.model.clone());
        id.serial = Some(idn.serial.clone());
        id.firmware = Some(idn.firmware.clone());

        let (supported_kinds, classification) = merge_classifications(
            vec![layer1],
            if kinds.is_empty() {
                None
            } else {
                Some(kinds.as_slice())
            },
        );

        let device = DiscoveredDevice {
            address: addr.clone(),
            identity: id,
            supported_kinds,
            classification,
            reachable: true,
            error: None,
        };

        Ok(Self::from_devices(Arc::new(opener), vec![device]))
    }

    pub fn devices(&self) -> &[DiscoveredDevice] {
        &self.devices
    }

    pub fn devices_by_kind(&self, kind: InstrumentKind) -> Vec<&DiscoveredDevice> {
        self.devices
            .iter()
            .filter(|d| d.supported_kinds.contains(&kind))
            .collect()
    }

    pub fn device(&self, address: &str) -> Result<DeviceRef> {
        let idx = self.by_address.get(address).copied().ok_or_else(|| {
            instrument_core::Error::DeviceNotFound {
                address: address.to_string(),
            }
        })?;
        Ok(self.device_at(idx))
    }

    pub fn device_by_id(&self, id: &DeviceId) -> Result<DeviceRef> {
        let idx = self.by_device_id.get(&id.0).copied().ok_or_else(|| {
            instrument_core::Error::DeviceNotFound {
                address: id.0.clone(),
            }
        })?;
        Ok(self.device_at(idx))
    }

    /// Reconnects to a device by stable identity after replacement or address change.
    pub fn reconnect_by_identity(&self, id: &DeviceId) -> Result<DeviceRef> {
        self.device_by_id(id)
    }

    pub fn health(&self, address: &str) -> Result<DeviceHealth> {
        let health = self.health_registry.get(address).ok_or_else(|| {
            instrument_core::Error::DeviceNotFound {
                address: address.to_string(),
            }
        })?;
        Ok(health.lock().unwrap().clone())
    }

    fn device_at(&self, idx: usize) -> DeviceRef {
        let dev = &self.devices[idx];
        let health = self
            .health_registry
            .get(&dev.address.raw)
            .cloned()
            .unwrap_or_else(|| Arc::new(Mutex::new(DeviceHealth::default())));
        DeviceRef {
            device: dev.clone(),
            opener: self.opener.clone(),
            connect_options: self.connect_options.clone(),
            health,
            observer: self.observer.clone(),
        }
    }

    pub fn open_dmm(&self, address: &str) -> Result<Dmm> {
        self.device(address)?.open_dmm()
    }

    pub fn open_dc_power_supply(&self, address: &str) -> Result<DcPowerSupply> {
        self.device(address)?.open_dc_power_supply()
    }

    pub fn open_function_generator(&self, address: &str) -> Result<FunctionGenerator> {
        self.device(address)?.open_function_generator()
    }

    pub fn print_summary(&self) {
        for dev in &self.devices {
            println!(
                "{} ({}) @ {} — kinds: {:?} reachable: {}",
                dev.identity.model.as_deref().unwrap_or("?"),
                dev.device_id(),
                dev.address.raw,
                dev.supported_kinds,
                dev.reachable
            );
        }
    }
}
