//! Consumer CI pattern: test application logic without VISA installed.
use instrument::prelude::*;

fn main() -> Result<()> {
    let fixture = ScriptedFixture::builder()
        .idn("Acme Corp", "SMU2602", "SN001", "1.0")
        .kinds([InstrumentKind::Dmm, InstrumentKind::DcPowerSupply])
        .on_query(":MEAS:VOLT:DC?", "3.300")
        .on_write(":SOUR1:VOLT 3.3")
        .build();

    let catalog = DeviceCatalog::from_fixture("mock://smu-1", fixture)?;

    let mut psu = catalog.open_dc_power_supply("mock://smu-1")?;
    let mut dmm = catalog.open_dmm("mock://smu-1")?;

    // Application code under test — identical path as production
    psu.set_voltage(1, 3.3)?;

    let volts = dmm.measure_voltage_dc(None)?;
    println!("DUT voltage: {volts} V");

    Ok(())
}
