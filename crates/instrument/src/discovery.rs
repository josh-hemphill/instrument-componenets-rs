use crate::catalog::DeviceCatalog;

use instrument_core::address::ResourceAddress;
use instrument_core::classifier::{
    classify_from_address, classify_from_identity, classify_from_transport_hint,
    classify_with_policy, merge_classifications,
};
use instrument_core::connect::ConnectOptions;
use instrument_core::diagnostics::CommsObserver;
use instrument_core::enumerator::{RawResource, ResourceEnumerator};
use instrument_core::error::Result;
use instrument_core::identity::DeviceIdentity;
use instrument_core::ieee4882::Ieee4882;
use instrument_core::kind::InstrumentKind;
use instrument_core::probe_policy::ProbePolicy;
use instrument_core::registry::ModelRegistry;
use instrument_core::session::{InstrumentSession, SessionOpener};
use instrument_core::DiscoveredDevice;
use std::collections::HashMap;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::Arc;
use std::thread;

const DEFAULT_PATTERNS: &[&str] = &["?*INSTR", "USB?*::INSTR", "GPIB?*INSTR", "ASRL?*INSTR"];

/// Discovery builder for scanning and classifying instruments.
pub struct Discovery {
    enumerator: Arc<dyn ResourceEnumerator>,
    opener: Arc<dyn SessionOpener>,
    registry: ModelRegistry,
    manual_addresses: Vec<String>,
    kind_overrides: HashMap<String, Vec<InstrumentKind>>,
    connect_options: ConnectOptions,
    probe_policy: ProbePolicy,
    probe_concurrency: usize,
    observer: Option<Arc<dyn CommsObserver>>,
}

impl Discovery {
    #[cfg(feature = "visa")]
    pub fn visa() -> Result<Self> {
        let rm = instrument_visa::SharedRm::new()?;
        Ok(Self::new(
            Arc::new(instrument_visa::VisaEnumerator::new(rm.clone())),
            Arc::new(instrument_visa::VisaSessionOpener::new(rm)),
            ModelRegistry::embedded(),
        ))
    }

    pub fn new(
        enumerator: Arc<dyn ResourceEnumerator>,
        opener: Arc<dyn SessionOpener>,
        registry: ModelRegistry,
    ) -> Self {
        Self {
            enumerator,
            opener,
            registry,
            manual_addresses: Vec::new(),
            kind_overrides: HashMap::new(),
            connect_options: ConnectOptions::default(),
            probe_policy: ProbePolicy::default(),
            probe_concurrency: 4,
            observer: None,
        }
    }

    pub fn manual_address(mut self, address: impl Into<String>) -> Self {
        self.manual_addresses.push(address.into());
        self
    }

    pub fn override_kinds(
        mut self,
        address: impl Into<String>,
        kinds: Vec<InstrumentKind>,
    ) -> Self {
        self.kind_overrides.insert(address.into(), kinds);
        self
    }

    pub fn with_registry(mut self, registry: ModelRegistry) -> Self {
        self.registry = registry;
        self
    }

    pub fn probe_policy(mut self, policy: ProbePolicy) -> Self {
        self.probe_policy = policy;
        self
    }

    pub fn connect_options(mut self, opts: ConnectOptions) -> Self {
        self.connect_options = opts;
        self
    }

    pub fn observer(mut self, observer: Arc<dyn CommsObserver>) -> Self {
        self.observer = Some(observer);
        self
    }

    pub fn scan(self) -> Result<DeviceCatalog> {
        let mut raw_map: HashMap<u64, RawResource> = HashMap::new();
        for pattern in DEFAULT_PATTERNS {
            if let Ok(list) = self.enumerator.list(pattern) {
                for res in list {
                    raw_map.insert(res.address.dedup_key(), res);
                }
            }
        }

        for manual in &self.manual_addresses {
            let address = ResourceAddress::parse(manual)?;
            raw_map.entry(address.dedup_key()).or_insert(RawResource {
                address,
                identity_hint: instrument_core::transport::TransportIdentity::default(),
            });
        }

        let candidates: Vec<RawResource> = raw_map.into_values().collect();
        let registry = self.registry.clone();
        let overrides = self.kind_overrides.clone();
        let opener = self.opener.clone();
        let opts = self.connect_options.clone();
        let policy = self.probe_policy;
        let concurrency = self.probe_concurrency;

        let devices = probe_devices_parallel(
            candidates,
            registry,
            overrides,
            opener,
            opts,
            policy,
            concurrency,
        )?;

        Ok(DeviceCatalog::from_devices_with_observer(
            self.opener.clone(),
            devices,
            self.observer,
        ))
    }
}

