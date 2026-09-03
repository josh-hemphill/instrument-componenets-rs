using InstrumentComponents.Classes;
using InstrumentComponents.Dialects;
using InstrumentComponents.Kind;

namespace InstrumentComponents.Tests;

public class DialectCommandTests
{
    private static DialectProfile TestProfile(params (string Key, string Command)[] commands) =>
        new()
        {
            Id = "test",
            Kind = InstrumentKind.Dmm,
            ManufacturerGlob = "*",
            ModelGlob = "*",
            Channels = 1,
            Commands = commands.ToDictionary(c => c.Key, c => c.Command),
        };

    [Fact]
    public void TryUsesDialectThenFallback()
    {
        var dmm = DialectRegistry.Resolve(InstrumentKind.Dmm);
        Assert.Equal("INIT", DialectCommand.Try(dmm, "initiate", "FALLBACK"));
        Assert.Equal("FALLBACK", DialectCommand.Try(dmm, "missing", "FALLBACK"));
    }

    [Fact]
    public void TryCommandFallsBackOnLeftoverPlaceholders()
    {
        var profile = TestProfile(("read_frequency", ":SOUR{channel}:FREQ?"));
        Assert.Equal("FALLBACK", DialectCommand.Try(profile, "read_frequency", "FALLBACK"));
    }

    [Fact]
    public void TryFallsBackWhenTemplateCannotTakeVars()
    {
        var dmm = DialectRegistry.Resolve(InstrumentKind.Dmm);
        const string fallback = ":MEAS:VOLT:DC? 10";
        Assert.Equal(fallback, DialectCommand.Try(dmm, "measure_voltage_dc", fallback, ("range", "10")));
        Assert.Equal(":MEAS:VOLT:DC?", DialectCommand.Try(dmm, "measure_voltage_dc", ":MEAS:VOLT:DC?"));
    }

    [Fact]
    public void TryFillsDialectPlaceholders()
    {
        var psu = DialectRegistry.Resolve(InstrumentKind.DcPowerSupply);
        var cmd = DialectCommand.Try(psu, "set_voltage", "FALLBACK", ("channel", "1"), ("volts", "3.3"));
        Assert.Equal(":SOUR1:VOLT 3.3", cmd);
    }

    [Fact]
    public void TryUnescapesQuotedScpi()
    {
        var counter = DialectRegistry.Resolve(InstrumentKind.Counter);
        var cmd = DialectCommand.Try(counter, "channel_select", "FALLBACK", ("channel", "1"));
        Assert.Equal(":SENSe:FUNCtion:ON \"FREQ 1\"", cmd);
    }

    [Fact]
    public void TryFallsBackOnLeftoverPlaceholders()
    {
        var fgen = DialectRegistry.Resolve(InstrumentKind.FunctionGenerator);
        Assert.Equal("FALLBACK", DialectCommand.Try(fgen, "set_waveform", "FALLBACK"));
    }

    [Fact]
    public void TryIgnoresExtraOptionalVars()
    {
        var profile = TestProfile(("configure_voltage_dc", ":CONF:VOLT:DC {range}"));
        var cmd = DialectCommand.Try(
            profile,
            "configure_voltage_dc",
            "FALLBACK",
            ("range", "10"),
            ("resolution", "0.001"));
        Assert.Equal(":CONF:VOLT:DC 10", cmd);
    }
}
