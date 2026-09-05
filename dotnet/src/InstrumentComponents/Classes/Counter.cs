using InstrumentComponents.Dialects;
using InstrumentComponents.Identity;
using InstrumentComponents.Kind;
using InstrumentComponents.Session;
using InstrumentComponents.Scpi;

namespace InstrumentComponents.Classes;

/// <summary>Frequency counter session view (IVI-inspired / SCPI :MEASure, :COUNter).</summary>
public sealed class Counter : IInstrumentIdentity, IInstrumentShutdown
{
    private readonly InstrumentSession _session;

    public Counter(InstrumentSession session) => _session = session;

    public InstrumentSession Session => _session;

    public Idn QueryIdn() => _session.Idn();

    public void Reset() => _session.Reset();

    /// <summary>Counters have no output stage; safe-shutdown is a no-op.</summary>
    public void OutputOff()
    {
    }

    private DialectProfile Dialect => _session.DialectFor(InstrumentKind.Counter);

    private string Cmd(string key, string fallback, params (string Name, string Value)[] vars) =>
        DialectCommand.Try(Dialect, key, fallback, vars);

    public double MeasureFrequency() =>
        QueryF64(Cmd("measure_frequency", ScpiCommands.CounterMeasureFrequency));

    public double MeasurePeriod() =>
        QueryF64(Cmd("measure_period", ScpiCommands.CounterMeasurePeriod));

    public void SetGateTime(double seconds) =>
        _session.Scpi.Write(Cmd("gate_time", ScpiCommands.CounterGateTime(seconds),
            ("seconds", ScpiFormat.Double(seconds))));

    public void SelectChannel(uint channel) =>
        _session.Scpi.Write(Cmd("channel_select", ScpiCommands.CounterChannelSelect(channel),
            ("channel", channel.ToString())));

    public void ResetTotalize() =>
        _session.Scpi.Write(Cmd("reset_totalize", ScpiCommands.CounterResetTotalize));

    public double ReadTotalize() =>
        QueryF64(Cmd("read_totalize", ScpiCommands.CounterReadTotalize));

    private double QueryF64(string cmd) =>
        ScpiSession.ParseF64(_session.Scpi.Query(cmd));
}
