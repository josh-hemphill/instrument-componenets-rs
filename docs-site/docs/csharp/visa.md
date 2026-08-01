# VISA (C#)

Hardware I/O uses the `InstrumentComponents.Visa` package over [IviFoundation.Visa](https://www.nuget.org/packages/IviFoundation.Visa).

## Packages

| Package | Role |
|---|---|
| `InstrumentComponents` | Discovery, typed classes, mocks — no VISA runtime (`net8.0`) |
| `InstrumentComponents.Visa` | VISA transport (`net8.0`; Windows **or** Linux with a vendor VISA install) |

NuGet publishing is deferred; use project references from this repo for now.

## Prerequisites

Install NI-VISA, Keysight IO Libraries, or another stack that provides VISA.NET compatible with `IviFoundation.Visa` 8.x.

Cross-platform **build** is unblocked on `net8.0`; **runtime** still needs a vendor VISA install on the target OS.

## Discovery

```csharp
using InstrumentComponents.Visa;

var catalog = VisaDiscovery.Create().Scan();
catalog.PrintSummary();
```

```bash
dotnet run --project examples/Discover
```

## Async transport

`VisaAsyncTransport` is a **sync bridge** (thread-pool offload), not vendor APM. Details: [C# async](async.md).

## Mock without VISA

Reference only `InstrumentComponents` and use `ScriptedFixture` — see [getting started](../getting-started.md) or:

```bash
dotnet run --project examples/MockFixtureCi
```
