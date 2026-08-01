```rust
use instrument::prelude::*;

fn main() -> Result<()> {
    let fixture = ScriptedFixture::builder()
        .idn("Keysight Technologies", "34401A", "SN1", "1.0")
        .kinds([InstrumentKind::Dmm])
        .on_query("*IDN?", "Keysight Technologies,34401A,SN1,1.0")
        .on_query(":MEAS:VOLT:DC?", "1.234")
        .build();

    let catalog = DeviceCatalog::from_fixture("mock://dmm-1", fixture)?;
    let volts = catalog.open_dmm("mock://dmm-1")?.measure_voltage_dc(None)?;
    println!("{volts} V");
    Ok(())
}
```
