use instrument_core::connect::ConnectOptions;
use instrument_core::error::{Error, Result, TransportError};
use instrument_core::mock::{MockTransport, ScriptStep};
use instrument_core::scpi::ScpiSession;
use instrument_core::transport::Transport;
use std::sync::{Arc, Mutex};
use std::time::Duration;

struct TimeoutSpy {
    last: Arc<Mutex<Option<Duration>>>,
}

impl Transport for TimeoutSpy {
    fn write(&mut self, _data: &[u8]) -> Result<()> {
        Ok(())
    }

    fn read(&mut self, _buf: &mut [u8]) -> Result<usize> {
        Err(Error::Transport(TransportError::Closed))
    }

    fn clear(&mut self) -> Result<()> {
        Ok(())
    }

    fn set_read_timeout(&mut self, timeout: Duration) -> Result<()> {
        *self.last.lock().expect("timeout spy") = Some(timeout);
        Ok(())
    }
}

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

#[test]
fn probe_opc_failure_restores_io_timeout() {
    let last = Arc::new(Mutex::new(None));
    let opts = ConnectOptions {
        read_timeout: Duration::from_secs(9),
        write_timeout: Duration::from_secs(4),
        reconnect_on_failure: false,
        ..Default::default()
    };
    let expected = opts.io_timeout();

    let mut session = ScpiSession::new(Box::new(TimeoutSpy { last: last.clone() }), opts).unwrap();
    assert!(!session.probe_opc());
    assert_eq!(*last.lock().expect("timeout spy"), Some(expected));
}
