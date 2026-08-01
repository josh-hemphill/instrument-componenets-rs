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

    public static string PathLabel(uint ch1, uint ch2) => Switch.PathLabel(ch1, ch2);

    public Task CloseRouteAsync(uint ch1, uint ch2, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(ScpiCommands.SwitchCloseRoute(ch1, ch2), cancellationToken);

    public Task OpenRouteAsync(uint ch1, uint ch2, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(ScpiCommands.SwitchOpenRoute(ch1, ch2), cancellationToken);

    public async Task<bool> IsClosedAsync(uint ch1, uint ch2, CancellationToken cancellationToken = default)
    {
        var resp = await _session.Scpi.QueryAsync(ScpiCommands.SwitchIsClosed(ch1, ch2), cancellationToken).ConfigureAwait(false);
        return Switch.ParseClosed(resp);
    }

    public Task OpenAllAsync(CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(ScpiCommands.SwitchOpenAll, cancellationToken);
}
