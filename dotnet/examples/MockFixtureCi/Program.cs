using InstrumentComponents.Catalog;
using InstrumentComponents.Kind;
using InstrumentComponents.Mock;

// Consumer CI pattern: test application logic without VISA installed.
var fixture = ScriptedFixture.Builder()
    .Idn("Acme Corp", "SMU2602", "SN001", "1.0")
    .Kinds(InstrumentKind.Dmm, InstrumentKind.DcPowerSupply)
    .OnQuery(":MEAS:VOLT:DC?", "3.300")
    .Build();

var catalog = DeviceCatalog.FromFixture("mock://smu-1", fixture);
var dmm = catalog.OpenDmm("mock://smu-1");
var volts = dmm.MeasureVoltageDc();
Console.WriteLine($"DUT voltage: {volts} V");
