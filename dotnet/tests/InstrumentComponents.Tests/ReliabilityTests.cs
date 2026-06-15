using InstrumentComponents.Connect;
using InstrumentComponents.Mock;
using InstrumentComponents.Scpi;

namespace InstrumentComponents.Tests;

public class ReliabilityTests
{
    [Fact]
    public void QueryRetriesAfterTimeoutThenSucceeds()
    {
        var transport = new MockTransport([
            new WriteStep { Data = ":MEAS:VOLT:DC?\n" },
            new ReadStep { Data = "1.0\n" },
            new WriteStep { Data = ":MEAS:VOLT:DC?\n" },
            new ReadStep { Data = "1.0\n" },
        ]).FailWrites(1);

        var opts = new ConnectOptions { Retries = 1, RetryBackoff = TimeSpan.FromMilliseconds(1) };
        var session = new ScpiSession(transport, opts);
        var volts = session.Query(":MEAS:VOLT:DC?");
        Assert.Equal("1.0", volts.Trim());
    }
}
