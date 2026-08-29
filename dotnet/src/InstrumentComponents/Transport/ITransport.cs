using InstrumentComponents.Connect;

namespace InstrumentComponents.Transport;

/// <summary>Swappable byte-level transport for instrument I/O.</summary>
public interface ITransport
{
    void Write(ReadOnlySpan<byte> data);
    int Read(Span<byte> buffer);
    void Clear();
    void SetReadTimeout(TimeSpan timeout);
    void Reconnect();
    TransportIdentity Identity { get; }
    void Configure(ConnectOptions opts);
}

public abstract class TransportBase : ITransport
{
    public abstract void Write(ReadOnlySpan<byte> data);
    public abstract int Read(Span<byte> buffer);
    public abstract void Clear();
    public abstract void SetReadTimeout(TimeSpan timeout);

    public virtual void Reconnect() =>
        throw new Errors.InstrumentUnsupportedException("reconnect");

    public virtual TransportIdentity Identity => new();

    public virtual void Configure(ConnectOptions opts) => SetReadTimeout(opts.IoTimeout());
}
