namespace InstrumentComponents.Scpi;

/// <summary>
/// Adapts write/query callbacks into <see cref="IScpiIo"/>.
/// HardwareTest wraps <c>IVisaSession</c> with these callbacks (no extra framing).
/// </summary>
public sealed class DelegateScpiIo : IScpiIo
{
    private readonly Action<string> _write;
    private readonly Func<string, string> _query;
    private readonly Action<TimeSpan>? _onTimeoutChanged;
    private readonly Action? _onDispose;
    private TimeSpan _ioTimeout;
    private bool _disposed;

    public DelegateScpiIo(
        Action<string> write,
        Func<string, string> query,
        TimeSpan? ioTimeout = null,
        Action<TimeSpan>? onTimeoutChanged = null,
        Action? onDispose = null)
    {
        _write = write ?? throw new ArgumentNullException(nameof(write));
        _query = query ?? throw new ArgumentNullException(nameof(query));
        _onTimeoutChanged = onTimeoutChanged;
        _onDispose = onDispose;
        IoTimeout = ioTimeout ?? TimeSpan.FromSeconds(5);
    }

    public TimeSpan IoTimeout
    {
        get => _ioTimeout;
        set
        {
            _ioTimeout = value;
            _onTimeoutChanged?.Invoke(value);
        }
    }

    public void Write(string command)
    {
        ObjectDisposedException.ThrowIf(_disposed, this);
        _write(command);
    }

    public string Query(string command)
    {
        ObjectDisposedException.ThrowIf(_disposed, this);
        return _query(command);
    }

    public void Dispose()
    {
        if (_disposed)
            return;
        _disposed = true;
        _onDispose?.Invoke();
        GC.SuppressFinalize(this);
    }
}
