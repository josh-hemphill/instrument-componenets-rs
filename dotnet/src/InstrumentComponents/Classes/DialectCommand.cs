using InstrumentComponents.Dialects;
using InstrumentComponents.Scpi;

namespace InstrumentComponents.Classes;

/// <summary>
/// Resolves a dialect SCPI template, falling back when the profile cannot emit the command.
/// Extra optional vars are ignored when the template has placeholders; leftover
/// <c>{ident}</c> placeholders and constant templates that cannot represent supplied vars fall back.
/// </summary>
internal static class DialectCommand
{
    public static string Try(DialectProfile dialect, string key, string fallback, params (string Name, string Value)[] vars)
    {
        var template = dialect.Command(key);
        if (template is null)
            return fallback;
        var extras = false;
        foreach (var (name, _) in vars)
        {
            if (!template.Contains("{" + name + "}", StringComparison.Ordinal))
                extras = true;
        }
        if (extras && !HasUnreplacedPlaceholder(template))
            return fallback;
        var formatted = dialect.FormatCommand(key, vars);
        if (formatted is null || HasUnreplacedPlaceholder(formatted))
            return fallback;
        return formatted;
    }

    public static (string Name, string Value)[] RangeVars(double? range) =>
        range is { } r ? [("range", ScpiFormat.Double(r))] : [];

    public static (string Name, string Value)[] RangeResolutionVars(double? range, double? resolution)
    {
        var vars = new List<(string Name, string Value)>(2);
        if (range is { } r)
            vars.Add(("range", ScpiFormat.Double(r)));
        if (resolution is { } res)
            vars.Add(("resolution", ScpiFormat.Double(res)));
        return [.. vars];
    }

    private static bool HasUnreplacedPlaceholder(string s)
    {
        for (var i = 0; i < s.Length; i++)
        {
            if (s[i] != '{')
                continue;
            var j = i + 1;
            while (j < s.Length && (char.IsAsciiLetterOrDigit(s[j]) || s[j] == '_'))
                j++;
            if (j > i + 1 && j < s.Length && s[j] == '}')
                return true;
        }
        return false;
    }
}
