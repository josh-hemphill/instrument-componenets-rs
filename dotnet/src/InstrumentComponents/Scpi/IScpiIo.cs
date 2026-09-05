namespace InstrumentComponents.Scpi;

/// <summary>
/// Message-level SCPI I/O (string Write/Query). Hosts inject an already-open session
/// instead of wrapping a byte <c>ITransport</c>, which would double-frame.
/// </summary>
public interface IScpiIo : IDisposable
{
    TimeSpan IoTimeout { get; set; }

    void Write(string command);

    string Query(string command);
}
