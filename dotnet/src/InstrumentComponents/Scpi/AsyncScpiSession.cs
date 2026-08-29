using System.Buffers;
using System.Text;
using InstrumentComponents.Connect;
using InstrumentComponents.Diagnostics;
using InstrumentComponents.Errors;
using InstrumentComponents.Transport;

namespace InstrumentComponents.Scpi;

/// <summary>Async SCPI session over a transport.</summary>
public sealed class AsyncScpiSession : IDisposable
{
    private readonly IAsyncTransport _transport;
    private readonly ConnectOptions _opts;
    private readonly List<byte> _readBuffer = new(4096);
    private bool? _systErrSupported;
    private bool? _opcSupported;
    private CommsDiagnostics? _diagnostics;
    private string? _pendingCommand;

    public static async Task<AsyncScpiSession> CreateAsync(
        IAsyncTransport transport,
        ConnectOptions opts,
        CancellationToken cancellationToken = default)
    {
        await transport.ConfigureAsync(opts, cancellationToken).ConfigureAwait(false);
        var session = new AsyncScpiSession(transport, opts);
        if (opts.ResetOnConnect)
        {
            try { await new global::InstrumentComponents.Ieee4882.AsyncIeee4882(session).ClearStatusAsync(cancellationToken).ConfigureAwait(false); } catch { }
            try { await new global::InstrumentComponents.Ieee4882.AsyncIeee4882(session).ResetAsync(cancellationToken).ConfigureAwait(false); } catch { }
            await session.RestoreIoTimeoutAsync(cancellationToken).ConfigureAwait(false);
        }
        return session;
    }

    private AsyncScpiSession(IAsyncTransport transport, ConnectOptions opts)
    {
        _transport = transport;
        _opts = opts;
    }

    public AsyncScpiSession WithDiagnostics(CommsDiagnostics diagnostics)
    {
        _diagnostics = diagnostics;
        return this;
    }

    public IAsyncTransport Transport => _transport;
    public ConnectOptions Options => _opts;

    public async Task FlushAsync(CancellationToken cancellationToken = default)
    {
        await _transport.SetReadTimeoutAsync(TimeSpan.FromMilliseconds(50), cancellationToken).ConfigureAwait(false);
        var chunk = ArrayPool<byte>.Shared.Rent(256);
        try
        {
            while (true)
            {
                try
                {
                    var n = await _transport.ReadAsync(chunk, cancellationToken).ConfigureAwait(false);
                    if (n == 0) break;
                }
                catch (InstrumentTimeoutException)
                {
                    break;
                }
            }
        }
        finally
        {
            ArrayPool<byte>.Shared.Return(chunk);
            await RestoreIoTimeoutAsync(cancellationToken).ConfigureAwait(false);
        }
        _readBuffer.Clear();
    }

    public Task WriteAsync(string command, CancellationToken cancellationToken = default) =>
        WriteWithRetryAsync(command, idempotent: false, cancellationToken);

    public Task<string> QueryAsync(string command, CancellationToken cancellationToken = default) =>
        QueryWithTimeoutAsync(command, EffectiveReadTimeout(), cancellationToken);

    public async Task<string> QueryWithTimeoutAsync(string command, TimeSpan timeout, CancellationToken cancellationToken = default)
    {
        await WriteWithRetryAsync(command, idempotent: true, cancellationToken).ConfigureAwait(false);
        var bytes = await ReadResponseAsync(timeout, cancellationToken).ConfigureAwait(false);
        return Encoding.UTF8.GetString(bytes).Trim();
    }

    private async Task WriteWithRetryAsync(string command, bool idempotent, CancellationToken cancellationToken)
    {
        var payload = ScpiProtocol.NormalizeCommand(command, _opts.Terminator);
        var data = Encoding.UTF8.GetBytes(payload);
        var attempts = 0u;
        var maxAttempts = ScpiProtocol.MaxWriteAttempts(idempotent, _opts.Retries);

        while (true)
        {
            attempts++;
            var started = DateTime.UtcNow;
            _pendingCommand = command;
            try
            {
                await _transport.WriteAsync(data, cancellationToken).ConfigureAwait(false);
                RecordSuccess(CommsEventKind.WriteOk, command, attempts, started);
                return;
            }
            catch (InstrumentTimeoutException) when (attempts < maxAttempts)
            {
                RecordFailure(CommsEventKind.Timeout, command, attempts, started, "write timeout");
                if (_opts.ReconnectOnFailure)
                    await TryReconnectAsync(cancellationToken).ConfigureAwait(false);
                await Task.Delay(_opts.RetryBackoff, cancellationToken).ConfigureAwait(false);
            }
            catch (InstrumentTimeoutException)
            {
                RecordFailure(CommsEventKind.Timeout, command, attempts, started, "write timeout");
                throw;
            }
            catch (Exception ex)
            {
                RecordFailure(CommsEventKind.WriteFailed, command, attempts, started, ex.Message);
                throw;
            }
        }
    }

