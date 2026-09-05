using OpenTap;

namespace InstrumentComponents.OpenTap;

[Display("Identity Query", Groups: ["Instrument Components", "Identity"], Description: "Query instrument *IDN?.")]
public sealed class IdentityQueryStep : TestStep
{
    [Display("Instrument")]
    public ScpiInstrument Instrument { get; set; } = null!;

    public override void Run()
    {
        if (Instrument is null)
        {
            UpgradeVerdict(Verdict.Error);
            Log.Error("No instrument assigned.");
            return;
        }

        var idn = Instrument.QueryIdn();
        Log.Info("IDN={0}", idn.FormatResponse());
        PhaseIResults.PublishIdentity(Results, idn.FormatResponse(), string.Empty);
        UpgradeVerdict(Verdict.Pass);
    }
}

[Display("Safe Shutdown", Groups: ["Instrument Components", "Safety"], Description: "Output off, then *RST.")]
public sealed class SafeShutdownStep : TestStep
{
    [Display("Instrument")]
    public ScpiInstrument Instrument { get; set; } = null!;

    public override void Run()
    {
        if (Instrument is null)
        {
            UpgradeVerdict(Verdict.Error);
            Log.Error("No instrument assigned.");
            return;
        }

        Instrument.OutputOff();
        Instrument.Reset();
        Log.Info("Safe shutdown complete for {0}", Instrument.Name);
        UpgradeVerdict(Verdict.Pass);
    }
}
