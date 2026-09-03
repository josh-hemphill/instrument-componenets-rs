# Roadmap

Dual-native work follows [dual-native-plan.md](dual-native-plan.md). Streams A–F are merged (`latest`). Stream G (this work) adds DMM/PSU golden transcripts and real vendor dialect profiles. Next: self-hosted hardware smoke.

**Still deferred:** package publishing (`0.2.0`), UniFFI, process-supervised HAL, IVI Config Store / `IIvi*` conformance, vendor VISA on GitHub-hosted CI.

Neither NI nor Keysight ships a CI-loadable VISA/instrument emulator. GitHub-hosted CI stays on MockTransport + transcripts. See [Hardware emulators](dual-native-plan.md#hardware-emulators-ni--keysight).

```mermaid
flowchart LR
  A[A_CI_identity]
  B[B_session_honesty]
  C[C_shared_contracts]
  D[D_parity_docs]
  E[E_session_io_honesty]
  F[F_dialect_remaining_classes]
  G[G_dmm_psu_transcripts]
  H[H_self_hosted_hardware]
  A --> B --> C --> D --> E --> F --> G --> H
```

Dialect profiles live under `crates/instrument-core/data/dialects/` and `spec/vendors/`, generated via `tools/gen-dialects.ts`.
