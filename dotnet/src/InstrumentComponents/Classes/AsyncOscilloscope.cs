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

    public Task RunAsync(CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(ScpiCommands.ScopeRun, cancellationToken);

    public Task StopAsync(CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(ScpiCommands.ScopeStop, cancellationToken);

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
