using InstrumentComponents.Visa;

// Discover instruments including a manually specified TCPIP/LXI address.
// Requires a vendor VISA install (NI-VISA / Keysight IO Libraries / etc.).
var catalog = VisaDiscovery.Create()
    .ManualAddress("TCPIP0::192.168.0.42::INSTR")
    .Scan();

catalog.PrintSummary();
