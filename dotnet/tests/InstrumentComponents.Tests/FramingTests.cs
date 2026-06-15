using InstrumentComponents.Errors;
using InstrumentComponents.Scpi;

namespace InstrumentComponents.Tests;

public class FramingTests
{
    [Fact]
    public void ReadsAsciiTerminated()
    {
        var data = ":MEAS:VOLT:DC?\n"u8.ToArray();
        var (payload, consumed) = ScpiFraming.ExtractResponse(data, "\n");
        Assert.Equal(":MEAS:VOLT:DC?"u8.ToArray(), payload);
        Assert.Equal(15, consumed);
    }

    [Fact]
    public void ReadsDefiniteBlockWithEmbeddedNewline()
    {
        var data = "#14\nemb\n"u8.ToArray();
        var (payload, consumed) = ScpiFraming.ExtractResponse(data, "\n");
        Assert.Equal("\nemb"u8.ToArray(), payload);
        Assert.Equal(7, consumed);
    }

    [Fact]
    public void ReadsIndefiniteBlock()
    {
        var data = "#0hello\n"u8.ToArray();
        var (payload, consumed) = ScpiFraming.ExtractResponse(data, "\n");
        Assert.Equal("hello"u8.ToArray(), payload);
        Assert.Equal(data.Length, consumed);
    }

    [Fact]
    public void EmptyBufferThrowsTimeout()
    {
        Assert.Throws<InstrumentTimeoutException>(() => ScpiFraming.ExtractResponse(ReadOnlySpan<byte>.Empty, "\n"));
    }
}
