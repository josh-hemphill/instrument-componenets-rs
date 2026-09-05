using InstrumentComponents.Address;
using InstrumentComponents.Classes;
using InstrumentComponents.Errors;
using InstrumentComponents.Identity;
using InstrumentComponents.Scpi;
using InstrumentComponents.Session;
using OpenTap;

namespace InstrumentComponents.OpenTap;

/// <summary>Shared SCPI resource: VisaAddress, injected session, identity, and extra class views.</summary>
public abstract class ScpiInstrument : Instrument, IInstrumentIdentity, IInstrumentShutdown
{
    public const int DefaultIoTimeoutMilliseconds = 5000;
    public const int MinIoTimeoutMilliseconds = 100;
    public const int MaxIoTimeoutMilliseconds = 120_000;

    private IScpiIo? _attached;
    private InstrumentSession? _session;
    private readonly DeviceIdentity _identity = new();

    protected ScpiInstrument()
    {
    }

    protected ScpiInstrument(IScpiIo io)
    {
        _attached = io ?? throw new ArgumentNullException(nameof(io));
    }

    [Display("Visa Address", Group: "Communication", Order: 1)]
    public string VisaAddress { get; set; } = string.Empty;

    [Display("IO Timeout (ms)", Group: "Communication", Order: 2)]
    public int IoTimeoutMilliseconds { get; set; } = DefaultIoTimeoutMilliseconds;

    [EmbedProperties]
    [Display("Identity", Order: 10)]
    public ScpiIdentityFields IdentityFields { get; set; } = new();

    /// <summary>Host injects an already-open message session after TestPlan.Load.</summary>
    public void AttachSession(IScpiIo io) =>
        _attached = io ?? throw new ArgumentNullException(nameof(io));

    public override void Open()
    {
        if (_attached is null)
        {
            throw new InvalidOperationException(
                "No SCPI session attached. The host must call AttachSession (or the IScpiIo constructor) before Open. This pack does not open a vendor VISA resource manager.");
        }

        _attached.IoTimeout = TimeSpan.FromMilliseconds(ClampTimeout());
        _session = InstrumentSession.FromIo(ParseOrFallback(VisaAddress), _attached, _identity);
        try
        {
            var idn = _session.Idn();
            _identity.Manufacturer = idn.Manufacturer;
            _identity.Model = idn.Model;
            _identity.Serial = idn.Serial;
            _identity.Firmware = idn.Firmware;
            IdentityFields.CopyFrom(idn);
        }
        catch (Exception ex)
        {
            Log.Warning("Could not query *IDN? for {0}: {1}", Name, ex.Message);
        }

        base.Open();
    }

    public override void Close()
    {
        var session = _session;
        _session = null;
        session?.Dispose();
        _attached = null;
        base.Close();
    }

    public Idn QueryIdn() => RequireSession().Idn();

    public void Reset() => RequireSession().Reset();

    public abstract void OutputOff();

    public Dmm AsDmm() => new(RequireSession());

    public DcPowerSupply AsDcPowerSupply() => new(RequireSession());

    public FunctionGenerator AsFunctionGenerator() => new(RequireSession());

    public Oscilloscope AsOscilloscope() => new(RequireSession());

    public Switch AsSwitch() => new(RequireSession());

    public Counter AsCounter() => new(RequireSession());

    public PowerMeter AsPowerMeter() => new(RequireSession());

    public SpectrumAnalyzer AsSpectrumAnalyzer() => new(RequireSession());

    protected InstrumentSession RequireSession() =>
        _session ?? throw new InvalidOperationException($"{GetType().Name} is not open.");

    protected int ClampTimeout() => Math.Clamp(
        IoTimeoutMilliseconds,
        MinIoTimeoutMilliseconds,
        MaxIoTimeoutMilliseconds);

    internal static ResourceAddress ParseOrFallback(string visaAddress)
    {
        try
        {
            if (!string.IsNullOrWhiteSpace(visaAddress))
                return ResourceAddress.Parse(visaAddress);
        }
        catch (InvalidAddressException)
        {
            // Host may leave an alias that this parser does not model.
        }

        return ResourceAddress.Parse("mock://unspecified");
    }
}

public sealed class ScpiIdentityFields
{
    [Display("Manufacturer", Order: 1)]
    public string Manufacturer { get; set; } = string.Empty;

    [Display("Model", Order: 2)]
    public string Model { get; set; } = string.Empty;

    [Display("Serial", Order: 3)]
    public string Serial { get; set; } = string.Empty;

    [Display("Firmware", Order: 4)]
    public string Firmware { get; set; } = string.Empty;

    public void CopyFrom(Idn idn)
    {
        Manufacturer = idn.Manufacturer;
        Model = idn.Model;
        Serial = idn.Serial;
        Firmware = idn.Firmware;
    }
}
