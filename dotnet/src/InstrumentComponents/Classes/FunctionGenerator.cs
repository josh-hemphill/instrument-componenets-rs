using InstrumentComponents.Dialects;
using InstrumentComponents.Kind;
using InstrumentComponents.Session;
using InstrumentComponents.Scpi;

namespace InstrumentComponents.Classes;

public enum Waveform
{
    Sine,
    Square,
    Ramp,
    Pulse,
    Noise,
    Dc,
}

public static class WaveformExtensions
{
    public static string ScpiName(this Waveform waveform) => waveform switch
    {
        Waveform.Sine => "SIN",
        Waveform.Square => "SQU",
        Waveform.Ramp => "RAMP",
        Waveform.Pulse => "PULS",
        Waveform.Noise => "NOIS",
        Waveform.Dc => "DC",
        _ => "SIN",
    };
}

/// <summary>Function / arbitrary waveform generator session view.</summary>
public sealed class FunctionGenerator
{
    private readonly InstrumentSession _session;

    public FunctionGenerator(InstrumentSession session) => _session = session;

    public InstrumentSession Session => _session;

    private DialectProfile Dialect => _session.DialectFor(InstrumentKind.FunctionGenerator);

    private string Cmd(string key, string fallback, params (string Name, string Value)[] vars) =>
        DialectCommand.Try(Dialect, key, fallback, vars);

    public void SetWaveform(Waveform waveform)
    {
        var name = waveform.ScpiName();
        _session.Scpi.Write(Cmd("set_waveform", ScpiCommands.FgenSetWaveform(name), ("scpi_name", name)));
    }

    public void SetFrequency(double hz) =>
        _session.Scpi.Write(Cmd("set_frequency", ScpiCommands.FgenSetFrequency(hz), ("hz", ScpiFormat.Double(hz))));

    public void SetAmplitude(double vpp) =>
        _session.Scpi.Write(Cmd("set_amplitude", ScpiCommands.FgenSetAmplitude(vpp), ("vpp", ScpiFormat.Double(vpp))));

    public void SetOffset(double volts) =>
        _session.Scpi.Write(Cmd("set_offset", ScpiCommands.FgenSetOffset(volts), ("volts", ScpiFormat.Double(volts))));

    public void SetDutyCycle(double percent) =>
        _session.Scpi.Write(Cmd("set_duty_cycle", ScpiCommands.FgenSetDutyCycle(percent), ("percent", ScpiFormat.Double(percent))));

    public void SetLoad(double ohms) =>
        _session.Scpi.Write(Cmd("set_load", ScpiCommands.FgenSetLoad(ohms), ("ohms", ScpiFormat.Double(ohms))));

    public void OutputEnable(bool enabled)
    {
        var state = enabled ? "ON" : "OFF";
        _session.Scpi.Write(Cmd("output_enable", ScpiCommands.FgenOutputEnable(enabled), ("state", state)));
    }

    public void SetBurstCount(uint count) =>
        _session.Scpi.Write(Cmd("burst_count", ScpiCommands.FgenBurstCount(count), ("count", count.ToString())));

    public void SetBurstState(bool enabled)
    {
        var state = enabled ? "ON" : "OFF";
        _session.Scpi.Write(Cmd("burst_state", ScpiCommands.FgenBurstState(state), ("state", state)));
    }

    public void SetBurstTriggerSource(string source) =>
        _session.Scpi.Write(Cmd("burst_trigger", ScpiCommands.FgenBurstTrigger(source), ("source", source)));

    public double ReadFrequency() =>
        ScpiSession.ParseF64(_session.Scpi.Query(Cmd("read_frequency", ScpiCommands.FgenReadFrequency)));
}
