using InstrumentComponents.Address;
using InstrumentComponents.Catalog;
using InstrumentComponents.Device;
using InstrumentComponents.Enumerator;
using InstrumentComponents.Errors;
using InstrumentComponents.Kind;
using InstrumentComponents.Registry;
using InstrumentComponents.Visa;
using DiscoveryBuilder = InstrumentComponents.Discovery.Discovery;

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

    [HardwareFact]
    [Trait("Category", "Hardware")]
    public void DmmMeasureVoltageDcSmoke()
    {
        Assert.True(HardwareResource.TryFromEnv(out var resource, out var error), error);
        var opener = new VisaSessionOpener();
        var catalog = new DiscoveryBuilder(
            StaticEnumerator.FromAddresses([resource]),
            opener,
            ModelRegistry.Embedded(),
            opener).Scan();

        var device = DeviceForResource(catalog, resource);
        Assert.True(
            device.Discovered.Reachable,
            $"{HardwareResource.VariableName} is not reachable ({resource}): {device.Discovered.Error}");
        Assert.Contains(InstrumentKind.Dmm, device.SupportedKinds);

        var volts = device.OpenDmm().MeasureVoltageDc();
        Assert.True(
            double.IsFinite(volts),
            $"DMM reading was not finite: {volts} (model {device.Discovered.Identity.Model})");
        Console.Error.WriteLine(
            $"hardware smoke: {device.Discovered.Identity.Manufacturer ?? "?"} {device.Discovered.Identity.Model ?? "?"} @ {resource} → {volts} V DC");
    }

    private static DeviceRef DeviceForResource(DeviceCatalog catalog, string resource)
    {
        try
        {
            return catalog.Device(resource);
        }
        catch (DeviceNotFoundException)
        {
            var wanted = ResourceAddress.Parse(resource);
            var match = catalog.Devices.FirstOrDefault(d => d.Address.DedupKey == wanted.DedupKey)
                ?? throw new DeviceNotFoundException(resource);
            return catalog.Device(match.Address.Raw);
        }
    }
}
