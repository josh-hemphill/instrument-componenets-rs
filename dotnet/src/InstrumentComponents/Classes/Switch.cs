using InstrumentComponents.Errors;
using InstrumentComponents.Session;

namespace InstrumentComponents.Classes;

/// <summary>
/// Switch / matrix session view (IVI-inspired / SCPI :ROUTe).
/// Path model: routes are matrix channel pairs (ch1, ch2). Use <see cref="PathLabel"/> for naming;
/// IVI ClosePath maps to <see cref="CloseRoute"/>.
/// </summary>
public sealed class Switch
{
    private readonly InstrumentSession _session;

    public Switch(InstrumentSession session) => _session = session;

    public InstrumentSession Session => _session;

    /// <summary>Formats a matrix path label for channels ch1 and ch2 (1-based).</summary>
    public static string PathLabel(uint ch1, uint ch2) => $"(@({ch1},{ch2}))";

    /// <summary>Closes a route between two channels (1-based). IVI ClosePath equivalent.</summary>
    public void CloseRoute(uint ch1, uint ch2) =>
        _session.Scpi.Write(ScpiCommands.SwitchCloseRoute(ch1, ch2));

    /// <summary>Opens a route between two channels (1-based). IVI OpenPath equivalent.</summary>
    public void OpenRoute(uint ch1, uint ch2) =>
        _session.Scpi.Write(ScpiCommands.SwitchOpenRoute(ch1, ch2));

    public bool IsClosed(uint ch1, uint ch2) =>
        ParseClosed(_session.Scpi.Query(ScpiCommands.SwitchIsClosed(ch1, ch2)));

    public void OpenAll() =>
        _session.Scpi.Write(ScpiCommands.SwitchOpenAll);

    internal static bool ParseClosed(string response)
    {
        var trimmed = response.Trim().ToUpperInvariant();
        return trimmed switch
        {
            "1" or "ON" or "CLOSED" => true,
            "0" or "OFF" or "OPEN" => false,
            _ => throw new ParseException($"expected route state, got '{response}'"),
        };
    }
}
