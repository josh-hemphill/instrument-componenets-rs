//! Integration tests requiring NI-VISA or Keysight VISA installed.
#![cfg(feature = "visa")]

use instrument::prelude::*;

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
