//! Async mock fixture — no VISA required.
use instrument::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let fixture = ScriptedFixture::builder()
        .idn("Acme Corp", "DMM1", "SN1", "1.0")
        .kinds([InstrumentKind::Dmm])
        .on_query(":MEAS:VOLT:DC?", "3.300")
        .build();

    let catalog = AsyncDeviceCatalog::from_fixture("mock://dmm", fixture).await?;
    let mut dmm = catalog.open_dmm("mock://dmm").await?;
    let volts = dmm.measure_voltage_dc(None).await?;
    println!("{volts} V");
    Ok(())
}
