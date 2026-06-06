use instrument_core::connect::ConnectOptions;
use instrument_core::mock::{MockTransport, ScriptStep};
use instrument_core::scpi::ScpiSession;
use std::time::Duration;

#[test]
fn query_retries_after_timeout_then_succeeds() {
    let transport = MockTransport::from_script(vec![
        ScriptStep::Write {
            data: ":MEAS:VOLT:DC?\n".into(),
        },
        ScriptStep::Read {
            data: "1.0\n".into(),
        },
        ScriptStep::Write {
            data: ":MEAS:VOLT:DC?\n".into(),
        },
        ScriptStep::Read {
            data: "1.0\n".into(),
        },
    ])
    .fail_writes(1);

    let mut opts = ConnectOptions::default();
    opts.retries = 1;
    opts.retry_backoff = Duration::from_millis(1);

    let mut session = ScpiSession::new(Box::new(transport), opts).unwrap();
    let volts = session.query(":MEAS:VOLT:DC?").unwrap();
    assert_eq!(volts.trim(), "1.0");
}
