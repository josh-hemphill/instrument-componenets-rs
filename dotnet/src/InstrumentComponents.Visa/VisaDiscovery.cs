using InstrumentComponents.Discovery;
using InstrumentComponents.Registry;
using InstrumentComponents.Session;

namespace InstrumentComponents.Visa;

public static class VisaDiscovery
{
    /// <summary>Creates a discovery builder wired to the local VISA resource manager.</summary>
    public static global::InstrumentComponents.Discovery.Discovery Create()
    {
        var opener = new VisaSessionOpener();
        return new global::InstrumentComponents.Discovery.Discovery(new VisaEnumerator(), opener, ModelRegistry.Embedded(), opener);
    }
}
