use instrument::prelude::*;

#[test]
fn mock_catalog_opens_counter() {
    let fixture = ScriptedFixture::builder()
        .idn("Keysight Technologies", "53230A", "SN001", "1.0")
        .kinds([InstrumentKind::Counter])
        .on_query(":MEASure:FREQuency?", "1000.0")
        .build();
    let catalog = DeviceCatalog::from_fixture("mock://counter-1", fixture).unwrap();
    let mut counter = catalog.open_counter("mock://counter-1").unwrap();
    let freq = counter.measure_frequency().unwrap();
    assert!((freq - 1000.0).abs() < 1e-6);
}

#[test]
fn mock_catalog_opens_switch() {
    let fixture = ScriptedFixture::builder()
        .idn("Keysight Technologies", "34970A", "SN001", "1.0")
        .kinds([InstrumentKind::Switch])
        .on_query(":ROUTe:CLOS? (@(1,2))", "1")
        .build();
    let catalog = DeviceCatalog::from_fixture("mock://switch-1", fixture).unwrap();
    let mut sw = catalog.open_switch("mock://switch-1").unwrap();
    assert!(sw.is_closed(1, 2).unwrap());
}

#[test]
fn mock_catalog_opens_oscilloscope() {
    let fixture = ScriptedFixture::builder()
        .idn("Rigol Technologies", "DS1054Z", "SN001", "1.0")
        .kinds([InstrumentKind::Oscilloscope])
        .on_write(":TIMebase:SCALe 0.001")
        .on_write(":WAVeform:SOURce CHAN1")
        .on_write(":WAVeform:FORMat ASCii")
        .on_query(":WAVeform:PREamble?", "0,0,3,0,1e-6,0,0,1,0,0")
        .on_query(":WAVeform:DATA?", "1.0,2.0,3.0")
        .build();
    let catalog = DeviceCatalog::from_fixture("mock://scope-1", fixture).unwrap();
    let mut scope = catalog.open_oscilloscope("mock://scope-1").unwrap();
    scope.set_timebase_scale(1e-3).unwrap();
    let trace = scope.capture_voltage_trace(1).unwrap();
    assert_eq!(trace.samples.len(), 3);
    assert!((trace.sample_interval_s - 1e-6).abs() < 1e-12);
}
