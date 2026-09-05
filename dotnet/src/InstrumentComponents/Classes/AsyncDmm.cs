using InstrumentComponents.Dialects;
using InstrumentComponents.Identity;
using InstrumentComponents.Kind;
using InstrumentComponents.Session;
using InstrumentComponents.Scpi;

namespace InstrumentComponents.Classes;

public sealed class AsyncDmm
{
    private readonly AsyncInstrumentSession _session;

    public AsyncDmm(AsyncInstrumentSession session) => _session = session;

    public AsyncInstrumentSession Session => _session;

    public Task<Idn> QueryIdnAsync(CancellationToken cancellationToken = default) =>
        _session.IdnAsync(cancellationToken);

    public Task ResetAsync(CancellationToken cancellationToken = default) =>
        _session.ResetAsync(cancellationToken);

    public Task OutputOffAsync(CancellationToken cancellationToken = default)
    {
        cancellationToken.ThrowIfCancellationRequested();
        return Task.CompletedTask;
    }

    private DialectProfile Dialect => _session.DialectFor(InstrumentKind.Dmm);

    private string Cmd(string key, string fallback, params (string Name, string Value)[] vars) =>
        DialectCommand.Try(Dialect, key, fallback, vars);

    public Task<double> MeasureVoltageDcAsync(double? range = null, CancellationToken cancellationToken = default) =>
        QueryF64Async(Cmd("measure_voltage_dc", ScpiCommands.DmmMeasureVoltageDc(range), DialectCommand.RangeVars(range)), cancellationToken);

    public Task<double> MeasureVoltageAcAsync(double? range = null, CancellationToken cancellationToken = default) =>
        QueryF64Async(Cmd("measure_voltage_ac", ScpiCommands.DmmMeasureVoltageAc(range), DialectCommand.RangeVars(range)), cancellationToken);

    public Task<double> MeasureCurrentDcAsync(double? range = null, CancellationToken cancellationToken = default) =>
        QueryF64Async(Cmd("measure_current_dc", ScpiCommands.DmmMeasureCurrentDc(range), DialectCommand.RangeVars(range)), cancellationToken);

    public Task<double> MeasureCurrentAcAsync(double? range = null, CancellationToken cancellationToken = default) =>
        QueryF64Async(Cmd("measure_current_ac", ScpiCommands.DmmMeasureCurrentAc(range), DialectCommand.RangeVars(range)), cancellationToken);

    public Task<double> MeasureResistanceAsync(double? range = null, CancellationToken cancellationToken = default) =>
        MeasureResistance2WireAsync(range, cancellationToken);

    public Task<double> MeasureResistance2WireAsync(double? range = null, CancellationToken cancellationToken = default) =>
        QueryF64Async(Cmd("measure_resistance_2w", ScpiCommands.DmmMeasureResistance2wire(range), DialectCommand.RangeVars(range)), cancellationToken);

    public Task<double> MeasureResistance4WireAsync(double? range = null, CancellationToken cancellationToken = default) =>
        QueryF64Async(Cmd("measure_resistance_4w", ScpiCommands.DmmMeasureResistance4wire(range), DialectCommand.RangeVars(range)), cancellationToken);

    public Task<double> MeasureTemperatureAsync(double? range = null, CancellationToken cancellationToken = default) =>
        QueryF64Async(Cmd("measure_temperature", ScpiCommands.DmmMeasureTemperature(range), DialectCommand.RangeVars(range)), cancellationToken);

    public Task ConfigureVoltageDcAsync(double? range = null, double? resolution = null, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(Cmd("configure_voltage_dc", ScpiCommands.DmmConfigureVoltageDc(range, resolution), DialectCommand.RangeResolutionVars(range, resolution)), cancellationToken);

    public Task ConfigureVoltageAcAsync(double? range = null, double? resolution = null, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(Cmd("configure_voltage_ac", ScpiCommands.DmmConfigureVoltageAc(range, resolution), DialectCommand.RangeResolutionVars(range, resolution)), cancellationToken);

    public Task ConfigureCurrentDcAsync(double? range = null, double? resolution = null, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(Cmd("configure_current_dc", ScpiCommands.DmmConfigureCurrentDc(range, resolution), DialectCommand.RangeResolutionVars(range, resolution)), cancellationToken);

    public Task ConfigureCurrentAcAsync(double? range = null, double? resolution = null, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(Cmd("configure_current_ac", ScpiCommands.DmmConfigureCurrentAc(range, resolution), DialectCommand.RangeResolutionVars(range, resolution)), cancellationToken);

    public Task ConfigureResistanceAsync(double? range = null, double? resolution = null, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(Cmd("configure_resistance", ScpiCommands.DmmConfigureResistance(range, resolution), DialectCommand.RangeResolutionVars(range, resolution)), cancellationToken);

    public Task ConfigureResistance4WireAsync(double? range = null, double? resolution = null, CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(Cmd("configure_resistance_4w", ScpiCommands.DmmConfigureResistance4wire(range, resolution), DialectCommand.RangeResolutionVars(range, resolution)), cancellationToken);

    public Task InitiateAsync(CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(Cmd("initiate", ScpiCommands.DmmInitiate), cancellationToken);

    public Task<double> FetchAsync(CancellationToken cancellationToken = default) =>
        QueryF64Async(Cmd("fetch", ScpiCommands.DmmFetch), cancellationToken);

    public Task<double> ReadAsync(CancellationToken cancellationToken = default) =>
        QueryF64Async(Cmd("read", ScpiCommands.DmmRead), cancellationToken);

    public Task SoftwareTriggerAsync(CancellationToken cancellationToken = default) =>
        _session.Scpi.WriteAsync(Cmd("software_trigger", ScpiCommands.DmmSoftwareTrigger), cancellationToken);

    private async Task<double> QueryF64Async(string cmd, CancellationToken cancellationToken)
    {
        var resp = await _session.Scpi.QueryAsync(cmd, cancellationToken).ConfigureAwait(false);
        return ScpiSession.ParseF64(resp);
    }
}
