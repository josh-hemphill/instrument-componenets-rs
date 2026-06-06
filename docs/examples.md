# Examples

Run from the repository root.

## Mock / CI (no VISA)

```bash
cargo run -p instrument-components --no-default-features --example mock_fixture_ci
cargo run -p instrument-components --no-default-features --features tokio --example mock_fixture_ci_async
```

## Hardware (requires VISA + instruments)

```bash
cargo run -p instrument-components --features visa --example discover
cargo run -p instrument-components --features visa --example dmm_measure
cargo run -p instrument-components --features visa --example manual_tcpip
cargo run -p instrument-components --features visa --example assign_instruments
cargo run -p instrument-components --features visa,tokio --example discover_async
```

| Example | What it demonstrates |
|---|---|
| `mock_fixture_ci` | Scripted fixture for CI pipelines |
| `mock_fixture_ci_async` | Async mock fixture + `AsyncDmm` |
| `discover_async` | Async VISA scan |
| `discover` | VISA scan and print summary |
| `dmm_measure` | Open a DMM and measure DC voltage |
| `manual_tcpip` | Manual TCPIP address + discovery |
| `assign_instruments` | List DMMs by `DeviceId` for app assignment |
