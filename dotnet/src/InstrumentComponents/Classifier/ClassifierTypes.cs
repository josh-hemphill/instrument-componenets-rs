using InstrumentComponents.Address;
using InstrumentComponents.Identity;
using InstrumentComponents.Kind;

namespace InstrumentComponents.Classifier;

public enum ClassifySource
{
    ResourceParse,
    VisaAttributes,
    ModelRegistry,
    ScpiIdn,
    CapabilityProbe,
    UserOverride,
}

public sealed record ClassifiedKind(InstrumentKind Kind, byte Confidence, ClassifySource Source);

/// <summary>A discovered device in the catalog.</summary>
public sealed class DiscoveredDevice
{
    public required ResourceAddress Address { get; init; }
    public required DeviceIdentity Identity { get; init; }
    public required IReadOnlyList<InstrumentKind> SupportedKinds { get; init; }
    public required IReadOnlyList<ClassifiedKind> Classification { get; init; }
    public required bool Reachable { get; init; }
    public string? Error { get; init; }

    public DeviceId GetDeviceId() => DeviceId.FromIdentity(Identity, Address.Raw);
}
