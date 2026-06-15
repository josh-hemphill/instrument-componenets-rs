using InstrumentComponents.Visa;

namespace InstrumentComponents.Visa.Tests;

public class HardwareTests
{
    [Fact(Skip = "requires VISA runtime and connected instruments")]
    [Trait("Category", "Hardware")]
    public void DiscoverRealDevices()
    {
        var catalog = VisaDiscovery.Create().Scan();
        Assert.NotNull(catalog);
    }
}
