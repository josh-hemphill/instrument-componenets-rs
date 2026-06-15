using System.Text.Json.Serialization;

namespace InstrumentComponents.Mock;

[JsonPolymorphic(TypeDiscriminatorPropertyName = "op")]
[JsonDerivedType(typeof(WriteStep), "write")]
[JsonDerivedType(typeof(ReadStep), "read")]
[JsonDerivedType(typeof(ClearStep), "clear")]
public abstract class ScriptStep;

public sealed class WriteStep : ScriptStep
{
    public string Data { get; set; } = "";
}

public sealed class ReadStep : ScriptStep
{
    public string Data { get; set; } = "";
}

public sealed class ClearStep : ScriptStep;
