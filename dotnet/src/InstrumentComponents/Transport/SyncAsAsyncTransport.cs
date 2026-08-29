using InstrumentComponents.Connect;

namespace InstrumentComponents.Transport;

/// <summary>Wraps a sync transport as an async transport (for mocks/tests).</summary>
public sealed class SyncAsAsyncTransport<T> : IAsyncTransport, IDisposable where T : ITransport
{
    private readonly T _inner;

    public SyncAsAsyncTransport(T inner) => _inner = inner;

    public T Inner => _inner;

    public ValueTask WriteAsync(ReadOnlyMemory<byte> data, CancellationToken cancellationToken = default)
    {
        cancellationToken.ThrowIfCancellationRequested();
        _inner.Write(data.Span);
        return ValueTask.CompletedTask;
    }

    public ValueTask<int> ReadAsync(Memory<byte> buffer, CancellationToken cancellationToken = default)
    {
        cancellationToken.ThrowIfCancellationRequested();
        return ValueTask.FromResult(_inner.Read(buffer.Span));
    }

    public ValueTask ClearAsync(CancellationToken cancellationToken = default)
    {
        cancellationToken.ThrowIfCancellationRequested();
        _inner.Clear();
        return ValueTask.CompletedTask;
    }

    public ValueTask SetReadTimeoutAsync(TimeSpan timeout, CancellationToken cancellationToken = default)
    {
        cancellationToken.ThrowIfCancellationRequested();
        _inner.SetReadTimeout(timeout);
        return ValueTask.CompletedTask;
    }

    public ValueTask ReconnectAsync(CancellationToken cancellationToken = default)
    {
        cancellationToken.ThrowIfCancellationRequested();
        _inner.Reconnect();
        return ValueTask.CompletedTask;
    }

    public TransportIdentity Identity => _inner.Identity;

    public ValueTask ConfigureAsync(ConnectOptions opts, CancellationToken cancellationToken = default)
    {
        cancellationToken.ThrowIfCancellationRequested();
        _inner.Configure(opts);
        return ValueTask.CompletedTask;
    }

    public void Dispose()
    {
        if (_inner is IDisposable disposable)
            disposable.Dispose();
        GC.SuppressFinalize(this);
    }
}
