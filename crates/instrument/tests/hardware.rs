//! Integration tests requiring NI-VISA or Keysight VISA installed.
//!
//! GitHub-hosted CI never enables the `visa` feature for these tests.
//! The self-hosted `Hardware smoke` workflow runs `dmm_measure_voltage_dc_smoke`
//! with `INSTRUMENT_RESOURCE` set to one DMM (`--ignored`).
//! C# `HardwareFact` skips when the env var is unset; this Rust test asserts.
#![cfg(feature = "visa")]

use instrument::prelude::*;
use instrument_core::{ModelRegistry, StaticEnumerator};
use instrument_visa::{SharedRm, VisaSessionOpener};
use std::sync::Arc;

const RESOURCE_ENV: &str = "INSTRUMENT_RESOURCE";
/// Rejects Keithley-style overload sentinels such as 9.9e37.
const MAX_ABS_VOLTS: f64 = 1_000_000.0;

fn require_instrument_resource() -> String {
    let raw = std::env::var(RESOURCE_ENV).unwrap_or_default();
    let trimmed = raw.trim();
    assert!(
        !trimmed.is_empty(),
        "{RESOURCE_ENV} must be set to a VISA resource string"
    );
    ResourceAddress::parse(trimmed)
        .unwrap_or_else(|e| panic!("{RESOURCE_ENV} is not a valid VISA address ({trimmed}): {e}"));
    trimmed.to_string()
}

fn catalog_for_resource(resource: &str) -> DeviceCatalog {
    let rm = SharedRm::new().expect("VISA resource manager");
    let opener = Arc::new(VisaSessionOpener::new(rm));
    let enumerator = Arc::new(
        StaticEnumerator::from_addresses([resource.to_string()])
            .expect("INSTRUMENT_RESOURCE parses"),
    );
    Discovery::new(enumerator, opener, ModelRegistry::embedded())
        .probe_policy(ProbePolicy::ReadOnly)
        .scan()
        .expect("scan INSTRUMENT_RESOURCE")
}

fn device_for_resource(catalog: &DeviceCatalog, resource: &str) -> DeviceRef {
    if let Ok(dev) = catalog.device(resource) {
        return dev;
    }
    let wanted = ResourceAddress::parse(resource).expect("valid resource");
    let raw = catalog
        .devices()
        .iter()
        .find(|d| d.address.dedup_key() == wanted.dedup_key())
        .map(|d| d.address.raw.clone())
        .unwrap_or_else(|| panic!("{RESOURCE_ENV} not in catalog: {resource}"));
    catalog.device(&raw).expect("matched INSTRUMENT_RESOURCE")
}

fn expected_vendor_dmm_dialect(model: Option<&str>) -> Option<&'static str> {
    let model = model.unwrap_or("").to_ascii_lowercase();
    if model.contains("dmm6500") {
        Some("keithley_dmm6500")
    } else {
        None
    }
}

fn assert_sane_dc_voltage(volts: f64, model: Option<&str>) {
    assert!(
        volts.is_finite(),
        "DMM reading was not finite: {volts} (model {model:?})"
    );
    assert!(
        volts.abs() < MAX_ABS_VOLTS,
        "DMM reading looks like overload/sentinel: {volts} (model {model:?})"
    );
}

#[test]
#[ignore = "requires VISA runtime and connected instruments"]
fn discover_real_devices() {
    let catalog = Discovery::visa()
        .unwrap()
        .probe_policy(ProbePolicy::ReadOnly)
        .scan()
        .unwrap();

    catalog.print_summary();

    for dev in catalog.devices() {
        if dev.reachable {
            let health = catalog.health(&dev.address.raw).unwrap();
            assert!(
                health.total_operations == 0 || health.is_healthy() || health.last_error.is_some()
            );
        }
    }
}

#[test]
#[ignore = "requires VISA runtime and connected DMM"]
fn idn_round_trip_on_first_reachable_device() {
    let catalog = Discovery::visa().unwrap().scan().unwrap();
    let device = catalog
        .devices()
        .iter()
        .find(|d| d.reachable)
        .expect("no reachable devices");

    let mut session = catalog
        .device(&device.address.raw)
        .unwrap()
        .open_session()
        .unwrap();
    let _idn = session.idn().expect("idn round-trip");
}

#[test]
#[ignore = "requires VISA runtime and INSTRUMENT_RESOURCE"]
fn dmm_measure_voltage_dc_smoke() {
    let resource = require_instrument_resource();
    let catalog = catalog_for_resource(&resource);
    let device = device_for_resource(&catalog, &resource);
    let discovered = device.discovered();
    assert!(
        discovered.reachable,
        "{RESOURCE_ENV} is not reachable ({resource}): {:?}",
        discovered.error
    );
    assert!(
        discovered.supported_kinds.contains(&InstrumentKind::Dmm),
        "{RESOURCE_ENV} is not classified as a DMM (kinds {:?}, model {:?})",
        discovered.supported_kinds,
        discovered.identity.model
    );

    let mut dmm = device.open_dmm().expect("open DMM");
    let dialect = dmm.session().dialect_for(InstrumentKind::Dmm);
    let volts = dmm.measure_voltage_dc(None).expect("measure DC voltage");
    assert_sane_dc_voltage(volts, discovered.identity.model.as_deref());
    eprintln!(
        "hardware smoke: {} {} dialect={} @ {resource} → {volts} V DC",
        discovered.identity.manufacturer.as_deref().unwrap_or("?"),
        discovered.identity.model.as_deref().unwrap_or("?"),
        dialect.id,
    );
    if let Some(expected) = expected_vendor_dmm_dialect(discovered.identity.model.as_deref()) {
        assert_eq!(
            dialect.id, expected,
            "live IDN looks like a G vendor DMM but resolved dialect {}",
            dialect.id
        );
    }
}
