using InstrumentComponents.Address;
using InstrumentComponents.Enumerator;
using InstrumentComponents.Transport;
using Ivi.Visa;

namespace InstrumentComponents.Visa;

/// <summary>VISA-backed resource enumerator.</summary>
public sealed class VisaEnumerator : IResourceEnumerator
{
    public IReadOnlyList<RawResource> List(string pattern)
    {
        var resources = new List<RawResource>();
        foreach (string raw in GlobalResourceManager.Find(pattern))
        {
            var address = ResourceAddress.Parse(raw);
            resources.Add(new RawResource
            {
                Address = address,
                IdentityHint = new TransportIdentity { Interface = address.Interface },
            });
        }
        return resources;
    }
}
