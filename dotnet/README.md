# InstrumentComponents (.NET)

Native C# port of [instrument-components-rs](../README.md) over vendor-neutral [IviFoundation.Visa](https://www.nuget.org/packages/IviFoundation.Visa).

## Packages

| Package | NuGet | Role |
|---|---|---|
| `InstrumentComponents` | core | Discovery, typed classes (DMM, PSU, FGen, oscilloscope, switch, counter), mocks — no VISA runtime |
| `InstrumentComponents.Visa` | backend | Windows VISA transport via IviFoundation.Visa |

## Mock quick start (CI, no VISA)

```bash
dotnet add package InstrumentComponents
```

```csharp
using InstrumentComponents.Catalog;
using InstrumentComponents.Kind;
using InstrumentComponents.Mock;

var fixture = ScriptedFixture.Builder()
    .Idn("Acme Corp", "SMU2602", "SN001", "1.0")
    .Kinds(InstrumentKind.Dmm, InstrumentKind.DcPowerSupply)
    .OnQuery(":MEAS:VOLT:DC?", "3.300")
    .Build();

var catalog = DeviceCatalog.FromFixture("mock://smu-1", fixture);
var dmm = catalog.OpenDmm("mock://smu-1");
var volts = dmm.MeasureVoltageDc();
Console.WriteLine($"{volts} V");
```

## Hardware quick start (Windows + VISA)

1. Install [NI-VISA](https://www.ni.com/en-us/support/downloads/drivers/download.ni-visa.html) or [Keysight IO Libraries](https://www.keysight.com/us/en/lib/software-detail/computer-software/io-libraries-suite-downloads-2175637.html).
2. Add both packages:

```bash
dotnet add package InstrumentComponents
dotnet add package InstrumentComponents.Visa
```

```csharp
using InstrumentComponents.Visa;

var catalog = VisaDiscovery.Create().Scan();
catalog.PrintSummary();

var dmm = catalog.OpenDmm(catalog.DevicesByKind(InstrumentComponents.Kind.InstrumentKind.Dmm)[0].Address.Raw);
var volts = dmm.MeasureVoltageDc();
```

## Async quick start

```csharp
var catalog = DeviceCatalog.FromFixture("mock://dmm", fixture);
var dmm = await catalog.Device("mock://dmm").OpenDmmAsync();
var volts = await dmm.MeasureVoltageDcAsync();
```

## Testing

```bash
cd dotnet
dotnet test tests/InstrumentComponents.Tests
dotnet test tests/InstrumentComponents.Visa.Tests --filter "Category!=Hardware"
```

## Registry drift check

```bash
deno run --allow-read --allow-write dotnet/tools/gen-registry.ts
git diff --exit-code dotnet/src/InstrumentComponents/Data/model_registry.json
```
