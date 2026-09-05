using System.Buffers;
using System.Text;
using InstrumentComponents.Connect;
using InstrumentComponents.Diagnostics;
using InstrumentComponents.Errors;
using InstrumentComponents.Transport;

namespace InstrumentComponents.Scpi;

/// <summary>SCPI session over a transport, or pass-through over injected message I/O.</summary>
public sealed class ScpiSession : IScpiIo
{
    private readonly ITransport? _transport;
    private readonly IScpiIo? _injected;
    private readonly bool _ownsInjected;
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

    /// <summary>
    /// Pass-through session: Write/Query go to <paramref name="injected"/> with no extra framing.
    /// When <paramref name="ownsIo"/> is true, <see cref="Dispose"/> disposes the adapter.
    /// </summary>
    public ScpiSession(IScpiIo injected, bool ownsIo = true)
    {
        _injected = injected ?? throw new ArgumentNullException(nameof(injected));
        _ownsInjected = ownsIo;
        _opts = new ConnectOptions();
        _opts.PerOpTimeout = injected.IoTimeout;
    }

    public ScpiSession WithDiagnostics(CommsDiagnostics diagnostics)
    {
        _diagnostics = diagnostics;
        return this;
    }

    public ITransport Transport => ByteTransport;

    private ITransport ByteTransport =>
        _transport ?? throw new InstrumentUnsupportedException("injected SCPI I/O has no byte transport");

    public ConnectOptions Options => _opts;

    public bool IsInjected => _injected is not null;

    /// <summary>I/O timeout for the next Write/Query. Injected sessions forward this to the host adapter.</summary>
    public TimeSpan IoTimeout
    {
        get => _injected?.IoTimeout ?? _opts.IoTimeout();
        set
        {
            if (_injected is not null)
            {
                _injected.IoTimeout = value;
                _opts.PerOpTimeout = value;
                return;
            }

            _opts.PerOpTimeout = value;
            RestoreIoTimeout();
        }
    }

