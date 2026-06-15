using InstrumentComponents.Session;
using InstrumentComponents.Scpi;

namespace InstrumentComponents.Classes;

public sealed class AsyncDcPowerSupply
{
    private readonly AsyncInstrumentSession _session;

    public AsyncDcPowerSupply(AsyncInstrumentSession session) => _session = session;

    public Task SetVoltageAsync(uint channel, double volts, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(ScpiCommands.PsuSetVoltage(channel, volts), cancellationToken);

    public Task SetCurrentLimitAsync(uint channel, double amps, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(ScpiCommands.PsuSetCurrentLimit(channel, amps), cancellationToken);

    public Task OutputEnableAsync(uint channel, bool enabled, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(ScpiCommands.PsuOutputEnable(channel, enabled), cancellationToken);

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
