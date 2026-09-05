using InstrumentComponents.Dialects;
using InstrumentComponents.Errors;
using InstrumentComponents.Identity;
using InstrumentComponents.Kind;
using InstrumentComponents.Session;
using InstrumentComponents.Scpi;

namespace InstrumentComponents.Classes;

/// <summary>Spectrum analyzer session view.</summary>
public sealed class SpectrumAnalyzer : IInstrumentIdentity, IInstrumentShutdown
{
    private readonly InstrumentSession _session;

    public SpectrumAnalyzer(InstrumentSession session) => _session = session;

    public InstrumentSession Session => _session;

    public Idn QueryIdn() => _session.Idn();

    public void Reset() => _session.Reset();

    public void OutputOff() => SweepContinuous(false);

    private DialectProfile Dialect => _session.DialectFor(InstrumentKind.SpectrumAnalyzer);

    private string RequireCommand(string key) =>
        Dialect.Command(key) ?? throw new InstrumentUnsupportedException($"spectrum analyzer dialect missing command '{key}'");

    private string RequireFormatted(string key, params (string Name, string Value)[] vars) =>
        Dialect.FormatCommand(key, vars) ?? throw new InstrumentUnsupportedException($"spectrum analyzer dialect missing command '{key}'");

    public void SetCenterFrequency(double hz) =>
        _session.Scpi.Write(RequireFormatted("center_frequency", ("hz", ScpiFormat.Double(hz))));

    public void SetSpan(double hz) =>
        _session.Scpi.Write(RequireFormatted("span", ("hz", ScpiFormat.Double(hz))));

    public void SetRbw(double hz) =>
        _session.Scpi.Write(RequireFormatted("rbw", ("hz", ScpiFormat.Double(hz))));

    public void SetVbw(double hz) =>
        _session.Scpi.Write(RequireFormatted("vbw", ("hz", ScpiFormat.Double(hz))));

    public void SetRefLevel(double dbm) =>
        _session.Scpi.Write(RequireFormatted("ref_level", ("dbm", ScpiFormat.Double(dbm))));

    public IReadOnlyList<double> FetchTraceAscii() =>
        ScpiSession.ParseF64Csv(_session.Scpi.Query(RequireCommand("trace_data")));

    public void MarkerPeak() =>
        _session.Scpi.Write(RequireCommand("marker_peak"));

    public double MarkerX() =>
        ScpiSession.ParseF64(_session.Scpi.Query(RequireCommand("marker_x")));

    public double MarkerY() =>
        ScpiSession.ParseF64(_session.Scpi.Query(RequireCommand("marker_y")));

    public void SweepContinuous(bool enabled) =>
        _session.Scpi.Write(RequireFormatted("sweep_continuous", ("state", enabled ? "ON" : "OFF")));

    public void SingleSweep() =>
        _session.Scpi.Write(RequireCommand("single_sweep"));

    public void WaitOpc() =>
        _ = _session.Scpi.Query(RequireCommand("wait_opc"));
}
