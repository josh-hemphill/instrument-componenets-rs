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

    public Task<double> MeasureCurrentAcAsync(double? range = null, CancellationToken cancellationToken = default) =>
        QueryF64Async(ScpiCommands.DmmMeasureCurrentAc(range), cancellationToken);

    public Task<double> MeasureResistanceAsync(double? range = null, CancellationToken cancellationToken = default) =>
        MeasureResistance2WireAsync(range, cancellationToken);

    public Task<double> MeasureResistance2WireAsync(double? range = null, CancellationToken cancellationToken = default) =>
        QueryF64Async(ScpiCommands.DmmMeasureResistance2wire(range), cancellationToken);

    public Task<double> MeasureResistance4WireAsync(double? range = null, CancellationToken cancellationToken = default) =>
        QueryF64Async(ScpiCommands.DmmMeasureResistance4wire(range), cancellationToken);

    public Task<double> MeasureTemperatureAsync(double? range = null, CancellationToken cancellationToken = default) =>
        QueryF64Async(ScpiCommands.DmmMeasureTemperature(range), cancellationToken);

    public Task ConfigureVoltageDcAsync(double? range = null, double? resolution = null, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(ScpiCommands.DmmConfigureVoltageDc(range, resolution), cancellationToken);

    public Task ConfigureVoltageAcAsync(double? range = null, double? resolution = null, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(ScpiCommands.DmmConfigureVoltageAc(range, resolution), cancellationToken);

    public Task ConfigureCurrentDcAsync(double? range = null, double? resolution = null, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(ScpiCommands.DmmConfigureCurrentDc(range, resolution), cancellationToken);

    public Task ConfigureCurrentAcAsync(double? range = null, double? resolution = null, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(ScpiCommands.DmmConfigureCurrentAc(range, resolution), cancellationToken);

    public Task ConfigureResistanceAsync(double? range = null, double? resolution = null, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(ScpiCommands.DmmConfigureResistance(range, resolution), cancellationToken);

    public Task ConfigureResistance4WireAsync(double? range = null, double? resolution = null, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(ScpiCommands.DmmConfigureResistance4wire(range, resolution), cancellationToken);

    public Task InitiateAsync(CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(ScpiCommands.DmmInitiate, cancellationToken);

    public Task<double> FetchAsync(CancellationToken cancellationToken = default) =>
        QueryF64Async(ScpiCommands.DmmFetch, cancellationToken);

    public Task<double> ReadAsync(CancellationToken cancellationToken = default) =>
        QueryF64Async(ScpiCommands.DmmRead, cancellationToken);

    public Task SoftwareTriggerAsync(CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(ScpiCommands.DmmSoftwareTrigger, cancellationToken);

    private async Task<double> QueryF64Async(string cmd, CancellationToken cancellationToken)
    {
        var resp = await _session.Scpi.QueryAsync(cmd, cancellationToken).ConfigureAwait(false);
        return ScpiSession.ParseF64(resp);
    }
}
