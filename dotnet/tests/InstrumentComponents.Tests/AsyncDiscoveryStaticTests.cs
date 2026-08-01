using InstrumentComponents.Address;
using InstrumentComponents.Classifier;
using InstrumentComponents.Discovery;
using InstrumentComponents.Enumerator;
using InstrumentComponents.Kind;
using InstrumentComponents.Mock;
using InstrumentComponents.Registry;
using InstrumentComponents.Transport;

namespace InstrumentComponents.Tests;

public class AsyncDiscoveryStaticTests
{
    [Fact]
    public async Task StaticEnumeratorMergeAndClassifyAsync()
    {
        var addr1 = ResourceAddress.Parse("USB0::0x0957::0x0607::SN1::INSTR");
        var addr2 = ResourceAddress.Parse("GPIB0::10::INSTR");

        var enumerator = new StaticEnumerator([
            new RawResource { Address = addr1, IdentityHint = new TransportIdentity() },
            new RawResource { Address = addr2, IdentityHint = new TransportIdentity() },
        ]);

        var opener = new MockSessionOpener();
        var catalog = await new global::InstrumentComponents.Discovery.Discovery(
                enumerator,
                opener,
                ModelRegistry.Embedded())
            .WithProbePolicy(Probe.ProbePolicy.None)
            .ScanAsync();

        Assert.Equal(2, catalog.Devices.Count);
        var usb = catalog.Device("USB0::0x0957::0x0607::SN1::INSTR");
        Assert.Contains(InstrumentKind.Dmm, usb.SupportedKinds);
    }
}
