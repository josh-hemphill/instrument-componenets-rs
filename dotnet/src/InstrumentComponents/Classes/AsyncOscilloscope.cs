using InstrumentComponents.Dialects;
using InstrumentComponents.Kind;
using InstrumentComponents.Session;
using InstrumentComponents.Scpi;

namespace InstrumentComponents.Classes;

public sealed class AsyncOscilloscope
{
    private readonly AsyncInstrumentSession _session;

    public AsyncOscilloscope(AsyncInstrumentSession session) => _session = session;

    public AsyncInstrumentSession Session => _session;

    private DialectProfile Dialect => _session.DialectFor(InstrumentKind.Oscilloscope);

    private string Cmd(string key, string fallback, params (string Name, string Value)[] vars) =>
        DialectCommand.Try(Dialect, key, fallback, vars);

    public Task SetTimebaseScaleAsync(double secondsPerDiv, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(Cmd("set_timebase_scale", ScpiCommands.ScopeSetTimebaseScale(secondsPerDiv),
            ("seconds_per_div", ScpiFormat.Double(secondsPerDiv))), cancellationToken);

    public async Task<double> ReadTimebaseScaleAsync(CancellationToken cancellationToken = default)
    {
        var resp = await _session.Scpi.QueryAsync(Cmd("read_timebase_scale", ScpiCommands.ScopeReadTimebaseScale), cancellationToken).ConfigureAwait(false);
        return ScpiSession.ParseF64(resp);
    }

    public Task SetChannelScaleAsync(uint channel, double voltsPerDiv, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(Cmd("set_channel_scale", ScpiCommands.ScopeSetChannelScale(channel, voltsPerDiv),
            ("channel", channel.ToString()), ("volts_per_div", ScpiFormat.Double(voltsPerDiv))), cancellationToken);

    public Task SetChannelDisplayAsync(uint channel, bool enabled, CancellationToken cancellationToken = default)
    {
        var state = enabled ? "ON" : "OFF";
        return _session.Scpi.WriteAsync(Cmd("channel_display", ScpiCommands.ScopeChannelDisplay(channel, state),
            ("channel", channel.ToString()), ("state", state)), cancellationToken);
    }

    public Task SetChannelCouplingAsync(uint channel, string coupling, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(Cmd("channel_coupling", ScpiCommands.ScopeChannelCoupling(channel, coupling),
            ("channel", channel.ToString()), ("coupling", coupling)), cancellationToken);

    public Task SetTriggerSourceAsync(string source, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(Cmd("trigger_source", ScpiCommands.ScopeTriggerSource(source), ("source", source)), cancellationToken);

    public Task SetTriggerLevelAsync(double volts, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(Cmd("trigger_level", ScpiCommands.ScopeTriggerLevel(volts), ("volts", ScpiFormat.Double(volts))), cancellationToken);

    public Task SetTriggerSlopeAsync(string slope, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(Cmd("trigger_slope", ScpiCommands.ScopeTriggerSlope(slope), ("slope", slope)), cancellationToken);

    public Task RunAsync(CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(Cmd("run", ScpiCommands.ScopeRun), cancellationToken);

    public Task StopAsync(CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(Cmd("stop", ScpiCommands.ScopeStop), cancellationToken);

    public Task SingleAsync(CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(Cmd("single", ScpiCommands.ScopeSingle), cancellationToken);

    public async Task<double> MeasureVppAsync(uint channel, CancellationToken cancellationToken = default)
    {
        var resp = await _session.Scpi.QueryAsync(Cmd("measure_vpp", ScpiCommands.ScopeMeasureVpp(channel),
            ("channel", channel.ToString())), cancellationToken).ConfigureAwait(false);
        return ScpiSession.ParseF64(resp);
    }

    public async Task<double> MeasureFrequencyAsync(uint channel, CancellationToken cancellationToken = default)
    {
        var resp = await _session.Scpi.QueryAsync(Cmd("measure_frequency", ScpiCommands.ScopeMeasureFrequency(channel),
            ("channel", channel.ToString())), cancellationToken).ConfigureAwait(false);
        return ScpiSession.ParseF64(resp);
    }

    public async Task<VoltageTrace> CaptureVoltageTraceAsync(uint channel, CancellationToken cancellationToken = default)
    {
        var source = Cmd("waveform_source", ScpiCommands.ScopeSetWaveformSource(channel), ("channel", channel.ToString()));
        var format = Cmd("waveform_format_ascii", ScpiCommands.ScopeWaveformFormatAscii);
        var preambleCmd = Cmd("waveform_preamble", ScpiCommands.ScopeWaveformPreamble);
        var dataCmd = Cmd("waveform_data", ScpiCommands.ScopeWaveformData);
        var scpi = _session.Scpi;
        await scpi.WriteAsync(source, cancellationToken).ConfigureAwait(false);
        await scpi.WriteAsync(format, cancellationToken).ConfigureAwait(false);

        double sampleIntervalS;
        try
        {
            var preamble = await scpi.QueryAsync(preambleCmd, cancellationToken).ConfigureAwait(false);
            sampleIntervalS = Oscilloscope.ParsePreambleXIncrement(preamble) ?? 0.0;
        }
        catch
        {
            sampleIntervalS = 0.0;
        }

        var data = await scpi.QueryAsync(dataCmd, cancellationToken).ConfigureAwait(false);
        return new VoltageTrace(ScpiSession.ParseF64Csv(data), sampleIntervalS);
    }
}
