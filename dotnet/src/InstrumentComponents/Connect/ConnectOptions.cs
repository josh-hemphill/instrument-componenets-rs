namespace InstrumentComponents.Connect;

/// <summary>Options when opening an instrument session.</summary>
public sealed class ConnectOptions
{
    public TimeSpan OpenTimeout { get; set; } = TimeSpan.FromSeconds(5);
    public TimeSpan ReadTimeout { get; set; } = TimeSpan.FromSeconds(10);
    public TimeSpan WriteTimeout { get; set; } = TimeSpan.FromSeconds(10);
    public TimeSpan? PerOpTimeout { get; set; }
    public string Terminator { get; set; } = "\n";
    public AccessMode AccessMode { get; set; } = AccessMode.NoLock;
    public bool ResetOnConnect { get; set; }
    public uint Retries { get; set; } = 2;
    public TimeSpan RetryBackoff { get; set; } = TimeSpan.FromMilliseconds(100);
    public bool ReconnectOnFailure { get; set; } = true;

    /// <summary>Single I/O timeout for backends (VISA) that cannot split read vs write.</summary>
    public TimeSpan IoTimeout() => PerOpTimeout ?? (ReadTimeout > WriteTimeout ? ReadTimeout : WriteTimeout);

    public ConnectOptions WithReadTimeout(TimeSpan timeout)
    {
        ReadTimeout = timeout;
        return this;
    }

    public ConnectOptions WithPerOpTimeout(TimeSpan timeout)
    {
        PerOpTimeout = timeout;
        return this;
    }
}
