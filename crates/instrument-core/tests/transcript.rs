use instrument_core::mock::MockTransport;
use instrument_core::Transcript;

#[test]
fn loads_recorded_fixture_json() {
    let json = include_str!("../../../fixtures/smu2602.json");
    let transcript = Transcript::from_json(json).unwrap();
    let transport = MockTransport::from_script(transcript.steps);
    let script = transport.script();
    assert_eq!(script.len(), 2);
}
