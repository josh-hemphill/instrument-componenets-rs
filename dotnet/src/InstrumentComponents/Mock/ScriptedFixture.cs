using InstrumentComponents.Address;
using InstrumentComponents.Identity;
using InstrumentComponents.Kind;
using InstrumentComponents.Transport;

namespace InstrumentComponents.Mock;

/// <summary>High-level fixture builder for common instrument patterns.</summary>
public sealed class ScriptedFixture
{
    public Idn Idn { get; }
    public IReadOnlyList<InstrumentKind> Kinds { get; }
    private readonly List<KeyValuePair<string, string>> _queryHandlers;
    private readonly List<string> _writeHandlers;

    private ScriptedFixture(Idn idn, IReadOnlyList<InstrumentKind> kinds,
        List<KeyValuePair<string, string>> queryHandlers, List<string> writeHandlers)
    {
        Idn = idn;
        Kinds = kinds;
        _queryHandlers = queryHandlers;
        _writeHandlers = writeHandlers;
    }

    public static ScriptedFixtureBuilder Builder() => new();

    public MockTransport IntoTransport()
    {
        var steps = new List<ScriptStep>();
        foreach (var cmd in _writeHandlers)
        {
            var data = cmd.EndsWith('\n') ? cmd : cmd + "\n";
            steps.Add(new WriteStep { Data = data });
        }
        foreach (var (query, response) in _queryHandlers)
        {
            steps.Add(new WriteStep { Data = query + "\n" });
            var resp = response.EndsWith('\n') ? response : response + "\n";
            steps.Add(new ReadStep { Data = resp });
        }
        var identity = new TransportIdentity
        {
            Manufacturer = Idn.Manufacturer,
            Model = Idn.Model,
            Serial = Idn.Serial,
            Interface = InterfaceKind.Unknown,
        };
        return new MockTransport(steps).WithIdentity(identity);
    }

    public sealed class ScriptedFixtureBuilder
    {
        private Idn _idn = new("", "", "", "");
        private readonly List<InstrumentKind> _kinds = new();
        private readonly List<KeyValuePair<string, string>> _queryHandlers = new();
        private readonly List<string> _writeHandlers = new();

        public ScriptedFixtureBuilder Idn(string manufacturer, string model, string serial, string firmware)
        {
            _idn = new Idn(manufacturer, model, serial, firmware);
            return this;
        }

        public ScriptedFixtureBuilder Kinds(params InstrumentKind[] kinds)
        {
            _kinds.Clear();
            _kinds.AddRange(kinds);
            return this;
        }

        public ScriptedFixtureBuilder OnQuery(string query, string response)
        {
            _queryHandlers.Add(new KeyValuePair<string, string>(query, response));
            return this;
        }

        public ScriptedFixtureBuilder OnWrite(string command)
        {
            _writeHandlers.Add(command);
            return this;
        }

        public ScriptedFixtureBuilder WithIdnProbe()
        {
            _queryHandlers.Add(new KeyValuePair<string, string>("*IDN?", _idn.FormatResponse()));
            return this;
        }

        public ScriptedFixture Build() =>
            new(_idn, _kinds, _queryHandlers, _writeHandlers);
    }
}

public static class MockAddress
{
    public static ResourceAddress Parse(string name) => ResourceAddress.Parse($"mock://{name}");
}
