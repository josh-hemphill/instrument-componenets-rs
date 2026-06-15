using InstrumentComponents.Address;
using InstrumentComponents.Connect;
using InstrumentComponents.Transport;

namespace InstrumentComponents.Session;

/// <summary>Opens transport sessions for a device address.</summary>
public interface ISessionOpener
{
    ITransport Open(ResourceAddress address, ConnectOptions opts);
}

/// <summary>Opens async transport sessions for a device address.</summary>
public interface IAsyncSessionOpener
{
    ValueTask<IAsyncTransport> OpenAsync(ResourceAddress address, ConnectOptions opts, CancellationToken cancellationToken = default);
}
