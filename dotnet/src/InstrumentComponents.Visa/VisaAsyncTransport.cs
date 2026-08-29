using InstrumentComponents.Connect;
using InstrumentComponents.Transport;

namespace InstrumentComponents.Visa;

/// <summary>
/// VISA transport exposed as async via sync bridge (thread-pool offload).
/// Not vendor APM — see docs/visa-async-csharp.md. Cancellation does not abort in-flight native VISA calls.
/// </summary>
public sealed class VisaAsyncTransport : IAsyncTransport, IDisposable
{
    private readonly SyncAsAsyncTransport<VisaTransport> _inner;

    public VisaAsyncTransport(VisaTransport transport) => _inner = new SyncAsAsyncTransport<VisaTransport>(transport);

    public VisaTransport Inner => _inner.Inner;

    public ValueTask WriteAsync(ReadOnlyMemory<byte> data, CancellationToken cancellationToken = default) =>
        _inner.WriteAsync(data, cancellationToken);

    public ValueTask<int> ReadAsync(Memory<byte> buffer, CancellationToken cancellationToken = default) =>
        _inner.ReadAsync(buffer, cancellationToken);

    public ValueTask ClearAsync(CancellationToken cancellationToken = default) =>
        _inner.ClearAsync(cancellationToken);

    public ValueTask SetReadTimeoutAsync(TimeSpan timeout, CancellationToken cancellationToken = default) =>
        _inner.SetReadTimeoutAsync(timeout, cancellationToken);

    public ValueTask ReconnectAsync(CancellationToken cancellationToken = default) =>
        _inner.ReconnectAsync(cancellationToken);

    public TransportIdentity Identity => _inner.Identity;

    public ValueTask ConfigureAsync(ConnectOptions opts, CancellationToken cancellationToken = default) =>
        _inner.ConfigureAsync(opts, cancellationToken);

    public void Dispose()
    {
        _inner.Dispose();
        GC.SuppressFinalize(this);
    }
}
