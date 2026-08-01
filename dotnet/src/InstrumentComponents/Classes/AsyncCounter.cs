using InstrumentComponents.Session;
using InstrumentComponents.Scpi;

namespace InstrumentComponents.Classes;

public sealed class AsyncCounter
{
    private readonly AsyncInstrumentSession _session;

    public AsyncCounter(AsyncInstrumentSession session) => _session = session;

    public AsyncInstrumentSession Session => _session;

    public Task<double> MeasureFrequencyAsync(CancellationToken cancellationToken = default) =>
        QueryF64Async(ScpiCommands.CounterMeasureFrequency, cancellationToken);

    public Task<double> MeasurePeriodAsync(CancellationToken cancellationToken = default) =>
        QueryF64Async(ScpiCommands.CounterMeasurePeriod, cancellationToken);

    public Task SetGateTimeAsync(double seconds, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(ScpiCommands.CounterGateTime(seconds), cancellationToken);

    public Task SelectChannelAsync(uint channel, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(ScpiCommands.CounterChannelSelect(channel), cancellationToken);

    public Task ResetTotalizeAsync(CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(ScpiCommands.CounterResetTotalize, cancellationToken);

    public Task<double> ReadTotalizeAsync(CancellationToken cancellationToken = default) =>
        QueryF64Async(ScpiCommands.CounterReadTotalize, cancellationToken);

    private async Task<double> QueryF64Async(string cmd, CancellationToken cancellationToken)
    {
        var resp = await _session.Scpi.QueryAsync(cmd, cancellationToken).ConfigureAwait(false);
        return ScpiSession.ParseF64(resp);
    }
}
