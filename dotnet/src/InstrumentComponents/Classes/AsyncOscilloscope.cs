using InstrumentComponents.Session;
using InstrumentComponents.Scpi;

namespace InstrumentComponents.Classes;

public sealed class AsyncOscilloscope
{
    private readonly AsyncInstrumentSession _session;

    public AsyncOscilloscope(AsyncInstrumentSession session) => _session = session;

    public AsyncInstrumentSession Session => _session;

    public Task SetTimebaseScaleAsync(double secondsPerDiv, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(ScpiCommands.ScopeSetTimebaseScale(secondsPerDiv), cancellationToken);

    public async Task<double> ReadTimebaseScaleAsync(CancellationToken cancellationToken = default)
    {
        var resp = await _session.Scpi.QueryAsync(ScpiCommands.ScopeReadTimebaseScale, cancellationToken).ConfigureAwait(false);
        return ScpiSession.ParseF64(resp);
    }

    public Task SetChannelScaleAsync(uint channel, double voltsPerDiv, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(ScpiCommands.ScopeSetChannelScale(channel, voltsPerDiv), cancellationToken);

    public Task SetChannelDisplayAsync(uint channel, bool enabled, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(ScpiCommands.ScopeChannelDisplay(channel, enabled ? "ON" : "OFF"), cancellationToken);

    public Task SetChannelCouplingAsync(uint channel, string coupling, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(ScpiCommands.ScopeChannelCoupling(channel, coupling), cancellationToken);

    public Task SetTriggerSourceAsync(string source, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(ScpiCommands.ScopeTriggerSource(source), cancellationToken);

    public Task SetTriggerLevelAsync(double volts, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(ScpiCommands.ScopeTriggerLevel(volts), cancellationToken);

    public Task SetTriggerSlopeAsync(string slope, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(ScpiCommands.ScopeTriggerSlope(slope), cancellationToken);

    public Task RunAsync(CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(ScpiCommands.ScopeRun, cancellationToken);

    public Task StopAsync(CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(ScpiCommands.ScopeStop, cancellationToken);

    public Task SingleAsync(CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(ScpiCommands.ScopeSingle, cancellationToken);

    public async Task<double> MeasureVppAsync(uint channel, CancellationToken cancellationToken = default)
    {
        var resp = await _session.Scpi.QueryAsync(ScpiCommands.ScopeMeasureVpp(channel), cancellationToken).ConfigureAwait(false);
        return ScpiSession.ParseF64(resp);
    }

    public async Task<double> MeasureFrequencyAsync(uint channel, CancellationToken cancellationToken = default)
    {
        var resp = await _session.Scpi.QueryAsync(ScpiCommands.ScopeMeasureFrequency(channel), cancellationToken).ConfigureAwait(false);
        return ScpiSession.ParseF64(resp);
    }

    public async Task<VoltageTrace> CaptureVoltageTraceAsync(uint channel, CancellationToken cancellationToken = default)
    {
        var scpi = _session.Scpi;
        await scpi.WriteAsync(ScpiCommands.ScopeSetWaveformSource(channel), cancellationToken).ConfigureAwait(false);
        await scpi.WriteAsync(ScpiCommands.ScopeWaveformFormatAscii, cancellationToken).ConfigureAwait(false);

        double sampleIntervalS;
        try
        {
            var preamble = await scpi.QueryAsync(ScpiCommands.ScopeWaveformPreamble, cancellationToken).ConfigureAwait(false);
            sampleIntervalS = Oscilloscope.ParsePreambleXIncrement(preamble) ?? 0.0;
        }
        catch
        {
            sampleIntervalS = 0.0;
        }

        var data = await scpi.QueryAsync(ScpiCommands.ScopeWaveformData, cancellationToken).ConfigureAwait(false);
        return new VoltageTrace(ScpiSession.ParseF64Csv(data), sampleIntervalS);
    }
}
