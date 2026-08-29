using InstrumentComponents.Connect;
using InstrumentComponents.Diagnostics;
using InstrumentComponents.Errors;
using InstrumentComponents.Mock;
using InstrumentComponents.Scpi;

namespace InstrumentComponents.Tests;

public class AsyncReliabilityTests
{
    [Fact]
    public async Task QueryRetriesAfterTimeoutThenSucceeds()
    {
        var transport = new MockTransport([
            new WriteStep { Data = ":MEAS:VOLT:DC?\n" },
            new ReadStep { Data = "1.0\n" },
            new WriteStep { Data = ":MEAS:VOLT:DC?\n" },
            new ReadStep { Data = "1.0\n" },
        ]).FailWrites(1);

        var opts = new ConnectOptions { Retries = 1, RetryBackoff = TimeSpan.FromMilliseconds(1) };
        var session = await AsyncScpiSession.CreateAsync(transport, opts);
        var volts = await session.QueryAsync(":MEAS:VOLT:DC?");
        Assert.Equal("1.0", volts.Trim());
    }
}

public class DiagnosticsTests
{
    [Fact]
    public void DiagnosticsRecordsFailuresAndObserverEvents()
    {
        var health = new DeviceHealth();
        var healthLock = new object();
        var events = new List<CommsEvent>();
        var observer = new RecordingObserver(events);

        var diag = new CommsDiagnostics("mock://dmm-1")
            .WithHealth(health, healthLock)
            .WithObserver(observer);

        var transport = new MockTransport([
            new WriteStep { Data = "*IDN?\n" },
            new ReadStep { Data = "Acme,123,SN,1.0\n" },
        ]).FailWrites(5);

        var opts = new ConnectOptions { Retries = 0 };
        var session = new ScpiSession(transport, opts).WithDiagnostics(diag);

        Assert.ThrowsAny<InstrumentException>(() => session.Query("*IDN?"));

        lock (healthLock)
        {
            Assert.Equal(1u, health.ConsecutiveFailures);
            Assert.Equal(1u, health.TotalFailures);
            Assert.NotNull(health.LastError);
        }

        Assert.Contains(events, e => e.Kind == CommsEventKind.Timeout);
        Assert.DoesNotContain(events, e => e.Kind == CommsEventKind.Reconnect);
    }
}

public class AsyncDiagnosticsTests
{
    [Fact]
    public async Task AsyncDiagnosticsRecordsFailuresAndObserverEvents()
    {
        var health = new DeviceHealth();
        var healthLock = new object();
        var events = new List<CommsEvent>();
        var observer = new RecordingObserver(events);

        var diag = new CommsDiagnostics("mock://dmm-1")
            .WithHealth(health, healthLock)
            .WithObserver(observer);

        var transport = new MockTransport([
            new WriteStep { Data = "*IDN?\n" },
            new ReadStep { Data = "Acme,123,SN,1.0\n" },
        ]).FailWrites(5);

        var opts = new ConnectOptions { Retries = 0 };
        var session = (await AsyncScpiSession.CreateAsync(transport, opts)).WithDiagnostics(diag);

        await Assert.ThrowsAnyAsync<InstrumentException>(() => session.QueryAsync("*IDN?"));

        lock (healthLock)
        {
            Assert.Equal(1u, health.ConsecutiveFailures);
            Assert.Equal(1u, health.TotalFailures);
            Assert.NotNull(health.LastError);
        }

        Assert.Contains(events, e => e.Kind == CommsEventKind.Timeout);
        Assert.DoesNotContain(events, e => e.Kind == CommsEventKind.Reconnect);
    }
}

sealed class RecordingObserver(List<CommsEvent> events) : ICommsObserver
{
    public void OnEvent(CommsEvent evt) => events.Add(evt);
}
