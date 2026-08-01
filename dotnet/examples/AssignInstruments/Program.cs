using InstrumentComponents.Kind;
using InstrumentComponents.Visa;

var catalog = VisaDiscovery.Create().Scan();

Console.WriteLine("Available DMMs:");
foreach (var dev in catalog.DevicesByKind(InstrumentKind.Dmm))
{
    Console.WriteLine(
        $"  {dev.GetDeviceId()} — {dev.Identity.Model ?? "unknown"} @ {dev.Address.Raw} (reachable: {dev.Reachable})");
}

var first = catalog.DevicesByKind(InstrumentKind.Dmm).FirstOrDefault();
if (first is null) return;

const string role = "main_dmm";
var deviceId = first.GetDeviceId();
Console.WriteLine($"\nAssigning role '{role}' to device {deviceId}");
var deviceRef = catalog.ReconnectByIdentity(deviceId);
Console.WriteLine($"  health: consecutiveFailures={deviceRef.Health().ConsecutiveFailures}");
