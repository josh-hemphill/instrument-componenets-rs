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
fn fixture_dmm_depth() {
    // ScriptedFixture emits all on_write steps before on_query steps.
    let fixture = ScriptedFixture::builder()
        .idn("Keysight Technologies", "34461A", "SN1", "1.0")
        .kinds([InstrumentKind::Dmm])
        .on_write(":CONF:VOLT:AC")
        .on_write("INIT")
        .on_write("*TRG")
        .on_query(":MEAS:CURR:AC?", "0.012")
        .on_query(":MEAS:RES?", "1000.0")
        .on_query(":MEAS:FRES?", "999.5")
        .on_query(":MEAS:TEMP?", "25.0")
        .on_query("FETC?", "1.234")
        .on_query("READ?", "2.345")
        .build();

    let catalog = DeviceCatalog::from_fixture("mock://dmm-depth", fixture).unwrap();
    let mut dmm = catalog.open_dmm("mock://dmm-depth").unwrap();
    dmm.configure_voltage_ac(None, None).unwrap();
    dmm.initiate().unwrap();
    dmm.software_trigger().unwrap();
    assert!((dmm.measure_current_ac(None).unwrap() - 0.012).abs() < 1e-9);
    assert!((dmm.measure_resistance_2wire(None).unwrap() - 1000.0).abs() < 1e-9);
    assert!((dmm.measure_resistance_4wire(None).unwrap() - 999.5).abs() < 1e-9);
    assert!((dmm.measure_temperature(None).unwrap() - 25.0).abs() < 1e-9);
    assert!((dmm.fetch().unwrap() - 1.234).abs() < 1e-9);
    assert!((dmm.read().unwrap() - 2.345).abs() < 1e-9);
}

#[test]
fn fixture_psu_depth() {
    let fixture = ScriptedFixture::builder()
        .idn("Keysight Technologies", "E36312A", "SN1", "1.0")
        .kinds([InstrumentKind::DcPowerSupply])
        .on_write(":SOUR1:VOLT:PROT 5.5")
        .on_write(":SOUR1:VOLT:PROT:STAT ON")
        .on_write(":OUTP1:SENS ON")
        .on_query(":OUTP1?", "1")
        .on_query(":SOUR1:VOLT:PROT:STAT?", "ON")
        .build();

    let catalog = DeviceCatalog::from_fixture("mock://psu-depth", fixture).unwrap();
    let mut psu = catalog.open_dc_power_supply("mock://psu-depth").unwrap();
    assert_eq!(psu.channel_count(), 1);
    psu.ovp_level(1, 5.5).unwrap();
    psu.ovp_enable(1, true).unwrap();
    psu.sense_enable(1, true).unwrap();
    assert!(psu.output_state_query(1).unwrap());
    assert!(psu.ovp_query(1).unwrap());
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

#[test]
fn catalog_preserves_connect_options() {
    let fixture = ScriptedFixture::builder()
        .idn("Acme", "DMM1", "SN1", "1.0")
        .kinds([InstrumentKind::Dmm])
        .on_query(":MEAS:VOLT:DC?", "1.0")
        .build();
    let mut opts = ConnectOptions::default();
    opts.retries = 9;
    opts.write_timeout = std::time::Duration::from_secs(3);
    let catalog = DeviceCatalog::from_fixture("mock://dmm", fixture)
        .unwrap()
        .with_connect_options(opts.clone());
    assert_eq!(catalog.connect_options().retries, 9);
    let session = catalog
        .device("mock://dmm")
        .unwrap()
        .open_session()
        .unwrap();
    assert_eq!(session.scpi().options().retries, 9);
    assert_eq!(session.scpi().options().io_timeout(), opts.io_timeout());
}

#[test]
fn fixture_dmm_measure_with_range_falls_back() {
    let fixture = ScriptedFixture::builder()
        .idn("Keysight Technologies", "34461A", "SN1", "1.0")
        .kinds([InstrumentKind::Dmm])
        .on_query(":MEAS:VOLT:DC? 10", "1.234")
        .build();
    let catalog = DeviceCatalog::from_fixture("mock://dmm-range", fixture).unwrap();
    let mut dmm = catalog.open_dmm("mock://dmm-range").unwrap();
    let volts = dmm.measure_voltage_dc(Some(10.0)).unwrap();
    assert!((volts - 1.234).abs() < 1e-9);
}

#[test]
fn fixture_fgen_read_frequency_falls_back() {
    let fixture = ScriptedFixture::builder()
        .idn("Keysight Technologies", "33522B", "SN1", "1.0")
        .kinds([InstrumentKind::FunctionGenerator])
        .on_query(":SOUR:FREQ?", "1000.0")
        .build();
    let catalog = DeviceCatalog::from_fixture("mock://fgen-freq", fixture).unwrap();
    let mut fgen = catalog.open_function_generator("mock://fgen-freq").unwrap();
    let hz = fgen.read_frequency().unwrap();
    assert!((hz - 1000.0).abs() < 1e-9);
}

#[test]
fn fixture_scope_read_timebase_falls_back() {
    let fixture = ScriptedFixture::builder()
        .idn("Rigol Technologies", "DS1054Z", "SN1", "1.0")
        .kinds([InstrumentKind::Oscilloscope])
        .on_query(":TIMebase:SCALe?", "0.001")
        .build();
    let catalog = DeviceCatalog::from_fixture("mock://scope-tb", fixture).unwrap();
    let mut scope = catalog.open_oscilloscope("mock://scope-tb").unwrap();
    let scale = scope.read_timebase_scale().unwrap();
    assert!((scale - 0.001).abs() < 1e-12);
}

#[test]
fn fixture_dmm_dialect_wins_over_generic() {
    let fixture = ScriptedFixture::builder()
        .idn("TestDialect Corp", "DMM-X", "SN1", "1.0")
        .kinds([InstrumentKind::Dmm])
        .on_write(":CONF:VOLT:DC 10")
        .on_query(":MEAS:VOLT:DC:TEST?", "1.0")
        .on_query("READ?", "2.345")
        .build();
    let catalog = DeviceCatalog::from_fixture("mock://dmm-dialect", fixture).unwrap();
    let mut dmm = catalog.open_dmm("mock://dmm-dialect").unwrap();
    dmm.configure_voltage_dc(Some(10.0), Some(0.001)).unwrap();
    assert!((dmm.measure_voltage_dc(None).unwrap() - 1.0).abs() < 1e-9);
    assert!((dmm.read().unwrap() - 2.345).abs() < 1e-9);
}

#[test]
fn fixture_psu_dialect_wins_over_generic() {
    let fixture = ScriptedFixture::builder()
        .idn("TestDialect Corp", "PSU-X", "SN1", "1.0")
        .kinds([InstrumentKind::DcPowerSupply])
        .on_write(":VOLT 3.3,(@1)")
        .build();
    let catalog = DeviceCatalog::from_fixture("mock://psu-dialect", fixture).unwrap();
    let mut psu = catalog.open_dc_power_supply("mock://psu-dialect").unwrap();
    psu.set_voltage(1, 3.3).unwrap();
}
