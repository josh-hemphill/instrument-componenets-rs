using InstrumentComponents.Dialects;
using InstrumentComponents.Identity;
using InstrumentComponents.Kind;
using InstrumentComponents.Session;

namespace InstrumentComponents.Classes;

/// <summary>
/// Async switch / matrix session view.
/// Path model: routes are matrix channel pairs (ch1, ch2). Use <see cref="Switch.PathLabel"/> for naming;
/// IVI ClosePath maps to <see cref="CloseRouteAsync"/>.
/// </summary>
public sealed class AsyncSwitch
{
    private readonly AsyncInstrumentSession _session;

    public AsyncSwitch(AsyncInstrumentSession session) => _session = session;

    public AsyncInstrumentSession Session => _session;

    public Task<Idn> QueryIdnAsync(CancellationToken cancellationToken = default) =>
        _session.IdnAsync(cancellationToken);

    public Task ResetAsync(CancellationToken cancellationToken = default) =>
        _session.ResetAsync(cancellationToken);

    public Task OutputOffAsync(CancellationToken cancellationToken = default) =>
        OpenAllAsync(cancellationToken);

    private DialectProfile Dialect => _session.DialectFor(InstrumentKind.Switch);

    private string Cmd(string key, string fallback, params (string Name, string Value)[] vars) =>
        DialectCommand.Try(Dialect, key, fallback, vars);

    public static string PathLabel(uint ch1, uint ch2) => Switch.PathLabel(ch1, ch2);

    public Task CloseRouteAsync(uint ch1, uint ch2, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(Cmd("close_route", ScpiCommands.SwitchCloseRoute(ch1, ch2),
            ("ch1", ch1.ToString()), ("ch2", ch2.ToString())), cancellationToken);

    public Task OpenRouteAsync(uint ch1, uint ch2, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(Cmd("open_route", ScpiCommands.SwitchOpenRoute(ch1, ch2),
            ("ch1", ch1.ToString()), ("ch2", ch2.ToString())), cancellationToken);

    public async Task<bool> IsClosedAsync(uint ch1, uint ch2, CancellationToken cancellationToken = default)
    {
        var resp = await _session.Scpi.QueryAsync(Cmd("is_closed", ScpiCommands.SwitchIsClosed(ch1, ch2),
            ("ch1", ch1.ToString()), ("ch2", ch2.ToString())), cancellationToken).ConfigureAwait(false);
        return Switch.ParseClosed(resp);
    }

    public Task OpenAllAsync(CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(Cmd("open_all", ScpiCommands.SwitchOpenAll), cancellationToken);
}
