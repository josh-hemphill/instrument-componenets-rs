using BenchmarkDotNet.Running;
using InstrumentComponents.Benchmarks;

BenchmarkSwitcher.FromAssembly(typeof(ScpiBenchmarks).Assembly).Run(args);
