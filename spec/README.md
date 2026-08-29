# Shared dual-native contracts

These files are the executable contract between the Rust and C# implementations. Both languages load the same JSON in tests. Do not encode the same rule twice in language-specific fixtures.

| File | Role |
|---|---|
| `scpi-vectors.json` | Dialect resolution, vendor overrides, and generic command strings |
| `classifier-cases.json` | Address / `*IDN?` / merge / override → kinds |
| `generic-scpi-map.json` | Maps `generic_*` dialect keys to `scpi_commands.toml` (codegen check) |
| `transcript.schema.json` | Schema for `fixtures/*.json` I/O transcripts |

## Rules

1. Edit TOML under `crates/instrument-core/data/`, then regenerate. Never hand-edit generated files.
2. New typed-class methods that emit SCPI need a row in `scpi-vectors.json` (or a shared transcript that asserts the value).
3. Transcripts under `fixtures/` must drive a typed-class action and assert **values**, not only step counts.

```bash
deno run --allow-read --allow-write --allow-run=rustfmt tools/gen-shared-tables.ts
deno run --allow-read --allow-write --allow-run=rustfmt tools/gen-dialects.ts
deno run --allow-read --allow-write dotnet/tools/gen-registry.ts
```
