using InstrumentComponents.Dialects;
using InstrumentComponents.Kind;
using InstrumentComponents.Session;
using InstrumentComponents.Scpi;

namespace InstrumentComponents.Classes;

/// <summary>Oscilloscope session view (IVI-inspired / SCPI :TIMebase, :CHANnel, :WAVeform).</summary>
public sealed class Oscilloscope
{
    private readonly InstrumentSession _session;

    public Oscilloscope(InstrumentSession session) => _session = session;

    public InstrumentSession Session => _session;

    private DialectProfile Dialect => _session.DialectFor(InstrumentKind.Oscilloscope);

    private string Cmd(string key, string fallback, params (string Name, string Value)[] vars) =>
        DialectCommand.Try(Dialect, key, fallback, vars);

    public void SetTimebaseScale(double secondsPerDiv) =>
        _session.Scpi.Write(Cmd("set_timebase_scale", ScpiCommands.ScopeSetTimebaseScale(secondsPerDiv),
            ("seconds_per_div", ScpiFormat.Double(secondsPerDiv))));

    public double ReadTimebaseScale() =>
        ScpiSession.ParseF64(_session.Scpi.Query(Cmd("read_timebase_scale", ScpiCommands.ScopeReadTimebaseScale)));

    public void SetChannelScale(uint channel, double voltsPerDiv) =>
        _session.Scpi.Write(Cmd("set_channel_scale", ScpiCommands.ScopeSetChannelScale(channel, voltsPerDiv),
            ("channel", channel.ToString()), ("volts_per_div", ScpiFormat.Double(voltsPerDiv))));

    public void SetChannelDisplay(uint channel, bool enabled)
    {
        var state = enabled ? "ON" : "OFF";
        _session.Scpi.Write(Cmd("channel_display", ScpiCommands.ScopeChannelDisplay(channel, state),
            ("channel", channel.ToString()), ("state", state)));
    }

    public void SetChannelCoupling(uint channel, string coupling) =>
        _session.Scpi.Write(Cmd("channel_coupling", ScpiCommands.ScopeChannelCoupling(channel, coupling),
            ("channel", channel.ToString()), ("coupling", coupling)));

    public void SetTriggerSource(string source) =>
        _session.Scpi.Write(Cmd("trigger_source", ScpiCommands.ScopeTriggerSource(source), ("source", source)));

    public void SetTriggerLevel(double volts) =>
        _session.Scpi.Write(Cmd("trigger_level", ScpiCommands.ScopeTriggerLevel(volts), ("volts", ScpiFormat.Double(volts))));

    public void SetTriggerSlope(string slope) =>
        _session.Scpi.Write(Cmd("trigger_slope", ScpiCommands.ScopeTriggerSlope(slope), ("slope", slope)));

    public void Run() => _session.Scpi.Write(Cmd("run", ScpiCommands.ScopeRun));

    public void Stop() => _session.Scpi.Write(Cmd("stop", ScpiCommands.ScopeStop));

    public void Single() => _session.Scpi.Write(Cmd("single", ScpiCommands.ScopeSingle));

    public double MeasureVpp(uint channel) =>
        ScpiSession.ParseF64(_session.Scpi.Query(Cmd("measure_vpp", ScpiCommands.ScopeMeasureVpp(channel),
            ("channel", channel.ToString()))));

    public double MeasureFrequency(uint channel) =>
        ScpiSession.ParseF64(_session.Scpi.Query(Cmd("measure_frequency", ScpiCommands.ScopeMeasureFrequency(channel),
            ("channel", channel.ToString()))));

    public VoltageTrace CaptureVoltageTrace(uint channel)
    {
        var source = Cmd("waveform_source", ScpiCommands.ScopeSetWaveformSource(channel), ("channel", channel.ToString()));
        var format = Cmd("waveform_format_ascii", ScpiCommands.ScopeWaveformFormatAscii);
        var preambleCmd = Cmd("waveform_preamble", ScpiCommands.ScopeWaveformPreamble);
        var dataCmd = Cmd("waveform_data", ScpiCommands.ScopeWaveformData);
        var scpi = _session.Scpi;
        scpi.Write(source);
        scpi.Write(format);
        double sampleIntervalS;
        try
        {
            var preamble = scpi.Query(preambleCmd);
            sampleIntervalS = ParsePreambleXIncrement(preamble) ?? 0.0;
        }
        catch
        {
            sampleIntervalS = 0.0;
        }

        var data = scpi.Query(dataCmd);
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
