using System.Globalization;
using InstrumentComponents.Classes;
using InstrumentComponents.Scpi;

namespace InstrumentComponents.Tests;

public class CultureInvariantTests
{
    [Fact]
    public void ScpiNumbersStayDotDecimalUnderGermanCulture()
    {
        var previous = CultureInfo.CurrentCulture;
        CultureInfo.CurrentCulture = CultureInfo.GetCultureInfo("de-DE");
        try
        {
            Assert.Equal(":MEAS:VOLT:DC? 1.5", ScpiCommands.DmmMeasureVoltageDc(1.5));
            Assert.Equal(1.5, ScpiProtocol.ParseF64("1.5"));
            Assert.Equal([1.0, 2.5], ScpiProtocol.ParseF64Csv("1.0,2.5"));
        }
        finally
        {
            CultureInfo.CurrentCulture = previous;
        }
    }
}
