using InstrumentComponents.Address;
using InstrumentComponents.Transport;

namespace InstrumentComponents.Enumerator;

public sealed class RawResource
{
    public required ResourceAddress Address { get; init; }
    public TransportIdentity IdentityHint { get; init; } = new();
}

/// <summary>Narrow seam for acquiring resources without coupling to VISA.</summary>
public interface IResourceEnumerator
{
    IReadOnlyList<RawResource> List(string pattern);
}

/// <summary>Static resource list for unit tests.</summary>
public sealed class StaticEnumerator : IResourceEnumerator
{
    private readonly List<RawResource> _resources;

    public StaticEnumerator(IReadOnlyList<RawResource> resources) => _resources = resources.ToList();

    public static StaticEnumerator FromAddresses(IEnumerable<string> addresses)
    {
        var resources = addresses.Select(raw => new RawResource
        {
            Address = ResourceAddress.Parse(raw),
            IdentityHint = new TransportIdentity(),
        }).ToList();
        return new StaticEnumerator(resources);
    }

    public IReadOnlyList<RawResource> List(string pattern)
    {
        if (pattern is "?*INSTR" or "?*")
            return _resources;
        var trimmed = pattern.Trim('?');
        return _resources.Where(r => r.Address.Raw.Contains(trimmed, StringComparison.Ordinal)).ToList();
    }
}
