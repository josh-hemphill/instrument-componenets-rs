namespace InstrumentComponents.Visa.Tests;

public class HardwareResourceTests
{
    [Fact]
    [Trait("Category", "Mock")]
    public void RejectsMissingAndEmpty()
    {
        Assert.False(HardwareResource.TryParse(null, out _, out var missing));
        Assert.Contains(HardwareResource.VariableName, missing);
        Assert.False(HardwareResource.TryParse("  ", out _, out _));
    }

    [Fact]
    [Trait("Category", "Mock")]
    public void ParsesUsbVisaResource()
    {
        Assert.True(HardwareResource.TryParse(
            " USB0::0x0957::0x0607::SN::INSTR ",
            out var resource,
            out var error));
        Assert.Null(error);
        Assert.Equal("USB0::0x0957::0x0607::SN::INSTR", resource);
    }
}
