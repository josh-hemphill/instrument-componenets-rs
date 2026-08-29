using My.Company;
using Xunit;

public class HardCasesTests
{
    [Fact]
    public void DmmLooksLikeAClassAndMeasures()
    {
        using var dmm = Dmm.Create();
        Assert.Equal(3.3, dmm.MeasureVoltageDc(), 3);
    }

    [Fact]
    public void TimeoutThrowsTypedEnumException()
    {
        using var dmm = Dmm.Create();
        var ex = Assert.Throws<EnumException<Error>>(() => dmm.FailTimeout());
        Assert.True(ex.Value.IsTimeout);
        Assert.Equal("Enum variant mismatch.", ex.Message);
    }

    [Fact]
    public async Task AsyncMeasureReturnsTaskAndHonorsDefaultToken()
    {
        using var dmm = AsyncDmm.Create();
        var volts = await dmm.MeasureVoltageDc(CancellationToken.None);
        Assert.Equal(1.25, volts, 3);
    }

    [Fact]
    public async Task CancellationTokenAbortsRustFuture()
    {
        using var dmm = AsyncDmm.Create();
        using var cts = new CancellationTokenSource(200);
        await Assert.ThrowsAnyAsync<Exception>(async () => await dmm.SleepForever(cts.Token));
    }

    [Fact]
    public void ObserverCallbackAcceptsCSharpLambda()
    {
        using var session = Session.Create();
        uint seen = 0;
        session.PingObserver(kind => seen = kind);
        Assert.Equal(7u, seen);
    }

    [Fact]
    public void SlicePinsCallerArrayWithoutOwningCopy()
    {
        using var session = Session.Create();
        using var slice = new byte[] { 1, 2, 3, 4 }.Slice();
        Assert.Equal(10u, session.ChecksumSlice(slice));
    }
}
