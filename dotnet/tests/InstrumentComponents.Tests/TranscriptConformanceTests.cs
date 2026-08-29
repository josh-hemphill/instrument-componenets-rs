using InstrumentComponents.Mock;

namespace InstrumentComponents.Tests;

public class TranscriptConformanceTests
{
    private static string FixturePath(string name) => RepoFiles.Fixture(name);

    [Theory]
    [InlineData("smu2602.json", 2)]
    [InlineData("scope_ds1054z.json", 7)]
    [InlineData("switch_34970a.json", 2)]
    [InlineData("counter_53230a.json", 2)]
    public void LoadsSharedFixture(string fileName, int expectedSteps)
    {
        var json = File.ReadAllText(FixturePath(fileName));
        var transcript = Transcript.FromJson(json);
        Assert.Equal(expectedSteps, transcript.Steps.Count);
        _ = new MockTransport(transcript.Steps);
    }

    [Fact]
    public void Smu2602QueryRoundTrip()
    {
        var json = File.ReadAllText(FixturePath("smu2602.json"));
        var transcript = Transcript.FromJson(json);
        var transport = new MockTransport(transcript.Steps);
        var session = new Scpi.ScpiSession(transport, new Connect.ConnectOptions());
        var volts = session.Query(":MEAS:VOLT:DC?");
        Assert.Equal("3.300", volts);
    }
}
