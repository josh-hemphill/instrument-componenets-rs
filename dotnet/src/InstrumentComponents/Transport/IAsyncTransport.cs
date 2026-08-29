using InstrumentComponents.Connect;

namespace InstrumentComponents.Transport;

/// <summary>Async byte-level transport for instrument I/O.</summary>
public interface IAsyncTransport
{
    ValueTask WriteAsync(ReadOnlyMemory<byte> data, CancellationToken cancellationToken = default);
    ValueTask<int> ReadAsync(Memory<byte> buffer, CancellationToken cancellationToken = default);
    ValueTask ClearAsync(CancellationToken cancellationToken = default);
    ValueTask SetReadTimeoutAsync(TimeSpan timeout, CancellationToken cancellationToken = default);
    ValueTask ReconnectAsync(CancellationToken cancellationToken = default);
    TransportIdentity Identity { get; }
    ValueTask ConfigureAsync(ConnectOptions opts, CancellationToken cancellationToken = default);
}

public abstract class AsyncTransportBase : IAsyncTransport
{
    public abstract ValueTask WriteAsync(ReadOnlyMemory<byte> data, CancellationToken cancellationToken = default);
    public abstract ValueTask<int> ReadAsync(Memory<byte> buffer, CancellationToken cancellationToken = default);

    public virtual ValueTask ClearAsync(CancellationToken cancellationToken = default) =>
        ValueTask.CompletedTask;

    public virtual ValueTask SetReadTimeoutAsync(TimeSpan timeout, CancellationToken cancellationToken = default) =>
        ValueTask.CompletedTask;

    public virtual ValueTask ReconnectAsync(CancellationToken cancellationToken = default) =>
        ValueTask.FromException(new Errors.InstrumentUnsupportedException("reconnect"));

    public virtual TransportIdentity Identity => new();

    public virtual ValueTask ConfigureAsync(ConnectOptions opts, CancellationToken cancellationToken = default) =>
        SetReadTimeoutAsync(opts.IoTimeout(), cancellationToken);
}
