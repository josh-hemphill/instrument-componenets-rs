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

## .NET examples

```bash
dotnet run --project dotnet/examples/MockFixtureCi
dotnet run --project dotnet/examples/MockFixtureCiAsync
dotnet run --project dotnet/examples/Discover          # needs VISA
dotnet run --project dotnet/examples/DiscoverAsync     # needs VISA
dotnet run --project dotnet/examples/DmmMeasure        # needs VISA
dotnet run --project dotnet/examples/ManualTcpip       # needs VISA
dotnet run --project dotnet/examples/AssignInstruments # needs VISA
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
| `dotnet/.../MockFixtureCi` | C# mock CI fixture |
| `dotnet/.../MockFixtureCiAsync` | C# async mock |
| `dotnet/.../Discover` | C# VISA discover |
| `dotnet/.../DiscoverAsync` | C# async VISA scan |
| `dotnet/.../DmmMeasure` | C# open a DMM and measure DC voltage |
| `dotnet/.../ManualTcpip` | C# manual TCPIP address + discovery |
| `dotnet/.../AssignInstruments` | C# device-id assignment |
