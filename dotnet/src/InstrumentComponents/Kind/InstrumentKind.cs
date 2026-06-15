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
        InstrumentKind.Unknown => "Unknown",
        _ => "Unknown",
    };
}
