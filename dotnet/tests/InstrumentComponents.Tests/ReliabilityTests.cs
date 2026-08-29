using InstrumentComponents.Connect;
using InstrumentComponents.Errors;
using InstrumentComponents.Mock;
using InstrumentComponents.Scpi;
using InstrumentComponents.Transport;

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

    [Fact]
    public void ProbeSystErrIsFalseWhenQueryFails()
    {
        var transport = new MockTransport([
            new WriteStep { Data = "SYST:ERR?\n" },
        ]).FailWrites(5);
        var session = new ScpiSession(transport, new ConnectOptions { Retries = 0, ReconnectOnFailure = false });
        Assert.False(session.ProbeSystErr());
    }

    [Fact]
    public void ResetOnConnectWritesIeee4882Commands()
    {
        var transport = new BufferTransport();
        var opts = new ConnectOptions { ResetOnConnect = true, ReconnectOnFailure = false };
        _ = new ScpiSession(transport, opts);
        var written = System.Text.Encoding.UTF8.GetString(transport.Written.ToArray());
        Assert.Contains("*CLS", written);
    }

    [Fact]
    public void ProbeOpcFailureRestoresIoTimeout()
    {
        var transport = new BufferTransport();
        var opts = new ConnectOptions
        {
            ReadTimeout = TimeSpan.FromSeconds(9),
            WriteTimeout = TimeSpan.FromSeconds(4),
            ReconnectOnFailure = false,
        };
        var session = new ScpiSession(transport, opts);
        Assert.False(session.ProbeOpc());
        Assert.Equal(opts.IoTimeout(), transport.LastReadTimeout);
    }

    [Fact]
    public async Task AsyncSessionDisposesWrappedSyncTransport()
    {
        var inner = new DisposableTransport();
        var session = await AsyncScpiSession.CreateAsync(
            new SyncAsAsyncTransport<DisposableTransport>(inner),
            new ConnectOptions { ReconnectOnFailure = false });
        session.Dispose();
        Assert.True(inner.Disposed);
    }

    private sealed class DisposableTransport : TransportBase, IDisposable
    {
        public bool Disposed { get; private set; }
        public override void Write(ReadOnlySpan<byte> data) { }
        public override int Read(Span<byte> buffer) => throw new TransportClosedException();
        public override void Clear() { }
        public override void SetReadTimeout(TimeSpan timeout) { }
        public void Dispose() => Disposed = true;
    }
}
