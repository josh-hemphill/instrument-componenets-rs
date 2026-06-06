# API overview

Quick reference for the main public types. Full rustdoc on [docs.rs](https://docs.rs/instrument-components) after publish.

## Prelude

```rust
use instrument::prelude::*;
```

Re-exports: `Discovery`, `DeviceCatalog`, `DeviceRef`, `Dmm`, `DcPowerSupply`, `FunctionGenerator`, `InstrumentKind`, `ProbePolicy`, `ScriptedFixture`, `Error`, `Result`, and more.

With `tokio`: `AsyncDiscovery`, `AsyncDeviceCatalog`, `AsyncDeviceRef`, `AsyncDmm`, `AsyncDcPowerSupply`, `AsyncFunctionGenerator`, `AsyncScpiSession`, `AsyncInstrumentSession`.

## Discovery and catalog

| Type | Description |
|---|---|
| `Discovery` | Builder: scan, manual addresses, probe policy, observer |
| `DeviceCatalog` | Discovered devices; open typed classes; health snapshots |
| `DeviceRef` | Handle to one device; opens sessions on demand |
| `DiscoveredDevice` | Address, identity, kinds, reachability, classification |
| `DeviceId` | Stable ID for instrument replacement |
| `ProbePolicy` | `None`, `ReadOnly`, `Full` capability probing |

## Sessions and transport

| Type | Description |
|---|---|
| `InstrumentSession` | Active session with SCPI and identity |
| `ScpiSession` | Low-level SCPI read/write/query |
| `SessionPool` | Share one session across typed views |
| `Transport` | Sync byte I/O trait (VISA, mock, custom) |
| `AsyncTransport` | Async byte I/O trait (`tokio` feature) |
| `AsyncScpiSession` | Async SCPI framing, retry, diagnostics |
| `AsyncInstrumentSession` | Async session with identity and IEEE 488.2 helpers |
| `AsyncSessionOpener` | Opens `DynAsyncTransport` for an address |
| `ConnectOptions` | Timeouts, retries, terminator, reset-on-connect |

## Typed instrument classes

| Type | Key methods |
|---|---|
| `Dmm` | `measure_voltage_dc`, `measure_voltage_ac`, `measure_current_dc`, `measure_resistance` |
| `DcPowerSupply` | `set_voltage`, `set_current`, `enable_output`, `read_voltage` |
| `FunctionGenerator` | `set_waveform`, `set_frequency`, `set_amplitude`, `enable_output` |

Async counterparts (`AsyncDmm`, etc.) expose the same methods with `.await`.

All numeric APIs use **SI base units** (V, A, Hz, s).

## Diagnostics

| Type | Description |
|---|---|
| `DeviceHealth` | Pollable comms snapshot |
| `CommsObserver` | Push callback trait |
| `CommsEvent` | Single I/O event (address, kind, command, detail) |
| `Diagnostics` | Injected into SCPI sessions |

## Mock / testing

| Type | Description |
|---|---|
| `ScriptedFixture` | Builder for mock instrument scenarios |
| `MockTransport` | Scripted request/response transport |
| `StaticEnumerator` | Fixed resource list for discovery tests |
| `RecordingTransport` | Record real I/O into mock scripts (`record` feature) |

## Errors

| Variant | When |
|---|---|
| `Error::Comm` | I/O failed with address + command context |
| `Error::Timeout` | Operation timed out |
| `Error::UnsupportedKind` | Device does not support requested class |
| `Error::DeviceNotFound` | Address or DeviceId not in catalog |
| `Error::Backend` | VISA backend error (boxed) |
