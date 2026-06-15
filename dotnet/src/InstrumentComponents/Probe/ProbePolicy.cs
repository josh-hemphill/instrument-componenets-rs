namespace InstrumentComponents.Probe;

/// <summary>How aggressively discovery probes instrument capabilities.</summary>
public enum ProbePolicy
{
    /// <summary>Registry, VISA attributes, and *IDN? only — no capability queries.</summary>
    None,
    /// <summary>Benign read-only state queries (default).</summary>
    ReadOnly,
    /// <summary>Includes acquisition-triggering probes such as :MEAS:VOLT:DC?.</summary>
    Full,
}