    private async Task<byte[]> ReadResponseAsync(TimeSpan timeout, CancellationToken cancellationToken)
    {
        await _transport.SetReadTimeoutAsync(timeout, cancellationToken).ConfigureAwait(false);
        _readBuffer.Clear();
        var command = _pendingCommand;
        var chunk = ArrayPool<byte>.Shared.Rent(1024);
        try
        {
            while (true)
            {
                var started = DateTime.UtcNow;
                try
                {
                    var n = await _transport.ReadAsync(chunk, cancellationToken).ConfigureAwait(false);
                    if (n == 0)
                    {
                        await Task.Delay(1, cancellationToken).ConfigureAwait(false);
                        continue;
                    }
                    for (var i = 0; i < n; i++)
                        _readBuffer.Add(chunk[i]);
                    try
                    {
                        var (payload, _) = ScpiFraming.ExtractResponse(_readBuffer.ToArray(), _opts.Terminator);
                        RecordSuccess(CommsEventKind.ReadOk, command, 1, started);
                        return payload;
                    }
                    catch (InstrumentTimeoutException) { }
                }
                catch (InstrumentTimeoutException)
                {
                    if (_readBuffer.Count > 0)
                    {
                        try
                        {
                            var (payload, _) = ScpiFraming.ExtractResponse(_readBuffer.ToArray(), _opts.Terminator);
                            RecordSuccess(CommsEventKind.ReadOk, command, 1, started);
                            return payload;
                        }
                        catch (InstrumentTimeoutException) { }
                    }
                    if (_opts.ReconnectOnFailure)
                        await TryReconnectAsync(cancellationToken).ConfigureAwait(false);
                    RecordFailure(CommsEventKind.Timeout, command, 1, started, "read timeout");
                    throw;
                }
                catch (Exception ex)
                {
                    RecordFailure(CommsEventKind.ReadFailed, command, 1, started, ex.Message);
                    throw;
                }
            }
        }
        finally
        {
            ArrayPool<byte>.Shared.Return(chunk);
            await RestoreIoTimeoutAsync(cancellationToken).ConfigureAwait(false);
        }
    }

    private void RecordSuccess(CommsEventKind kind, string? command, uint attempt, DateTime started) =>
        _diagnostics?.RecordSuccess(kind, command, attempt, DateTime.UtcNow - started);

    private void RecordFailure(CommsEventKind kind, string? command, uint attempt, DateTime started, string detail) =>
        _diagnostics?.RecordFailure(kind, command, attempt, DateTime.UtcNow - started, detail);

    private void RecordReconnect() =>
        _diagnostics?.RecordSuccess(CommsEventKind.Reconnect, null, 1, TimeSpan.Zero);

    private async Task TryReconnectAsync(CancellationToken cancellationToken)
    {
        try
        {
            await _transport.ReconnectAsync(cancellationToken).ConfigureAwait(false);
            RecordReconnect();
        }
        catch
        {
            // Unsupported or failed reconnect must not look like success.
        }
    }

    private TimeSpan EffectiveReadTimeout() => _opts.PerOpTimeout ?? _opts.ReadTimeout;

    /// Restores the configured I/O timeout after a short probe, flush, or query.
    private ValueTask RestoreIoTimeoutAsync(CancellationToken cancellationToken) =>
        _transport.SetReadTimeoutAsync(_opts.IoTimeout(), cancellationToken);

    public async Task<bool> ProbeSystErrAsync(CancellationToken cancellationToken = default)
    {
        if (_systErrSupported is { } v) return v;
        try
        {
            await QueryWithTimeoutAsync("SYST:ERR?", TimeSpan.FromMilliseconds(500), cancellationToken).ConfigureAwait(false);
            _systErrSupported = true;
        }
        catch
        {
            _systErrSupported = false;
        }
        return _systErrSupported.Value;
    }

    public async Task<bool> ProbeOpcAsync(CancellationToken cancellationToken = default)
    {
        if (_opcSupported is { } v) return v;
        try
        {
            await QueryWithTimeoutAsync("*OPC?", TimeSpan.FromMilliseconds(500), cancellationToken).ConfigureAwait(false);
            _opcSupported = true;
        }
        catch
        {
            _opcSupported = false;
        }
        return _opcSupported.Value;
    }

    public async Task<IReadOnlyList<string>> CheckErrorsAsync(CancellationToken cancellationToken = default)
    {
        if (!await ProbeSystErrAsync(cancellationToken).ConfigureAwait(false))
            return Array.Empty<string>();
        var errors = new List<string>();
        while (true)
        {
            var resp = await QueryAsync("SYST:ERR?", cancellationToken).ConfigureAwait(false);
            if (resp.StartsWith("0,", StringComparison.Ordinal) || resp.StartsWith("+0,", StringComparison.Ordinal))
                break;
            errors.Add(resp);
            if (errors.Count > 50) break;
        }
        return errors;
    }

    public void Dispose()
    {
        if (_transport is IDisposable disposable)
            disposable.Dispose();
        GC.SuppressFinalize(this);
    }
}
