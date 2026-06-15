using InstrumentComponents.Scpi;

namespace InstrumentComponents.Classifier;

internal static class CapabilityProbes
{
    internal static readonly TimeSpan ProbeTimeout = TimeSpan.FromMilliseconds(800);

    internal static readonly string[] DmmReadonlyCommands =
        [":SENS:FUNC?", "SENS:FUNC?", ":FUNC?", "FUNC?"];

    internal static readonly string[] PsuReadonlyCommands =
        [":OUTP? 1", "OUTP? 1", ":OUTP?", "OUTP?"];

    internal static readonly string[] FgenReadonlyCommands =
        [":SOUR:FUNC?", "SOUR:FUNC?"];

    internal static readonly string[] ScopeReadonlyCommands =
        [":TIMebase:SCALe?", "TIMebase:SCALe?", ":CHAN1:SCALe?", "CHAN1:SCALe?", ":WAVeform:SOURce?", "WAVeform:SOURce?"];

    internal static readonly string[] SwitchReadonlyCommands =
        [":ROUTe:CLOS?", "ROUT:CLOS?", ":ROUTe:CAT?", "ROUT:CAT?"];

    internal static readonly string[] CounterReadonlyCommands =
        [":COUNter:DATA?", "COUN:DATA?", ":COUNter:A:DATA?", "COUN:A:DATA?"];

    internal static readonly string[] DmmAcquisitionCommands =
        [":MEAS:VOLT:DC?", "MEAS:VOLT:DC?"];

    public static bool ProbeAny(ScpiSession session, string[] commands, TimeSpan timeout)
    {
        foreach (var cmd in commands)
        {
            try
            {
                session.QueryWithTimeout(cmd, timeout);
                return true;
            }
            catch
            {
                // try next spelling
            }
        }
        return false;
    }

    public static async Task<bool> ProbeAnyAsync(AsyncScpiSession session, string[] commands, TimeSpan timeout, CancellationToken cancellationToken = default)
    {
        foreach (var cmd in commands)
        {
            try
            {
                await session.QueryWithTimeoutAsync(cmd, timeout, cancellationToken).ConfigureAwait(false);
                return true;
            }
            catch
            {
                // try next spelling
            }
        }
        return false;
    }
}
