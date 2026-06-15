using InstrumentComponents.Session;
using InstrumentComponents.Scpi;

namespace InstrumentComponents.Classes;

/// <summary>Frequency counter session view (IVI-inspired / SCPI :MEASure, :COUNter).</summary>
public sealed class Counter
{
    private readonly InstrumentSession _session;

    public Counter(InstrumentSession session) => _session = session;

    public InstrumentSession Session => _session;

    public double MeasureFrequency() =>
        QueryF64(ScpiCommands.CounterMeasureFrequency);

    public double MeasurePeriod() =>
        QueryF64(ScpiCommands.CounterMeasurePeriod);

    public void ResetTotalize() =>
        _session.Scpi.Write(ScpiCommands.CounterResetTotalize);

    public double ReadTotalize() =>
        QueryF64(ScpiCommands.CounterReadTotalize);

    private double QueryF64(string cmd) =>
        ScpiSession.ParseF64(_session.Scpi.Query(cmd));
}
