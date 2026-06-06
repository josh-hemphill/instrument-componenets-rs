//! Lists discovered DMMs with stable device IDs for application role assignment.
#![cfg(feature = "visa")]

use instrument::prelude::*;

fn main() -> Result<()> {
    let catalog = Discovery::visa()?.scan()?;

    println!("Available DMMs:");
    for dev in catalog.devices_by_kind(InstrumentKind::Dmm) {
        println!(
            "  {} — {} @ {} (reachable: {})",
            dev.device_id(),
            dev.identity.model.as_deref().unwrap_or("unknown"),
            dev.address.raw,
            dev.reachable
        );
    }

    if let Some(first) = catalog.devices_by_kind(InstrumentKind::Dmm).first() {
        let role = "main_dmm";
        let device_id = first.device_id();
        println!("\nAssigning role '{role}' to device {device_id}");
        let device_ref = catalog.reconnect_by_identity(&device_id)?;
        println!("  health: {:?}", device_ref.health());
    }

    Ok(())
}
