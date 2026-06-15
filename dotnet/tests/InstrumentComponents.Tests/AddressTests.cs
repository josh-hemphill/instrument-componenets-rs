using InstrumentComponents.Address;

namespace InstrumentComponents.Tests;

public class AddressTests
{
    [Fact]
    public void ParsesUsbAddress()
    {
        var addr = ResourceAddress.Parse("USB0::0x0957::0x0607::MY123::INSTR");
        Assert.Equal(InterfaceKind.Usb, addr.Interface);
        Assert.Equal("0957", addr.Components.Vid);
        Assert.Equal("0607", addr.Components.Pid);
    }

    [Fact]
    public void DedupIsCaseInsensitive()
    {
        var a = ResourceAddress.Parse("usb0::0x0957::0x0607::SN::INSTR");
        var b = ResourceAddress.Parse("USB0::0x0957::0x0607::SN::INSTR");
        Assert.Equal(a.DedupKey, b.DedupKey);
    }
}
