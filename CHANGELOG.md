# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Full async API behind `tokio` feature: `AsyncScpiSession`, `AsyncInstrumentSession`, `AsyncDiscovery`, `AsyncDeviceCatalog`, `AsyncDeviceRef`
- Async typed classes: `AsyncDmm`, `AsyncDcPowerSupply`, `AsyncFunctionGenerator`, `AsyncOscilloscope`, `AsyncSwitch`, `AsyncCounter`, `AsyncPowerMeter`, `AsyncSpectrumAnalyzer`
- Sync typed classes: `Oscilloscope`, `Switch`, `Counter`, `PowerMeter`, `SpectrumAnalyzer` (in addition to DMM / PSU / FGen)
- Class depth APIs: DMM AC/Ω/temp + INIT/FETC/READ/*TRG; PSU OVP/sense/channel count; scope trigger/measure; FGen duty/burst; switch path labels; counter gate/channel
- Dialect profiles + codegen (`tools/gen-dialects.ts`) for vendor SCPI variance including power meter and spectrum analyzer
- Curated model registry growth (≥40 models across classes) with PowerMeter / SpectrumAnalyzer hints
- Readonly capability probes for PowerMeter and SpectrumAnalyzer
- `VisaAsyncTransport` and `VisaAsyncSessionOpener` for async VISA I/O
- `MockTransport` async impl and `MockAsyncSessionOpener` for CI
- Shared SCPI protocol helpers (`scpi/protocol.rs`) used by sync and async sessions
- Shared TOML tables + codegen for SCPI commands and capability probes (Rust + C#)
- Examples: `mock_fixture_ci_async`, `discover_async`; C# examples under `dotnet/examples/`
- Async integration tests and CI tokio job coverage
- Docs: `docs/roadmap.md`, `docs/parity-checklist.md`, `docs/capability-matrix.md`, `docs/dotnet-getting-started.md`, `docs/visa-async-csharp.md`

### Changed

- C# `InstrumentComponents.Visa` targets `net8.0` (Linux-capable builds via `IviFoundation.Visa`; runtime still needs a vendor VISA install)

## [0.1.0] - 2026-06-06

### Added

- `instrument-components` facade with `instrument` lib name for VISA instrument control
- `instrument-core`: `Transport`, `ScpiSession`, mock fixtures, layered classifier, diagnostics
- `instrument-visa`: NI-VISA / Keysight backend via visa-rs 0.7.0-alpha.1
- Auto-discovery for USB, GPIB, serial; manual TCPIP addresses
- Typed classes: `Dmm`, `DcPowerSupply`, `FunctionGenerator` (SI units)
- `ProbePolicy` for tiered capability probing during discovery
- `DeviceHealth` polling and `CommsObserver` push diagnostics
- Stable `DeviceId` for instrument replacement workflows
- Feature flags: `visa` (default), `tokio`, `cross-compile`, `record`
- Examples: discover, dmm_measure, mock_fixture_ci, assign_instruments
- Documentation in `docs/` and CONTRIBUTING guide

### Notes

- Depends on `visa-rs` 0.7.0-alpha.1 (pre-release)
- Async APIs shipped in Unreleased (enable `tokio`); originally noted as planned for a later minor

[0.1.0]: https://github.com/josh-hemphill/instrument-components-rs/releases/tag/v0.1.0
