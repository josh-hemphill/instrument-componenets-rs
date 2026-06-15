namespace InstrumentComponents.Classes;

internal static class ScpiCommands
{
    public static string DmmMeasureVoltageDc(double? range) =>
        range is { } r ? $":MEAS:VOLT:DC? {r}" : ":MEAS:VOLT:DC?";

    public static string DmmMeasureVoltageAc(double? range) =>
        range is { } r ? $":MEAS:VOLT:AC? {r}" : ":MEAS:VOLT:AC?";

    public static string DmmMeasureCurrentDc(double? range) =>
        range is { } r ? $":MEAS:CURR:DC? {r}" : ":MEAS:CURR:DC?";

    public static string DmmMeasureResistance(double? range) =>
        range is { } r ? $":MEAS:RES? {r}" : ":MEAS:RES?";

    public static string DmmConfigureVoltageDc(double? range, double? resolution) => (range, resolution) switch
    {
        ({ } r, { } res) => $":CONF:VOLT:DC {r},{res}",
        ({ } r, null) => $":CONF:VOLT:DC {r}",
        (null, { } res) => $":CONF:VOLT:DC DEF,{res}",
        _ => ":CONF:VOLT:DC",
    };

    public static string PsuSetVoltage(uint channel, double volts) => $":SOUR{channel}:VOLT {volts}";
    public static string PsuSetCurrentLimit(uint channel, double amps) => $":SOUR{channel}:CURR {amps}";
    public static string PsuOutputEnable(uint channel, bool enabled) => $":OUTP{channel} {(enabled ? "ON" : "OFF")}";
    public static string PsuReadVoltage(uint channel) => $":MEAS:VOLT? {channel}";
    public static string PsuReadCurrent(uint channel) => $":MEAS:CURR? {channel}";

    public static string FgenSetWaveform(string scpiName) => $":SOUR:FUNC {scpiName}";
    public static string FgenSetFrequency(double hz) => $":SOUR:FREQ {hz}";
    public static string FgenSetAmplitude(double vpp) => $":SOUR:VOLT {vpp}";
    public static string FgenSetOffset(double volts) => $":SOUR:VOLT:OFFS {volts}";
    public static string FgenOutputEnable(bool enabled) => $":OUTP {(enabled ? "ON" : "OFF")}";
    public const string FgenReadFrequency = ":SOUR:FREQ?";

    public static string ScopeSetTimebaseScale(double secondsPerDiv) => $":TIMebase:SCALe {secondsPerDiv}";
    public const string ScopeReadTimebaseScale = ":TIMebase:SCALe?";
    public static string ScopeSetChannelScale(uint channel, double voltsPerDiv) =>
        $":CHANnel{channel}:SCALe {voltsPerDiv}";
    public const string ScopeRun = ":RUN";
    public const string ScopeStop = ":STOP";
    public static string ScopeSetWaveformSource(uint channel) => $":WAVeform:SOURce CHAN{channel}";
    public const string ScopeWaveformFormatAscii = ":WAVeform:FORMat ASCii";
    public const string ScopeWaveformPreamble = ":WAVeform:PREamble?";
    public const string ScopeWaveformData = ":WAVeform:DATA?";

    public static string SwitchCloseRoute(uint ch1, uint ch2) => $":ROUTe:CLOS (@({ch1},{ch2}))";
    public static string SwitchOpenRoute(uint ch1, uint ch2) => $":ROUTe:OPEN (@({ch1},{ch2}))";
    public static string SwitchIsClosed(uint ch1, uint ch2) => $":ROUTe:CLOS? (@({ch1},{ch2}))";
    public const string SwitchOpenAll = ":ROUTe:OPEN:ALL";

    public const string CounterMeasureFrequency = ":MEASure:FREQuency?";
    public const string CounterMeasurePeriod = ":MEASure:PERiod?";
    public const string CounterResetTotalize = ":COUNter:CRESet";
    public const string CounterReadTotalize = ":COUNter:DATA?";
}
