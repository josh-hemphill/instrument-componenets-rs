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

    public double MeasureResistance(double? range = null) =>
        QueryF64(ScpiCommands.DmmMeasureResistance(range));

    public void ConfigureVoltageDc(double? range = null, double? resolution = null) =>
        _session.Scpi.Write(ScpiCommands.DmmConfigureVoltageDc(range, resolution));

    private double QueryF64(string cmd) =>
        ScpiSession.ParseF64(_session.Scpi.Query(cmd));
}
