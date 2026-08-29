using InstrumentComponents.Errors;
using System.Globalization;

namespace InstrumentComponents.Scpi;

internal static class ScpiProtocol
{
    public static string NormalizeCommand(string command, string terminator)
    {
        if (!command.EndsWith(terminator, StringComparison.Ordinal))
            return command + terminator;
        return command;
    }

    public static uint MaxWriteAttempts(bool idempotent, uint retries) => idempotent ? retries + 1 : 1;

    public static double ParseF64(string response)
    {
        if (!double.TryParse(response.Trim(), NumberStyles.Float, CultureInfo.InvariantCulture, out var value))
            throw new ParseException($"expected number, got '{response}'");
        return value;
    }

    public static IReadOnlyList<double> ParseF64Csv(string response)
    {
        var values = new List<double>();
        foreach (var part in response.Split(','))
        {
            var trimmed = part.Trim();
            if (trimmed.Length == 0)
                continue;
            if (!double.TryParse(trimmed, NumberStyles.Float, CultureInfo.InvariantCulture, out var value))
                throw new ParseException($"expected number, got '{trimmed}'");
            values.Add(value);
        }
        return values;
    }
}

