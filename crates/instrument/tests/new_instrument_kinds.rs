use instrument::prelude::*;

#[test]
fn mock_catalog_opens_counter() {
    let fixture = ScriptedFixture::builder()
        .idn("Keysight Technologies", "53230A", "SN001", "1.0")
        .kinds([InstrumentKind::Counter])
        .on_write(":SENSe:FREQuency:APERture 0.1")
        .on_write(":SENSe:FUNCtion:ON \"FREQ 1\"")
        .on_query(":MEASure:FREQuency?", "1000.0")
        .build();
    let catalog = DeviceCatalog::from_fixture("mock://counter-1", fixture).unwrap();
    let mut counter = catalog.open_counter("mock://counter-1").unwrap();
    counter.set_gate_time(0.1).unwrap();
    counter.select_channel(1).unwrap();
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
    assert_eq!(Switch::path_label(1, 2), "(@(1,2))");
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

#[test]
fn mock_catalog_scope_trigger_and_measure() {
    let fixture = ScriptedFixture::builder()
        .idn("Rigol Technologies", "DS1054Z", "SN001", "1.0")
        .kinds([InstrumentKind::Oscilloscope])
        .on_write(":CHANnel1:DISP ON")
        .on_write(":CHANnel1:COUP DC")
        .on_write(":TRIGger:EDGE:SOURce CHAN1")
        .on_write(":TRIGger:EDGE:LEVel 0.5")
        .on_write(":TRIGger:EDGE:SLOPe POS")
        .on_write(":SINGle")
        .on_query(":MEASure:VPP? CHAN1", "2.0")
        .on_query(":MEASure:FREQuency? CHAN1", "1000.0")
        .build();
    let catalog = DeviceCatalog::from_fixture("mock://scope-2", fixture).unwrap();
    let mut scope = catalog.open_oscilloscope("mock://scope-2").unwrap();
    scope.set_channel_display(1, true).unwrap();
    scope.set_channel_coupling(1, "DC").unwrap();
    scope.set_trigger_source("CHAN1").unwrap();
    scope.set_trigger_level(0.5).unwrap();
    scope.set_trigger_slope("POS").unwrap();
    scope.single().unwrap();
    assert!((scope.measure_vpp(1).unwrap() - 2.0).abs() < 1e-9);
    assert!((scope.measure_frequency(1).unwrap() - 1000.0).abs() < 1e-9);
}

#[test]
fn mock_catalog_opens_fgen_depth() {
    let fixture = ScriptedFixture::builder()
        .idn("Keysight Technologies", "33522B", "SN1", "1.0")
        .kinds([InstrumentKind::FunctionGenerator])
        .on_write(":SOUR:FUNC:SQU:DCYC 25")
        .on_write(":OUTP:LOAD 50")
        .on_write(":SOUR:BURS:NCYC 4")
        .on_write(":SOUR:BURS:STAT ON")
        .on_write(":TRIG:SOUR BUS")
        .build();
    let catalog = DeviceCatalog::from_fixture("mock://fgen-1", fixture).unwrap();
    let mut fgen = catalog.open_function_generator("mock://fgen-1").unwrap();
    fgen.set_duty_cycle(25.0).unwrap();
    fgen.set_load(50.0).unwrap();
    fgen.set_burst_count(4).unwrap();
    fgen.set_burst_state(true).unwrap();
    fgen.set_burst_trigger_source("BUS").unwrap();
}

#[test]
fn mock_catalog_opens_power_meter() {
    let fixture = ScriptedFixture::builder()
        .idn("Keysight Technologies", "U2001A", "SN1", "1.0")
        .kinds([InstrumentKind::PowerMeter])
        .on_write(":UNIT:POW DBM")
        .on_write(":SENS:POW:RANG:AUTO ON")
        .on_write(":SENS:AVER:COUN:AUTO ON")
        .on_write(&format!(":SENS:FREQ {}", 1e9))
        .on_query("READ?", "-10.5")
        .build();
    let catalog = DeviceCatalog::from_fixture("mock://pm-1", fixture).unwrap();
    let mut pm = catalog.open_power_meter("mock://pm-1").unwrap();
    pm.configure_measurement(PowerUnit::Dbm, true, true, Some(1e9), None)
        .unwrap();
    assert!((pm.read().unwrap() + 10.5).abs() < 1e-9);
}

#[test]
fn mock_catalog_opens_spectrum_analyzer() {
    let fixture = ScriptedFixture::builder()
        .idn("Keysight Technologies", "N9010B", "SN1", "1.0")
        .kinds([InstrumentKind::SpectrumAnalyzer])
        .on_write(&format!(":FREQ:CENT {}", 1e9))
        .on_write(&format!(":FREQ:SPAN {}", 1e6))
        .on_write(&format!(":BAND {}", 1000.0))
        .on_write(&format!(":BAND:VID {}", 1000.0))
        .on_write(&format!(":DISP:WIND:TRAC:Y:RLEV {}", 0.0))
        .on_write(":INIT:CONT OFF")
        .on_write(":INIT:IMM")
        .on_write(":CALC:MARK:MAX")
        .on_query("*OPC?", "1")
        .on_query(":CALC:MARK:X?", "1e9")
        .on_query(":CALC:MARK:Y?", "-20")
        .on_query(":TRAC:DATA? TRACE1", "-80,-70,-60")
        .build();
    let catalog = DeviceCatalog::from_fixture("mock://sa-1", fixture).unwrap();
    let mut sa = catalog.open_spectrum_analyzer("mock://sa-1").unwrap();
    sa.set_center_frequency(1e9).unwrap();
    sa.set_span(1e6).unwrap();
    sa.set_rbw(1000.0).unwrap();
    sa.set_vbw(1000.0).unwrap();
    sa.set_ref_level(0.0).unwrap();
    sa.sweep_continuous(false).unwrap();
    sa.single_sweep().unwrap();
    sa.marker_peak().unwrap();
    sa.wait_opc().unwrap();
    assert!((sa.marker_x().unwrap() - 1e9).abs() < 1.0);
    assert!((sa.marker_y().unwrap() + 20.0).abs() < 1e-9);
    let trace = sa.fetch_trace_ascii().unwrap();
    assert_eq!(trace.len(), 3);
}

#[test]
fn registry_hint_for_power_meter_and_specan() {
    let registry = instrument_core::ModelRegistry::embedded();
    let pm = registry
        .lookup_model("Keysight Technologies", "U2001A")
        .unwrap();
    assert!(pm.contains(&InstrumentKind::PowerMeter));
    let sa = registry
        .lookup_model("Keysight Technologies", "N9010B")
        .unwrap();
    assert!(sa.contains(&InstrumentKind::SpectrumAnalyzer));
}
