using InstrumentComponents.Enumerator;
using InstrumentComponents.OpenTap;
using InstrumentComponents.Scpi;
using OpenTap;

namespace InstrumentComponents.OpenTap.Tests;

public class ScpiInstrumentTests
{
    [Fact]
    public void AllEightTypesHaveWritableVisaAddress()
    {
        ScpiInstrument[] instruments =
        [
            new DmmInstrument(),
            new DcPowerSupplyInstrument(),
            new FunctionGeneratorInstrument(),
            new OscilloscopeInstrument(),
            new SwitchInstrument(),
            new CounterInstrument(),
            new PowerMeterInstrument(),
            new SpectrumAnalyzerInstrument(),
        ];

        foreach (var instrument in instruments)
        {
            instrument.VisaAddress = "TCPIP0::192.168.0.10::inst0::INSTR";
            Assert.Equal("TCPIP0::192.168.0.10::inst0::INSTR", instrument.VisaAddress);
            Assert.True(instrument.GetType().GetProperty(nameof(ScpiInstrument.VisaAddress))!.CanWrite);
        }
    }

    [Fact]
    public void OpenWithoutSessionThrowsWithoutOpeningVisa()
    {
        var dmm = new DmmInstrument { VisaAddress = "TCPIP0::127.0.0.1::inst0::INSTR" };
        var ex = Assert.Throws<InvalidOperationException>(() => dmm.Open());
        Assert.Contains("does not open a vendor VISA", ex.Message, StringComparison.Ordinal);
    }

    [Fact]
    public void AttachSessionOpenQueryIdnAndClose()
    {
        var io = new ScriptedIo(("*IDN?", "Acme,DMM1,SN-1,2.0"));
        var dmm = new DmmInstrument();
        dmm.VisaAddress = "USB0::0x2A8D::0x1301::INSTR";
        dmm.AttachSession(io);
        dmm.Open();
        try
        {
            Assert.True(dmm.IsConnected);
            var idn = dmm.QueryIdn();
            Assert.Equal("Acme", idn.Manufacturer);
            Assert.Equal("DMM1", dmm.IdentityFields.Model);
            Assert.InRange(dmm.Dmm.MeasureVoltageDc(), 1.0, 1.0);
        }
        finally
        {
            dmm.Close();
        }

        Assert.True(io.Disposed);
        Assert.Throws<InvalidOperationException>(() => dmm.QueryIdn());
    }

    [Fact]
    public void ConstructorInjectionOpens()
    {
        var io = new ScriptedIo(("*IDN?", "Keysight,E36312A,SN,1.0"));
        var psu = new DcPowerSupplyInstrument(io) { VisaAddress = "TCPIP0::1.2.3.4::INSTR" };
        psu.Open();
        try
        {
            psu.OutputOff();
            Assert.Contains(io.Writes, w => w.Contains("OFF", StringComparison.OrdinalIgnoreCase));
        }
        finally
        {
            psu.Close();
        }
    }

    [Fact]
    public void DiscoveryIsEmptyWithoutEnumeratorAndListsWhenRegistered()
    {
        var discovery = new ScpiVisaAddressDiscovery();
        var attribute = new VisaAddressAttribute();
        Assert.True(discovery.CanDetect(attribute));
        Assert.Empty(discovery.DetectDeviceAddresses(attribute));

        OpenTapResourceEnumeration.Register(StaticEnumerator.FromAddresses(["USB0::0x1::0x2::INSTR"]));
        Assert.Equal(["USB0::0x1::0x2::INSTR"], discovery.DetectDeviceAddresses(attribute));
    }

    [Fact]
    public void TestPlanRoundTripsDmmTypeWithoutBroker()
    {
        var plan = new TestPlan();
        var path = Path.Combine(Path.GetTempPath(), $"ic-dmm-{Guid.NewGuid():N}.TapPlan");
        plan.Save(path);
        try
        {
            var loaded = TestPlan.Load(path);
            Assert.NotNull(loaded);
            Assert.Contains("DmmInstrument", typeof(DmmInstrument).FullName);
            Assert.Equal("InstrumentComponents.OpenTap.DmmInstrument", typeof(DmmInstrument).FullName);
        }
        finally
        {
            File.Delete(path);
        }
    }

    private sealed class ScriptedIo : IScpiIo
    {
        private readonly Dictionary<string, string> _queries;
        private bool _disposed;

        public ScriptedIo(params (string Command, string Response)[] queries)
        {
            _queries = new Dictionary<string, string>(StringComparer.OrdinalIgnoreCase)
            {
                [":MEAS:VOLT:DC?"] = "1.0",
            };
            foreach (var (command, response) in queries)
                _queries[command] = response;
        }

        public List<string> Writes { get; } = [];
        public bool Disposed { get; private set; }
        public TimeSpan IoTimeout { get; set; } = TimeSpan.FromSeconds(5);

        public void Write(string command)
        {
            ObjectDisposedException.ThrowIf(_disposed, this);
            Writes.Add(command);
        }

        public string Query(string command)
        {
            ObjectDisposedException.ThrowIf(_disposed, this);
            return _queries.TryGetValue(command.Trim(), out var response) ? response : "";
        }

        public void Dispose()
        {
            _disposed = true;
            Disposed = true;
        }
    }
}
