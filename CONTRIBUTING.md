# Contributing

Thank you for contributing to instrument-components-rs.

## Development setup

```bash
git clone https://github.com/josh-hemphill/instrument-componenets-rs.git
cd instrument-componenets-rs
cargo test --workspace --no-default-features
```

## Project layout

| Crate | Path | Role |
|---|---|---|
| `instrument-components` | `crates/instrument` | Public facade (lib name: `instrument`) |
| `instrument-core` | `crates/instrument-core` | Transport, SCPI, mocks — no VISA |
| `instrument-visa` | `crates/instrument-visa` | visa-rs backend |

## Test matrix

| Command | When |
|---|---|
| `cargo test --workspace --no-default-features` | CI gate — mock path, no VISA |
| `cargo test -p instrument-core --features async` | Async SCPI + mock transport |
| `cargo test -p instrument-components --features tokio --no-default-features` | Async facade + catalog |
| `cargo test -p instrument-components --features visa -- --ignored` | Local hardware smoke tests |
| `cargo check -p instrument-components --features visa,tokio` | Async + VISA compiles |
| `cargo clippy -p instrument-core --features async -- -D warnings` | Async clippy |
| `cargo check -p instrument-visa --features cross-compile --target x86_64-pc-windows-gnu` | Cross-compile repr check |

CI does **not** link against VISA (no NI-VISA on runners). Hardware verification is local.

## Code style

```bash
cargo fmt --all
cargo clippy --workspace --no-default-features -- -D warnings
```

## Adding instrument models

Edit `crates/instrument-core/data/model_registry.toml`:

```toml
[[entry]]
manufacturer = "Keysight Technologies"
model = "34401A"
kinds = ["Dmm"]

[[usb_entry]]
vid = "0957"
pid = "0607"
manufacturer = "Keysight Technologies"
model = "34401A"
kinds = ["Dmm"]
```

Then regenerate the C# embed:

```bash
deno run --allow-read --allow-write dotnet/tools/gen-registry.ts
```

Registry entries are **hints only** — capability probes and `*IDN?` can override.

## Shared SCPI / probe tables

Edit TOML under `crates/instrument-core/data/`, then:

```bash
deno run --allow-read --allow-write --allow-run=rustfmt tools/gen-shared-tables.ts
deno run --allow-read --allow-write --allow-run=rustfmt tools/gen-dialects.ts
deno run --allow-read --allow-write dotnet/tools/gen-registry.ts
```

This regenerates Rust `probes.rs` / `scpi_commands.rs` / `dialect.rs` and C# `CapabilityProbes.cs` / `ScpiCommands.cs` / `DialectRegistry.cs`. Codegen runs `rustfmt` on the Rust outputs. CI fails on drift.

## Pull requests

- Include tests for behavior changes (mock path preferred).
- Keep diffs focused — no drive-by refactors.
- Update `CHANGELOG.md` under `[Unreleased]` for user-visible changes.

## Publishing (maintainers)

Publish order on crates.io:

1. `instrument-core`
2. `instrument-visa`
3. `instrument-components`

Set `CARGO_REGISTRY_TOKEN` in GitHub repository secrets. Tag `v*` triggers the release workflow.

Pre-first-publish: `cargo publish -p instrument-core --dry-run --allow-dirty` validates packaging. Dependent crates require `instrument-core` on crates.io before their dry-run verify step succeeds.
