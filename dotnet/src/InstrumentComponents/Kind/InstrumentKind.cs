namespace InstrumentComponents.Kind;

/// <summary>Instrument functional class (IVI-inspired).</summary>
public enum InstrumentKind
{
    Dmm,
    DcPowerSupply,
    FunctionGenerator,
    Oscilloscope,
    Switch,
    Counter,
    PowerMeter,
    SpectrumAnalyzer,
    Unknown,
}

public static class InstrumentKindExtensions
{
    public static InstrumentKind? FromLabel(string label) => label switch
    {
        "Dmm" => InstrumentKind.Dmm,
        "DcPowerSupply" => InstrumentKind.DcPowerSupply,
        "FunctionGenerator" => InstrumentKind.FunctionGenerator,
        "Oscilloscope" => InstrumentKind.Oscilloscope,
        "Switch" => InstrumentKind.Switch,
        "Counter" => InstrumentKind.Counter,
        "PowerMeter" => InstrumentKind.PowerMeter,
        "SpectrumAnalyzer" => InstrumentKind.SpectrumAnalyzer,
        "Unknown" => InstrumentKind.Unknown,
        _ => null,
    };

    public static string ToLabel(this InstrumentKind kind) => kind switch
    {
        InstrumentKind.Dmm => "Dmm",
        InstrumentKind.DcPowerSupply => "DcPowerSupply",
        InstrumentKind.FunctionGenerator => "FunctionGenerator",
        InstrumentKind.Oscilloscope => "Oscilloscope",
        InstrumentKind.Switch => "Switch",
        InstrumentKind.Counter => "Counter",
        InstrumentKind.PowerMeter => "PowerMeter",
        InstrumentKind.SpectrumAnalyzer => "SpectrumAnalyzer",
        InstrumentKind.Unknown => "Unknown",
        _ => "Unknown",
    };
}