fn probe_devices_parallel(
    candidates: Vec<RawResource>,
    registry: ModelRegistry,
    overrides: HashMap<String, Vec<InstrumentKind>>,
    opener: Arc<dyn SessionOpener>,
    opts: ConnectOptions,
    policy: ProbePolicy,
    concurrency: usize,
) -> Result<Vec<DiscoveredDevice>> {
    if candidates.is_empty() {
        return Ok(Vec::new());
    }

    let chunk_size = (candidates.len() + concurrency - 1) / concurrency;
    let mut handles = Vec::new();
    for chunk in candidates.chunks(chunk_size.max(1)) {
        let chunk = chunk.to_vec();
        let registry = registry.clone();
        let overrides = overrides.clone();
        let opener = opener.clone();
        let opts = opts.clone();
        handles.push(thread::spawn(move || {
            chunk
                .into_iter()
                .map(|raw| {
                    let fallback = raw.clone();
                    match catch_unwind(AssertUnwindSafe(|| {
                        probe_one(raw, &registry, &overrides, opener.as_ref(), &opts, policy)
                    })) {
                        Ok(device) => device,
                        Err(_) => panic_fallback_device(&registry, &overrides, fallback),
                    }
                })
                .collect::<Vec<_>>()
        }));
    }

    let mut devices = Vec::new();
    for (handle_idx, handle) in handles.into_iter().enumerate() {
        match handle.join() {
            Ok(chunk) => devices.extend(chunk),
            Err(_) => {
                tracing::warn!(thread = handle_idx, "discovery probe thread panicked");
            }
        }
    }

    Ok(devices)
}

fn probe_one(
    raw: RawResource,
    registry: &ModelRegistry,
    overrides: &HashMap<String, Vec<InstrumentKind>>,
    opener: &dyn SessionOpener,
    opts: &ConnectOptions,
    policy: ProbePolicy,
) -> DiscoveredDevice {
    let override_kinds = overrides.get(&raw.address.raw).map(|v| v.as_slice());
    let (mut identity, layer1) = classify_from_address(&raw.address, registry);
    let (hint_identity, layer2) = classify_from_transport_hint(&raw.identity_hint, registry);
    identity.merge(&hint_identity);
    let mut layers = vec![layer1, layer2];

    match opener.open(&raw.address, opts) {
        Ok(transport) => {
            match InstrumentSession::new(
                raw.address.clone(),
                transport,
                opts.clone(),
                identity.clone(),
            ) {
                Ok(mut session) => {
                    let _ = session.clear_status();
                    let _ = session.scpi_mut().flush();

                    if let Ok(idn) = Ieee4882::new(session.scpi_mut()).idn() {
                        let (idn_identity, layer4) = classify_from_identity(&idn, registry);
                        identity.merge(&idn_identity);
                        layers.push(layer4);

                        if let Ok(opt) = Ieee4882::new(session.scpi_mut()).options() {
                            identity.options = Some(opt);
                        }
                    }

                    if policy != ProbePolicy::None {
                        let probe_kinds = classify_with_policy(session.scpi_mut(), policy);
                        if !probe_kinds.is_empty() {
                            layers.push(probe_kinds);
                        }
                    }

                    let (supported_kinds, classification) =
                        merge_classifications(layers, override_kinds);

                    DiscoveredDevice {
                        address: raw.address,
                        identity,
                        supported_kinds,
                        classification,
                        reachable: true,
                        error: None,
                    }
                }
                Err(e) => unreachable_device(raw, identity, layers, override_kinds, e.to_string()),
            }
        }
        Err(e) => unreachable_device(raw, identity, layers, override_kinds, e.to_string()),
    }
}

fn panic_fallback_device(
    registry: &ModelRegistry,
    overrides: &HashMap<String, Vec<InstrumentKind>>,
    raw: RawResource,
) -> DiscoveredDevice {
    let override_kinds = overrides.get(&raw.address.raw).map(|v| v.as_slice());
    let (mut identity, layer1) = classify_from_address(&raw.address, registry);
    let (hint_identity, layer2) = classify_from_transport_hint(&raw.identity_hint, registry);
    identity.merge(&hint_identity);
    unreachable_device(
        raw,
        identity,
        vec![layer1, layer2],
        override_kinds,
        "probe panicked".into(),
    )
}

fn unreachable_device(
    raw: RawResource,
    identity: DeviceIdentity,
    layers: Vec<Vec<instrument_core::classifier::ClassifiedKind>>,
    override_kinds: Option<&[InstrumentKind]>,
    error: String,
) -> DiscoveredDevice {
    let (supported_kinds, classification) = merge_classifications(layers, override_kinds);
    DiscoveredDevice {
        address: raw.address,
        identity,
        supported_kinds,
        classification,
        reachable: false,
        error: Some(error),
    }
}
