using InstrumentComponents.Mock;

namespace InstrumentComponents.Tests;

public class TranscriptConformanceTests
{
    [Fact]
    public void LoadsSharedRustFixture()
    {
        var path = Path.GetFullPath(Path.Combine(
            AppContext.BaseDirectory, "..", "..", "..", "..", "..", "..", "fixtures", "smu2602.json"));
        var json = File.ReadAllText(path);
        var transcript = Transcript.FromJson(json);
        Assert.Equal(2, transcript.Steps.Count);
        var transport = new MockTransport(transcript.Steps);
        var session = new Scpi.ScpiSession(transport, new Connect.ConnectOptions());
        var volts = session.Query(":MEAS:VOLT:DC?");
        Assert.Equal("3.300", volts);
    }
}
