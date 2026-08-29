using InstrumentComponents.Kind;
using InstrumentComponents.Visa;

// Requires a vendor VISA install (NI-VISA / Keysight IO Libraries / etc.).
var catalog = VisaDiscovery.Create().Scan();
var dmms = catalog.DevicesByKind(InstrumentKind.Dmm);
if (dmms.Count == 0)
{
    Console.Error.WriteLine("no DMM found");
    return;
}

var addr = dmms[0].Address.Raw;
var dmm = catalog.OpenDmm(addr);
var volts = dmm.MeasureVoltageDc();
Console.WriteLine($"{addr}: {volts} V DC");
