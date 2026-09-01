use instrument_core::connect::ConnectOptions;
use instrument_core::error::{Error, Result, TransportError};
use instrument_core::ieee4882::Ieee4882;
use instrument_core::mock::{MockTransport, ScriptStep};
use instrument_core::scpi::{is_opc_supported_reply, is_syst_err_supported_reply, ScpiSession};
use instrument_core::transport::Transport;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

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

struct ZeroByteTransport;

impl Transport for ZeroByteTransport {
    fn write(&mut self, _data: &[u8]) -> Result<()> {
        Ok(())
    }

    fn read(&mut self, _buf: &mut [u8]) -> Result<usize> {
        Ok(0)
    }

    fn clear(&mut self) -> Result<()> {
        Ok(())
    }

    fn set_read_timeout(&mut self, _timeout: Duration) -> Result<()> {
        Ok(())
    }
}

fn retry_opts() -> ConnectOptions {
    ConnectOptions {
        retries: 1,
        retry_backoff: Duration::from_millis(1),
        reconnect_on_failure: false,
        ..Default::default()
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
fn query_retries_read_timeout_flushes_stale_then_succeeds() {
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
            data: "3.3\n".into(),
        },
    ])
    .fail_reads(1);

    let mut session = ScpiSession::new(Box::new(transport), retry_opts()).unwrap();
    let volts = session.query(":MEAS:VOLT:DC?").unwrap();
    assert_eq!(volts.trim(), "3.3");
}

#[test]
fn query_read_retries_exhausted_is_timeout() {
    let transport = MockTransport::from_script(vec![
        ScriptStep::Write {
            data: ":MEAS:VOLT:DC?\n".into(),
        },
        ScriptStep::Write {
            data: ":MEAS:VOLT:DC?\n".into(),
        },
    ])
    .fail_reads(2);

    let mut session = ScpiSession::new(Box::new(transport), retry_opts()).unwrap();
    let err = session.query(":MEAS:VOLT:DC?").unwrap_err();
    assert!(matches!(err, Error::Timeout));
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

#[test]
fn probe_opc_undefined_header_is_unsupported() {
    let transport = MockTransport::from_script(vec![
        ScriptStep::Write {
            data: "*OPC?\n".into(),
        },
        ScriptStep::Read {
            data: "-113,\"Undefined header\"\n".into(),
        },
    ]);
    let mut session = ScpiSession::new(Box::new(transport), retry_opts()).unwrap();
    assert!(!session.probe_opc());
    Ieee4882::new(&mut session).wait_complete().unwrap();
}

#[test]
fn probe_opc_one_is_supported() {
    let transport = MockTransport::from_script(vec![
        ScriptStep::Write {
            data: "*OPC?\n".into(),
        },
        ScriptStep::Read { data: "1\n".into() },
    ]);
    let mut session = ScpiSession::new(Box::new(transport), retry_opts()).unwrap();
    assert!(session.probe_opc());
}

#[test]
fn probe_syst_err_ok_is_unsupported() {
    let transport = MockTransport::from_script(vec![
        ScriptStep::Write {
            data: "SYST:ERR?\n".into(),
        },
        ScriptStep::Read {
            data: "OK\n".into(),
        },
    ]);
    let mut session = ScpiSession::new(Box::new(transport), retry_opts()).unwrap();
    assert!(!session.probe_syst_err());
}

#[test]
fn probe_syst_err_no_error_is_supported() {
    let transport = MockTransport::from_script(vec![
        ScriptStep::Write {
            data: "SYST:ERR?\n".into(),
        },
        ScriptStep::Read {
            data: "0,\"No error\"\n".into(),
        },
    ]);
    let mut session = ScpiSession::new(Box::new(transport), retry_opts()).unwrap();
    assert!(session.probe_syst_err());
}

#[test]
fn zero_byte_read_is_timeout_without_spin() {
    let mut opts = retry_opts();
    opts.retries = 0;
    opts.read_timeout = Duration::from_secs(10);
    let mut session = ScpiSession::new(Box::new(ZeroByteTransport), opts).unwrap();
    let started = Instant::now();
    let err = session.query("*IDN?").unwrap_err();
    assert!(matches!(err, Error::Timeout));
    assert!(
        started.elapsed() < Duration::from_secs(2),
        "zero-byte read spun instead of failing closed"
    );
}

#[test]
fn opc_and_syst_err_reply_parsers() {
    assert!(is_opc_supported_reply("1"));
    assert!(is_opc_supported_reply("+1"));
    assert!(!is_opc_supported_reply("-113,\"Undefined header\""));
    assert!(is_syst_err_supported_reply("0,\"No error\""));
    assert!(!is_syst_err_supported_reply("OK"));
}
