use instrument::prelude::*;

#[tokio::test]
async fn fixture_dmm_measure_async() {
    let fixture = ScriptedFixture::builder()
        .idn("Acme Corp", "SMU2602", "SN001", "1.0")
        .kinds([InstrumentKind::Dmm, InstrumentKind::DcPowerSupply])
        .on_query(":MEAS:VOLT:DC?", "3.300")
        .build();

    let catalog = AsyncDeviceCatalog::from_fixture("mock://smu-1", fixture)
        .await
        .unwrap();
    let mut dmm = catalog.open_dmm("mock://smu-1").await.unwrap();
    let volts = dmm.measure_voltage_dc(None).await.unwrap();
    assert!((volts - 3.3).abs() < f64::EPSILON);
}

#[tokio::test]
async fn fixture_dmm_measure_with_range_falls_back_async() {
    let fixture = ScriptedFixture::builder()
        .idn("Keysight Technologies", "34461A", "SN1", "1.0")
        .kinds([InstrumentKind::Dmm])
        .on_query(":MEAS:VOLT:DC? 10", "1.234")
        .build();
    let catalog = AsyncDeviceCatalog::from_fixture("mock://dmm-range", fixture)
        .await
        .unwrap();
    let mut dmm = catalog.open_dmm("mock://dmm-range").await.unwrap();
    let volts = dmm.measure_voltage_dc(Some(10.0)).await.unwrap();
    assert!((volts - 1.234).abs() < 1e-9);
}
