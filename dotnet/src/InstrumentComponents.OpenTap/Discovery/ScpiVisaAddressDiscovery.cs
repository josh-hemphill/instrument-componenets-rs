using InstrumentComponents.Enumerator;
using OpenTap;

namespace InstrumentComponents.OpenTap;

/// <summary>
/// Host-registered VISA enumerator for pack <see cref="IDeviceDiscovery"/>.
/// OpenTAP session-local (not a process-wide broker).
/// </summary>
public static class OpenTapResourceEnumeration
{
    private static readonly SessionLocal<IResourceEnumerator?> Binding = new(null, autoDispose: false);

    public static void Register(IResourceEnumerator enumerator) =>
        Binding.Value = enumerator ?? throw new ArgumentNullException(nameof(enumerator));

    public static IResourceEnumerator? Current => Binding.Value;
}

/// <summary>Lists VisaAddress strings from a host-registered enumerator (empty if none).</summary>
[Display("Instrument Components VISA", Groups: ["Instrument Components"])]
public sealed class ScpiVisaAddressDiscovery : IDeviceDiscovery
{
    public bool CanDetect(DeviceAddressAttribute addressType) => addressType is VisaAddressAttribute;

    public string[] DetectDeviceAddresses(DeviceAddressAttribute addressType)
    {
        if (!CanDetect(addressType))
            return [];

        var enumerator = OpenTapResourceEnumeration.Current;
        if (enumerator is null)
            return [];

        return enumerator.List("?*INSTR")
            .Select(resource => resource.Address.Raw)
            .Distinct(StringComparer.OrdinalIgnoreCase)
            .OrderBy(address => address, StringComparer.OrdinalIgnoreCase)
            .ToArray();
    }
}
