using InstrumentComponents.Dialects;
using InstrumentComponents.Kind;
using InstrumentComponents.Session;
using InstrumentComponents.Scpi;

namespace InstrumentComponents.Classes;

/// <summary>DC power supply session view.</summary>
public sealed class DcPowerSupply
{
    private readonly InstrumentSession _session;

    public DcPowerSupply(InstrumentSession session) => _session = session;

    public InstrumentSession Session => _session;

    public uint ChannelCount =>
        Math.Max(1, DialectRegistry.Resolve(
            InstrumentKind.DcPowerSupply,
            _session.Identity.Manufacturer,
            _session.Identity.Model).Channels);

    public void SetVoltage(uint channel, double volts) =>
        _session.Scpi.Write(ScpiCommands.PsuSetVoltage(channel, volts));

    public void SetCurrentLimit(uint channel, double amps) =>
        _session.Scpi.Write(ScpiCommands.PsuSetCurrentLimit(channel, amps));

    public void OutputEnable(uint channel, bool enabled) =>
        _session.Scpi.Write(ScpiCommands.PsuOutputEnable(channel, enabled));

    public bool OutputStateQuery(uint channel) =>
        ParseOnOff(_session.Scpi.Query(ScpiCommands.PsuOutputStateQuery(channel)));

    public void OvpLevel(uint channel, double volts) =>
        _session.Scpi.Write(ScpiCommands.PsuOvpLevel(channel, volts));

    public void OvpEnable(uint channel, bool enabled) =>
        _session.Scpi.Write(ScpiCommands.PsuOvpEnable(channel, enabled ? "ON" : "OFF"));

    public bool OvpQuery(uint channel) =>
        ParseOnOff(_session.Scpi.Query(ScpiCommands.PsuOvpQuery(channel)));

    public void SenseEnable(uint channel, bool enabled) =>
        _session.Scpi.Write(ScpiCommands.PsuSenseEnable(channel, enabled ? "ON" : "OFF"));

    public double ReadVoltage(uint channel) =>
        ScpiSession.ParseF64(_session.Scpi.Query(ScpiCommands.PsuReadVoltage(channel)));

    public double ReadCurrent(uint channel) =>
        ScpiSession.ParseF64(_session.Scpi.Query(ScpiCommands.PsuReadCurrent(channel)));

    internal static bool ParseOnOff(string response)
    {
        var trimmed = response.Trim().ToUpperInvariant();
        return trimmed switch
        {
            "1" or "ON" => true,
            "0" or "OFF" => false,
            _ => throw new FormatException($"expected ON/OFF state, got '{response}'"),
        };
    }
}
