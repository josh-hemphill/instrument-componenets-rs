using InstrumentComponents.Classifier;
using InstrumentComponents.Discovery;
using InstrumentComponents.Enumerator;
using InstrumentComponents.Kind;
using InstrumentComponents.Mock;
using InstrumentComponents.Registry;

namespace InstrumentComponents.Tests;

public class DiscoveryStaticTests
{
    [Fact]
    public void StaticDiscoveryClassifiesMockDevice()
    {
        var enumerator = StaticEnumerator.FromAddresses(["mock://dmm-1"]);
        var opener = new MockSessionOpener();
        opener.Register("mock://dmm-1", ScriptedFixture.Builder()
            .Idn("Keysight Technologies", "34401A", "SN1", "1.0")
            .Kinds(InstrumentKind.Dmm)
            .Build().IntoTransport());

        var catalog = new global::InstrumentComponents.Discovery.Discovery(enumerator, opener, ModelRegistry.Embedded())
            .OverrideKinds("mock://dmm-1", [InstrumentKind.Dmm])
            .WithProbePolicy(Probe.ProbePolicy.None)
            .Scan();

        Assert.Single(catalog.Devices);
        Assert.Contains(InstrumentKind.Dmm, catalog.Devices[0].SupportedKinds);
    }
}
