using InstrumentComponents.Address;
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

    [Fact]
    [Trait("Category", "Mock")]
    public void OpenSharedLockThrowsUnsupportedWithoutVisa()
    {
        var opener = new VisaSessionOpener();
        var opts = new ConnectOptions { AccessMode = AccessMode.SharedLockMode };
        var ex = Assert.Throws<InstrumentUnsupportedException>(() =>
            opener.Open(ResourceAddress.Parse("TCPIP0::127.0.0.1::inst0::INSTR"), opts));
        Assert.Contains("shared lock", ex.Message, StringComparison.OrdinalIgnoreCase);
    }
}
