namespace InstrumentComponents.Diagnostics;

public enum CommsEventKind
{
    WriteOk,
    WriteFailed,
    ReadOk,
    ReadFailed,
    Timeout,
    Reconnect,
}

public sealed record CommsEvent(
    string Address,
    CommsEventKind Kind,
    string? Command,
    uint Attempt,
    ulong ElapsedMs,
    string? Detail);

/// <summary>Receives push notifications for instrument communication events.</summary>
public interface ICommsObserver
{
    void OnEvent(CommsEvent evt);
}

/// <summary>Pollable health snapshot for a device address.</summary>
public sealed class DeviceHealth
{
    public uint ConsecutiveFailures { get; set; }
    public ulong TotalOperations { get; set; }
    public ulong TotalFailures { get; set; }
    public string? LastError { get; set; }
    public ulong? LastSuccessUnixMs { get; set; }
    public ulong? LastFailureUnixMs { get; set; }

    public bool IsHealthy => ConsecutiveFailures == 0;
}
