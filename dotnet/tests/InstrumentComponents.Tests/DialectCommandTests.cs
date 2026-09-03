using InstrumentComponents.Classes;
using InstrumentComponents.Dialects;
using InstrumentComponents.Kind;

namespace InstrumentComponents.Tests;

public class DialectCommandTests
{
    [Fact]
    public void TryUsesDialectThenFallback()
    {
        var dmm = DialectRegistry.Resolve(InstrumentKind.Dmm);
        Assert.Equal("INIT", DialectCommand.Try(dmm, "initiate", "FALLBACK"));
        Assert.Equal("FALLBACK", DialectCommand.Try(dmm, "missing", "FALLBACK"));
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
    public void TryFallsBackOnLeftoverPlaceholders()
    {
        var fgen = DialectRegistry.Resolve(InstrumentKind.FunctionGenerator);
        Assert.Equal("FALLBACK", DialectCommand.Try(fgen, "set_waveform", "FALLBACK"));
    }
}
