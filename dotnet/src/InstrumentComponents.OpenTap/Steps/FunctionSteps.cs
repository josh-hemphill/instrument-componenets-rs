using InstrumentComponents.Classes;
using OpenTap;

namespace InstrumentComponents.OpenTap;

[Display("FGen Configure Output", Groups: ["Instrument Components", "FGen"], Description: "Set waveform and output.")]
public sealed class FgenConfigureOutputStep : TestStep
{
    [Display("Instrument", Order: 1)]
    public FunctionGeneratorInstrument Instrument { get; set; } = null!;

    [Display("Waveform", Order: 2)]
    public Waveform Waveform { get; set; } = Waveform.Sine;

    [Display("Frequency (Hz)", Order: 3)]
    public double FrequencyHz { get; set; } = 1000;

    [Display("Amplitude (Vpp)", Order: 4)]
    public double AmplitudeVpp { get; set; } = 1;

    [Display("Offset (V)", Order: 5)]
    public double OffsetVolts { get; set; }

    [Display("Output enabled", Order: 6)]
    public bool OutputEnabled { get; set; } = true;

    public override void Run()
    {
        if (Instrument is null)
        {
            UpgradeVerdict(Verdict.Error);
            return;
        }

        var fgen = Instrument.Generator;
        fgen.SetWaveform(Waveform);
        fgen.SetFrequency(FrequencyHz);
        fgen.SetAmplitude(AmplitudeVpp);
        fgen.SetOffset(OffsetVolts);
        fgen.OutputEnable(OutputEnabled);
        PhaseIResults.PublishScalar(Results, "Frequency", FrequencyHz, "Hz");
        PhaseIResults.PublishScalar(Results, "Amplitude", AmplitudeVpp, "V");
        UpgradeVerdict(Verdict.Pass);
    }
}

[Display("Scope Measure Vpp", Groups: ["Instrument Components", "Scope"], Description: "Peak-to-peak voltage.")]
public sealed class ScopeMeasureVppStep : TestStep
{
    [Display("Instrument", Order: 1)]
    public OscilloscopeInstrument Instrument { get; set; } = null!;

    [Display("Channel", Order: 2)]
    public uint Channel { get; set; } = 1;

    public override void Run()
    {
        if (Instrument is null)
        {
            UpgradeVerdict(Verdict.Error);
            return;
        }

        var value = Instrument.Scope.MeasureVpp(Channel);
        PhaseIResults.PublishScalar(Results, $"CH{Channel}.Vpp", value, "V");
        UpgradeVerdict(Verdict.Pass);
    }
}

[Display("Scope Measure Frequency", Groups: ["Instrument Components", "Scope"], Description: "Frequency measurement.")]
public sealed class ScopeMeasureFrequencyStep : TestStep
{
    [Display("Instrument", Order: 1)]
    public OscilloscopeInstrument Instrument { get; set; } = null!;

    [Display("Channel", Order: 2)]
    public uint Channel { get; set; } = 1;

    public override void Run()
    {
        if (Instrument is null)
        {
            UpgradeVerdict(Verdict.Error);
            return;
        }

        var value = Instrument.Scope.MeasureFrequency(Channel);
        PhaseIResults.PublishScalar(Results, $"CH{Channel}.Freq", value, "Hz");
        UpgradeVerdict(Verdict.Pass);
    }
}

[Display("Scope Capture Trace", Groups: ["Instrument Components", "Scope"], Description: "ASCII voltage trace as Sample rows.")]
public sealed class ScopeCaptureTraceStep : TestStep
{
    [Display("Instrument", Order: 1)]
    public OscilloscopeInstrument Instrument { get; set; } = null!;

    [Display("Channel", Order: 2)]
    public uint Channel { get; set; } = 1;

    public override void Run()
    {
        if (Instrument is null)
        {
            UpgradeVerdict(Verdict.Error);
            return;
        }

        var trace = Instrument.Scope.CaptureVoltageTrace(Channel);
        for (var i = 0; i < trace.Samples.Count; i++)
            PhaseIResults.PublishSample(Results, $"CH{Channel}", i, trace.Samples[i]);
        UpgradeVerdict(Verdict.Pass);
    }
}

[Display("Switch Close Route", Groups: ["Instrument Components", "Switch"], Description: "Close a matrix route.")]
public sealed class SwitchCloseRouteStep : TestStep
{
    [Display("Instrument", Order: 1)]
    public SwitchInstrument Instrument { get; set; } = null!;

