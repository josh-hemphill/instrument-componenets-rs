use crate::async_catalog::AsyncDeviceCatalog;

use instrument_core::address::ResourceAddress;
use instrument_core::async_session::AsyncSessionOpener;
use instrument_core::classifier::{
    classify_from_address, classify_from_identity, classify_from_transport_hint,
    classify_with_policy_async, merge_classifications,
};
use instrument_core::connect::ConnectOptions;
use instrument_core::diagnostics::CommsObserver;
use instrument_core::enumerator::{RawResource, ResourceEnumerator};
use instrument_core::error::Result;
use instrument_core::identity::DeviceIdentity;
use instrument_core::ieee4882::AsyncIeee4882;
use instrument_core::kind::InstrumentKind;
use instrument_core::probe_policy::ProbePolicy;
use instrument_core::registry::ModelRegistry;
use instrument_core::{AsyncInstrumentSession, DiscoveredDevice};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;

const DEFAULT_PATTERNS: &[&str] = &["?*INSTR", "USB?*::INSTR", "GPIB?*INSTR", "ASRL?*INSTR"];

/// Async discovery builder for scanning and classifying instruments.
pub struct AsyncDiscovery {
    enumerator: Arc<dyn ResourceEnumerator>,
    opener: Arc<dyn AsyncSessionOpener>,
    registry: ModelRegistry,
    manual_addresses: Vec<String>,
    kind_overrides: HashMap<String, Vec<InstrumentKind>>,
    connect_options: ConnectOptions,
    probe_policy: ProbePolicy,
    probe_concurrency: usize,
    observer: Option<Arc<dyn CommsObserver>>,
}

impl AsyncDiscovery {
    #[cfg(feature = "visa")]
    pub fn visa() -> Result<Self> {
        let rm = instrument_visa::SharedRm::new()?;
        Ok(Self::new(
            Arc::new(instrument_visa::VisaEnumerator::new(rm.clone())),
            Arc::new(instrument_visa::VisaAsyncSessionOpener::new(rm)),
            ModelRegistry::embedded(),
        ))
    }

    pub fn new(
        enumerator: Arc<dyn ResourceEnumerator>,
        opener: Arc<dyn AsyncSessionOpener>,
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

    pub async fn scan(self) -> Result<AsyncDeviceCatalog> {
        let enumerator = self.enumerator.clone();
        let manual_addresses = self.manual_addresses.clone();

        let raw_map: HashMap<u64, RawResource> = tokio::task::spawn_blocking(move || {
            let mut map = HashMap::new();
            for pattern in DEFAULT_PATTERNS {
                if let Ok(list) = enumerator.list(pattern) {
                    for res in list {
                        map.insert(res.address.dedup_key(), res);
                    }
                }
            }
            for manual in &manual_addresses {
                if let Ok(address) = ResourceAddress::parse(manual) {
                    map.entry(address.dedup_key()).or_insert(RawResource {
                        address,
                        identity_hint: instrument_core::transport::TransportIdentity::default(),
                    });
                }
            }
            map
        })
        .await
        .map_err(|e| instrument_core::Error::Parse(e.to_string()))?;

        let candidates: Vec<RawResource> = raw_map.into_values().collect();
        let registry = self.registry.clone();
        let overrides = self.kind_overrides.clone();
        let opener = self.opener.clone();
        let opts = self.connect_options.clone();
        let policy = self.probe_policy;
        let concurrency = self.probe_concurrency;

        let devices = probe_devices_concurrent(
            candidates,
            registry,
            overrides,
            opener,
            opts,
            policy,
            concurrency,
        )
        .await?;

        Ok(AsyncDeviceCatalog::from_devices_with_observer(
            self.opener.clone(),
            devices,
            self.observer,
        )
        .with_connect_options(self.connect_options))
    }
}

async fn probe_devices_concurrent(
    candidates: Vec<RawResource>,
    registry: ModelRegistry,
    overrides: HashMap<String, Vec<InstrumentKind>>,
    opener: Arc<dyn AsyncSessionOpener>,
    opts: ConnectOptions,
    policy: ProbePolicy,
    concurrency: usize,
) -> Result<Vec<DiscoveredDevice>> {
    if candidates.is_empty() {
        return Ok(Vec::new());
    }

    let semaphore = Arc::new(Semaphore::new(concurrency.max(1)));
    let mut join_set = JoinSet::new();

    for raw in candidates {
        let permit = semaphore
            .clone()
            .acquire_owned()
            .await
            .map_err(|e| instrument_core::Error::Parse(format!("semaphore closed: {e}")))?;
        let registry = registry.clone();
        let overrides = overrides.clone();
        let opener = opener.clone();
        let opts = opts.clone();

        join_set.spawn(async move {
            let _permit = permit;
            probe_one(raw, &registry, &overrides, opener.as_ref(), &opts, policy).await
        });
    }

    let mut devices = Vec::new();
    while let Some(result) = join_set.join_next().await {
        match result {
            Ok(device) => devices.push(device),
            Err(e) => {
                tracing::warn!(error = %e, "async discovery probe task failed");
            }
        }
    }

    Ok(devices)
}

async fn probe_one(
    raw: RawResource,
    registry: &ModelRegistry,
    overrides: &HashMap<String, Vec<InstrumentKind>>,
    opener: &dyn AsyncSessionOpener,
    opts: &ConnectOptions,
    policy: ProbePolicy,
) -> DiscoveredDevice {
    let override_kinds = overrides.get(&raw.address.raw).map(|v| v.as_slice());
    let (mut identity, layer1) = classify_from_address(&raw.address, registry);
    let (hint_identity, layer2) = classify_from_transport_hint(&raw.identity_hint, registry);
    identity.merge(&hint_identity);
    let mut layers = vec![layer1, layer2];

    match opener.open(&raw.address, opts).await {
        Ok(transport) => {
            match AsyncInstrumentSession::new(
                raw.address.clone(),
                transport,
                opts.clone(),
                identity.clone(),
            )
            .await
            {
                Ok(mut session) => {
                    let _ = session.clear_status().await;
                    let _ = session.scpi_mut().flush().await;

                    if let Ok(idn) = AsyncIeee4882::new(session.scpi_mut()).idn().await {
                        let (idn_identity, layer4) = classify_from_identity(&idn, registry);
                        identity.merge(&idn_identity);
                        layers.push(layer4);

                        if let Ok(opt) = AsyncIeee4882::new(session.scpi_mut()).options().await {
                            identity.options = Some(opt);
                        }
                    }

                    if policy != ProbePolicy::None {
                        let probe_kinds =
                            classify_with_policy_async(session.scpi_mut(), policy).await;
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
