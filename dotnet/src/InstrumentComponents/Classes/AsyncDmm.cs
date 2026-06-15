using InstrumentComponents.Session;
using InstrumentComponents.Scpi;

namespace InstrumentComponents.Classes;

public sealed class AsyncDmm
{
    private readonly AsyncInstrumentSession _session;

    public AsyncDmm(AsyncInstrumentSession session) => _session = session;

    public AsyncInstrumentSession Session => _session;

    public Task<double> MeasureVoltageDcAsync(double? range = null, CancellationToken cancellationToken = default) =>
        QueryF64Async(ScpiCommands.DmmMeasureVoltageDc(range), cancellationToken);

    public Task<double> MeasureVoltageAcAsync(double? range = null, CancellationToken cancellationToken = default) =>
        QueryF64Async(ScpiCommands.DmmMeasureVoltageAc(range), cancellationToken);

    public Task<double> MeasureCurrentDcAsync(double? range = null, CancellationToken cancellationToken = default) =>
        QueryF64Async(ScpiCommands.DmmMeasureCurrentDc(range), cancellationToken);

    public Task<double> MeasureResistanceAsync(double? range = null, CancellationToken cancellationToken = default) =>
        QueryF64Async(ScpiCommands.DmmMeasureResistance(range), cancellationToken);

    public Task ConfigureVoltageDcAsync(double? range = null, double? resolution = null, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(ScpiCommands.DmmConfigureVoltageDc(range, resolution), cancellationToken);

    private async Task<double> QueryF64Async(string cmd, CancellationToken cancellationToken)
    {
        var resp = await _session.Scpi.QueryAsync(cmd, cancellationToken).ConfigureAwait(false);
        return ScpiSession.ParseF64(resp);
    }
}
