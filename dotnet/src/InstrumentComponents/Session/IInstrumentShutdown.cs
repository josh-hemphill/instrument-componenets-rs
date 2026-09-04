namespace InstrumentComponents.Session;

/// <summary>Return an instrument to a safe idle, then IEEE 488.2 reset.</summary>
public interface IInstrumentShutdown
{
    void OutputOff();

    void Reset();
}
