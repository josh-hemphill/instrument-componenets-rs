using InstrumentComponents.Address;
using InstrumentComponents.Classes;
using InstrumentComponents.Connect;
using InstrumentComponents.Identity;
using InstrumentComponents.Mock;
using InstrumentComponents.Session;

namespace InstrumentComponents.Tests;

public class TranscriptBehaviorTests
{
    private static InstrumentSession SessionFromFixture(string name, string manufacturer, string model)
    {
        var json = File.ReadAllText(RepoFiles.Fixture(name));
        var transcript = Transcript.FromJson(json);
        var identity = new DeviceIdentity
        {
            Manufacturer = manufacturer,
            Model = model,
            Serial = "SN",
            Firmware = "1.0",
        };
        return new InstrumentSession(
            ResourceAddress.Parse("TCPIP0::127.0.0.1::inst0::INSTR"),
            new MockTransport(transcript.Steps),
            new ConnectOptions(),
            identity);
    }

    [Fact]
    public void Smu2602MeasureVoltageDc()
    {
        var dmm = new Dmm(SessionFromFixture("smu2602.json", "Keithley Instruments", "2602B"));
        Assert.Equal(3.3, dmm.MeasureVoltageDc(), precision: 9);
    }

    [Fact]
    public void ScopeDs1054zCaptureTrace()
    {
        var scope = new Oscilloscope(SessionFromFixture("scope_ds1054z.json", "Rigol Technologies", "DS1054Z"));
        scope.SetTimebaseScale(1e-3);
        var trace = scope.CaptureVoltageTrace(1);
        Assert.Equal([1.0, 2.0, 3.0], trace.Samples);
        Assert.Equal(1e-6, trace.SampleIntervalS, precision: 12);
    }

    [Fact]
    public void Switch34970aIsClosed()
    {
        var sw = new Switch(SessionFromFixture("switch_34970a.json", "Keysight Technologies", "34970A"));
        Assert.True(sw.IsClosed(1, 2));
    }

    [Fact]
    public void Counter53230aMeasureFrequency()
    {
        var counter = new Counter(SessionFromFixture("counter_53230a.json", "Keysight Technologies", "53230A"));
        Assert.Equal(1000.0, counter.MeasureFrequency(), precision: 9);
    }
}
