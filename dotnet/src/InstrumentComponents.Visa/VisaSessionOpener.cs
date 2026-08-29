using InstrumentComponents.Address;
using InstrumentComponents.Connect;
using InstrumentComponents.Errors;
using InstrumentComponents.Session;
using InstrumentComponents.Transport;
using Ivi.Visa;

namespace InstrumentComponents.Visa;

/// <summary>Opens VISA sessions for InstrumentComponents.</summary>
public sealed class VisaSessionOpener : ISessionOpener, IAsyncSessionOpener
{
    public ITransport Open(ResourceAddress address, ConnectOptions opts)
    {
        var accessMode = MapAccessMode(opts.AccessMode);
        try
        {
            var session = (IMessageBasedSession)GlobalResourceManager.Open(
                address.Raw,
                accessMode,
                (int)opts.OpenTimeout.TotalMilliseconds);
            return new VisaTransport(session);
        }
        catch (Exception ex) when (ex.Message.Contains("session", StringComparison.OrdinalIgnoreCase) ||
                                   ex.Message.Contains("limit", StringComparison.OrdinalIgnoreCase))
        {
            throw new SessionLimitException(address.Raw);
        }
        catch (Exception ex)
        {
            throw new TransportException(ex.Message);
        }
    }

    public ValueTask<IAsyncTransport> OpenAsync(ResourceAddress address, ConnectOptions opts, CancellationToken cancellationToken = default)
    {
        cancellationToken.ThrowIfCancellationRequested();
        var transport = new VisaAsyncTransport((VisaTransport)Open(address, opts));
        return ValueTask.FromResult<IAsyncTransport>(transport);
    }

    internal static AccessModes MapAccessMode(AccessMode mode)
    {
        if (mode.SharedLock)
            throw new InstrumentUnsupportedException(
                "Ivi.Visa AccessModes does not support shared lock");
        return mode.ExclusiveLock ? AccessModes.ExclusiveLock : AccessModes.None;
    }
}
