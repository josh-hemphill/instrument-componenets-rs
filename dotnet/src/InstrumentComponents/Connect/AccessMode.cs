namespace InstrumentComponents.Connect;

/// <summary>VISA access mode (maps to Ivi.Visa AccessModes when using VISA backend).</summary>
public readonly struct AccessMode
{
    public bool ExclusiveLock { get; init; }
    public bool SharedLock { get; init; }

    public static AccessMode NoLock => default;

    public static AccessMode SharedLockMode => new() { SharedLock = true };

    public static AccessMode ExclusiveLockMode => new() { ExclusiveLock = true };
}
