using System.Globalization;

namespace InstrumentComponents.Scpi;

internal static class ScpiFormat
{
    public static string Double(double value) =>
        value.ToString(CultureInfo.InvariantCulture);
}
