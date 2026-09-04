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
    /// <summary>Rejects Keithley-style overload sentinels such as 9.9e37.</summary>
    private const double MaxAbsVolts = 1_000_000;

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

        var dmm = device.OpenDmm();
        var dialect = dmm.Session.DialectFor(InstrumentKind.Dmm);
        var volts = dmm.MeasureVoltageDc();
        Assert.True(
            double.IsFinite(volts) && Math.Abs(volts) < MaxAbsVolts,
            $"DMM reading looks like overload/sentinel: {volts} (model {device.Discovered.Identity.Model})");
        Console.Error.WriteLine(
            $"hardware smoke: {device.Discovered.Identity.Manufacturer ?? "?"} {device.Discovered.Identity.Model ?? "?"} dialect={dialect.Id} @ {resource} → {volts} V DC");
        var model = device.Discovered.Identity.Model ?? "";
        if (model.Contains("DMM6500", StringComparison.OrdinalIgnoreCase))
            Assert.Equal("keithley_dmm6500", dialect.Id);
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
