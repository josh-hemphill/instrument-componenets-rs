using InstrumentComponents.Dialects;
using InstrumentComponents.Errors;
using InstrumentComponents.Identity;
using InstrumentComponents.Kind;
using InstrumentComponents.Session;
using InstrumentComponents.Scpi;

namespace InstrumentComponents.Classes;

public enum PowerUnit
{
    Watt,
    Dbm,
}

public static class PowerUnitExtensions
{
    public static string ScpiName(this PowerUnit unit) => unit switch
    {
        PowerUnit.Watt => "W",
        PowerUnit.Dbm => "DBM",
        _ => "DBM",
    };
}

/// <summary>RF / microwave power meter session view.</summary>
public sealed class PowerMeter : IInstrumentIdentity, IInstrumentShutdown
{
    private readonly InstrumentSession _session;

    public PowerMeter(InstrumentSession session) => _session = session;

    public InstrumentSession Session => _session;

    public Idn QueryIdn() => _session.Idn();

    public void Reset() => _session.Reset();

    /// <summary>Power meters have no output stage; safe-shutdown is a no-op.</summary>
    public void OutputOff()
    {
    }

    private DialectProfile Dialect => _session.DialectFor(InstrumentKind.PowerMeter);

    private string RequireCommand(string key) =>
        Dialect.Command(key) ?? throw new InstrumentUnsupportedException($"power meter dialect missing command '{key}'");

    private string RequireFormatted(string key, params (string Name, string Value)[] vars) =>
        Dialect.FormatCommand(key, vars) ?? throw new InstrumentUnsupportedException($"power meter dialect missing command '{key}'");

    public void ConfigureMeasurement(
        PowerUnit unit,
        bool autoRange,
        bool autoAverage,
        double? correctionFreqHz = null,
        double? offsetDb = null)
    {
        _session.Scpi.Write(RequireFormatted("unit", ("unit", unit.ScpiName())));
        _session.Scpi.Write(RequireFormatted("auto_range", ("state", autoRange ? "ON" : "OFF")));
        _session.Scpi.Write(RequireFormatted("auto_average", ("state", autoAverage ? "ON" : "OFF")));
        if (correctionFreqHz is { } hz)
            _session.Scpi.Write(RequireFormatted("correction_frequency", ("hz", ScpiFormat.Double(hz))));
        if (offsetDb is { } db)
            _session.Scpi.Write(RequireFormatted("offset", ("db", ScpiFormat.Double(db))));
    }

    public void Initiate() => _session.Scpi.Write(RequireCommand("initiate"));

    public double Fetch() => ScpiSession.ParseF64(_session.Scpi.Query(RequireCommand("fetch")));

    public double Read() => ScpiSession.ParseF64(_session.Scpi.Query(RequireCommand("read")));
}
