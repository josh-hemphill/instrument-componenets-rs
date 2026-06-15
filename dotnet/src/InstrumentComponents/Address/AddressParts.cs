namespace InstrumentComponents.Address;

/// <summary>Parsed components of a VISA resource string.</summary>
public sealed class AddressParts
{
    public uint? Board { get; set; }
    public uint? PrimaryAddress { get; set; }
    public uint? SecondaryAddress { get; set; }
    public string? Vid { get; set; }
    public string? Pid { get; set; }
    public string? Serial { get; set; }
    public string? Host { get; set; }
    public ushort? Port { get; set; }
    public string? Lane { get; set; }
}
