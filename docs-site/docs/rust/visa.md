# VISA (Rust)

Hardware I/O goes through the `visa` feature (on by default) and the `instrument-visa` crate, which wraps [visa-rs](https://crates.io/crates/visa-rs).

## Install

1. Install [NI-VISA](https://www.ni.com/en-us/support/downloads/drivers/download.ni-visa.html) or [Keysight IO Libraries](https://www.keysight.com/us/en/lib/software-detail/computer-software/io-libraries-suite-downloads-2175637.html).
2. Depend on the facade crate:

```toml
[dependencies]
instrument-components = "0.1"
```

## Linking

| Variable | Purpose |
|---|---|
| `LIB_VISA_PATH` | Directory containing `visa64.lib` / `visa.lib` |
| `LIB_VISA_NAME` | Library name (`visa64`, `visa32`, `visa`) |

CI runners typically use `default-features = false` so they never link a native VISA library.

## Discovery entry point

```rust
use instrument::prelude::*;

let catalog = Discovery::visa()?.scan()?;
catalog.print_summary();
```

Async: `AsyncDiscovery::visa()?.scan().await?` with the `tokio` feature. Session **open** is still sync; subsequent read/write are async.

## Feature flags

| Feature | Default | Description |
|---|---|---|
| `visa` | yes | VISA backend via `instrument-visa` |
| `tokio` | no | Async API over visa-rs tokio adapter |
| `cross-compile` | no | Cross-compile enum repr for visa-rs |
| `record` | no | Record real I/O into mock scripts |

```toml
# Cross-compile example
instrument-components = { version = "0.2", features = ["visa", "cross-compile"] }
```

```bash
cargo build --features cross-compile --target x86_64-pc-windows-gnu
```

You still need a target VISA import library at link time (`LIB_VISA_PATH`).

## Mock without VISA

```toml
instrument-components = { version = "0.1", default-features = false }
```

Use `ScriptedFixture` / `DeviceCatalog::from_fixture` — see [getting started](../getting-started.md).
