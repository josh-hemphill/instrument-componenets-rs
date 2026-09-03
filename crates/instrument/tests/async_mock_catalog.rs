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

#[tokio::test]
async fn fixture_fgen_read_frequency_falls_back_async() {
    let fixture = ScriptedFixture::builder()
        .idn("Keysight Technologies", "33522B", "SN1", "1.0")
        .kinds([InstrumentKind::FunctionGenerator])
        .on_query(":SOUR:FREQ?", "1000.0")
        .build();
    let catalog = AsyncDeviceCatalog::from_fixture("mock://fgen-freq", fixture)
        .await
        .unwrap();
    let mut fgen = catalog
        .open_function_generator("mock://fgen-freq")
        .await
        .unwrap();
    let hz = fgen.read_frequency().await.unwrap();
    assert!((hz - 1000.0).abs() < 1e-9);
}

#[tokio::test]
async fn fixture_scope_read_timebase_falls_back_async() {
    let fixture = ScriptedFixture::builder()
        .idn("Rigol Technologies", "DS1054Z", "SN1", "1.0")
        .kinds([InstrumentKind::Oscilloscope])
        .on_query(":TIMebase:SCALe?", "0.001")
        .build();
    let catalog = AsyncDeviceCatalog::from_fixture("mock://scope-tb", fixture)
        .await
        .unwrap();
    let mut scope = catalog.open_oscilloscope("mock://scope-tb").await.unwrap();
    let scale = scope.read_timebase_scale().await.unwrap();
    assert!((scale - 0.001).abs() < 1e-12);
}

#[tokio::test]
async fn fixture_dmm_dialect_wins_over_generic_async() {
    let fixture = ScriptedFixture::builder()
        .idn("TestDialect Corp", "DMM-X", "SN1", "1.0")
        .kinds([InstrumentKind::Dmm])
        .on_write(":CONF:VOLT:DC 10")
        .on_query(":MEAS:VOLT:DC:TEST?", "1.0")
        .on_query("READ?", "2.345")
        .build();
    let catalog = AsyncDeviceCatalog::from_fixture("mock://dmm-dialect", fixture)
        .await
        .unwrap();
    let mut dmm = catalog.open_dmm("mock://dmm-dialect").await.unwrap();
    dmm.configure_voltage_dc(Some(10.0), Some(0.001))
        .await
        .unwrap();
    assert!((dmm.measure_voltage_dc(None).await.unwrap() - 1.0).abs() < 1e-9);
    assert!((dmm.read().await.unwrap() - 2.345).abs() < 1e-9);
}
