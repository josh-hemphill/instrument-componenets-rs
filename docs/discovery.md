# Discovery and device assignment

## Auto-discovery

Sync: `Discovery::visa()?.scan()?`  
Async (`tokio` feature): `AsyncDiscovery::visa()?.scan().await?`

Both enumerate:

- `?*INSTR` (all instruments)
- `USB?*::INSTR`
- `GPIB?*INSTR`
- `ASRL?*INSTR` (serial)

USB and GPIB require **no manual address**. TCPIP/LXI usually does:

```rust
Discovery::visa()?
    .manual_address("TCPIP0::192.168.0.42::INSTR")
    .scan()?;
```

## Probe policy

Controls how aggressively devices are classified during scan:

| Policy | Behavior |
|---|---|
| `ReadOnly` (default) | Registry + `*IDN?` + benign state queries |
| `None` | Registry + `*IDN?` only |
| `Full` | ReadOnly plus `:MEAS:VOLT:DC?` (triggers acquisition) |

```rust
let catalog = Discovery::visa()?
    .probe_policy(ProbePolicy::ReadOnly)
    .scan()?;
```

## Listing devices by type

```rust
for dev in catalog.devices_by_kind(InstrumentKind::Dmm) {
    println!("{} — {}", dev.device_id(), dev.address.raw);
}
```

Serialize for a UI:

```rust
let json = serde_json::to_string_pretty(catalog.devices())?;
```

## Instrument replacement

`DeviceId` is derived from manufacturer + model + serial (falls back to VISA address):

```rust
let id = dev.device_id();
// save `id` in app config
let device = catalog.reconnect_by_identity(&id)?;
let dmm = device.open_dmm()?;
```

When a DMM is swapped, rescan and match by `DeviceId` — the VISA address may change but serial-based ID stays stable.

## User overrides

Force kinds when classification is wrong:

```rust
Discovery::visa()?
    .override_kinds("GPIB0::10::INSTR", vec![InstrumentKind::Dmm])
    .scan()?;
```

## Adding models to the registry

Edit `crates/instrument-core/data/model_registry.toml` and add entries. See [CONTRIBUTING.md](../CONTRIBUTING.md).
