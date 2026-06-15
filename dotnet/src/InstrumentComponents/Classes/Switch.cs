using InstrumentComponents.Errors;
using InstrumentComponents.Session;

namespace InstrumentComponents.Classes;

/// <summary>Switch / matrix session view (IVI-inspired / SCPI :ROUTe).</summary>
public sealed class Switch
{
    private readonly InstrumentSession _session;

    public Switch(InstrumentSession session) => _session = session;

    public InstrumentSession Session => _session;

    public void CloseRoute(uint ch1, uint ch2) =>
        _session.Scpi.Write(ScpiCommands.SwitchCloseRoute(ch1, ch2));

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
