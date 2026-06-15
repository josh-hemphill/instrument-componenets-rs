using InstrumentComponents.Catalog;
using InstrumentComponents.Kind;
using InstrumentComponents.Mock;

namespace InstrumentComponents.Tests;

public class MockCatalogTests
{
    [Fact]
    public void FixtureDmmMeasure()
    {
        var fixture = ScriptedFixture.Builder()
            .Idn("Acme Corp", "SMU2602", "SN001", "1.0")
            .Kinds(InstrumentKind.Dmm, InstrumentKind.DcPowerSupply)
            .OnQuery(":MEAS:VOLT:DC?", "3.300")
            .Build();

        var catalog = DeviceCatalog.FromFixture("mock://smu-1", fixture);
        var dmm = catalog.OpenDmm("mock://smu-1");
        var volts = dmm.MeasureVoltageDc();
        Assert.InRange(volts, 3.299, 3.301);
    }

    [Fact]
    public async Task AsyncFixtureDmmMeasure()
    {
        var fixture = ScriptedFixture.Builder()
            .Idn("Acme", "DMM1", "SN1", "1.0")
            .Kinds(InstrumentKind.Dmm)
            .OnQuery(":MEAS:VOLT:DC?", "1.0")
            .Build();

        var catalog = DeviceCatalog.FromFixture("mock://dmm", fixture);
        var dmm = await catalog.Device("mock://dmm").OpenDmmAsync();
        var volts = await dmm.MeasureVoltageDcAsync();
        Assert.Equal(1.0, volts);
    }
}
