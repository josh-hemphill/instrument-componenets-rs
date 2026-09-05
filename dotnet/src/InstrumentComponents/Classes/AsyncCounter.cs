using InstrumentComponents.Dialects;
using InstrumentComponents.Identity;
using InstrumentComponents.Kind;
using InstrumentComponents.Session;
using InstrumentComponents.Scpi;

namespace InstrumentComponents.Classes;

public sealed class AsyncCounter
{
    private readonly AsyncInstrumentSession _session;

    public AsyncCounter(AsyncInstrumentSession session) => _session = session;

    public AsyncInstrumentSession Session => _session;

    public Task<Idn> QueryIdnAsync(CancellationToken cancellationToken = default) =>
        _session.IdnAsync(cancellationToken);

    public Task ResetAsync(CancellationToken cancellationToken = default) =>
        _session.ResetAsync(cancellationToken);

    public Task OutputOffAsync(CancellationToken cancellationToken = default)
    {
        cancellationToken.ThrowIfCancellationRequested();
        return Task.CompletedTask;
    }

    private DialectProfile Dialect => _session.DialectFor(InstrumentKind.Counter);

    private string Cmd(string key, string fallback, params (string Name, string Value)[] vars) =>
        DialectCommand.Try(Dialect, key, fallback, vars);

    public Task<double> MeasureFrequencyAsync(CancellationToken cancellationToken = default) =>
        QueryF64Async(Cmd("measure_frequency", ScpiCommands.CounterMeasureFrequency), cancellationToken);

    public Task<double> MeasurePeriodAsync(CancellationToken cancellationToken = default) =>
        QueryF64Async(Cmd("measure_period", ScpiCommands.CounterMeasurePeriod), cancellationToken);

    public Task SetGateTimeAsync(double seconds, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(Cmd("gate_time", ScpiCommands.CounterGateTime(seconds),
            ("seconds", ScpiFormat.Double(seconds))), cancellationToken);

    public Task SelectChannelAsync(uint channel, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(Cmd("channel_select", ScpiCommands.CounterChannelSelect(channel),
            ("channel", channel.ToString())), cancellationToken);

    public Task ResetTotalizeAsync(CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(Cmd("reset_totalize", ScpiCommands.CounterResetTotalize), cancellationToken);

    public Task<double> ReadTotalizeAsync(CancellationToken cancellationToken = default) =>
        QueryF64Async(Cmd("read_totalize", ScpiCommands.CounterReadTotalize), cancellationToken);

    private async Task<double> QueryF64Async(string cmd, CancellationToken cancellationToken)
    {
        var resp = await _session.Scpi.QueryAsync(cmd, cancellationToken).ConfigureAwait(false);
        return ScpiSession.ParseF64(resp);
    }
}
