# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Full async API behind `tokio` feature: `AsyncScpiSession`, `AsyncInstrumentSession`, `AsyncDiscovery`, `AsyncDeviceCatalog`, `AsyncDeviceRef`
- Async typed classes: `AsyncDmm`, `AsyncDcPowerSupply`, `AsyncFunctionGenerator`
- `VisaAsyncTransport` and `VisaAsyncSessionOpener` for async VISA I/O
- `MockTransport` async impl and `MockAsyncSessionOpener` for CI
- Shared SCPI protocol helpers (`scpi/protocol.rs`) used by sync and async sessions
- Examples: `mock_fixture_ci_async`, `discover_async`
- Async integration tests and CI tokio job coverage

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
- Async `ScpiSession` and typed async classes planned for v0.2

[0.1.0]: https://github.com/josh-hemphill/instrument-components-rs/releases/tag/v0.1.0
