using InstrumentComponents.Session;
using InstrumentComponents.Scpi;

namespace InstrumentComponents.Classes;

/// <summary>Oscilloscope session view (IVI-inspired / SCPI :TIMebase, :CHANnel, :WAVeform).</summary>
public sealed class Oscilloscope
{
    private readonly InstrumentSession _session;

    public Oscilloscope(InstrumentSession session) => _session = session;

    public InstrumentSession Session => _session;

    public void SetTimebaseScale(double secondsPerDiv) =>
        _session.Scpi.Write(ScpiCommands.ScopeSetTimebaseScale(secondsPerDiv));

    public double ReadTimebaseScale() =>
        ScpiSession.ParseF64(_session.Scpi.Query(ScpiCommands.ScopeReadTimebaseScale));

    public void SetChannelScale(uint channel, double voltsPerDiv) =>
        _session.Scpi.Write(ScpiCommands.ScopeSetChannelScale(channel, voltsPerDiv));

    public void SetChannelDisplay(uint channel, bool enabled) =>
        _session.Scpi.Write(ScpiCommands.ScopeChannelDisplay(channel, enabled ? "ON" : "OFF"));

    public void SetChannelCoupling(uint channel, string coupling) =>
        _session.Scpi.Write(ScpiCommands.ScopeChannelCoupling(channel, coupling));

    public void SetTriggerSource(string source) =>
        _session.Scpi.Write(ScpiCommands.ScopeTriggerSource(source));

    public void SetTriggerLevel(double volts) =>
        _session.Scpi.Write(ScpiCommands.ScopeTriggerLevel(volts));

    public void SetTriggerSlope(string slope) =>
        _session.Scpi.Write(ScpiCommands.ScopeTriggerSlope(slope));

    public void Run() => _session.Scpi.Write(ScpiCommands.ScopeRun);

    public void Stop() => _session.Scpi.Write(ScpiCommands.ScopeStop);

    public void Single() => _session.Scpi.Write(ScpiCommands.ScopeSingle);

    public double MeasureVpp(uint channel) =>
        ScpiSession.ParseF64(_session.Scpi.Query(ScpiCommands.ScopeMeasureVpp(channel)));

    public double MeasureFrequency(uint channel) =>
        ScpiSession.ParseF64(_session.Scpi.Query(ScpiCommands.ScopeMeasureFrequency(channel)));

    public VoltageTrace CaptureVoltageTrace(uint channel)
    {
        var scpi = _session.Scpi;
        scpi.Write(ScpiCommands.ScopeSetWaveformSource(channel));
        scpi.Write(ScpiCommands.ScopeWaveformFormatAscii);
        double sampleIntervalS;
        try
        {
            var preamble = scpi.Query(ScpiCommands.ScopeWaveformPreamble);
            sampleIntervalS = ParsePreambleXIncrement(preamble) ?? 0.0;
        }
        catch
        {
            sampleIntervalS = 0.0;
        }

        var data = scpi.Query(ScpiCommands.ScopeWaveformData);
        return new VoltageTrace(ScpiSession.ParseF64Csv(data), sampleIntervalS);
    }

    internal static double? ParsePreambleXIncrement(string preamble)
    {
        var fields = preamble.Split(',');
        if (fields.Length < 5)
            return null;
        return double.TryParse(
            fields[4].Trim(),
            System.Globalization.NumberStyles.Float,
            System.Globalization.CultureInfo.InvariantCulture,
            out var value)
            ? value
            : null;
    }
}
