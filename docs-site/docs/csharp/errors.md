# Errors (C#)

Exceptions derive from `InstrumentException` under `InstrumentComponents.Errors`.

## Hierarchy

| Type | When |
|---|---|
| `CommunicationException` | I/O failed with address + command + attempts |
| `InstrumentTimeoutException` | Operation timed out |
| `UnsupportedKindException` | Device does not support requested class |
| `DeviceNotFoundException` | Address or `DeviceId` not in catalog |
| `ScpiCommandException` | SCPI command failed |
| `ParseException` | Response could not be parsed |
| `TransportException` / `TransportClosedException` | Low-level transport failure |
| `MockExhaustedException` / `MockMismatchException` | Scripted fixture mismatch |
| `SessionLimitException` | Too many sessions for an address |
| `InvalidAddressException` | Bad resource string |

## Handling

```csharp
using InstrumentComponents.Errors;
using InstrumentComponents.Visa;

try
{
    var catalog = VisaDiscovery.Create().Scan();
    var dmm = catalog.OpenDmm(address);
    Console.WriteLine(dmm.MeasureVoltageDc());
}
catch (UnsupportedKindException ex)
{
    Console.Error.WriteLine($"{ex.Address}: {ex.Kind} not in [{string.Join(", ", ex.Supported)}]");
}
catch (CommunicationException ex)
{
    Console.Error.WriteLine($"comm @ {ex.Address} after {ex.Attempts} tries ({ex.Command}): {ex.Message}");
}
```

Rust mirrors these as `Error` enum variants — see [Rust errors](../rust/errors.md).
