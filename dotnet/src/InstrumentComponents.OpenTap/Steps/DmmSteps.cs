using OpenTap;

namespace InstrumentComponents.OpenTap;

[Display("DMM Measure Voltage DC", Groups: ["Instrument Components", "DMM"], Description: "Acquire VDC samples.")]
public sealed class DmmMeasureVoltageDcStep : TestStep
{
    [Display("Instrument", Order: 1)]
    public DmmInstrument Instrument { get; set; } = null!;

    [Display("Channel", Order: 2)]
    public string Channel { get; set; } = "VDC";

    [Display("Sample Count", Order: 3)]
    public int SampleCount { get; set; } = 1;

    [Display("Interval Ms", Order: 4)]
    public int IntervalMs { get; set; }

    public override void Run()
    {
        if (Instrument is null)
        {
            UpgradeVerdict(Verdict.Error);
            return;
        }

        var count = Math.Max(1, SampleCount);
        for (var i = 0; i < count; i++)
        {
            TapThread.ThrowIfAborted();
            var value = Instrument.Dmm.MeasureVoltageDc();
            PhaseIResults.PublishSample(Results, Channel, i, value);
            if (IntervalMs > 0 && i < count - 1)
                TapThread.Sleep(IntervalMs);
        }

        UpgradeVerdict(Verdict.Pass);
    }
}

[Display("DMM Measure Scalar", Groups: ["Instrument Components", "DMM"], Description: "One VDC reading with optional limits.")]
public sealed class DmmMeasureScalarStep : TestStep
{
    [Display("Instrument", Order: 1)]
    public DmmInstrument Instrument { get; set; } = null!;

    [Display("Name", Order: 2)]
    public string MetricName { get; set; } = "VDC";

    [Display("Unit", Order: 3)]
    public string Unit { get; set; } = "V";

    [Display("Limit low", Order: 4)]
    public double? LimitLow { get; set; }

    [Display("Limit high", Order: 5)]
    public double? LimitHigh { get; set; }

    public override void Run()
    {
        if (Instrument is null)
        {
            UpgradeVerdict(Verdict.Error);
            return;
        }

        var value = Instrument.Dmm.MeasureVoltageDc();
        PhaseIResults.PublishScalar(Results, MetricName, value, Unit, LimitLow, LimitHigh);
        if (PhaseIResults.IsOutOfBand(value, LimitLow, LimitHigh))
        {
            Log.Error("{0}={1} outside limits", MetricName, value);
            UpgradeVerdict(Verdict.Fail);
            return;
        }

        UpgradeVerdict(Verdict.Pass);
    }
}
