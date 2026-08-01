using InstrumentComponents.Session;
using InstrumentComponents.Scpi;

namespace InstrumentComponents.Classes;

/// <summary>Digital multimeter session view (IVI-inspired / SCPI :MEASure).</summary>
public sealed class Dmm
{
    private readonly InstrumentSession _session;

    public Dmm(InstrumentSession session) => _session = session;

    public InstrumentSession Session => _session;

    public double MeasureVoltageDc(double? range = null) =>
        QueryF64(ScpiCommands.DmmMeasureVoltageDc(range));

    public double MeasureVoltageAc(double? range = null) =>
        QueryF64(ScpiCommands.DmmMeasureVoltageAc(range));

    public double MeasureCurrentDc(double? range = null) =>
        QueryF64(ScpiCommands.DmmMeasureCurrentDc(range));

    public double MeasureCurrentAc(double? range = null) =>
        QueryF64(ScpiCommands.DmmMeasureCurrentAc(range));

    public double MeasureResistance(double? range = null) =>
        MeasureResistance2Wire(range);

    public double MeasureResistance2Wire(double? range = null) =>
        QueryF64(ScpiCommands.DmmMeasureResistance2wire(range));

    public double MeasureResistance4Wire(double? range = null) =>
        QueryF64(ScpiCommands.DmmMeasureResistance4wire(range));

    public double MeasureTemperature(double? range = null) =>
        QueryF64(ScpiCommands.DmmMeasureTemperature(range));

    public void ConfigureVoltageDc(double? range = null, double? resolution = null) =>
        _session.Scpi.Write(ScpiCommands.DmmConfigureVoltageDc(range, resolution));

    public void ConfigureVoltageAc(double? range = null, double? resolution = null) =>
        _session.Scpi.Write(ScpiCommands.DmmConfigureVoltageAc(range, resolution));

    public void ConfigureCurrentDc(double? range = null, double? resolution = null) =>
        _session.Scpi.Write(ScpiCommands.DmmConfigureCurrentDc(range, resolution));

    public void ConfigureCurrentAc(double? range = null, double? resolution = null) =>
        _session.Scpi.Write(ScpiCommands.DmmConfigureCurrentAc(range, resolution));

    public void ConfigureResistance(double? range = null, double? resolution = null) =>
        _session.Scpi.Write(ScpiCommands.DmmConfigureResistance(range, resolution));

    public void ConfigureResistance4Wire(double? range = null, double? resolution = null) =>
        _session.Scpi.Write(ScpiCommands.DmmConfigureResistance4wire(range, resolution));

    public void Initiate() => _session.Scpi.Write(ScpiCommands.DmmInitiate);

    public double Fetch() => QueryF64(ScpiCommands.DmmFetch);

    public double Read() => QueryF64(ScpiCommands.DmmRead);

    public void SoftwareTrigger() => _session.Scpi.Write(ScpiCommands.DmmSoftwareTrigger);

    private double QueryF64(string cmd) =>
        ScpiSession.ParseF64(_session.Scpi.Query(cmd));
}
