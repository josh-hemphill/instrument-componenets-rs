namespace InstrumentComponents.Diagnostics;

/// <summary>Shared diagnostics context injected into SCPI sessions.</summary>
public sealed class CommsDiagnostics
{
    private readonly string _address;
    private object? _healthLock;
    private DeviceHealth? _health;
    private ICommsObserver? _observer;

    public CommsDiagnostics(string address) => _address = address;

    public CommsDiagnostics WithHealth(DeviceHealth health, object healthLock)
    {
        _health = health;
        _healthLock = healthLock;
        return this;
    }

    public CommsDiagnostics WithObserver(ICommsObserver observer)
    {
        _observer = observer;
        return this;
    }

    public string Address => _address;

    public void RecordSuccess(CommsEventKind kind, string? command, uint attempt, TimeSpan elapsed)
    {
        if (_health is not null && _healthLock is not null)
        {
            lock (_healthLock)
            {
                _health.ConsecutiveFailures = 0;
                _health.TotalOperations++;
                _health.LastSuccessUnixMs = NowUnixMs();
            }
        }
        Emit(kind, command, attempt, elapsed, null);
    }

    public void RecordFailure(CommsEventKind kind, string? command, uint attempt, TimeSpan elapsed, string detail)
    {
        if (_health is not null && _healthLock is not null)
        {
            lock (_healthLock)
            {
                _health.ConsecutiveFailures++;
                _health.TotalOperations++;
                _health.TotalFailures++;
                _health.LastError = detail;
                _health.LastFailureUnixMs = NowUnixMs();
            }
        }
        Emit(kind, command, attempt, elapsed, detail);
    }

    private void Emit(CommsEventKind kind, string? command, uint attempt, TimeSpan elapsed, string? detail)
    {
        if (_observer is null) return;
        _observer.OnEvent(new CommsEvent(_address, kind, command, attempt, (ulong)elapsed.TotalMilliseconds, detail));
    }

    private static ulong NowUnixMs() =>
        (ulong)DateTimeOffset.UtcNow.ToUnixTimeMilliseconds();
}
