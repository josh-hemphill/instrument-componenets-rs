# Comms diagnostics

## Pollable health (pull)

Each device in a catalog has a shared health record updated by all sessions:

```rust
let health = catalog.health("USB0::0x0957::0x0607::SN::INSTR")?;
if !health.is_healthy() {
    eprintln!("failures: {}", health.consecutive_failures);
    eprintln!("last error: {:?}", health.last_error);
}
```

`DeviceHealth` fields:

| Field | Meaning |
|---|---|
| `consecutive_failures` | Streak since last success |
| `total_operations` | All I/O attempts |
| `total_failures` | Failed operations |
| `last_error` | Most recent error message |
| `last_success_unix_ms` | Timestamp of last success |
| `last_failure_unix_ms` | Timestamp of last failure |

## Push observer

Register a callback during discovery for real-time comms events:

```rust
use std::sync::Arc;

struct MyObserver;
impl CommsObserver for MyObserver {
    fn on_event(&self, event: &CommsEvent) {
        eprintln!("{:?}", event);
    }
}

let catalog = Discovery::visa()?
    .observer(Arc::new(MyObserver))
    .scan()?;
```

Events are only allocated when an observer is registered or `tracing` is enabled at DEBUG/WARN.

## Structured logging

Enable `tracing` in your app for logs without an observer:

```rust
tracing_subscriber::fmt::init();
```

## Error context

Session-level errors include device address and command:

```rust
match session.idn() {
    Err(e) => eprintln!("{e}"), // Error::Comm with address + command
    Ok(idn) => println!("{idn:?}"),
}
```
