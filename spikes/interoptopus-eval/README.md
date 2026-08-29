# Interoptopus 0.16 eval vs UniFFI

Standalone spike (not a workspace member). Interoptopus **0.16.4** (C# is a tier-1 backend) vs the earlier UniFFI 0.31 + `uniffi-bindgen-cs` experiment.

## How to run

```bash
cd spikes/interoptopus-eval
cargo test generate_bindings   # writes csharp/generated/Interop.cs
cargo build                    # libinteroptopus_eval.so / .dll / .dylib
dotnet test csharp/InteroptopusEval.Tests.csproj
```

## What we actually ran

Six xunit cases against generated C# + the native cdylib, all passing:

| Hard case (same questions as UniFFI) | Result |
|---|---|
| Service as a C# **class** (`Dmm.Create()`, `MeasureVoltageDc()`) | Pass |
| Async Rust → `Task<T>` with **`CancellationToken`** | Pass — token aborts the Rust future (~200ms) |
| C# lambda observer callback | Pass — `session.PingObserver(kind => …)` |
| Borrowed `byte[]` slice (not owned copy) | Pass — `using var slice = bytes.Slice()` |
| Typed error throw | Pass — `EnumException<Error>` with `ex.Value.IsTimeout` |

Reverse interop (C# implementing an async transport that Rust calls) was **not** re-hosted here. Interoptopus documents it as `plugin!` + generated `IFoo<T>` interfaces with `Task` + `CancellationToken`; their `reference_plugins` tests cover it. That feature is behind `unstable-plugins` and is not semver-stable.

## Generated C# (this is the interesting part)

Interoptopus emits `IDisposable` classes, not a P/Invoke bag. Method names are PascalCase. Async methods take `CancellationToken` with a default:

```csharp
using var dmm = Dmm.Create();
double volts = dmm.MeasureVoltageDc();

using var asyncDmm = AsyncDmm.Create();
double v = await asyncDmm.MeasureVoltageDc(ct);
await asyncDmm.SleepForever(cts.Token); // cancelled → exception

session.PingObserver(kind => seen = kind);
using var slice = new byte[] { 1, 2, 3, 4 }.Slice();
session.ChecksumSlice(slice);
```

That is closer to hand-written `dotnet/` than UniFFI was. UniFFI gave us `Task<T>` but **no** `CancellationToken`, owned `byte[]` only, and weaker types around errors/observers.

## Comparison

| Topic | Hand-written `dotnet/` | UniFFI 0.31 + bindgen-cs | Interoptopus 0.16.4 |
|---|---|---|---|
| C# shape | Idiomatic classes, no `Dispose` on views | Bindgen types; less “C# class” feel | `IDisposable` classes + factories (`Create`) |
| Async | `*Async(..., CancellationToken)` on the **same** type | `Task<T>`, no CT | Separate async service; `Task`/`Task<T>` + CT |
| Cancel in-flight native work | C# VISA is still a sync bridge | No | Yes — dropping the future when the token fires (proven) |
| C# implements transport | First-class interfaces | Worked in the UniFFI spike | Official plugin path; **unstable** |
| Bytes | `byte[]` / spans as we choose | Owned `byte[]` only | Borrowed `SliceByte` (`using`, must unpin) |
| Errors | `InstrumentException` with messages | Weak messages | `EnumException<Error>` — **typed variant**, but `Message` is always `"Enum variant mismatch."` |
| Observer | `ICommsObserver` | Sync observer worked | C# lambdas / named callbacks; exceptions from Rust-invoked delegates rethrow on `Dispose` |
| Packaging | Pure managed + optional Visa.NET | Native lib + bindgen-cs (third-party) | Native cdylib + first-party C# backend |
| Runtime cost | None extra | Native + UniFFI scaffolding | Each `Async*` service constructs a **multi-thread Tokio runtime** (`Tokio::new()`) |
| Maintenance | Two APIs, shared TOML | Bindgen-cs version coupling | One Rust inventory; C# backend is the project’s tier-1 target |

## Verdict

The user’s hunch is right: **Interoptopus C# is more intuitive than UniFFI** — real classes, PascalCase methods, lambdas for callbacks, `Task` + `CancellationToken`, and borrowed slices.

It is still **not a drop-in replacement** for the hand-written C# port:

1. Sync and async are **two services** (`Dmm` vs `AsyncDmm`), not `MeasureVoltageDc` / `MeasureVoltageDcAsync` on one object.
2. Callers must `Dispose` services and slices.
3. Exception **messages** are not useful; you inspect `EnumException<Error>.Value`.
4. Shipping a cdylib (+ Linux/macOS/Windows natives) is a product/packaging problem we do not have with managed `InstrumentComponents`.
5. C#-owned VISA / foreign transports need the **unstable** plugin path, or we keep a C# implementation anyway.
6. Per-service Tokio runtimes are a poor fit for a catalog of many instruments unless we share one runtime.

**Keep dual native implementations.** Treat Interoptopus as the better FFI option *if* we later want a “Rust is the source of truth, C# is generated” product SKU — UniFFI is the weaker C# story. Do not switch `dotnet/` over without solving packaging, one-type async, and error messages.
