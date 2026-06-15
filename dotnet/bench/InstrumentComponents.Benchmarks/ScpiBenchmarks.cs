using BenchmarkDotNet.Attributes;
using InstrumentComponents.Connect;
using InstrumentComponents.Mock;
using InstrumentComponents.Scpi;

namespace InstrumentComponents.Benchmarks;

[MemoryDiagnoser]
public class ScpiBenchmarks
{
    private ScpiSession _session = null!;
    private byte[] _asciiResponse = null!;

    [GlobalSetup]
    public void Setup()
    {
        var transport = new MockTransport([
            new WriteStep { Data = ":MEAS:VOLT:DC?\n" },
            new ReadStep { Data = "3.300\n" },
        ]);
        _session = new ScpiSession(transport, new ConnectOptions());
        _asciiResponse = "3.300\n"u8.ToArray();
    }

    [Benchmark]
    public void FramingAscii()
    {
        ScpiFraming.ExtractResponse(_asciiResponse, "\n");
    }

    [Benchmark]
    public void QueryRoundTrip()
    {
        var transport = new MockTransport([
            new WriteStep { Data = ":MEAS:VOLT:DC?\n" },
            new ReadStep { Data = "3.300\n" },
        ]);
        var session = new ScpiSession(transport, new ConnectOptions());
        _ = session.Query(":MEAS:VOLT:DC?");
    }
}
