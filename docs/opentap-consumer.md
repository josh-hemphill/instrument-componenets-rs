# OpenTAP / HardwareTest consumer contract

Contributor planning for how this library supports
[dotnet-avalonia-hardwaretest-template](https://github.com/josh-hemphill/dotnet-avalonia-hardwaretest-template)
without becoming a second operator shell. C# only; Rust stays dual-native for
session, dialects, and mocks. Stream F is already merged
([#11](https://github.com/josh-hemphill/instrument-components/pull/11)). G and H
are independent of this track.

**Status:** understanding locked from HardwareTest’s OpenTAP/VISA work; no pack
code in this repo yet.

## One-line role

This library owns **SCPI session + typed capabilities + protocol mocks**.
HardwareTest owns **broker, gate, Avalonia, mixins, operator dialogs, program
catalog**. An optional OpenTAP pack here is a *plugin the shell consumes*, not a
bench application.

## Layering (who owns what)

```text
HardwareTest Avalonia shell
  Instruments page, Run board, Presentation widgets, operator prompts
        │
HardwareTest.Core
  IVisaBroker / VisaSessionGate / IBenchOperationCoordinator
  (process-wide mock↔real, Safety Stop preemption)
        │ injects already-open Write/Query session
        ▼
OpenTAP wrapper (optional pack in THIS repo)
  Instrument + VisaAddress + nested DMM/PSU/scope views
        │
InstrumentComponents (this repo, no OpenTAP)
  IScpiIo / InstrumentSession / Dmm / DcPowerSupply / Oscilloscope / …
  ScriptedFixture protocol mocks
        │
InstrumentComponents.Visa  — NOT referenced by the OpenTAP pack
  Ivi.Visa open / enumerate  (HardwareTest.Core already does this for the bench)
```

HardwareTest’s `VisaDmmInstrument` today opens through `IVisaBroker` and talks
raw SCPI (`*IDN?`, `CONF:VOLT:DC`, `READ?`). The intended end state is: that
plugin stops growing; product plans reference **this pack’s instruments**;
HardwareTest injects a broker-backed session into our wrapper so we never call
`GlobalResourceManager.Open` from a plugin.

## Determinations (expanded)

### 1. Session: own Open / Write / Query (timeout, dispose)

**What HardwareTest needs.** A session object this library fully owns:

| Member | Why |
| --- | --- |
| Open (or construct-from-injected) | Wrapper `Instrument.Open` must not call IVI |
| `Write(command)` | SCPI set / configure / output-off / `*RST` |
| `Query(command)` | `*IDN?`, measure, output state |
| Timeout | Maps onto HardwareTest `IoTimeoutMilliseconds` / `ConnectOptions` |
| Dispose | `Instrument.Close` / plan teardown |

**What we must not add here.** `IVisaBroker`, `VisaSessionGate`,
`IBenchOperationCoordinator`, Avalonia types, or `HardwareTest.Core`. Those
serialize Instruments-page I/O with plan I/O and Safety Stop. HardwareTest
keeps them. We accept an **already-opened** message session.

**Injection shape (the important seam).** HardwareTest `IVisaSession` is
**string** Write/Query (IVI `FormattedIO`). Our `ITransport` is a **byte
stream**. Wrapping `IVisaSession` as `ITransport` would double-frame (their
FormattedIO already appends `\n`; `ScpiSession` normalizes terminators again)
and cannot implement `Read` because `IVisaSession` has no read-only API.

Preferred C# seam (not implemented yet):

```csharp
public interface IScpiIo : IDisposable
{
    TimeSpan IoTimeout { get; set; }
    void Write(string command);
    string Query(string command);
}
```

- Native path: `ScpiSession` implements `IScpiIo` (current framing, retry, probes).
- HardwareTest path: adapter `IVisaSession` → `IScpiIo` (pass-through strings,
  timeout, sync-over-async at the wrapper). Typed classes keep using
  `InstrumentSession`.
- `InstrumentSession` constructible from `IScpiIo` **or** `ITransport`.
- OpenTAP wrapper: default ctor for XML load; `ctor(IScpiIo)` / settable
  session for the host. `Open()` uses the injected session; it does not open
  VISA.

Constructor injection must be documented so HardwareTest does **not** need a
process-global host inside this library. HardwareTest already has
`VisaBrokerHost` (`SessionLocal`) for *its* plugins; we should not clone that
pattern here. Tests and Host pass the session in.

`TestPlan.Load` / TUI round-trip must succeed **without** a broker: default ctor
+ writable `VisaAddress` + stable type names. Live I/O happens later, when the
host injects a session and OpenTAP calls `Instrument.Open` during execute.

### 2. Optional OpenTAP pack (what HardwareTest consumes)

Ship only if we want product `.TapPlan` files to reference types from this
repo. The pack is C#-only and is **not** required for Rust, NuGet core, or
dual-native G/H (F is already merged).

| Rule | Detail |
| --- | --- |
| One `Instrument` per physical device | An SMU that is DMM+PSU is **one** resource, not two slots |
| Writable `VisaAddress` | HardwareTest `InstrumentResourceAccess` prefers this name, then `ResourceName`, then `Address` |
| `IDeviceDiscovery` for `VisaAddress` | So **Discover OpenTAP** lists addresses. Must not call `Ivi.Visa` from the pack |
| Nested capabilities | Methods on the instrument **or** `[EmbedProperties]` child objects (`Measurement`, `Output`, `Identity`). Not extra `Instrument` resources |
| Stable type names + package version | TapPlan XML stores `type` + package. Rename/break version → TUI and `TestPlan.Load` fail even with no broker |

**Discovery without IVI in the pack.** HardwareTest already enumerates via Core
`VisaResourceDiscovery` **and** `PluginManager.GetPlugins<IDeviceDiscovery>`.
A pack-level `IDeviceDiscovery` should take an injected `ResourceEnumerator`
(or return empty when none is registered). Do not reference
`InstrumentComponents.Visa` from the pack. Standalone TUI users who want
in-plugin Find can load a **separate** optional Visa discovery assembly that
HardwareTest does **not** search.

**Nested capabilities vs extra resources.** HardwareTest slots are “every
`Instrument`-typed step property.” Two OpenTAP instruments for one box creates
two Instruments-page rows and two VisaAddress overrides. Use one instrument
with nested DMM/PSU/scope views (same pattern as our `SessionPool`: one
underlying session, multiple typed views).

### 3. Capability surfaces HardwareTest steps bind to

HardwareTest `IdentityCheckStep` / `SafeShutdownStep` today take `HardwareDmm`
(`QueryIdn`, `OutputOff`, `Reset`, plus measure). Product plans must not depend
on growing `Plugins.Basic`. This library’s instruments (or small interfaces the
wrapper implements) must expose:

| Surface | DMM | PSU | Scope | Notes |
| --- | --- | --- | --- | --- |
| Identity | `*IDN?` / `QueryIdn` | same | same | Session already has `InstrumentSession.Idn()` |
| Measurement | V/I/Ω configure + read | voltage/current readback | Vpp / freq / trace | Existing typed classes |
| Output | no-op `OutputOff` | `OutputEnable(ch, false)` as `OutputOff` | stop / display off as needed | SafeShutdown must be callable on every instrument type |
| Reset / shutdown | `*RST` | `*RST` + outputs off | `*RST` / stop | Session already has `Reset()` |

**Bind without `HardwareDmm`.** HardwareTest will retarget those steps to a
more generic `Instrument` (or an interface both packs implement). This repo
should publish the **methods** (and optionally tiny interfaces in
`InstrumentComponents`, OpenTAP-free) so the wrapper can forward them. Do not
take a dependency on `HardwareTest.OpenTap.Plugins.Basic`.

Today the gap is placement, not SCPI: identity/reset live on `InstrumentSession`
/ `Ieee4882`; typed classes (`Dmm`, `DcPowerSupply`, `Oscilloscope`) do not
surface `QueryIdn` / `OutputOff` / `Reset`. OpenTAP authors and HardwareTest
steps need those on the instrument object they bind, not only on the inner
session.

**Mocks.** Protocol-level `ScriptedFixture` / `MockTransport` / golden
transcripts stay in this library. HardwareTest `MockVisaSession` is a demo
sine-wave stub; it must not grow a second dialect table. HardwareTest only
**wraps** our fixtures (adapter to `IScpiIo` / `IVisaSession`).

### 4. Function steps (only if we ship any)

HardwareTest Phase I publish contract — do not invent a parallel one:

| Table | Columns |
| --- | --- |
| `Sample` | `Channel`, `Index`, `Value` |
| `Scalar` | `Name`, `Value`, `Unit`, `LimitLow`, `LimitHigh` |

Do **not** ship Presentation mixins, result sidecars, or operator dialogs.
Authors attach HardwareTest `PresentationMixin` / `AnnotationMixin` in TUI.
Do not add `DialogStep`, WinForms, or WPF prompts (appliance forbids floating
dialogs). Prefer **not** shipping steps in v1: HardwareTest already has
Acquire/Mean/Identity/SafeShutdown; the scarce resource is a **bindable
instrument**, not more demo steps.

If we later ship steps, they live in the pack, depend only on OpenTAP + our
assemblies, and publish those two tables.

### 5. Packaging

- Versioned `.TapPackage` (`CreateOpenTapPackage` / `tap package`).
- `Dependencies`: **OpenTAP** + this pack’s own assemblies (`InstrumentComponents`).
- **Not** `InstrumentComponents.Visa`, **not** `HardwareTest.Core`.
- Product plans depend on **this pack**, not on adding types to
  `HardwareTest.OpenTap.Plugins.Basic`.
- Document the injection ctor so Host composition is obvious.

Package id and **public type FullNames** are part of the TapPlan contract.
Treat renames as breaking; bump the OpenTAP package version in lockstep with
the assembly that contains the instrument types.

### 6. Hard nos

| Do not | Why |
| --- | --- |
| Reference `HardwareTest.Core` | Shell/broker stay in HardwareTest; this library stays reusable |
| Open `Ivi.Visa` / `GlobalResourceManager` from the OpenTAP pack | HardwareTest architecture tests forbid IVI in plugins; broker is the only IVI open on the bench |
| Put “needs DMM+PSU” (or any instrument inventory) in `program.json` | That sidecar is HardwareTest catalog (display name, DUT family, session requirements, `selectionIncludesCleanup`). Hardware needs belong in the TapPlan resources + Instruments slot overrides |
| Clone Instruments UI, Presentation, or operator interaction | Avalonia shell owns those |
| A second process-wide broker/locator inside this library | Injection from HardwareTest Host |

## Current repo vs the contract

| Need | Today | Gap |
| --- | --- | --- |
| Message-level `IScpiIo` | `ScpiSession.Write` / `Query` + timeouts + dispose | No interface; `InstrumentSession` only constructs from `ITransport` |
| Injected session into typed classes | `new Dmm(InstrumentSession)` works | No documented HardwareTest adapter; opening still goes through `ISessionOpener` → VISA |
| Identity / reset on session | `InstrumentSession.Idn()` / `Reset()` | Not on `Dmm` / `DcPowerSupply` / `Oscilloscope` |
| Output-off | PSU `OutputEnable(ch, false)` | No portable `OutputOff()`; DMM/scope have nothing SafeShutdown can call |
| Protocol mocks | `ScriptedFixture`, `MockTransport`, `spec/` + `fixtures/` | No identity+shutdown+measure fixture set aimed at an OpenTAP wrapper |
| OpenTAP `Instrument` | none | New optional project |
| Writable `VisaAddress` | n/a | Pack instrument property |
| `IDeviceDiscovery` | `VisaEnumerator` in `.Visa` (IVI) | Pack-safe enumerator injection; do not move IVI into the pack |
| `.TapPackage` | NuGet ids only (`InstrumentComponents` 0.1.0) | OpenTAP package metadata + versioning story |
| Function steps | none | Defer; HardwareTest already publishes Sample/Scalar |

`InstrumentComponents.Visa` opening IVI is **correct** for library consumers
and examples. The restriction is **plugin / pack** code that HardwareTest will
load, not the Visa package itself.

## Recommended path (this repo)

Independent of dual-native G (transcripts) / H (hardware smoke). F (dialect
remaining classes) is already merged. Do not block G/H. C# APIs below should
stay thin enough that Rust does not need equivalents until a non-OpenTAP
consumer wants `IScpiIo`.

### P1 — Session injection seam (`InstrumentComponents` only)

1. Introduce `IScpiIo` (Write / Query / timeout / dispose).
2. Implement it on `ScpiSession` (or a thin adapter over it).
3. Allow `InstrumentSession` to wrap an injected `IScpiIo` without `ITransport`.
4. Document a HardwareTest adapter: `IVisaSession` → `IScpiIo` (no framing of
   our own on that path).
5. Tests: mock `IScpiIo` drives `Dmm.MeasureVoltageDc` without `ITransport`;
   injected timeout is honored.

This is the smallest change that unblocks a HardwareTest wrapper **even before**
we ship a pack (they can wrap `Dmm` in their plugin).

### P2 — Portable identity / shutdown on typed classes

1. Add `QueryIdn()` / `Reset()` on each typed class (delegate to session).
2. Add `OutputOff()`: PSU turns all channels off; DMM no-op; FGen
   `enable_output false`; scope `Stop`; switch `open_all`.
3. Optional OpenTAP-free interfaces (`IInstrumentIdentity`,
   `IInstrumentShutdown`) so HardwareTest can bind without referencing class
   types.
4. Scripted fixtures + tests for `*IDN?`, `*RST`, and output-off per class.

### P3 — Optional OpenTAP pack (new project)

New `dotnet/src/InstrumentComponents.OpenTap/` (name TBD):

- `ScpiInstrument : Instrument` with writable `VisaAddress`, optional
  `IoTimeoutMilliseconds`.
- Default ctor (XML); `ctor(IScpiIo session)` for Host/tests.
- Nested capability objects (`Dmm`, `Output`, `Identity`) via methods or
  `[EmbedProperties]`, **one** `Instrument` type per device (or one type with
  kind flags — avoid a type explosion that churns TapPlan `type` attributes).
- `Open`/`Close` bind/dispose the injected session only.
- `IDeviceDiscovery` that uses a host-registered enumerator; empty if none.
- `.csproj`: OpenTAP package ref; project ref to `InstrumentComponents` only.
- Architecture test in *this* repo: pack sources/csproj must not mention
  `Ivi.Visa`, `GlobalResourceManager`, or `IviFoundation.Visa`.
- Golden TapPlan in tests: serialize → `TestPlan.Load` with no broker → type
  names round-trip.

Keep pack types’ namespaces stable from the first published `.TapPackage`.

### P4 — Function steps (defer)

Only after P3 is consumed by a product plan. If shipped: Sample + Scalar
columns only; no Presentation/operator types.

## HardwareTest-side work (out of scope here)

Called out so we do not accidentally implement them in this repo:

- Retarget `IdentityCheckStep` / `SafeShutdownStep` away from `HardwareDmm`.
- Host: wrap `IVisaSession` as `IScpiIo`, inject into our instrument ctor
  (or a setter) before execute — no `VisaBrokerHost` clone inside this library.
- Stop growing `VisaDmmInstrument` / `Plugins.Basic` for product SCPI.
- `program.json` stays display/DUT/session flags; never instrument inventory.
- Load our `.TapPackage` via existing offline package / plugin-dir provisioning.

## Decision log

| Topic | Decision |
| --- | --- |
| Become a HardwareTest fork? | No |
| Broker in this library? | No — inject session |
| OpenTAP pack required for core/Rust? | No — optional C# plugin |
| IVI in the pack? | No |
| IVI in `InstrumentComponents.Visa`? | Yes, remains the native backend |
| Message vs byte injection? | Message (`IScpiIo`); do not wrap `IVisaSession` as `ITransport` |
| One vs many OpenTAP instruments per box? | One, nested capabilities |
| Steps in v1? | No |
| `program.json` in this repo? | No |
