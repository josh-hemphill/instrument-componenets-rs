using InstrumentComponents.Dialects;
using InstrumentComponents.Kind;
using InstrumentComponents.Session;
using InstrumentComponents.Scpi;

namespace InstrumentComponents.Classes;

public sealed class AsyncDcPowerSupply
{
    private readonly AsyncInstrumentSession _session;

    public AsyncDcPowerSupply(AsyncInstrumentSession session) => _session = session;

    public AsyncInstrumentSession Session => _session;

    public uint ChannelCount =>
        Math.Max(1, DialectRegistry.Resolve(
            InstrumentKind.DcPowerSupply,
            _session.Identity.Manufacturer,
            _session.Identity.Model).Channels);

    public Task SetVoltageAsync(uint channel, double volts, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(ScpiCommands.PsuSetVoltage(channel, volts), cancellationToken);

    public Task SetCurrentLimitAsync(uint channel, double amps, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(ScpiCommands.PsuSetCurrentLimit(channel, amps), cancellationToken);

    public Task OutputEnableAsync(uint channel, bool enabled, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(ScpiCommands.PsuOutputEnable(channel, enabled), cancellationToken);

    public async Task<bool> OutputStateQueryAsync(uint channel, CancellationToken cancellationToken = default)
    {
        var resp = await _session.Scpi.QueryAsync(ScpiCommands.PsuOutputStateQuery(channel), cancellationToken).ConfigureAwait(false);
        return DcPowerSupply.ParseOnOff(resp);
    }

    public Task OvpLevelAsync(uint channel, double volts, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(ScpiCommands.PsuOvpLevel(channel, volts), cancellationToken);

    public Task OvpEnableAsync(uint channel, bool enabled, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(ScpiCommands.PsuOvpEnable(channel, enabled ? "ON" : "OFF"), cancellationToken);

    public async Task<bool> OvpQueryAsync(uint channel, CancellationToken cancellationToken = default)
    {
        var resp = await _session.Scpi.QueryAsync(ScpiCommands.PsuOvpQuery(channel), cancellationToken).ConfigureAwait(false);
        return DcPowerSupply.ParseOnOff(resp);
    }

    public Task SenseEnableAsync(uint channel, bool enabled, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(ScpiCommands.PsuSenseEnable(channel, enabled ? "ON" : "OFF"), cancellationToken);

    public async Task<double> ReadVoltageAsync(uint channel, CancellationToken cancellationToken = default)
    {
        var resp = await _session.Scpi.QueryAsync(ScpiCommands.PsuReadVoltage(channel), cancellationToken).ConfigureAwait(false);
        return ScpiSession.ParseF64(resp);
    }

    public async Task<double> ReadCurrentAsync(uint channel, CancellationToken cancellationToken = default)
    {
        var resp = await _session.Scpi.QueryAsync(ScpiCommands.PsuReadCurrent(channel), cancellationToken).ConfigureAwait(false);
        return ScpiSession.ParseF64(resp);
    }
}
