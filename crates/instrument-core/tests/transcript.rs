use instrument_core::mock::MockTransport;
use instrument_core::Transcript;

fn load_fixture(name: &str) -> Transcript {
    let path = format!("{}/../../fixtures/{name}", env!("CARGO_MANIFEST_DIR"));
    let json = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {path}: {e}"));
    Transcript::from_json(&json).unwrap()
}

#[test]
fn loads_recorded_fixture_json() {
    let transcript = load_fixture("smu2602.json");
    let transport = MockTransport::from_script(transcript.steps);
    assert_eq!(transport.script().len(), 2);
}

#[test]
fn loads_scope_switch_counter_fixtures() {
    assert_eq!(load_fixture("scope_ds1054z.json").steps.len(), 7);
    assert_eq!(load_fixture("switch_34970a.json").steps.len(), 2);
    assert_eq!(load_fixture("counter_53230a.json").steps.len(), 2);
}

#[test]
fn loads_dmm_psu_vendor_fixtures() {
    assert_eq!(load_fixture("dmm_dmm6500.json").steps.len(), 4);
    assert_eq!(load_fixture("psu_n6705c.json").steps.len(), 4);
}
