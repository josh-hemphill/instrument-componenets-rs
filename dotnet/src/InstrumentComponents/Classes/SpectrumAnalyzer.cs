using InstrumentComponents.Dialects;
using InstrumentComponents.Kind;
using InstrumentComponents.Session;
using InstrumentComponents.Scpi;

namespace InstrumentComponents.Classes;

/// <summary>Spectrum analyzer session view.</summary>
public sealed class SpectrumAnalyzer
{
    private readonly InstrumentSession _session;

    public SpectrumAnalyzer(InstrumentSession session) => _session = session;

    public InstrumentSession Session => _session;

    private DialectProfile Dialect =>
        DialectRegistry.Resolve(
            InstrumentKind.SpectrumAnalyzer,
            _session.Identity.Manufacturer,
            _session.Identity.Model);

    public void SetCenterFrequency(double hz) =>
        _session.Scpi.Write(ScpiCommands.SpecanCenterFrequency(hz));

    public void SetSpan(double hz) =>
        _session.Scpi.Write(ScpiCommands.SpecanSpan(hz));

    public void SetRbw(double hz) =>
        _session.Scpi.Write(ScpiCommands.SpecanRbw(hz));

    public void SetVbw(double hz) =>
        _session.Scpi.Write(ScpiCommands.SpecanVbw(hz));

    public void SetRefLevel(double dbm) =>
        _session.Scpi.Write(ScpiCommands.SpecanRefLevel(dbm));

    public IReadOnlyList<double> FetchTraceAscii()
    {
        var cmd = Dialect.Command("trace_data") ?? ScpiCommands.SpecanTraceData;
        return ScpiSession.ParseF64Csv(_session.Scpi.Query(cmd));
    }

    public void MarkerPeak()
    {
        var cmd = Dialect.Command("marker_peak") ?? ScpiCommands.SpecanMarkerPeak;
        _session.Scpi.Write(cmd);
    }

    public double MarkerX()
    {
        var cmd = Dialect.Command("marker_x") ?? ScpiCommands.SpecanMarkerX;
        return ScpiSession.ParseF64(_session.Scpi.Query(cmd));
    }

    public double MarkerY()
    {
        var cmd = Dialect.Command("marker_y") ?? ScpiCommands.SpecanMarkerY;
        return ScpiSession.ParseF64(_session.Scpi.Query(cmd));
    }

    public void SweepContinuous(bool enabled) =>
        _session.Scpi.Write(ScpiCommands.SpecanSweepContinuous(enabled ? "ON" : "OFF"));

    public void SingleSweep()
    {
        var cmd = Dialect.Command("single_sweep") ?? ScpiCommands.SpecanSingleSweep;
        _session.Scpi.Write(cmd);
    }

    public void WaitOpc()
    {
        var cmd = Dialect.Command("wait_opc") ?? ScpiCommands.SpecanWaitOpc;
        _ = _session.Scpi.Query(cmd);
    }
}
