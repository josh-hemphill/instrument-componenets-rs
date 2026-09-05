namespace InstrumentComponents.OpenTap.Tests;

public class ArchitectureTests
{
    [Fact]
    public void PackSourcesDoNotReferenceIviVisa()
    {
        var packDir = Path.Combine(RepoRoot(), "dotnet", "src", "InstrumentComponents.OpenTap");
        Assert.True(Directory.Exists(packDir), packDir);
        string[] forbidden =
        [
            "Ivi.Visa",
            "GlobalResourceManager",
            "IviFoundation.Visa",
            "HardwareTest.Core",
            "InstrumentComponents.Visa",
        ];
        foreach (var file in Directory.EnumerateFiles(packDir, "*.*", SearchOption.AllDirectories))
        {
            if (file.Contains($"{Path.DirectorySeparatorChar}bin{Path.DirectorySeparatorChar}", StringComparison.Ordinal) ||
                file.Contains($"{Path.DirectorySeparatorChar}obj{Path.DirectorySeparatorChar}", StringComparison.Ordinal))
            {
                continue;
            }

            var text = File.ReadAllText(file);
            foreach (var token in forbidden)
            {
                Assert.False(
                    text.Contains(token, StringComparison.Ordinal),
                    $"{file} contains forbidden '{token}'");
            }
        }
    }

    private static string RepoRoot()
    {
        var dir = new DirectoryInfo(AppContext.BaseDirectory);
        while (dir is not null)
        {
            if (File.Exists(Path.Combine(dir.FullName, "spec", "scpi-vectors.json")))
                return dir.FullName;
            dir = dir.Parent;
        }

        throw new DirectoryNotFoundException("could not find repo root");
    }
}
