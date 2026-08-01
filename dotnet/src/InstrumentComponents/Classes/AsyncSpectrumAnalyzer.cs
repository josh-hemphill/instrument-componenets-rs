using InstrumentComponents.Dialects;
using InstrumentComponents.Kind;
using InstrumentComponents.Session;
using InstrumentComponents.Scpi;

namespace InstrumentComponents.Classes;

public sealed class AsyncSpectrumAnalyzer
{
    private readonly AsyncInstrumentSession _session;

    public AsyncSpectrumAnalyzer(AsyncInstrumentSession session) => _session = session;

    public AsyncInstrumentSession Session => _session;

    private DialectProfile Dialect =>
        DialectRegistry.Resolve(
            InstrumentKind.SpectrumAnalyzer,
            _session.Identity.Manufacturer,
            _session.Identity.Model);

    public Task SetCenterFrequencyAsync(double hz, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(ScpiCommands.SpecanCenterFrequency(hz), cancellationToken);

    public Task SetSpanAsync(double hz, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(ScpiCommands.SpecanSpan(hz), cancellationToken);

    public Task SetRbwAsync(double hz, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(ScpiCommands.SpecanRbw(hz), cancellationToken);

    public Task SetVbwAsync(double hz, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(ScpiCommands.SpecanVbw(hz), cancellationToken);

    public Task SetRefLevelAsync(double dbm, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(ScpiCommands.SpecanRefLevel(dbm), cancellationToken);

    public async Task<IReadOnlyList<double>> FetchTraceAsciiAsync(CancellationToken cancellationToken = default)
    {
        var cmd = Dialect.Command("trace_data") ?? ScpiCommands.SpecanTraceData;
        var resp = await _session.Scpi.QueryAsync(cmd, cancellationToken).ConfigureAwait(false);
        return ScpiSession.ParseF64Csv(resp);
    }

    public Task MarkerPeakAsync(CancellationToken cancellationToken = default)
    {
        var cmd = Dialect.Command("marker_peak") ?? ScpiCommands.SpecanMarkerPeak;
        return _session.Scpi.WriteAsync(cmd, cancellationToken);
    }

    public async Task<double> MarkerXAsync(CancellationToken cancellationToken = default)
    {
        var cmd = Dialect.Command("marker_x") ?? ScpiCommands.SpecanMarkerX;
        var resp = await _session.Scpi.QueryAsync(cmd, cancellationToken).ConfigureAwait(false);
        return ScpiSession.ParseF64(resp);
    }

    public async Task<double> MarkerYAsync(CancellationToken cancellationToken = default)
    {
        var cmd = Dialect.Command("marker_y") ?? ScpiCommands.SpecanMarkerY;
        var resp = await _session.Scpi.QueryAsync(cmd, cancellationToken).ConfigureAwait(false);
        return ScpiSession.ParseF64(resp);
    }

    public Task SweepContinuousAsync(bool enabled, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(ScpiCommands.SpecanSweepContinuous(enabled ? "ON" : "OFF"), cancellationToken);

    public Task SingleSweepAsync(CancellationToken cancellationToken = default)
    {
        var cmd = Dialect.Command("single_sweep") ?? ScpiCommands.SpecanSingleSweep;
        return _session.Scpi.WriteAsync(cmd, cancellationToken);
    }

    public async Task WaitOpcAsync(CancellationToken cancellationToken = default)
    {
        var cmd = Dialect.Command("wait_opc") ?? ScpiCommands.SpecanWaitOpc;
        _ = await _session.Scpi.QueryAsync(cmd, cancellationToken).ConfigureAwait(false);
    }
}
