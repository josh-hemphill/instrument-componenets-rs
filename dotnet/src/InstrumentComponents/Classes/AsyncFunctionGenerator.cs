using InstrumentComponents.Dialects;
using InstrumentComponents.Kind;
using InstrumentComponents.Session;
using InstrumentComponents.Scpi;

namespace InstrumentComponents.Classes;

public sealed class AsyncFunctionGenerator
{
    private readonly AsyncInstrumentSession _session;

    public AsyncFunctionGenerator(AsyncInstrumentSession session) => _session = session;

    public AsyncInstrumentSession Session => _session;

    private DialectProfile Dialect => _session.DialectFor(InstrumentKind.FunctionGenerator);

    private string Cmd(string key, string fallback, params (string Name, string Value)[] vars) =>
        DialectCommand.Try(Dialect, key, fallback, vars);

    public Task SetWaveformAsync(Waveform waveform, CancellationToken cancellationToken = default)
    {
        var name = waveform.ScpiName();
        return _session.Scpi.WriteAsync(Cmd("set_waveform", ScpiCommands.FgenSetWaveform(name), ("scpi_name", name)), cancellationToken);
    }

    public Task SetFrequencyAsync(double hz, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(Cmd("set_frequency", ScpiCommands.FgenSetFrequency(hz), ("hz", ScpiFormat.Double(hz))), cancellationToken);

    public Task SetAmplitudeAsync(double vpp, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(Cmd("set_amplitude", ScpiCommands.FgenSetAmplitude(vpp), ("vpp", ScpiFormat.Double(vpp))), cancellationToken);

    public Task SetOffsetAsync(double volts, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(Cmd("set_offset", ScpiCommands.FgenSetOffset(volts), ("volts", ScpiFormat.Double(volts))), cancellationToken);

    public Task SetDutyCycleAsync(double percent, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(Cmd("set_duty_cycle", ScpiCommands.FgenSetDutyCycle(percent), ("percent", ScpiFormat.Double(percent))), cancellationToken);

    public Task SetLoadAsync(double ohms, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(Cmd("set_load", ScpiCommands.FgenSetLoad(ohms), ("ohms", ScpiFormat.Double(ohms))), cancellationToken);

    public Task OutputEnableAsync(bool enabled, CancellationToken cancellationToken = default)
    {
        var state = enabled ? "ON" : "OFF";
        return _session.Scpi.WriteAsync(Cmd("output_enable", ScpiCommands.FgenOutputEnable(enabled), ("state", state)), cancellationToken);
    }

    public Task SetBurstCountAsync(uint count, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(Cmd("burst_count", ScpiCommands.FgenBurstCount(count), ("count", count.ToString())), cancellationToken);

    public Task SetBurstStateAsync(bool enabled, CancellationToken cancellationToken = default)
    {
        var state = enabled ? "ON" : "OFF";
        return _session.Scpi.WriteAsync(Cmd("burst_state", ScpiCommands.FgenBurstState(state), ("state", state)), cancellationToken);
    }

    public Task SetBurstTriggerSourceAsync(string source, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(Cmd("burst_trigger", ScpiCommands.FgenBurstTrigger(source), ("source", source)), cancellationToken);

    public async Task<double> ReadFrequencyAsync(CancellationToken cancellationToken = default)
    {
        var resp = await _session.Scpi.QueryAsync(Cmd("read_frequency", ScpiCommands.FgenReadFrequency), cancellationToken).ConfigureAwait(false);
        return ScpiSession.ParseF64(resp);
    }
}
