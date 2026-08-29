using InstrumentComponents.Address;
using InstrumentComponents.Connect;
using InstrumentComponents.Diagnostics;
using InstrumentComponents.Errors;
using InstrumentComponents.Identity;
using InstrumentComponents.Ieee4882;
using InstrumentComponents.Kind;
using InstrumentComponents.Scpi;
using InstrumentComponents.Transport;

namespace InstrumentComponents.Session;

/// <summary>Active instrument session with SCPI and cached identity.</summary>
public sealed class InstrumentSession : IDisposable
{
    public ResourceAddress Address { get; }
    public ScpiSession Scpi { get; }
    private readonly DeviceIdentity _identity;

    public InstrumentSession(
        ResourceAddress address,
        ITransport transport,
        ConnectOptions opts,
        DeviceIdentity identity,
        CommsDiagnostics? diagnostics = null)
    {
        Address = address;
        _identity = identity;
        var scpi = new ScpiSession(transport, opts);
        if (diagnostics is not null)
            scpi = scpi.WithDiagnostics(diagnostics!);
        Scpi = scpi;
    }

    public string AddressStr => Address.Raw;
    public DeviceIdentity Identity => _identity;

    public Idn Idn() => ScpiComm("idn", scpi => new global::InstrumentComponents.Ieee4882.Ieee4882(scpi).Idn());

    public void Reset() => ScpiComm("*RST", scpi => { new global::InstrumentComponents.Ieee4882.Ieee4882(scpi).Reset(); return true; });

    public void ClearStatus() => ScpiComm("*CLS", scpi => { new global::InstrumentComponents.Ieee4882.Ieee4882(scpi).ClearStatus(); return true; });

    public void WaitComplete() => ScpiComm("*OPC?", scpi => { new global::InstrumentComponents.Ieee4882.Ieee4882(scpi).WaitComplete(); return true; });

    public IReadOnlyList<string> CheckErrors() => ScpiComm("SYST:ERR?", scpi => scpi.CheckErrors());

    private T ScpiComm<T>(string command, Func<ScpiSession, T> action)
    {
        try
        {
            return action(Scpi);
        }
        catch (InstrumentException ex) when (ex is not CommunicationException)
        {
            throw new CommunicationException(Address.Raw, command, 1, ex);
        }
    }

    public void Dispose() => Scpi.Dispose();
}

/// <summary>Reuses a single underlying session across typed views.</summary>
public sealed class SessionPool
{
    private readonly InstrumentSession _session;
    private readonly object _lock = new();

    public SessionPool(InstrumentSession session) => _session = session;

    public InstrumentSession Lock() { lock (_lock) return _session; }
}

public static class SessionHelpers
{
    public static void EnsureKindSupported(ResourceAddress address, InstrumentKind kind, IReadOnlyList<InstrumentKind> supported)
    {
        if (supported.Contains(kind)) return;
        throw new UnsupportedKindException(address.Raw, kind, supported);
    }
}
