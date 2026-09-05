namespace InstrumentComponents.Identity;

/// <summary>Query instrument identity (*IDN?).</summary>
public interface IInstrumentIdentity
{
    Idn QueryIdn();
}
