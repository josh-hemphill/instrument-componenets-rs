using InstrumentComponents.Dialects;
using InstrumentComponents.Errors;
using InstrumentComponents.Identity;
using InstrumentComponents.Kind;
using InstrumentComponents.Session;
using InstrumentComponents.Scpi;

namespace InstrumentComponents.Classes;

public sealed class AsyncSpectrumAnalyzer
{
    private readonly AsyncInstrumentSession _session;

    public AsyncSpectrumAnalyzer(AsyncInstrumentSession session) => _session = session;

    public AsyncInstrumentSession Session => _session;

    public Task<Idn> QueryIdnAsync(CancellationToken cancellationToken = default) =>
        _session.IdnAsync(cancellationToken);

    public Task ResetAsync(CancellationToken cancellationToken = default) =>
        _session.ResetAsync(cancellationToken);

    public Task OutputOffAsync(CancellationToken cancellationToken = default) =>
        SweepContinuousAsync(false, cancellationToken);

    private DialectProfile Dialect => _session.DialectFor(InstrumentKind.SpectrumAnalyzer);

    private string RequireCommand(string key) =>
        Dialect.Command(key) ?? throw new InstrumentUnsupportedException($"spectrum analyzer dialect missing command '{key}'");

    private string RequireFormatted(string key, params (string Name, string Value)[] vars) =>
        Dialect.FormatCommand(key, vars) ?? throw new InstrumentUnsupportedException($"spectrum analyzer dialect missing command '{key}'");

    public Task SetCenterFrequencyAsync(double hz, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(RequireFormatted("center_frequency", ("hz", ScpiFormat.Double(hz))), cancellationToken);

    public Task SetSpanAsync(double hz, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(RequireFormatted("span", ("hz", ScpiFormat.Double(hz))), cancellationToken);

    public Task SetRbwAsync(double hz, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(RequireFormatted("rbw", ("hz", ScpiFormat.Double(hz))), cancellationToken);

    public Task SetVbwAsync(double hz, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(RequireFormatted("vbw", ("hz", ScpiFormat.Double(hz))), cancellationToken);

    public Task SetRefLevelAsync(double dbm, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(RequireFormatted("ref_level", ("dbm", ScpiFormat.Double(dbm))), cancellationToken);

    public async Task<IReadOnlyList<double>> FetchTraceAsciiAsync(CancellationToken cancellationToken = default)
    {
        var resp = await _session.Scpi.QueryAsync(RequireCommand("trace_data"), cancellationToken).ConfigureAwait(false);
        return ScpiSession.ParseF64Csv(resp);
    }

    public Task MarkerPeakAsync(CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(RequireCommand("marker_peak"), cancellationToken);

    public async Task<double> MarkerXAsync(CancellationToken cancellationToken = default)
    {
        var resp = await _session.Scpi.QueryAsync(RequireCommand("marker_x"), cancellationToken).ConfigureAwait(false);
        return ScpiSession.ParseF64(resp);
    }

    public async Task<double> MarkerYAsync(CancellationToken cancellationToken = default)
    {
        var resp = await _session.Scpi.QueryAsync(RequireCommand("marker_y"), cancellationToken).ConfigureAwait(false);
        return ScpiSession.ParseF64(resp);
    }

    public Task SweepContinuousAsync(bool enabled, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(RequireFormatted("sweep_continuous", ("state", enabled ? "ON" : "OFF")), cancellationToken);

    public Task SingleSweepAsync(CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(RequireCommand("single_sweep"), cancellationToken);

    public async Task WaitOpcAsync(CancellationToken cancellationToken = default) =>
        _ = await _session.Scpi.QueryAsync(RequireCommand("wait_opc"), cancellationToken).ConfigureAwait(false);
}
