# Rust ↔ C# parity checklist

Shared scenarios both implementations must cover. Update this table when adding tests or closing gaps.

| Scenario | Rust | C# | Notes |
|---|---|---|---|
| Mock DMM measure | `crates/instrument/tests/mock_catalog.rs` | `MockCatalogTests.FixtureDmmMeasure` | |
| Async mock DMM measure | `crates/instrument/tests/async_mock_catalog.rs` | `MockCatalogTests.AsyncFixtureDmmMeasure` | |
| Static discovery classify | `crates/instrument/tests/discovery_static.rs` | `DiscoveryStaticTests` | |
| Async static discovery | `crates/instrument/tests/async_discovery_static.rs` | `AsyncDiscoveryStaticTests` | |
| SCPI query retry after timeout | `crates/instrument-core/tests/reliability.rs` | `ReliabilityTests` | |
| Async SCPI query retry | `crates/instrument-core/tests/async_reliability.rs` | `AsyncReliabilityTests` | |
| Diagnostics observer + health | `crates/instrument-core/tests/diagnostics.rs` | `DiagnosticsTests` | |
| Async diagnostics | `crates/instrument-core/tests/async_diagnostics.rs` | `AsyncDiagnosticsTests` | |
| Transcript `smu2602` | `crates/instrument/tests/transcript_behavior.rs` | `TranscriptBehaviorTests` | `fixtures/smu2602.json` — asserts 3.3 V |
| Transcript scope | same | same | `fixtures/scope_ds1054z.json` — asserts samples + interval |
| Transcript switch | same | same | `fixtures/switch_34970a.json` — asserts closed |
| Transcript counter | same | same | `fixtures/counter_53230a.json` — asserts 1000 Hz |
| Shared SCPI vectors | `crates/instrument-core/tests/shared_contracts.rs` | `SharedContractTests` | `spec/scpi-vectors.json` |
| Shared classifier cases | same | same | `spec/classifier-cases.json` |
| New kinds (scope/switch/counter) | `crates/instrument/tests/new_instrument_kinds.rs` | `NewInstrumentKindTests` | |
| Class depth Dmm/Psu | `crates/instrument/tests/mock_catalog.rs` | `MockCatalogTests` | AC/Ω/temp, INIT/FETC, OVP/sense |
| Class depth Scope/Fgen | `new_instrument_kinds.rs` | `NewInstrumentKindTests` | Trigger/measure, burst/duty |
| PowerMeter + SpectrumAnalyzer | `new_instrument_kinds.rs` | `NewInstrumentKindTests` | Open + configure/read / sweep |
| SCPI framing | `instrument-core` framing unit tests | `FramingTests` | |
| Address parse | core address tests | `AddressTests` | |
| Classifier / registry | core classifier + registry | `ClassifierTests`, registry hints in `NewInstrumentKindTests` | |
| Hardware discover smoke | `crates/instrument/tests/hardware.rs` (`#[ignore]`) | `HardwareTests` (skipped) | Local VISA only |
| Mock example runnable | `examples/mock_fixture_ci` | `dotnet/examples/MockFixtureCi` | |
| Async mock example | `examples/mock_fixture_ci_async` | `dotnet/examples/MockFixtureCiAsync` | |
| Discover example | `examples/discover` | `dotnet/examples/Discover` | Needs VISA |

## Known intentional differences

| Topic | Rust | C# |
|---|---|---|
| Async catalog API | Separate `AsyncDeviceCatalog` (`tokio`) | Same `DeviceCatalog` with `*Async` methods |
| Cancellation | Futures only | `CancellationToken` throughout |
| VISA async I/O | `visa-rs` tokio adapter | Sync bridge (`SyncAsAsyncTransport`) — see [visa-async-csharp.md](visa-async-csharp.md) |
| VISA package TFM | Cross-platform where `visa-rs` links | `net8.0` (Windows or Linux + vendor VISA) |
| Recording transport | `record` feature | Always available |

## Regenerating shared tables

```bash
deno run --allow-read --allow-write tools/gen-shared-tables.ts
deno run --allow-read --allow-write dotnet/tools/gen-registry.ts
```

CI fails if generated outputs drift from committed files.
