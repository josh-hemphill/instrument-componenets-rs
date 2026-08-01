using InstrumentComponents.Catalog;
using InstrumentComponents.Classifier;
using InstrumentComponents.Classes;
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
    public void PwrmeterReadonlyProbeSucceeds()
    {
        var transport = new MockTransport([
            new WriteStep { Data = ":UNIT:POW?\n" },
            new ReadStep { Data = "DBM\n" },
        ]);
        var session = new ScpiSession(transport, new ConnectOptions());
        Assert.True(CapabilityProbes.ProbeAny(session, CapabilityProbes.PwrmeterReadonlyCommands, CapabilityProbes.ProbeTimeout));
    }

    [Fact]
    public void SpecanReadonlyProbeSucceeds()
    {
        var transport = new MockTransport([
            new WriteStep { Data = ":FREQ:CENT?\n" },
            new ReadStep { Data = "1e9\n" },
        ]);
        var session = new ScpiSession(transport, new ConnectOptions());
        Assert.True(CapabilityProbes.ProbeAny(session, CapabilityProbes.SpecanReadonlyCommands, CapabilityProbes.ProbeTimeout));
    }

    [Fact]
    public void MockCatalogOpensCounter()
    {
        var fixture = ScriptedFixture.Builder()
            .Idn("Keysight Technologies", "53230A", "SN001", "1.0")
            .Kinds([InstrumentKind.Counter])
            .OnWrite(":SENSe:FREQuency:APERture 0.1")
            .OnWrite(":SENSe:FUNCtion:ON \"FREQ 1\"")
            .OnQuery(":MEASure:FREQuency?", "1000.0")
            .Build();

        var catalog = DeviceCatalog.FromFixture("mock://counter-1", fixture);
        var counter = catalog.OpenCounter("mock://counter-1");
        counter.SetGateTime(0.1);
        counter.SelectChannel(1);
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
        Assert.Equal("(@(1,2))", Switch.PathLabel(1, 2));
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
    public void MockCatalogScopeTriggerAndMeasure()
    {
        var fixture = ScriptedFixture.Builder()
            .Idn("Rigol Technologies", "DS1054Z", "SN001", "1.0")
            .Kinds([InstrumentKind.Oscilloscope])
            .OnWrite(":CHANnel1:DISP ON")
            .OnWrite(":CHANnel1:COUP DC")
            .OnWrite(":TRIGger:EDGE:SOURce CHAN1")
            .OnWrite(":TRIGger:EDGE:LEVel 0.5")
            .OnWrite(":TRIGger:EDGE:SLOPe POS")
            .OnWrite(":SINGle")
            .OnQuery(":MEASure:VPP? CHAN1", "2.0")
            .OnQuery(":MEASure:FREQuency? CHAN1", "1000.0")
            .Build();

        var catalog = DeviceCatalog.FromFixture("mock://scope-2", fixture);
        var scope = catalog.OpenOscilloscope("mock://scope-2");
        scope.SetChannelDisplay(1, true);
        scope.SetChannelCoupling(1, "DC");
        scope.SetTriggerSource("CHAN1");
        scope.SetTriggerLevel(0.5);
        scope.SetTriggerSlope("POS");
        scope.Single();
        Assert.Equal(2.0, scope.MeasureVpp(1), precision: 9);
        Assert.Equal(1000.0, scope.MeasureFrequency(1), precision: 9);
    }

    [Fact]
    public void MockCatalogOpensFgenDepth()
    {
        var fixture = ScriptedFixture.Builder()
            .Idn("Keysight Technologies", "33522B", "SN1", "1.0")
            .Kinds([InstrumentKind.FunctionGenerator])
            .OnWrite(":SOUR:FUNC:SQU:DCYC 25")
            .OnWrite(":OUTP:LOAD 50")
            .OnWrite(":SOUR:BURS:NCYC 4")
            .OnWrite(":SOUR:BURS:STAT ON")
            .OnWrite(":TRIG:SOUR BUS")
            .Build();

        var catalog = DeviceCatalog.FromFixture("mock://fgen-1", fixture);
        var fgen = catalog.OpenFunctionGenerator("mock://fgen-1");
        fgen.SetDutyCycle(25);
        fgen.SetLoad(50);
        fgen.SetBurstCount(4);
        fgen.SetBurstState(true);
        fgen.SetBurstTriggerSource("BUS");
    }

    [Fact]
    public void MockCatalogOpensPowerMeter()
    {
        var fixture = ScriptedFixture.Builder()
            .Idn("Keysight Technologies", "U2001A", "SN1", "1.0")
            .Kinds([InstrumentKind.PowerMeter])
            .OnWrite(":UNIT:POW DBM")
            .OnWrite(":SENS:POW:RANG:AUTO ON")
            .OnWrite(":SENS:AVER:COUN:AUTO ON")
            .OnWrite($":SENS:FREQ {1e9}")
            .OnQuery("READ?", "-10.5")
            .Build();

        var catalog = DeviceCatalog.FromFixture("mock://pm-1", fixture);
        var pm = catalog.OpenPowerMeter("mock://pm-1");
        pm.ConfigureMeasurement(PowerUnit.Dbm, true, true, 1e9);
        Assert.Equal(-10.5, pm.Read(), precision: 9);
    }

    [Fact]
    public void MockCatalogOpensSpectrumAnalyzer()
    {
        var fixture = ScriptedFixture.Builder()
            .Idn("Keysight Technologies", "N9010B", "SN1", "1.0")
            .Kinds([InstrumentKind.SpectrumAnalyzer])
            .OnWrite($":FREQ:CENT {1e9}")
            .OnWrite($":FREQ:SPAN {1e6}")
            .OnWrite($":BAND {1000.0}")
            .OnWrite($":BAND:VID {1000.0}")
            .OnWrite($":DISP:WIND:TRAC:Y:RLEV {0.0}")
            .OnWrite(":INIT:CONT OFF")
            .OnWrite(":INIT:IMM")
            .OnWrite(":CALC:MARK:MAX")
            .OnQuery("*OPC?", "1")
            .OnQuery(":CALC:MARK:X?", "1e9")
            .OnQuery(":CALC:MARK:Y?", "-20")
            .OnQuery(":TRAC:DATA? TRACE1", "-80,-70,-60")
            .Build();

        var catalog = DeviceCatalog.FromFixture("mock://sa-1", fixture);
        var sa = catalog.OpenSpectrumAnalyzer("mock://sa-1");
        sa.SetCenterFrequency(1e9);
        sa.SetSpan(1e6);
        sa.SetRbw(1000);
        sa.SetVbw(1000);
        sa.SetRefLevel(0);
        sa.SweepContinuous(false);
        sa.SingleSweep();
        sa.MarkerPeak();
        sa.WaitOpc();
        Assert.Equal(1e9, sa.MarkerX(), precision: 0);
        Assert.Equal(-20, sa.MarkerY(), precision: 9);
        Assert.Equal(3, sa.FetchTraceAscii().Count);
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

    [Fact]
    public void RegistryHintForPowerMeterAndSpecan()
    {
        var registry = ModelRegistry.Embedded();
        Assert.Contains(InstrumentKind.PowerMeter, registry.LookupModel("Keysight Technologies", "U2001A")!);
        Assert.Contains(InstrumentKind.SpectrumAnalyzer, registry.LookupModel("Keysight Technologies", "N9010B")!);
    }
}
