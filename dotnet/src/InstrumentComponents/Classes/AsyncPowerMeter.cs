using InstrumentComponents.Dialects;
using InstrumentComponents.Errors;
using InstrumentComponents.Kind;
using InstrumentComponents.Session;
using InstrumentComponents.Scpi;

namespace InstrumentComponents.Classes;

public sealed class AsyncPowerMeter
{
    private readonly AsyncInstrumentSession _session;

    public AsyncPowerMeter(AsyncInstrumentSession session) => _session = session;

    public AsyncInstrumentSession Session => _session;

    private DialectProfile Dialect => _session.DialectFor(InstrumentKind.PowerMeter);

    private string RequireCommand(string key) =>
        Dialect.Command(key) ?? throw new InstrumentUnsupportedException($"power meter dialect missing command '{key}'");

    private string RequireFormatted(string key, params (string Name, string Value)[] vars) =>
        Dialect.FormatCommand(key, vars) ?? throw new InstrumentUnsupportedException($"power meter dialect missing command '{key}'");

    public async Task ConfigureMeasurementAsync(
        PowerUnit unit,
        bool autoRange,
        bool autoAverage,
        double? correctionFreqHz = null,
        double? offsetDb = null,
        CancellationToken cancellationToken = default)
    {
        await _session.Scpi.WriteAsync(RequireFormatted("unit", ("unit", unit.ScpiName())), cancellationToken).ConfigureAwait(false);
        await _session.Scpi.WriteAsync(RequireFormatted("auto_range", ("state", autoRange ? "ON" : "OFF")), cancellationToken).ConfigureAwait(false);
        await _session.Scpi.WriteAsync(RequireFormatted("auto_average", ("state", autoAverage ? "ON" : "OFF")), cancellationToken).ConfigureAwait(false);
        if (correctionFreqHz is { } hz)
            await _session.Scpi.WriteAsync(RequireFormatted("correction_frequency", ("hz", ScpiFormat.Double(hz))), cancellationToken).ConfigureAwait(false);
        if (offsetDb is { } db)
            await _session.Scpi.WriteAsync(RequireFormatted("offset", ("db", ScpiFormat.Double(db))), cancellationToken).ConfigureAwait(false);
    }

    public Task InitiateAsync(CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(RequireCommand("initiate"), cancellationToken);

    public async Task<double> FetchAsync(CancellationToken cancellationToken = default)
    {
        var resp = await _session.Scpi.QueryAsync(RequireCommand("fetch"), cancellationToken).ConfigureAwait(false);
        return ScpiSession.ParseF64(resp);
    }

    public async Task<double> ReadAsync(CancellationToken cancellationToken = default)
    {
        var resp = await _session.Scpi.QueryAsync(RequireCommand("read"), cancellationToken).ConfigureAwait(false);
        return ScpiSession.ParseF64(resp);
    }
}