    /// <summary>Drains pending bytes from the transport read buffer.</summary>
    public void Flush()
    {
        if (_injected is not null)
            return;

        var shortTimeout = TimeSpan.FromMilliseconds(50);
        ByteTransport.SetReadTimeout(shortTimeout);
        var chunk = ArrayPool<byte>.Shared.Rent(256);
        try
        {
            while (true)
            {
                try
                {
                    var n = ByteTransport.Read(chunk);
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

    public void Write(string command)
    {
        if (_injected is not null)
        {
            WriteInjected(command);
            return;
        }

        WriteWithRetry(command, idempotent: false);
    }

    public string Query(string command) => QueryWithTimeout(command, EffectiveReadTimeout());

    public string QueryWithTimeout(string command, TimeSpan timeout)
    {
        if (_injected is not null)
            return QueryInjected(command, timeout);

        var maxAttempts = ScpiProtocol.MaxWriteAttempts(true, _opts.Retries);
        uint attempts = 0;
        while (true)
        {
            attempts++;
            WriteWithRetry(command, idempotent: true);
            try
            {
                var bytes = ReadResponse(timeout);
                return Encoding.UTF8.GetString(bytes).Trim();
            }
            catch (InstrumentTimeoutException) when (attempts < maxAttempts)
            {
                try { Flush(); } catch { /* best-effort drain */ }
                if (_opts.ReconnectOnFailure)
                    TryReconnect();
                Thread.Sleep(_opts.RetryBackoff * (int)attempts);
            }
            catch (InstrumentTimeoutException)
            {
                try { Flush(); } catch { /* best-effort drain */ }
                throw;
            }
        }
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
                ByteTransport.Write(data);
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
        ByteTransport.SetReadTimeout(timeout);
        _readBuffer.Clear();
        var command = _pendingCommand;
        var chunk = ArrayPool<byte>.Shared.Rent(1024);
        try
        {
            while (true)
            {
                var started = DateTime.UtcNow;
                int n;
                try
                {
                    n = ByteTransport.Read(chunk);
                }
                catch (InstrumentTimeoutException)
                {
                    if (TryCompleteBufferedFrame(command, started, out var timedOutPayload))
                        return timedOutPayload;
                    RecordFailure(CommsEventKind.Timeout, command, 1, started, "read timeout");
                    throw;
                }
                catch (Exception ex)
                {
                    RecordFailure(CommsEventKind.ReadFailed, command, 1, started, ex.Message);
                    throw;
                }

                if (n == 0)
                {
                    if (TryCompleteBufferedFrame(command, started, out var zeroPayload))
                        return zeroPayload;
                    RecordFailure(CommsEventKind.Timeout, command, 1, started, "zero-byte read");
                    throw new InstrumentTimeoutException();
                }

                for (var i = 0; i < n; i++)
                    _readBuffer.Add(chunk[i]);
                if (TryCompleteBufferedFrame(command, started, out var payload))
                    return payload;
            }
        }
        finally
        {
            ArrayPool<byte>.Shared.Return(chunk);
            RestoreIoTimeout();
        }
    }

    private bool TryCompleteBufferedFrame(string? command, DateTime started, out byte[] payload)
    {
        payload = [];
        if (_readBuffer.Count == 0)
            return false;
        try
        {
            (payload, _) = ScpiFraming.ExtractResponse(_readBuffer.ToArray(), _opts.Terminator);
            RecordSuccess(CommsEventKind.ReadOk, command, 1, started);
            return true;
        }
        catch (InstrumentTimeoutException)
        {
            return false;
        }
    }

    private void RecordSuccess(CommsEventKind kind, string? command, uint attempt, DateTime started) =>
        _diagnostics?.RecordSuccess(kind, command, attempt, DateTime.UtcNow - started);

    private void RecordFailure(CommsEventKind kind, string? command, uint attempt, DateTime started, string detail) =>
        _diagnostics?.RecordFailure(kind, command, attempt, DateTime.UtcNow - started, detail);

    private void RecordReconnect() =>
        _diagnostics?.RecordSuccess(CommsEventKind.Reconnect, null, 1, TimeSpan.Zero);

    private void WriteInjected(string command)
    {
        var started = DateTime.UtcNow;
        _pendingCommand = command;
        try
        {
            _injected!.Write(command);
            RecordSuccess(CommsEventKind.WriteOk, command, 1, started);
        }
        catch (InstrumentTimeoutException)
        {
            RecordFailure(CommsEventKind.Timeout, command, 1, started, "write timeout");
            throw;
        }
        catch (Exception ex)
        {
            RecordFailure(CommsEventKind.WriteFailed, command, 1, started, ex.Message);
            throw;
        }
    }

    private string QueryInjected(string command, TimeSpan timeout)
    {
        var previous = _injected!.IoTimeout;
        _injected.IoTimeout = timeout;
        var started = DateTime.UtcNow;
        _pendingCommand = command;
        try
        {
            var response = _injected.Query(command);
            RecordSuccess(CommsEventKind.ReadOk, command, 1, started);
            return response;
        }
        catch (InstrumentTimeoutException)
        {
            RecordFailure(CommsEventKind.Timeout, command, 1, started, "read timeout");
            throw;
        }
        catch (Exception ex)
        {
            RecordFailure(CommsEventKind.ReadFailed, command, 1, started, ex.Message);
            throw;
        }
        finally
        {
            try { _injected.IoTimeout = previous; } catch { /* best-effort restore */ }
        }
    }

    private void TryReconnect()
    {
        if (_transport is null)
            return;

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
        if (_transport is null)
            return;

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
            var resp = QueryWithTimeout("SYST:ERR?", TimeSpan.FromMilliseconds(500));
            _systErrSupported = ScpiProtocol.IsSystErrSupportedReply(resp);
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
            var resp = QueryWithTimeout("*OPC?", TimeSpan.FromMilliseconds(500));
            _opcSupported = ScpiProtocol.IsOpcSupportedReply(resp);
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
        if (_injected is not null)
        {
            if (_ownsInjected)
                _injected.Dispose();
            GC.SuppressFinalize(this);
            return;
        }

        if (_transport is IDisposable disposable)
            disposable.Dispose();
        GC.SuppressFinalize(this);
    }
}
