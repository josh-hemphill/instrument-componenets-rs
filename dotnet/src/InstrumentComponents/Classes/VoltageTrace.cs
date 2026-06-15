namespace InstrumentComponents.Classes;

/// <summary>Captured voltage waveform samples in SI units.</summary>
public sealed record VoltageTrace(IReadOnlyList<double> Samples, double SampleIntervalS);
