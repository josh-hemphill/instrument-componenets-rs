namespace InstrumentComponents.Identity;

/// <summary>Parsed *IDN? response.</summary>
public sealed record Idn(string Manufacturer, string Model, string Serial, string Firmware)
{
    public static Idn Parse(string response)
    {
        var trimmed = response.Trim();
        var parts = trimmed.Split(',', 4);
        return new Idn(
            parts.Length > 0 ? parts[0].Trim() : "",
            parts.Length > 1 ? parts[1].Trim() : "",
            parts.Length > 2 ? parts[2].Trim() : "",
            parts.Length > 3 ? parts[3].Trim() : "");
    }

    public string FormatResponse() => $"{Manufacturer},{Model},{Serial},{Firmware}";
}

/// <summary>Merged device identity from all classification layers.</summary>
public sealed class DeviceIdentity
{
    public string? Manufacturer { get; set; }
    public string? Model { get; set; }
    public string? Serial { get; set; }
    public string? Firmware { get; set; }
    public string? Options { get; set; }

    public static DeviceIdentity FromIdn(Idn idn) => new()
    {
        Manufacturer = idn.Manufacturer,
        Model = idn.Model,
        Serial = idn.Serial,
        Firmware = idn.Firmware,
    };

    public void Merge(DeviceIdentity other)
    {
        Manufacturer ??= other.Manufacturer;
        Model ??= other.Model;
        Serial ??= other.Serial;
        Firmware ??= other.Firmware;
        Options ??= other.Options;
    }
}

/// <summary>Stable device identity for replacement and reconnection workflows.</summary>
public readonly record struct DeviceId(string Value)
{
    public static DeviceId FromIdentity(DeviceIdentity identity, string address)
    {
        if (!string.IsNullOrEmpty(identity.Serial) &&
            identity.Manufacturer is { } m &&
            identity.Model is { } model)
        {
            return new DeviceId($"{m}|{model}|{identity.Serial}");
        }
        return new DeviceId(address);
    }

    public override string ToString() => Value;
}