    [Display("Channel 1", Order: 2)]
    public uint Channel1 { get; set; } = 1;

    [Display("Channel 2", Order: 3)]
    public uint Channel2 { get; set; } = 2;

    public override void Run()
    {
        if (Instrument is null)
        {
            UpgradeVerdict(Verdict.Error);
            return;
        }

        Instrument.Matrix.CloseRoute(Channel1, Channel2);
        UpgradeVerdict(Verdict.Pass);
    }
}

[Display("Switch Open Route", Groups: ["Instrument Components", "Switch"], Description: "Open a matrix route.")]
public sealed class SwitchOpenRouteStep : TestStep
{
    [Display("Instrument", Order: 1)]
    public SwitchInstrument Instrument { get; set; } = null!;

    [Display("Channel 1", Order: 2)]
    public uint Channel1 { get; set; } = 1;

    [Display("Channel 2", Order: 3)]
    public uint Channel2 { get; set; } = 2;

    public override void Run()
    {
        if (Instrument is null)
        {
            UpgradeVerdict(Verdict.Error);
            return;
        }

        Instrument.Matrix.OpenRoute(Channel1, Channel2);
        UpgradeVerdict(Verdict.Pass);
    }
}

[Display("Switch Open All", Groups: ["Instrument Components", "Switch"], Description: "Open all routes.")]
public sealed class SwitchOpenAllStep : TestStep
{
    [Display("Instrument")]
    public SwitchInstrument Instrument { get; set; } = null!;

    public override void Run()
    {
        if (Instrument is null)
        {
            UpgradeVerdict(Verdict.Error);
            return;
        }

        Instrument.Matrix.OpenAll();
        UpgradeVerdict(Verdict.Pass);
    }
}

[Display("Counter Measure Frequency", Groups: ["Instrument Components", "Counter"], Description: "Measure frequency.")]
public sealed class CounterMeasureFrequencyStep : TestStep
{
    [Display("Instrument")]
    public CounterInstrument Instrument { get; set; } = null!;

    public override void Run()
    {
        if (Instrument is null)
        {
            UpgradeVerdict(Verdict.Error);
            return;
        }

        var value = Instrument.Counter.MeasureFrequency();
        PhaseIResults.PublishScalar(Results, "Frequency", value, "Hz");
        UpgradeVerdict(Verdict.Pass);
    }
}

[Display("Counter Measure Period", Groups: ["Instrument Components", "Counter"], Description: "Measure period.")]
public sealed class CounterMeasurePeriodStep : TestStep
{
    [Display("Instrument")]
    public CounterInstrument Instrument { get; set; } = null!;

    public override void Run()
    {
        if (Instrument is null)
        {
            UpgradeVerdict(Verdict.Error);
            return;
        }

        var value = Instrument.Counter.MeasurePeriod();
        PhaseIResults.PublishScalar(Results, "Period", value, "s");
        UpgradeVerdict(Verdict.Pass);
    }
}

[Display("Power Meter Read", Groups: ["Instrument Components", "PowerMeter"], Description: "Read power.")]
public sealed class PowerMeterReadStep : TestStep
{
    [Display("Instrument")]
    public PowerMeterInstrument Instrument { get; set; } = null!;

    public override void Run()
    {
        if (Instrument is null)
        {
            UpgradeVerdict(Verdict.Error);
            return;
        }

        var value = Instrument.Meter.Read();
        PhaseIResults.PublishScalar(Results, "Power", value, "");
        UpgradeVerdict(Verdict.Pass);
    }
}

[Display("Spectrum Analyzer Marker Peak", Groups: ["Instrument Components", "SpecAn"], Description: "Peak marker X/Y.")]
public sealed class SpectrumAnalyzerMarkerPeakStep : TestStep
{
    [Display("Instrument")]
    public SpectrumAnalyzerInstrument Instrument { get; set; } = null!;

    public override void Run()
    {
        if (Instrument is null)
        {
            UpgradeVerdict(Verdict.Error);
            return;
        }

        var analyzer = Instrument.Analyzer;
        analyzer.MarkerPeak();
        var hz = analyzer.MarkerX();
        var dbm = analyzer.MarkerY();
        PhaseIResults.PublishScalar(Results, "MarkerX", hz, "Hz");
        PhaseIResults.PublishScalar(Results, "MarkerY", dbm, "dBm");
        UpgradeVerdict(Verdict.Pass);
    }
}
