using InstrumentComponents.Session;
using InstrumentComponents.Scpi;

namespace InstrumentComponents.Classes;

public sealed class AsyncPowerMeter
{
    private readonly AsyncInstrumentSession _session;

    public AsyncPowerMeter(AsyncInstrumentSession session) => _session = session;

    public AsyncInstrumentSession Session => _session;

    public async Task ConfigureMeasurementAsync(
        PowerUnit unit,
        bool autoRange,
        bool autoAverage,
        double? correctionFreqHz = null,
        double? offsetDb = null,
        CancellationToken cancellationToken = default)
    {
        var scpi = _session.Scpi;
        await scpi.WriteAsync(ScpiCommands.PwrmeterUnit(unit.ScpiName()), cancellationToken).ConfigureAwait(false);
        await scpi.WriteAsync(ScpiCommands.PwrmeterAutoRange(autoRange ? "ON" : "OFF"), cancellationToken).ConfigureAwait(false);
        await scpi.WriteAsync(ScpiCommands.PwrmeterAutoAverage(autoAverage ? "ON" : "OFF"), cancellationToken).ConfigureAwait(false);
        if (correctionFreqHz is { } hz)
            await scpi.WriteAsync(ScpiCommands.PwrmeterCorrectionFrequency(hz), cancellationToken).ConfigureAwait(false);
        if (offsetDb is { } db)
            await scpi.WriteAsync(ScpiCommands.PwrmeterOffset(db), cancellationToken).ConfigureAwait(false);
    }

    public Task InitiateAsync(CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(ScpiCommands.PwrmeterInitiate, cancellationToken);

    public async Task<double> FetchAsync(CancellationToken cancellationToken = default)
    {
        var resp = await _session.Scpi.QueryAsync(ScpiCommands.PwrmeterFetch, cancellationToken).ConfigureAwait(false);
        return ScpiSession.ParseF64(resp);
    }

    public async Task<double> ReadAsync(CancellationToken cancellationToken = default)
    {
        var resp = await _session.Scpi.QueryAsync(ScpiCommands.PwrmeterRead, cancellationToken).ConfigureAwait(false);
        return ScpiSession.ParseF64(resp);
    }
}
