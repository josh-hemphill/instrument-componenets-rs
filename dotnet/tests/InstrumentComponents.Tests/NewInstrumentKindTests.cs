using InstrumentComponents.Catalog;
using InstrumentComponents.Classifier;
using InstrumentComponents.Connect;
using InstrumentComponents.Kind;
using InstrumentComponents.Mock;
using InstrumentComponents.Registry;
using InstrumentComponents.Scpi;

namespace InstrumentComponents.Tests;

public class NewInstrumentKindTests
{
    [Fact]
    public void ScopeReadonlyProbeSucceeds()
    {
        var transport = new MockTransport([
            new WriteStep { Data = ":TIMebase:SCALe?\n" },
            new ReadStep { Data = "1e-3\n" },
        ]);
        var session = new ScpiSession(transport, new ConnectOptions());
        Assert.True(CapabilityProbes.ProbeAny(session, CapabilityProbes.ScopeReadonlyCommands, CapabilityProbes.ProbeTimeout));
    }

    [Fact]
    public void CounterReadonlyProbeSucceeds()
    {
        var transport = new MockTransport([
            new WriteStep { Data = ":COUNter:DATA?\n" },
            new ReadStep { Data = "42\n" },
        ]);
        var session = new ScpiSession(transport, new ConnectOptions());
        Assert.True(CapabilityProbes.ProbeAny(session, CapabilityProbes.CounterReadonlyCommands, CapabilityProbes.ProbeTimeout));
    }

    [Fact]
    public void MockCatalogOpensCounter()
    {
        var fixture = ScriptedFixture.Builder()
            .Idn("Keysight Technologies", "53230A", "SN001", "1.0")
            .Kinds([InstrumentKind.Counter])
            .OnQuery(":MEASure:FREQuency?", "1000.0")
            .Build();

        var catalog = DeviceCatalog.FromFixture("mock://counter-1", fixture);
        var counter = catalog.OpenCounter("mock://counter-1");
        Assert.Equal(1000.0, counter.MeasureFrequency(), precision: 6);
    }

    [Fact]
    public void MockCatalogOpensSwitch()
    {
        var fixture = ScriptedFixture.Builder()
            .Idn("Keysight Technologies", "34970A", "SN001", "1.0")
            .Kinds([InstrumentKind.Switch])
            .OnQuery(":ROUTe:CLOS? (@(1,2))", "1")
            .Build();

        var catalog = DeviceCatalog.FromFixture("mock://switch-1", fixture);
        var sw = catalog.OpenSwitch("mock://switch-1");
        Assert.True(sw.IsClosed(1, 2));
    }

    [Fact]
    public void MockCatalogOpensOscilloscope()
    {
        var fixture = ScriptedFixture.Builder()
            .Idn("Rigol Technologies", "DS1054Z", "SN001", "1.0")
            .Kinds([InstrumentKind.Oscilloscope])
            .OnWrite(":TIMebase:SCALe 0.001")
            .OnWrite(":WAVeform:SOURce CHAN1")
            .OnWrite(":WAVeform:FORMat ASCii")
            .OnQuery(":WAVeform:PREamble?", "0,0,3,0,1e-6,0,0,1,0,0")
            .OnQuery(":WAVeform:DATA?", "1.0,2.0,3.0")
            .Build();

        var catalog = DeviceCatalog.FromFixture("mock://scope-1", fixture);
        var scope = catalog.OpenOscilloscope("mock://scope-1");
        scope.SetTimebaseScale(1e-3);
        var trace = scope.CaptureVoltageTrace(1);
        Assert.Equal(3, trace.Samples.Count);
        Assert.Equal(1e-6, trace.SampleIntervalS, precision: 12);
    }

    [Fact]
    public void ParseF64CsvHandlesSpacesAndTrailingComma()
    {
        var values = ScpiSession.ParseF64Csv(" 1.0, 2.5 ,3.0,");
        Assert.Equal([1.0, 2.5, 3.0], values);
    }

    [Fact]
    public void RegistryHintForOscilloscopeModel()
    {
        var registry = ModelRegistry.Embedded();
        var kinds = registry.LookupModel("Rigol Technologies", "DS1054Z");
        Assert.NotNull(kinds);
        Assert.Contains(InstrumentKind.Oscilloscope, kinds);
    }
}
