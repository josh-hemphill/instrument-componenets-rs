using InstrumentComponents.Address;
using InstrumentComponents.Classifier;
using InstrumentComponents.Kind;
using InstrumentComponents.Registry;

namespace InstrumentComponents.Tests;

public class ClassifierTests
{
    [Fact]
    public void RegistryHintForUsbVidPid()
    {
        var registry = ModelRegistry.Embedded();
        var addr = ResourceAddress.Parse("USB0::0x0957::0x0607::SN::INSTR");
        var (_, kinds) = Classifier.Classifier.ClassifyFromAddress(addr, registry);
        Assert.Contains(kinds, k => k.Kind == InstrumentKind.Dmm);
    }

    [Fact]
    public void UserOverrideWins()
    {
        var (kinds, _) = Classifier.Classifier.MergeClassifications(
            [[new ClassifiedKind(InstrumentKind.Unknown, 10, ClassifySource.ResourceParse)]],
            [InstrumentKind.Dmm]);
        Assert.Equal([InstrumentKind.Dmm], kinds);
    }
}
