using OpenTap;

namespace InstrumentComponents.OpenTap;

[Display("PSU Configure Output", Groups: ["Instrument Components", "PSU"], Description: "Set voltage, current limit, and output enable.")]
public sealed class PsuConfigureOutputStep : TestStep
{
    [Display("Instrument", Order: 1)]
    public DcPowerSupplyInstrument Instrument { get; set; } = null!;

    [Display("Channel", Order: 2)]
    public uint Channel { get; set; } = 1;

    [Display("Voltage", Order: 3)]
    public double Voltage { get; set; }

    [Display("Current limit", Order: 4)]
    public double CurrentLimit { get; set; }

    [Display("Output enabled", Order: 5)]
    public bool OutputEnabled { get; set; } = true;

    public override void Run()
    {
        if (Instrument is null)
        {
            UpgradeVerdict(Verdict.Error);
            return;
        }

        var psu = Instrument.Supply;
        psu.SetVoltage(Channel, Voltage);
        psu.SetCurrentLimit(Channel, CurrentLimit);
        psu.OutputEnable(Channel, OutputEnabled);
        PhaseIResults.PublishScalar(Results, "Voltage", Voltage, "V");
        PhaseIResults.PublishScalar(Results, "CurrentLimit", CurrentLimit, "A");
        UpgradeVerdict(Verdict.Pass);
    }
}

[Display("PSU Readback", Groups: ["Instrument Components", "PSU"], Description: "Read voltage and current.")]
public sealed class PsuReadbackStep : TestStep
{
    [Display("Instrument", Order: 1)]
    public DcPowerSupplyInstrument Instrument { get; set; } = null!;

    [Display("Channel", Order: 2)]
    public uint Channel { get; set; } = 1;

    public override void Run()
    {
        if (Instrument is null)
        {
            UpgradeVerdict(Verdict.Error);
            return;
        }

        var psu = Instrument.Supply;
        var volts = psu.ReadVoltage(Channel);
        var amps = psu.ReadCurrent(Channel);
        PhaseIResults.PublishSample(Results, $"CH{Channel}.V", 0, volts);
        PhaseIResults.PublishSample(Results, $"CH{Channel}.I", 0, amps);
        PhaseIResults.PublishScalar(Results, $"CH{Channel}.V", volts, "V");
        PhaseIResults.PublishScalar(Results, $"CH{Channel}.I", amps, "A");
        UpgradeVerdict(Verdict.Pass);
    }
}
