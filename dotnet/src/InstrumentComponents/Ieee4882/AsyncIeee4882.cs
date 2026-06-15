using InstrumentComponents.Identity;
using InstrumentComponents.Scpi;

namespace InstrumentComponents.Ieee4882;

/// <summary>IEEE 488.2 common commands (async).</summary>
public readonly struct AsyncIeee4882
{
    private readonly AsyncScpiSession _session;

    public AsyncIeee4882(AsyncScpiSession session) => _session = session;

    public async Task<Idn> IdnAsync(CancellationToken cancellationToken = default)
    {
        var resp = await _session.QueryAsync("*IDN?", cancellationToken).ConfigureAwait(false);
        return Identity.Idn.Parse(resp);
    }

    public async Task ResetAsync(CancellationToken cancellationToken = default)
    {
        await _session.WriteAsync("*RST", cancellationToken).ConfigureAwait(false);
        await WaitCompleteAsync(cancellationToken).ConfigureAwait(false);
    }

    public Task ClearStatusAsync(CancellationToken cancellationToken = default) =>
        _session.WriteAsync("*CLS", cancellationToken);

    public async Task<bool> OpcQueryAsync(CancellationToken cancellationToken = default)
    {
        if (!await _session.ProbeOpcAsync(cancellationToken).ConfigureAwait(false)) return true;
        return (await _session.QueryAsync("*OPC?", cancellationToken).ConfigureAwait(false)).Trim() == "1";
    }

    public async Task WaitCompleteAsync(CancellationToken cancellationToken = default)
    {
        if (await _session.ProbeOpcAsync(cancellationToken).ConfigureAwait(false))
            _ = await _session.QueryWithTimeoutAsync("*OPC?", TimeSpan.FromSeconds(30), cancellationToken).ConfigureAwait(false);
    }

    public Task<string> OptionsAsync(CancellationToken cancellationToken = default) =>
        _session.QueryAsync("*OPT?", cancellationToken);
}
