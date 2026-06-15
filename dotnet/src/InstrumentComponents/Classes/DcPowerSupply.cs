using InstrumentComponents.Session;
using InstrumentComponents.Scpi;

namespace InstrumentComponents.Classes;

/// <summary>DC power supply session view.</summary>
public sealed class DcPowerSupply
{
    private readonly InstrumentSession _session;

    public DcPowerSupply(InstrumentSession session) => _session = session;

    public InstrumentSession Session => _session;

    public void SetVoltage(uint channel, double volts) =>
        _session.Scpi.Write(ScpiCommands.PsuSetVoltage(channel, volts));

    public void SetCurrentLimit(uint channel, double amps) =>
        _session.Scpi.Write(ScpiCommands.PsuSetCurrentLimit(channel, amps));

    public void OutputEnable(uint channel, bool enabled) =>
        _session.Scpi.Write(ScpiCommands.PsuOutputEnable(channel, enabled));

    public double ReadVoltage(uint channel) =>
        ScpiSession.ParseF64(_session.Scpi.Query(ScpiCommands.PsuReadVoltage(channel)));

    public double ReadCurrent(uint channel) =>
        ScpiSession.ParseF64(_session.Scpi.Query(ScpiCommands.PsuReadCurrent(channel)));
}
