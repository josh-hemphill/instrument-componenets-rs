using InstrumentComponents.Session;
using InstrumentComponents.Scpi;

namespace InstrumentComponents.Classes;

public sealed class AsyncFunctionGenerator
{
    private readonly AsyncInstrumentSession _session;

    public AsyncFunctionGenerator(AsyncInstrumentSession session) => _session = session;

    public AsyncInstrumentSession Session => _session;

    public Task SetWaveformAsync(Waveform waveform, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(ScpiCommands.FgenSetWaveform(waveform.ScpiName()), cancellationToken);

    public Task SetFrequencyAsync(double hz, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(ScpiCommands.FgenSetFrequency(hz), cancellationToken);

    public Task SetAmplitudeAsync(double vpp, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(ScpiCommands.FgenSetAmplitude(vpp), cancellationToken);

    public Task SetOffsetAsync(double volts, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(ScpiCommands.FgenSetOffset(volts), cancellationToken);

    public Task SetDutyCycleAsync(double percent, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(ScpiCommands.FgenSetDutyCycle(percent), cancellationToken);

    public Task SetLoadAsync(double ohms, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(ScpiCommands.FgenSetLoad(ohms), cancellationToken);

    public Task OutputEnableAsync(bool enabled, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(ScpiCommands.FgenOutputEnable(enabled), cancellationToken);

    public Task SetBurstCountAsync(uint count, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(ScpiCommands.FgenBurstCount(count), cancellationToken);

    public Task SetBurstStateAsync(bool enabled, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(ScpiCommands.FgenBurstState(enabled ? "ON" : "OFF"), cancellationToken);

    public Task SetBurstTriggerSourceAsync(string source, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(ScpiCommands.FgenBurstTrigger(source), cancellationToken);

    public async Task<double> ReadFrequencyAsync(CancellationToken cancellationToken = default)
    {
        var resp = await _session.Scpi.QueryAsync(ScpiCommands.FgenReadFrequency, cancellationToken).ConfigureAwait(false);
        return ScpiSession.ParseF64(resp);
    }
}
