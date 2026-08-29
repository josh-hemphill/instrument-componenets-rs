using InstrumentComponents.Connect;
using InstrumentComponents.Errors;
using Ivi.Visa;

namespace InstrumentComponents.Visa.Tests;

public class AccessModeTests
{
    [Fact]
    [Trait("Category", "Mock")]
    public void MapAccessModeExclusiveLock()
    {
        Assert.Equal(AccessModes.ExclusiveLock, VisaSessionOpener.MapAccessMode(AccessMode.ExclusiveLockMode));
        Assert.Equal(AccessModes.None, VisaSessionOpener.MapAccessMode(AccessMode.NoLock));
    }

    [Fact]
    [Trait("Category", "Mock")]
    public void MapAccessModeSharedLockIsUnsupported()
    {
        Assert.Throws<InstrumentUnsupportedException>(() =>
            VisaSessionOpener.MapAccessMode(AccessMode.SharedLockMode));
    }
}
