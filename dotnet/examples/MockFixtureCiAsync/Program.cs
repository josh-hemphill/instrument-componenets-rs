using InstrumentComponents.Catalog;
using InstrumentComponents.Kind;
using InstrumentComponents.Mock;

var fixture = ScriptedFixture.Builder()
    .Idn("Acme", "DMM1", "SN1", "1.0")
    .Kinds(InstrumentKind.Dmm)
    .OnQuery(":MEAS:VOLT:DC?", "1.0")
    .Build();

var catalog = DeviceCatalog.FromFixture("mock://dmm", fixture);
var dmm = await catalog.Device("mock://dmm").OpenDmmAsync();
var volts = await dmm.MeasureVoltageDcAsync();
Console.WriteLine($"{volts} V");
