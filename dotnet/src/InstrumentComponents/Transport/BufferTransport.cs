using InstrumentComponents.Errors;

namespace InstrumentComponents.Transport;

/// <summary>In-memory buffer transport for testing read/write plumbing.</summary>
public sealed class BufferTransport : TransportBase
{
    public List<byte> Written { get; } = new();
    public byte[] ReadData { get; set; } = Array.Empty<byte>();
    private int _readPos;

    public override void Write(ReadOnlySpan<byte> data) => Written.AddRange(data.ToArray());

    public override int Read(Span<byte> buffer)
    {
        if (_readPos >= ReadData.Length)
            throw new TransportClosedException();
        var n = Math.Min(buffer.Length, ReadData.Length - _readPos);
        ReadData.AsSpan(_readPos, n).CopyTo(buffer);
        _readPos += n;
        return n;
    }

    public override void Clear() => _readPos = 0;

    public override void SetReadTimeout(TimeSpan timeout) { }

    public override void Reconnect() { }
}
