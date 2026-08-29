using System.Buffers;
using System.Text;
using InstrumentComponents.Connect;
using InstrumentComponents.Diagnostics;
using InstrumentComponents.Errors;
using InstrumentComponents.Transport;

namespace InstrumentComponents.Scpi;

/// <summary>SCPI session over a transport.</summary>
public sealed class ScpiSession : IDisposable
{
    private readonly ITransport _transport;
    private readonly ConnectOptions _opts;
    private readonly List<byte> _readBuffer = new(4096);
    private bool? _systErrSupported;
    private bool? _opcSupported;
    private CommsDiagnostics? _diagnostics;
    private string? _pendingCommand;

    public ScpiSession(ITransport transport, ConnectOptions opts)
    {
        _transport = transport;
        _opts = opts;
        transport.Configure(opts);
        if (opts.ResetOnConnect)
        {
            try { new global::InstrumentComponents.Ieee4882.Ieee4882(this).ClearStatus(); } catch { /* best-effort */ }
            try { new global::InstrumentComponents.Ieee4882.Ieee4882(this).Reset(); } catch { /* best-effort */ }
            RestoreIoTimeout();
        }
    }

    public ScpiSession WithDiagnostics(CommsDiagnostics diagnostics)
    {
        _diagnostics = diagnostics;
        return this;
    }

    public ITransport Transport => _transport;
    public ConnectOptions Options => _opts;

    /// <summary>Drains pending bytes from the transport read buffer.</summary>
    public void Flush()
    {
        var shortTimeout = TimeSpan.FromMilliseconds(50);
        _transport.SetReadTimeout(shortTimeout);
        var chunk = ArrayPool<byte>.Shared.Rent(256);
        try
        {
            while (true)
            {
                try
                {
                    var n = _transport.Read(chunk);
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
            RestoreIoTimeout();
        }
        _readBuffer.Clear();
    }

    public void Write(string command) => WriteWithRetry(command, idempotent: false);

    public string Query(string command) => QueryWithTimeout(command, EffectiveReadTimeout());

    public string QueryWithTimeout(string command, TimeSpan timeout)
    {
        WriteWithRetry(command, idempotent: true);
        var bytes = ReadResponse(timeout);
        return Encoding.UTF8.GetString(bytes).Trim();
    }

    private void WriteWithRetry(string command, bool idempotent)
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
                _transport.Write(data);
                RecordSuccess(CommsEventKind.WriteOk, command, attempts, started);
                return;
            }
            catch (InstrumentTimeoutException) when (attempts < maxAttempts)
            {
                RecordFailure(CommsEventKind.Timeout, command, attempts, started, "write timeout");
                if (_opts.ReconnectOnFailure)
                    TryReconnect();
                Thread.Sleep(_opts.RetryBackoff);
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

    private byte[] ReadResponse(TimeSpan timeout)
    {
        _transport.SetReadTimeout(timeout);
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
                    var n = _transport.Read(chunk);
                    if (n == 0)
                    {
                        Thread.Sleep(1);
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
                    catch (InstrumentTimeoutException)
                    {
                        // incomplete frame, keep reading
                    }
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
                        catch (InstrumentTimeoutException) { /* fall through */ }
                    }
                    if (_opts.ReconnectOnFailure)
                        TryReconnect();
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
            RestoreIoTimeout();
        }
    }

    private void RecordSuccess(CommsEventKind kind, string? command, uint attempt, DateTime started) =>
        _diagnostics?.RecordSuccess(kind, command, attempt, DateTime.UtcNow - started);

    private void RecordFailure(CommsEventKind kind, string? command, uint attempt, DateTime started, string detail) =>
        _diagnostics?.RecordFailure(kind, command, attempt, DateTime.UtcNow - started, detail);

    private void RecordReconnect() =>
        _diagnostics?.RecordSuccess(CommsEventKind.Reconnect, null, 1, TimeSpan.Zero);

    private void TryReconnect()
    {
        try
        {
            _transport.Reconnect();
            RecordReconnect();
        }
        catch
        {
            // Unsupported or failed reconnect must not look like success.
        }
    }

    private TimeSpan EffectiveReadTimeout() => _opts.PerOpTimeout ?? _opts.ReadTimeout;

    /// Restores the configured I/O timeout after a short probe, flush, or query.
    /// Best-effort: a restore failure must not hide the original I/O result or fail session create.
    private void RestoreIoTimeout()
    {
        try
        {
            _transport.SetReadTimeout(_opts.IoTimeout());
        }
        catch
        {
            // ignored
        }
    }

    public bool ProbeSystErr()
    {
        if (_systErrSupported is { } v) return v;
        try
        {
            _ = QueryWithTimeout("SYST:ERR?", TimeSpan.FromMilliseconds(500));
            _systErrSupported = true;
        }
        catch
        {
            _systErrSupported = false;
        }
        return _systErrSupported.Value;
    }

    public bool ProbeOpc()
    {
        if (_opcSupported is { } v) return v;
        try
        {
            QueryWithTimeout("*OPC?", TimeSpan.FromMilliseconds(500));
            _opcSupported = true;
        }
        catch
        {
            _opcSupported = false;
        }
        return _opcSupported.Value;
    }

    public IReadOnlyList<string> CheckErrors()
    {
        if (!ProbeSystErr()) return Array.Empty<string>();
        var errors = new List<string>();
        while (true)
        {
            var resp = Query("SYST:ERR?");
            if (resp.StartsWith("0,", StringComparison.Ordinal) || resp.StartsWith("+0,", StringComparison.Ordinal))
                break;
            errors.Add(resp);
            if (errors.Count > 50) break;
        }
        return errors;
    }

    public static double ParseF64(string response) => ScpiProtocol.ParseF64(response);

    public static IReadOnlyList<double> ParseF64Csv(string response) => ScpiProtocol.ParseF64Csv(response);

    public void Dispose()
    {
        if (_transport is IDisposable disposable)
            disposable.Dispose();
        GC.SuppressFinalize(this);
    }
}
