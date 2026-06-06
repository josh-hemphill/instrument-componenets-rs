//! Measure DC voltage from the first discovered DMM.
use instrument::prelude::*;

fn main() -> Result<()> {
    let catalog = Discovery::visa()?.scan()?;
    let dmms = catalog.devices_by_kind(InstrumentKind::Dmm);
    let Some(dmm_dev) = dmms.first() else {
        eprintln!("no DMM found");
        return Ok(());
    };

    let addr = &dmm_dev.address.raw;
    let mut dmm = catalog.open_dmm(addr)?;
    let volts = dmm.measure_voltage_dc(None)?;
    println!("{addr}: {volts} V DC");
    Ok(())
}
