using InstrumentComponents.Dialects;
using InstrumentComponents.Errors;
using InstrumentComponents.Kind;
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

    private DialectProfile Dialect => _session.DialectFor(InstrumentKind.Switch);

    private string Cmd(string key, string fallback, params (string Name, string Value)[] vars) =>
        DialectCommand.Try(Dialect, key, fallback, vars);

    /// <summary>Formats a matrix path label for channels ch1 and ch2 (1-based).</summary>
    public static string PathLabel(uint ch1, uint ch2) => $"(@({ch1},{ch2}))";

    /// <summary>Closes a route between two channels (1-based). IVI ClosePath equivalent.</summary>
    public void CloseRoute(uint ch1, uint ch2) =>
        _session.Scpi.Write(Cmd("close_route", ScpiCommands.SwitchCloseRoute(ch1, ch2),
            ("ch1", ch1.ToString()), ("ch2", ch2.ToString())));

    /// <summary>Opens a route between two channels (1-based). IVI OpenPath equivalent.</summary>
    public void OpenRoute(uint ch1, uint ch2) =>
        _session.Scpi.Write(Cmd("open_route", ScpiCommands.SwitchOpenRoute(ch1, ch2),
            ("ch1", ch1.ToString()), ("ch2", ch2.ToString())));

    public bool IsClosed(uint ch1, uint ch2) =>
        ParseClosed(_session.Scpi.Query(Cmd("is_closed", ScpiCommands.SwitchIsClosed(ch1, ch2),
            ("ch1", ch1.ToString()), ("ch2", ch2.ToString()))));

    public void OpenAll() =>
        _session.Scpi.Write(Cmd("open_all", ScpiCommands.SwitchOpenAll));

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
