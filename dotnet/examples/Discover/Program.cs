using InstrumentComponents.Visa;

// Requires a vendor VISA install (NI-VISA / Keysight IO Libraries / etc.).
var catalog = VisaDiscovery.Create().Scan();
catalog.PrintSummary();
