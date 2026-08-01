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
    public void FixtureDmmDepth()
    {
        // ScriptedFixture emits all OnWrite steps before OnQuery steps.
        var fixture = ScriptedFixture.Builder()
            .Idn("Keysight Technologies", "34461A", "SN1", "1.0")
            .Kinds(InstrumentKind.Dmm)
            .OnWrite(":CONF:VOLT:AC")
            .OnWrite("INIT")
            .OnWrite("*TRG")
            .OnQuery(":MEAS:CURR:AC?", "0.012")
            .OnQuery(":MEAS:RES?", "1000.0")
            .OnQuery(":MEAS:FRES?", "999.5")
            .OnQuery(":MEAS:TEMP?", "25.0")
            .OnQuery("FETC?", "1.234")
            .OnQuery("READ?", "2.345")
            .Build();

        var catalog = DeviceCatalog.FromFixture("mock://dmm-depth", fixture);
        var dmm = catalog.OpenDmm("mock://dmm-depth");
        dmm.ConfigureVoltageAc();
        dmm.Initiate();
        dmm.SoftwareTrigger();
        Assert.Equal(0.012, dmm.MeasureCurrentAc(), precision: 9);
        Assert.Equal(1000.0, dmm.MeasureResistance2Wire(), precision: 9);
        Assert.Equal(999.5, dmm.MeasureResistance4Wire(), precision: 9);
        Assert.Equal(25.0, dmm.MeasureTemperature(), precision: 9);
        Assert.Equal(1.234, dmm.Fetch(), precision: 9);
        Assert.Equal(2.345, dmm.Read(), precision: 9);
    }

    [Fact]
    public void FixturePsuDepth()
    {
        var fixture = ScriptedFixture.Builder()
            .Idn("Keysight Technologies", "E36312A", "SN1", "1.0")
            .Kinds(InstrumentKind.DcPowerSupply)
            .OnWrite(":SOUR1:VOLT:PROT 5.5")
            .OnWrite(":SOUR1:VOLT:PROT:STAT ON")
            .OnWrite(":OUTP1:SENS ON")
            .OnQuery(":OUTP1?", "1")
            .OnQuery(":SOUR1:VOLT:PROT:STAT?", "ON")
            .Build();

        var catalog = DeviceCatalog.FromFixture("mock://psu-depth", fixture);
        var psu = catalog.OpenDcPowerSupply("mock://psu-depth");
        Assert.Equal(1u, psu.ChannelCount);
        psu.OvpLevel(1, 5.5);
        psu.OvpEnable(1, true);
        psu.SenseEnable(1, true);
        Assert.True(psu.OutputStateQuery(1));
        Assert.True(psu.OvpQuery(1));
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
