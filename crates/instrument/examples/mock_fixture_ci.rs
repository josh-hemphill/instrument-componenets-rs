//! Consumer CI pattern: test application logic without VISA installed.
use instrument::prelude::*;

fn main() -> Result<()> {
    let fixture = ScriptedFixture::builder()
        .idn("Acme Corp", "SMU2602", "SN001", "1.0")
        .kinds([InstrumentKind::Dmm, InstrumentKind::DcPowerSupply])
        .on_query(":MEAS:VOLT:DC?", "3.300")
        .build();

    let catalog = DeviceCatalog::from_fixture("mock://smu-1", fixture)?;
    let mut dmm = catalog.open_dmm("mock://smu-1")?;
    let volts = dmm.measure_voltage_dc(None)?;
    println!("DUT voltage: {volts} V");

    Ok(())
}
