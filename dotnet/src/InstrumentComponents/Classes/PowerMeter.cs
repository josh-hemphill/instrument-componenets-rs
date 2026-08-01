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
public sealed class PowerMeter
{
    private readonly InstrumentSession _session;

    public PowerMeter(InstrumentSession session) => _session = session;

    public InstrumentSession Session => _session;

    public void ConfigureMeasurement(
        PowerUnit unit,
        bool autoRange,
        bool autoAverage,
        double? correctionFreqHz = null,
        double? offsetDb = null)
    {
        var scpi = _session.Scpi;
        scpi.Write(ScpiCommands.PwrmeterUnit(unit.ScpiName()));
        scpi.Write(ScpiCommands.PwrmeterAutoRange(autoRange ? "ON" : "OFF"));
        scpi.Write(ScpiCommands.PwrmeterAutoAverage(autoAverage ? "ON" : "OFF"));
        if (correctionFreqHz is { } hz)
            scpi.Write(ScpiCommands.PwrmeterCorrectionFrequency(hz));
        if (offsetDb is { } db)
            scpi.Write(ScpiCommands.PwrmeterOffset(db));
    }

    public void Initiate() => _session.Scpi.Write(ScpiCommands.PwrmeterInitiate);

    public double Fetch() => ScpiSession.ParseF64(_session.Scpi.Query(ScpiCommands.PwrmeterFetch));

    public double Read() => ScpiSession.ParseF64(_session.Scpi.Query(ScpiCommands.PwrmeterRead));
}
