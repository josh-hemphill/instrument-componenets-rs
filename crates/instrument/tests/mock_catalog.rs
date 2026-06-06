use instrument::prelude::*;

#[test]
fn fixture_dmm_measure() {
    let fixture = ScriptedFixture::builder()
        .idn("Acme Corp", "SMU2602", "SN001", "1.0")
        .kinds([InstrumentKind::Dmm, InstrumentKind::DcPowerSupply])
        .on_query(":MEAS:VOLT:DC?", "3.300")
        .build();

    let catalog = DeviceCatalog::from_fixture("mock://smu-1", fixture).unwrap();
    let mut dmm = catalog.open_dmm("mock://smu-1").unwrap();
    let volts = dmm.measure_voltage_dc(None).unwrap();
    assert!((volts - 3.3).abs() < f64::EPSILON);
}

#[test]
fn multi_session_same_device() {
    let fixture = ScriptedFixture::builder()
        .idn("Acme", "PSU", "1", "1.0")
        .kinds([InstrumentKind::Dmm, InstrumentKind::DcPowerSupply])
        .on_query(":MEAS:VOLT:DC?", "1.0")
        .on_write(":SOUR1:VOLT 3.3")
        .build();

    let catalog = DeviceCatalog::from_fixture("mock://dev", fixture).unwrap();
    let dev = catalog.device("mock://dev").unwrap();
    let _dmm = dev.open_dmm().unwrap();
    let _psu = dev.open_dc_power_supply().unwrap();
}
