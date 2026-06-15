using InstrumentComponents.Address;
using InstrumentComponents.Connect;
using InstrumentComponents.Errors;
using InstrumentComponents.Session;
using InstrumentComponents.Transport;

namespace InstrumentComponents.Mock;

public sealed class MockSessionOpener : ISessionOpener, IAsyncSessionOpener
{
    private readonly Dictionary<string, MockTransport> _templates = new();

    public void Register(string address, MockTransport transport) => _templates[address] = transport;

    public ITransport Open(ResourceAddress address, ConnectOptions opts)
    {
        if (!_templates.TryGetValue(address.Raw, out var template))
            throw new DeviceNotFoundException(address.Raw);
        return template.Reopen();
    }

    public ValueTask<IAsyncTransport> OpenAsync(ResourceAddress address, ConnectOptions opts, CancellationToken cancellationToken = default)
    {
        if (!_templates.TryGetValue(address.Raw, out var template))
            throw new DeviceNotFoundException(address.Raw);
        return ValueTask.FromResult<IAsyncTransport>(template.Reopen());
    }
}
