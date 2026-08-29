using InstrumentComponents.Visa;

// Async VISA discovery — requires NI-VISA / Keysight VISA installed.
var catalog = await VisaDiscovery.Create().ScanAsync();
catalog.PrintSummary();
