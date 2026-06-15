using System.Text;
using InstrumentComponents.Connect;
using InstrumentComponents.Mock;
using InstrumentComponents.Transport;

namespace InstrumentComponents.Recording;

/// <summary>Records I/O on a wrapped transport for replay as a MockTransport script.</summary>
public sealed class RecordingTransport<T> : TransportBase where T : ITransport
{
    private T _inner;
    public List<ScriptStep> Steps { get; } = new();

    public RecordingTransport(T inner) => _inner = inner;

    public IReadOnlyList<ScriptStep> IntoScript() => Steps;

    public T IntoInner() => _inner;

    public override void Write(ReadOnlySpan<byte> data)
    {
        Steps.Add(new WriteStep { Data = Encoding.UTF8.GetString(data) });
        _inner.Write(data);
    }

    public override int Read(Span<byte> buffer)
    {
        var n = _inner.Read(buffer);
        Steps.Add(new ReadStep { Data = Encoding.UTF8.GetString(buffer[..n]) });
        return n;
    }

    public override void Clear()
    {
        Steps.Add(new ClearStep());
        _inner.Clear();
    }

    public override void SetReadTimeout(TimeSpan timeout) => _inner.SetReadTimeout(timeout);

    public override void Reconnect() => _inner.Reconnect();

    public override TransportIdentity Identity => _inner.Identity;

    public override void Configure(ConnectOptions opts) => _inner.Configure(opts);
}
