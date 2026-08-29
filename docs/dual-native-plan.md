# Dual-native reliability plan

This is the working plan for keeping **two native implementations** (Rust and C#) of the same instrument library. Shared behavior is the product; each language is an idiomatic realization, not a secondary port.

Status key: `todo` / `in-progress` / `done` / `later`.

## Goals

1. **Usefulness** — a lab engineer can discover an instrument, open a typed class, and get a trustworthy SI-unit result with mocks in CI and VISA on the bench.
2. **Reliability** — timeouts, retries, cancellation, reconnect, diagnostics, and errors mean the same thing in both languages and match the documented contract.
3. **Non-drift** — changing SCPI, classification, or session policy in one language without the other is a CI failure, not a code-review hope.

Do **not** add new instrument classes or claim broad model support until the gates below exist. Breadth without verified behavior is inventory, not a library.

## Current state (baseline)

What is already in good shape:

- Layered transport → SCPI session → catalog → typed classes in both languages.
- Shared TOML for SCPI templates, probes, dialects, and the model registry, with codegen drift checks.
- Mirrored mock tests for a named scenario list (`docs/parity-checklist.md`).
- Mock-first CI (no VISA on GitHub-hosted runners).
- Documented intentional API differences (async type shape, cancellation, C# VISA sync-bridge).

What is not:

- Parity is a checklist of similarly named tests, not a shared executable contract.
- Several `ConnectOptions` fields are stored but unused or mis-reported.
- Release can publish NuGet after only the Rust workflow succeeds.
- Docs advertise `0.2` and a misspelled GitHub repo while packages remain `0.1.0` on `josh-hemphill/instrument-components`.
- Dialect profiles exist but most class methods emit generic `scpi_commands` anyway.
- No automated hardware evidence; registry entries are classification hints.

## Support tiers

Every model and class method should be labeled as one of:

| Tier | Meaning | Gate |
|---|---|---|
| `generic` | Generic SCPI; may work on a given instrument | Compiles + mock script |
| `transcript` | Recorded I/O replayed in both languages | Shared fixture JSON + class-level assertions |
| `hardware` | Observed on a named instrument + VISA provider + OS | Ignored/self-hosted job, documented matrix |

The curated registry stays **hints only** until a row is `transcript` or `hardware`.

## Stacked workstreams

Work is stacked so each PR is reviewable and later PRs sit on earlier branches.

```
latest
  └─ PR A  ci-gates-and-plan          (this stream)
       └─ PR B  session-reliability
            └─ PR C  shared-contracts
                 └─ PR D  parity-and-docs
```

Hardware runners, crates.io/NuGet publish, UniFFI, and IVI conformance stay **later**.

---

### Stream A — Identity, CI, and planning (`done`)

**Why first:** nothing else is trustworthy if release and docs describe a different product than CI tests.

1. Treat `https://github.com/josh-hemphill/instrument-components` as the canonical repo and GitHub Pages URL. Replace leftover `instrument-componenets-rs` links.
2. Keep package versions at `0.1.0`. Stop telling users to depend on `0.2`. Document that async and later classes live on `latest` until a real `0.2.0` publish.
3. Run `.NET` CI on every PR/push to `latest` (no path filter that skips C# when only Rust/session code changes).
4. Make `release.yml` require **both** `ci.yml` and `dotnet.yml` before crates.io or NuGet publish.
5. Compile-and-run `InstrumentComponents.Visa.Tests` in CI with `Category!=Hardware` so the VISA package is at least load-tested.
6. Replace the “all phases done” roadmap with this plan as the source of truth.

**Done when:** a tag cannot publish without .NET; README/docs/site/package metadata agree on repo + version; this document is linked from `docs/roadmap.md`.

---

### Stream B — Session and transport honesty (`todo`)

**Why second:** options and errors are the reliability surface. Shared SCPI tables cannot save a session that ignores timeouts.

Fixes (both languages unless noted):

1. **Connect options propagate.** `Discovery.connect_options` must be the catalog’s default for opened sessions, not reset to `ConnectOptions::default()`.
2. **I/O timeout.** `WriteTimeout` must affect I/O. VISA has one session timeout: configure it from `per_op_timeout`, else `max(read_timeout, write_timeout)`. Document that.
3. **Reconnect is not a lie.** Record a reconnect diagnostic only when `reconnect()` succeeds. Default VISA reconnect should reopen or return unsupported — never a silent no-op that still logs success. C# `VisaTransport.Reconnect()` is currently empty.
4. **C# access modes.** Map `SharedLock` to `Ivi.Visa.AccessModes.SharedLock`, not `None`.
5. **C# culture.** Parse and format SCPI numbers with `CultureInfo.InvariantCulture` (codegen + `ScpiProtocol`).
6. **C# `ProbeSystErr`.** Match `ProbeOpc`: a query that returns a string is not proof of support; catch timeout/transport failure.
7. **C# `ResetOnConnect`.** Apply `*CLS` / `*RST` in `ScpiSession` construction, as Rust already does.
8. **C# disposal.** `VisaTransport` (and sessions that uniquely own it) must dispose `IMessageBasedSession`.
9. **Reconnect-on-failure tests** that distinguish “retry write” from “pretend reconnect succeeded.”

**Done when:** a Discovery-configured retry/timeout is visible on `catalog.device(...).open_session().options()`, and the new tests fail if any of the above regress.

---

### Stream C — Executable shared contracts (`todo`)

**Why third:** dual-native only works if the contract is data, not two copies of judgment.

1. **Single SCPI rule.** Typed classes always emit commands from the resolved dialect profile. `scpi_commands.toml` remains the generic template source; `dialects/profiles.toml` overrides per vendor. Add a generator or test that generic profiles match `scpi_commands.toml`.
2. **Golden vectors** under `spec/scpi-vectors.json` (or TOML): `(kind, method, args, idn?) → exact command string`. Rust and C# tests load the same file.
3. **Classifier fixtures** under `spec/classifier-cases.json`: address / IDN / probe outcomes → supported kinds + confidence sources. Both languages must match.
4. **Transcripts become behavioral.** Shared `fixtures/*.json` must drive a typed-class action and assert values, not only step counts. Add a JSON schema for transcript steps.
5. **Vendor dialect tests.** At least `rigol_dsa` vs `generic_specan` (`:TRAC?` vs `:TRAC:DATA?`) and one power-meter profile resolution test.
6. **Document the rule in `CONTRIBUTING.md`:** edit TOML, regenerate, never hand-edit generated files; new class methods require a golden vector.

**Done when:** changing a generic command in one language’s generated file fails CI; changing TOML and regenerating updates both; a dialect mismatch fails a named test.

---

### Stream D — Parity, docs, and remaining class depth (`todo`)

**Why fourth:** usefulness after the contract exists.

1. Fill parity-checklist holes: C# multi-session test; C# examples for `dmm_measure`, `manual_tcpip`, `discover_async` (or explicitly mark hardware-only Rust examples).
2. `docs/api-overview.md` includes PowerMeter and SpectrumAnalyzer.
3. Counter: define and test gate-time / channel / timeout behavior, or mark timeout `deferred` with a reason.
4. Oscilloscope binary waveform: implement behind the dialect/golden-vector path, or keep `todo` with no “release-ready” claim.
5. Docs site is the published surface; keep `docs/` as contributor planning or fold overlapping guides so they cannot diverge.
6. README/architecture introduce **both** languages as first-class, not “Rust plus a C# port.”

**Done when:** the parity checklist has no silent gaps, and the capability matrix matches tested behavior.

---

### Later (do not start until A–C land)

- Self-hosted hardware smoke (1 DMM + 1 other class; Windows NI and optionally Linux Keysight). Publish artifacts, never require GitHub-hosted VISA.
- `0.2.0` publish (crates.io + NuGet) only after A–C and a changelog that does not promise unverified models.
- C# true async VISA / APM spike (`docs/visa-async-csharp.md` criteria 2–4).
- Benchmarks in CI (C# BenchmarkDotNet already exists; add Rust criterion for the same SCPI framing/query cases) — compare mode, no flaky fail-on-regression at first.
- API reference for C# equivalent to docs.rs.
- UniFFI, process-supervised HAL, IVI Config Store / `IIvi*` — still correctly deferred.

## Working rules

- Prefer adding a shared fixture or spec row over a one-sided unit test.
- Do not expand the model registry as a proxy for support.
- Intentional language differences stay listed in `docs/parity-checklist.md`; accidental ones become bugs.
- Generated files (`scpi_commands.rs`, `ScpiCommands.cs`, `dialect.rs`, `DialectRegistry.cs`, `probes.rs`, `CapabilityProbes.cs`, `model_registry.json`) are never the source of truth.
