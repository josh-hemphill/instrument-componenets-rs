using InstrumentComponents.Address;
using InstrumentComponents.Identity;
using InstrumentComponents.Kind;
using InstrumentComponents.Probe;
using InstrumentComponents.Registry;
using InstrumentComponents.Scpi;
using InstrumentComponents.Transport;

namespace InstrumentComponents.Classifier;

public static class Classifier
{
    public static (DeviceIdentity Identity, List<ClassifiedKind> Kinds) ClassifyFromAddress(
        ResourceAddress address, ModelRegistry registry)
    {
        var identity = new DeviceIdentity();
        var kinds = new List<ClassifiedKind>();

        if (address.Components.Vid is { } vid && address.Components.Pid is { } pid)
        {
            if (registry.LookupUsb(vid, pid) is { } hint)
            {
                identity.Manufacturer = hint.Manufacturer;
                identity.Model = hint.Model;
                foreach (var kind in hint.Kinds)
                    kinds.Add(new ClassifiedKind(kind, 40, ClassifySource.ModelRegistry));
            }
        }

        identity.Serial ??= address.Components.Serial;

        if (kinds.Count == 0)
            kinds.Add(new ClassifiedKind(InstrumentKind.Unknown, 10, ClassifySource.ResourceParse));

        return (identity, kinds);
    }

    public static (DeviceIdentity Identity, List<ClassifiedKind> Kinds) ClassifyFromTransportHint(
        TransportIdentity hint, ModelRegistry registry)
    {
        var identity = new DeviceIdentity
        {
            Manufacturer = hint.Manufacturer,
            Model = hint.Model,
            Serial = hint.Serial,
        };
        var kinds = new List<ClassifiedKind>();

        if (hint.Manufacturer is { } m && hint.Model is { } model &&
            registry.LookupModel(m, model) is { } registryKinds)
        {
            foreach (var kind in registryKinds)
                kinds.Add(new ClassifiedKind(kind, 45, ClassifySource.ModelRegistry));
        }

        if (kinds.Count == 0 && hint.ManfId is not null)
            kinds.Add(new ClassifiedKind(InstrumentKind.Unknown, 15, ClassifySource.VisaAttributes));

        return (identity, kinds);
    }

    public static (DeviceIdentity Identity, List<ClassifiedKind> Kinds) ClassifyFromIdentity(
        Idn idn, ModelRegistry registry)
    {
        var identity = DeviceIdentity.FromIdn(idn);
        var kinds = new List<ClassifiedKind>();

        if (registry.LookupModel(idn.Manufacturer, idn.Model) is { } registryKinds)
        {
            foreach (var kind in registryKinds)
                kinds.Add(new ClassifiedKind(kind, 60, ClassifySource.ModelRegistry));
        }

        kinds.Add(new ClassifiedKind(InstrumentKind.Unknown, 30, ClassifySource.ScpiIdn));
        return (identity, kinds);
    }

    public static List<ClassifiedKind> ClassifyWithPolicy(ScpiSession session, ProbePolicy policy) =>
        policy switch
        {
            ProbePolicy.None => new List<ClassifiedKind>(),
            ProbePolicy.ReadOnly => ClassifyReadonlyProbes(session),
            ProbePolicy.Full => ClassifyReadonlyProbes(session).Concat(ClassifyAcquisitionProbes(session)).ToList(),
            _ => new List<ClassifiedKind>(),
        };

    public static async Task<List<ClassifiedKind>> ClassifyWithPolicyAsync(
        AsyncScpiSession session, ProbePolicy policy, CancellationToken cancellationToken = default) =>
        policy switch
        {
            ProbePolicy.None => new List<ClassifiedKind>(),
            ProbePolicy.ReadOnly => await ClassifyReadonlyProbesAsync(session, cancellationToken).ConfigureAwait(false),
            ProbePolicy.Full => (await ClassifyReadonlyProbesAsync(session, cancellationToken).ConfigureAwait(false))
                .Concat(await ClassifyAcquisitionProbesAsync(session, cancellationToken).ConfigureAwait(false)).ToList(),
            _ => new List<ClassifiedKind>(),
        };

    private static List<ClassifiedKind> ClassifyReadonlyProbes(ScpiSession session)
    {
        var kinds = new List<ClassifiedKind>();
        if (CapabilityProbes.ProbeAny(session, CapabilityProbes.DmmReadonlyCommands, CapabilityProbes.ProbeTimeout))
            kinds.Add(new ClassifiedKind(InstrumentKind.Dmm, 80, ClassifySource.CapabilityProbe));
        if (CapabilityProbes.ProbeAny(session, CapabilityProbes.PsuReadonlyCommands, CapabilityProbes.ProbeTimeout))
            kinds.Add(new ClassifiedKind(InstrumentKind.DcPowerSupply, 85, ClassifySource.CapabilityProbe));
        if (CapabilityProbes.ProbeAny(session, CapabilityProbes.FgenReadonlyCommands, CapabilityProbes.ProbeTimeout))
            kinds.Add(new ClassifiedKind(InstrumentKind.FunctionGenerator, 85, ClassifySource.CapabilityProbe));
        if (CapabilityProbes.ProbeAny(session, CapabilityProbes.ScopeReadonlyCommands, CapabilityProbes.ProbeTimeout))
            kinds.Add(new ClassifiedKind(InstrumentKind.Oscilloscope, 85, ClassifySource.CapabilityProbe));
        if (CapabilityProbes.ProbeAny(session, CapabilityProbes.SwitchReadonlyCommands, CapabilityProbes.ProbeTimeout))
            kinds.Add(new ClassifiedKind(InstrumentKind.Switch, 85, ClassifySource.CapabilityProbe));
        if (CapabilityProbes.ProbeAny(session, CapabilityProbes.CounterReadonlyCommands, CapabilityProbes.ProbeTimeout))
            kinds.Add(new ClassifiedKind(InstrumentKind.Counter, 85, ClassifySource.CapabilityProbe));
        return kinds;
    }

