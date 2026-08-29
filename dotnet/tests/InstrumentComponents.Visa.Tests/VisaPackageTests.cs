namespace InstrumentComponents.Visa.Tests;

public class VisaPackageTests
{
    [Fact]
    [Trait("Category", "Mock")]
    public void VisaTransportTypeIsPublic()
    {
        Assert.Equal("VisaTransport", typeof(VisaTransport).Name);
        Assert.Equal("VisaSessionOpener", typeof(VisaSessionOpener).Name);
    }
}
