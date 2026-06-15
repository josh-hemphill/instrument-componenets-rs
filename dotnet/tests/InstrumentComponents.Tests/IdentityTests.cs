using InstrumentComponents.Identity;

namespace InstrumentComponents.Tests;

public class IdentityTests
{
    [Fact]
    public void ParsesIdn()
    {
        var idn = Idn.Parse("Keysight Technologies,34401A,MY123,1.0\n");
        Assert.Equal("Keysight Technologies", idn.Manufacturer);
        Assert.Equal("34401A", idn.Model);
    }
}