    private static List<ClassifiedKind> ClassifyAcquisitionProbes(ScpiSession session)
    {
        var kinds = new List<ClassifiedKind>();
        if (CapabilityProbes.ProbeAny(session, CapabilityProbes.DmmAcquisitionCommands, CapabilityProbes.ProbeTimeout))
            kinds.Add(new ClassifiedKind(InstrumentKind.Dmm, 90, ClassifySource.CapabilityProbe));
        return kinds;
    }

    private static async Task<List<ClassifiedKind>> ClassifyReadonlyProbesAsync(AsyncScpiSession session, CancellationToken cancellationToken)
    {
        var kinds = new List<ClassifiedKind>();
        if (await CapabilityProbes.ProbeAnyAsync(session, CapabilityProbes.DmmReadonlyCommands, CapabilityProbes.ProbeTimeout, cancellationToken).ConfigureAwait(false))
            kinds.Add(new ClassifiedKind(InstrumentKind.Dmm, 80, ClassifySource.CapabilityProbe));
        if (await CapabilityProbes.ProbeAnyAsync(session, CapabilityProbes.PsuReadonlyCommands, CapabilityProbes.ProbeTimeout, cancellationToken).ConfigureAwait(false))
            kinds.Add(new ClassifiedKind(InstrumentKind.DcPowerSupply, 85, ClassifySource.CapabilityProbe));
        if (await CapabilityProbes.ProbeAnyAsync(session, CapabilityProbes.FgenReadonlyCommands, CapabilityProbes.ProbeTimeout, cancellationToken).ConfigureAwait(false))
            kinds.Add(new ClassifiedKind(InstrumentKind.FunctionGenerator, 85, ClassifySource.CapabilityProbe));
        if (await CapabilityProbes.ProbeAnyAsync(session, CapabilityProbes.ScopeReadonlyCommands, CapabilityProbes.ProbeTimeout, cancellationToken).ConfigureAwait(false))
            kinds.Add(new ClassifiedKind(InstrumentKind.Oscilloscope, 85, ClassifySource.CapabilityProbe));
        if (await CapabilityProbes.ProbeAnyAsync(session, CapabilityProbes.SwitchReadonlyCommands, CapabilityProbes.ProbeTimeout, cancellationToken).ConfigureAwait(false))
            kinds.Add(new ClassifiedKind(InstrumentKind.Switch, 85, ClassifySource.CapabilityProbe));
        if (await CapabilityProbes.ProbeAnyAsync(session, CapabilityProbes.CounterReadonlyCommands, CapabilityProbes.ProbeTimeout, cancellationToken).ConfigureAwait(false))
            kinds.Add(new ClassifiedKind(InstrumentKind.Counter, 85, ClassifySource.CapabilityProbe));
        return kinds;
    }

    private static async Task<List<ClassifiedKind>> ClassifyAcquisitionProbesAsync(AsyncScpiSession session, CancellationToken cancellationToken)
    {
        var kinds = new List<ClassifiedKind>();
        if (await CapabilityProbes.ProbeAnyAsync(session, CapabilityProbes.DmmAcquisitionCommands, CapabilityProbes.ProbeTimeout, cancellationToken).ConfigureAwait(false))
            kinds.Add(new ClassifiedKind(InstrumentKind.Dmm, 90, ClassifySource.CapabilityProbe));
        return kinds;
    }

    public static (List<InstrumentKind> Supported, List<ClassifiedKind> All) MergeClassifications(
        IEnumerable<List<ClassifiedKind>> layers,
        IReadOnlyList<InstrumentKind>? userOverride = null)
    {
        if (userOverride is not null)
        {
            var classified = userOverride
                .Select(k => new ClassifiedKind(k, 100, ClassifySource.UserOverride))
                .ToList();
            return (userOverride.ToList(), classified);
        }

        var byKind = new Dictionary<InstrumentKind, ClassifiedKind>();
        var all = new List<ClassifiedKind>();

        foreach (var layer in layers)
        {
            foreach (var entry in layer)
            {
                all.Add(entry);
                if (byKind.TryGetValue(entry.Kind, out var existing))
                {
                    if (entry.Confidence > existing.Confidence)
                        byKind[entry.Kind] = entry;
                }
                else
                {
                    byKind[entry.Kind] = entry;
                }
            }
        }

        var supported = byKind.Values
            .Where(k => k.Kind != InstrumentKind.Unknown && k.Confidence >= 40)
            .Select(k => k.Kind)
            .OrderBy(k => k.ToString())
            .Distinct()
            .ToList();

        if (supported.Count == 0)
            supported.Add(InstrumentKind.Unknown);

        return (supported, all);
    }
}
