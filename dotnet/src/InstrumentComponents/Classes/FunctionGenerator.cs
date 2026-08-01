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

    public void SetWaveform(Waveform waveform) =>
        _session.Scpi.Write(ScpiCommands.FgenSetWaveform(waveform.ScpiName()));

    public void SetFrequency(double hz) =>
        _session.Scpi.Write(ScpiCommands.FgenSetFrequency(hz));

    public void SetAmplitude(double vpp) =>
        _session.Scpi.Write(ScpiCommands.FgenSetAmplitude(vpp));

    public void SetOffset(double volts) =>
        _session.Scpi.Write(ScpiCommands.FgenSetOffset(volts));

    public void SetDutyCycle(double percent) =>
        _session.Scpi.Write(ScpiCommands.FgenSetDutyCycle(percent));

    public void SetLoad(double ohms) =>
        _session.Scpi.Write(ScpiCommands.FgenSetLoad(ohms));

    public void OutputEnable(bool enabled) =>
        _session.Scpi.Write(ScpiCommands.FgenOutputEnable(enabled));

    public void SetBurstCount(uint count) =>
        _session.Scpi.Write(ScpiCommands.FgenBurstCount(count));

    public void SetBurstState(bool enabled) =>
        _session.Scpi.Write(ScpiCommands.FgenBurstState(enabled ? "ON" : "OFF"));

    public void SetBurstTriggerSource(string source) =>
        _session.Scpi.Write(ScpiCommands.FgenBurstTrigger(source));

    public double ReadFrequency() =>
        ScpiSession.ParseF64(_session.Scpi.Query(ScpiCommands.FgenReadFrequency));
}
