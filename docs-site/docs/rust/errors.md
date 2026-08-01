# Errors (Rust)

Core errors live in `instrument_core::Error` (re-exported via `instrument::prelude::*` as `Error` / `Result`).

## Common variants

| Variant | When |
|---|---|
| `Error::Comm` | I/O failed with address + command context |
| `Error::Timeout` | Operation timed out |
| `Error::UnsupportedKind` | Device does not support requested class |
| `Error::DeviceNotFound` | Address or `DeviceId` not in catalog |
| `Error::Backend` | VISA / backend error (boxed) |
| `Error::ScpiCommand` | SCPI command failed with message |
| `Error::Parse` | Response could not be parsed |
| `Error::Transport` | Low-level transport I/O / closed |
| `Error::MockExhausted` / `MockMismatch` | Scripted fixture mismatch |

## Pattern matching

```rust
use instrument::prelude::*;

match catalog.open_dmm(&addr) {
    Ok(mut dmm) => {
        let v = dmm.measure_voltage_dc(None)?;
        println!("{v}");
    }
    Err(Error::UnsupportedKind { address, kind, supported }) => {
        eprintln!("{address}: {kind:?} not in {supported:?}");
    }
    Err(Error::Comm { address, command, attempts, source }) => {
        eprintln!("comm @ {address} after {attempts} tries ({command:?}): {source}");
    }
    Err(e) => return Err(e),
}
```

## Comm context

Retries wrap underlying failures with `Error::with_comm_context(address, command, attempts)` so technicians see which device and SCPI string failed.
