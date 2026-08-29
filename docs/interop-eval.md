# FFI generator eval (UniFFI vs Interoptopus)

Agent notes. The runnable Interoptopus spike lives in [`spikes/interoptopus-eval/`](../spikes/interoptopus-eval/README.md).

UniFFI 0.31 + `uniffi-bindgen-cs` proved the hard FFI cases (async `Task<T>`, C#-implemented async transport, sync observer, nested exceptions) but missed `CancellationToken`, borrowed slices, and strong exception messages. Verdict: viable bindings, not a stand-in for `dotnet/`.

Interoptopus **0.16.4** (C# tier-1) generates `IDisposable` classes with PascalCase methods, `Task` + `CancellationToken` (token cancels the Rust future), C# lambdas for callbacks, and borrowed `byte[]` slices. Errors are typed (`EnumException<Error>`) but `Exception.Message` is a generic variant-mismatch string. Reverse interop exists (`plugin!`) and is **unstable**.

**Decision:** keep dual native Rust + C#. Interoptopus is the better generated-C# path if we ever add a Rust-sourced SKU; it is not a replacement for the hand-written port today.
