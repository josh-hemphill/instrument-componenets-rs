```csharp
using InstrumentComponents.Catalog;
using InstrumentComponents.Kind;
using InstrumentComponents.Mock;

var fixture = ScriptedFixture.Builder()
    .Idn("Keysight Technologies", "34401A", "SN1", "1.0")
    .Kinds(InstrumentKind.Dmm)
    .OnQuery("*IDN?", "Keysight Technologies,34401A,SN1,1.0")
    .OnQuery(":MEAS:VOLT:DC?", "1.234")
    .Build();

var catalog = DeviceCatalog.FromFixture("mock://dmm-1", fixture);
var volts = catalog.OpenDmm("mock://dmm-1").MeasureVoltageDc();
Console.WriteLine($"{volts} V");
```
