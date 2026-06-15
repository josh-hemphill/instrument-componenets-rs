using InstrumentComponents.Address;

namespace InstrumentComponents.Transport;

/// <summary>Optional pre-SCPI identity hints from the transport backend.</summary>
public sealed class TransportIdentity
{
    public string? Manufacturer { get; init; }
    public string? Model { get; init; }
    public string? Serial { get; init; }
    public InterfaceKind Interface { get; init; } = InterfaceKind.Unknown;
    public uint? ManfId { get; init; }
    public uint? ModelCode { get; init; }
}
