# Roadmap

## Completed: Parity / C# competitiveness

| Phase | Goal | Status |
|---|---|---|
| 0 — Foundation | Shared parity definition + roadmap docs | Done |
| 1 — Parity CI | .NET on `latest`; async/diagnostics/discovery tests; more fixtures | Done |
| 2 — Shared tables | TOML → codegen for SCPI commands + capability probes | Done |
| 3 — C# time-to-first-volt | `dotnet/examples` + getting-started docs | Done |
| 4 — C# VISA cross-platform | `net8.0` TFM; Ubuntu compile CI; Linux docs | Done |
| 5 — VISA async honesty | Document sync-bridge; APM spike go/no-go | Done (keep sync bridge) |

**Deferred (unchanged):** package publishing, UniFFI / Interoptopus as a replacement for `dotnet/` (Interoptopus 0.16 is the stronger C# generator — see [interop-eval.md](interop-eval.md)), process-supervised HAL, IVI Config Store / `IIvi*` conformance.

## In progress: Class depth, RF classes, docs site

See [capability-matrix.md](capability-matrix.md) for Base vs Extension rows.

| Phase | Goal | Status |
|---|---|---|
| 0 — Foundation | Capability matrix + dialect profile schema/codegen | Done |
| 1 — Dmm + DcPwr | IVI-style base depth | Done |
| 2 — Scope + Fgen | Trigger, burst, waveform improvements | Done |
| 3 — Switch/Counter + registry | Polish + curated registry growth | Done |
| 4 — PowerMeter | Kind + common API + dialect profiles | Done |
| 5 — SpectrumAnalyzer | Kind + common API + dialect profiles | Done |
| 6 — GitHub Pages | MkDocs Material, Rust\|C# tabs, lang deep dives | Done |

All class-depth / RF / docs-site phases are complete. Next release train is package publishing (deferred).

```mermaid
flowchart LR
  P0[P0_Foundation]
  P1[P1_Dmm_DcPwr]
  P2[P2_Scope_Fgen]
  P3[P3_Switch_Counter_Registry]
  P4[P4_PowerMeter]
  P5[P5_SpecAn]
  P6[P6_GitHubPages]
  P0 --> P1 --> P2 --> P3
  P3 --> P4 --> P5
  P0 --> P6
  P3 --> P6
```

Dialect profiles live under `crates/instrument-core/data/dialects/` and are generated via `tools/gen-dialects.ts`.
