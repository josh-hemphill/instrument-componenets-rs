using InstrumentComponents.Dialects;
using InstrumentComponents.Identity;
using InstrumentComponents.Kind;
using InstrumentComponents.Session;
using InstrumentComponents.Scpi;

namespace InstrumentComponents.Classes;

/// <summary>Digital multimeter session view (IVI-inspired / SCPI :MEASure).</summary>
public sealed class Dmm : IInstrumentIdentity, IInstrumentShutdown
{
    private readonly InstrumentSession _session;

    public Dmm(InstrumentSession session) => _session = session;

    public InstrumentSession Session => _session;

    public Idn QueryIdn() => _session.Idn();

    public void Reset() => _session.Reset();

    /// <summary>DMMs have no output stage; safe-shutdown is a no-op.</summary>
    public void OutputOff()
    {
    }

    private DialectProfile Dialect => _session.DialectFor(InstrumentKind.Dmm);

    private string Cmd(string key, string fallback, params (string Name, string Value)[] vars) =>
        DialectCommand.Try(Dialect, key, fallback, vars);

    public double MeasureVoltageDc(double? range = null) =>
        QueryF64(Cmd("measure_voltage_dc", ScpiCommands.DmmMeasureVoltageDc(range), DialectCommand.RangeVars(range)));

    public double MeasureVoltageAc(double? range = null) =>
        QueryF64(Cmd("measure_voltage_ac", ScpiCommands.DmmMeasureVoltageAc(range), DialectCommand.RangeVars(range)));

    public double MeasureCurrentDc(double? range = null) =>
        QueryF64(Cmd("measure_current_dc", ScpiCommands.DmmMeasureCurrentDc(range), DialectCommand.RangeVars(range)));

    public double MeasureCurrentAc(double? range = null) =>
        QueryF64(Cmd("measure_current_ac", ScpiCommands.DmmMeasureCurrentAc(range), DialectCommand.RangeVars(range)));

    public double MeasureResistance(double? range = null) =>
        MeasureResistance2Wire(range);

    public double MeasureResistance2Wire(double? range = null) =>
        QueryF64(Cmd("measure_resistance_2w", ScpiCommands.DmmMeasureResistance2wire(range), DialectCommand.RangeVars(range)));

    public double MeasureResistance4Wire(double? range = null) =>
        QueryF64(Cmd("measure_resistance_4w", ScpiCommands.DmmMeasureResistance4wire(range), DialectCommand.RangeVars(range)));

    public double MeasureTemperature(double? range = null) =>
        QueryF64(Cmd("measure_temperature", ScpiCommands.DmmMeasureTemperature(range), DialectCommand.RangeVars(range)));

    public void ConfigureVoltageDc(double? range = null, double? resolution = null) =>
        _session.Scpi.Write(Cmd("configure_voltage_dc", ScpiCommands.DmmConfigureVoltageDc(range, resolution), DialectCommand.RangeResolutionVars(range, resolution)));

    public void ConfigureVoltageAc(double? range = null, double? resolution = null) =>
        _session.Scpi.Write(Cmd("configure_voltage_ac", ScpiCommands.DmmConfigureVoltageAc(range, resolution), DialectCommand.RangeResolutionVars(range, resolution)));

    public void ConfigureCurrentDc(double? range = null, double? resolution = null) =>
        _session.Scpi.Write(Cmd("configure_current_dc", ScpiCommands.DmmConfigureCurrentDc(range, resolution), DialectCommand.RangeResolutionVars(range, resolution)));

    public void ConfigureCurrentAc(double? range = null, double? resolution = null) =>
        _session.Scpi.Write(Cmd("configure_current_ac", ScpiCommands.DmmConfigureCurrentAc(range, resolution), DialectCommand.RangeResolutionVars(range, resolution)));

    public void ConfigureResistance(double? range = null, double? resolution = null) =>
        _session.Scpi.Write(Cmd("configure_resistance", ScpiCommands.DmmConfigureResistance(range, resolution), DialectCommand.RangeResolutionVars(range, resolution)));

    public void ConfigureResistance4Wire(double? range = null, double? resolution = null) =>
        _session.Scpi.Write(Cmd("configure_resistance_4w", ScpiCommands.DmmConfigureResistance4wire(range, resolution), DialectCommand.RangeResolutionVars(range, resolution)));

    public void Initiate() => _session.Scpi.Write(Cmd("initiate", ScpiCommands.DmmInitiate));

    public double Fetch() => QueryF64(Cmd("fetch", ScpiCommands.DmmFetch));

    public double Read() => QueryF64(Cmd("read", ScpiCommands.DmmRead));

    public void SoftwareTrigger() => _session.Scpi.Write(Cmd("software_trigger", ScpiCommands.DmmSoftwareTrigger));

    private double QueryF64(string cmd) =>
        ScpiSession.ParseF64(_session.Scpi.Query(cmd));
}
