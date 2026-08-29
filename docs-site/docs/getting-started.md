# Getting started

Walkthrough for a first measurement with a mock fixture, then real hardware.

## Prerequisites

=== "Rust"

    - Rust 1.70+ (`rustup update stable`)
    - For hardware: NI-VISA or Keysight IO Libraries installed

=== "C#"

    - .NET 8 SDK
    - For hardware: NI-VISA, Keysight IO Libraries, or another VISA that provides the VISA.NET shared components expected by [IviFoundation.Visa](https://www.nuget.org/packages/IviFoundation.Visa)

## Step 1: Add the dependency

=== "Rust"

    **Mock / CI (no VISA):**

    ```toml
    [dependencies]
    instrument-components = { version = "0.1", default-features = false }
    ```

    **Hardware (VISA default):**

    ```toml
    [dependencies]
    instrument-components = "0.1"
    ```

=== "C#"

    Package publishing is deferred. From the repo:

    ```bash
    cd dotnet
    # Mock / CI
    dotnet run --project examples/MockFixtureCi

    # Hardware
    dotnet run --project examples/Discover
    ```

    Or project-reference `InstrumentComponents` (and `InstrumentComponents.Visa` for hardware).

## Step 2: Mock path (recommended first)

=== "Rust"

    --8<-- "snippets/rust/mock-dmm.md"

    Run: `cargo run` (no extra features needed).

=== "C#"

    --8<-- "snippets/csharp/mock-dmm.md"

    Or: `dotnet run --project examples/MockFixtureCi`

## Step 3: Discover real instruments

=== "Rust"

    ```rust
    use instrument::prelude::*;

    fn main() -> Result<()> {
        let catalog = Discovery::visa()?.scan()?;
        for dev in catalog.devices() {
            println!(
                "{} @ {} — {:?}",
                dev.identity.model.as_deref().unwrap_or("?"),
                dev.address.raw,
                dev.supported_kinds,
            );
        }
        Ok(())
    }
    ```

    USB and GPIB are found automatically. Add TCPIP manually:

    ```rust
    Discovery::visa()?
        .manual_address("TCPIP0::192.168.0.42::INSTR")
        .scan()?;
    ```

=== "C#"

    ```csharp
    using InstrumentComponents.Visa;

    var catalog = VisaDiscovery.Create().Scan();
    catalog.PrintSummary();
    ```

    ```bash
    dotnet run --project examples/Discover
    ```

## Step 4: Open a typed class

=== "Rust"

    ```rust
    let catalog = Discovery::visa()?.scan()?;
    let dmms = catalog.devices_by_kind(InstrumentKind::Dmm);
    if let Some(dev) = dmms.first() {
        let mut dmm = catalog.open_dmm(&dev.address.raw)?;
        let v = dmm.measure_voltage_dc(None)?;
        println!("{v} V");
    }
    ```

=== "C#"

    ```csharp
    using InstrumentComponents.Kind;
    using InstrumentComponents.Visa;

    var catalog = VisaDiscovery.Create().Scan();
    var dmm = catalog.OpenDmm(
        catalog.DevicesByKind(InstrumentKind.Dmm)[0].Address.Raw);
    Console.WriteLine(dmm.MeasureVoltageDc());
    ```

## Step 5: Assign instruments in an app

Use stable `DeviceId` so replacing hardware does not break saved config:

=== "Rust"

    ```rust
    let device_id = dev.device_id();
    // persist device_id in your app config
    let dmm = catalog.reconnect_by_identity(&device_id)?.open_dmm()?;
    ```

=== "C#"

    ```bash
    dotnet run --project examples/AssignInstruments
    ```

    Persist `DeviceId` from discovery, then reconnect by identity after a rescan.

## Optional: Async

=== "Rust"

    Add `features = ["tokio"]` (and a tokio runtime). See [Rust async](rust/async.md).

    ```toml
    [dependencies]
    instrument-components = { version = "0.1", default-features = false, features = ["tokio"] }
    tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
    ```

=== "C#"

    Core async APIs take `CancellationToken` and return `Task` / `ValueTask`.
    VISA async is currently a sync bridge — see [C# async](csharp/async.md).

    ```bash
    dotnet run --project examples/MockFixtureCiAsync
    ```

## Environment variables (Rust VISA linking)

| Variable | Purpose |
|---|---|
| `LIB_VISA_PATH` | Directory containing `visa64.lib` / `visa.lib` |
| `LIB_VISA_NAME` | Library name (`visa64`, `visa32`, `visa`) |

## Next steps

- [Discovery](discovery.md) — probe policy and classification
- [DMM class](classes/dmm.md) — measure and configure
- [Capability matrix](capability-matrix.md) — Base vs Extension status
