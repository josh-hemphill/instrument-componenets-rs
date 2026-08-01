# Discovery and device assignment

How instruments are found, classified, and rebound after hardware swaps.

## Auto-discovery

=== "Rust"

    Sync: `Discovery::visa()?.scan()?`  
    Async (`tokio` feature): `AsyncDiscovery::visa()?.scan().await?`

=== "C#"

    ```csharp
    using InstrumentComponents.Visa;

    var catalog = VisaDiscovery.Create().Scan();
    ```

Both enumerate (via the VISA resource manager):

- `?*INSTR` (all instruments)
- `USB?*::INSTR`
- `GPIB?*INSTR`
- `ASRL?*INSTR` (serial)

USB and GPIB require **no manual address**. TCPIP/LXI usually does:

=== "Rust"

    ```rust
    Discovery::visa()?
        .manual_address("TCPIP0::192.168.0.42::INSTR")
        .scan()?;
    ```

=== "C#"

    ```csharp
    using InstrumentComponents.Visa;

    var catalog = VisaDiscovery.Create()
        .ManualAddress("TCPIP0::192.168.0.42::INSTR")
        .Scan();
    ```

## Probe policy

Controls how aggressively devices are classified during scan:

| Policy | Behavior |
|---|---|
| `ReadOnly` (default) | Registry + `*IDN?` + benign state queries |
| `None` | Registry + `*IDN?` only |
| `Full` | ReadOnly plus `:MEAS:VOLT:DC?` (triggers acquisition) |

=== "Rust"

    ```rust
    let catalog = Discovery::visa()?
        .probe_policy(ProbePolicy::ReadOnly)
        .scan()?;
    ```

=== "C#"

    ```csharp
    using InstrumentComponents.Probe;
    using InstrumentComponents.Visa;

    var catalog = VisaDiscovery.Create()
        .WithProbePolicy(ProbePolicy.ReadOnly)
        .Scan();
    ```

## Listing devices by type

=== "Rust"

    ```rust
    for dev in catalog.devices_by_kind(InstrumentKind::Dmm) {
        println!("{} — {}", dev.device_id(), dev.address.raw);
    }
    ```

    Serialize for a UI:

    ```rust
    let json = serde_json::to_string_pretty(catalog.devices())?;
    ```

=== "C#"

    ```csharp
    using InstrumentComponents.Kind;

    foreach (var dev in catalog.DevicesByKind(InstrumentKind.Dmm))
    {
        Console.WriteLine($"{dev.GetDeviceId()} — {dev.Address.Raw}");
    }
    ```

## Instrument replacement

`DeviceId` is derived from manufacturer + model + serial (falls back to VISA address):

=== "Rust"

    ```rust
    let id = dev.device_id();
    // save `id` in app config
    let device = catalog.reconnect_by_identity(&id)?;
    let dmm = device.open_dmm()?;
    ```

=== "C#"

    Persist `DeviceId` from a discovered device, rescan after a swap, and open by identity — the VISA address may change but a serial-based ID stays stable.
    See `dotnet/examples/AssignInstruments`.

## User overrides

Force kinds when classification is wrong:

=== "Rust"

    ```rust
    Discovery::visa()?
        .override_kinds("GPIB0::10::INSTR", vec![InstrumentKind::Dmm])
        .scan()?;
    ```

=== "C#"

    ```csharp
    using InstrumentComponents.Kind;
    using InstrumentComponents.Visa;

    var catalog = VisaDiscovery.Create()
        .OverrideKinds("GPIB0::10::INSTR", [InstrumentKind.Dmm])
        .Scan();
    ```

## Adding models to the registry

Edit `crates/instrument-core/data/model_registry.toml` and regenerate shared tables. See the repo `CONTRIBUTING.md`.
