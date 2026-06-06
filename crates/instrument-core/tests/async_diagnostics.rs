use instrument_core::connect::ConnectOptions;
use instrument_core::diagnostics::{
    CommsEvent, CommsEventKind, CommsObserver, DeviceHealth, Diagnostics,
};
use instrument_core::mock::{MockTransport, ScriptStep};
use instrument_core::AsyncScpiSession;
use std::sync::{Arc, Mutex};

struct TestObserver {
    events: Arc<Mutex<Vec<CommsEvent>>>,
}

impl CommsObserver for TestObserver {
    fn on_event(&self, event: &CommsEvent) {
        self.events.lock().unwrap().push(event.clone());
    }
}

#[tokio::test]
async fn async_diagnostics_records_failures_and_observer_events() {
    let health = Arc::new(Mutex::new(DeviceHealth::default()));
    let events = Arc::new(Mutex::new(Vec::new()));
    let observer = Arc::new(TestObserver {
        events: events.clone(),
    });

    let diag = Diagnostics::new("mock://dmm-1")
        .with_health(health.clone())
        .with_observer(observer);

    let transport = MockTransport::from_script(vec![
        ScriptStep::Write {
            data: "*IDN?\n".into(),
        },
        ScriptStep::Read {
            data: "Acme,123,SN,1.0\n".into(),
        },
    ])
    .fail_writes(5);

    let opts = ConnectOptions {
        retries: 0,
        ..ConnectOptions::default()
    };
    let mut session = AsyncScpiSession::new(Box::new(transport), opts)
        .await
        .unwrap()
        .with_diagnostics(diag);

    let result = session.query("*IDN?").await;
    assert!(result.is_err());

    let h = health.lock().unwrap();
    assert_eq!(h.consecutive_failures, 1);
    assert_eq!(h.total_failures, 1);
    assert!(h.last_error.is_some());

    let recorded = events.lock().unwrap();
    assert!(recorded.iter().any(|e| e.kind == CommsEventKind::Timeout));
}
