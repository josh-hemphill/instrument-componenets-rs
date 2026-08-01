# DMM

Digital multimeter session view (IVI-inspired / SCPI `:MEASure`). All numeric values use SI base units (V, A, Ω).

See [capability matrix](../capability-matrix.md#dmm) for Base vs Extension status.

## Open

=== "Rust"

    ```rust
    let mut dmm = catalog.open_dmm(&address)?;
    ```

=== "C#"

    ```csharp
    var dmm = catalog.OpenDmm(address);
    ```

## Measure

=== "Rust"

    ```rust
    let vdc = dmm.measure_voltage_dc(None)?;
    let vac = dmm.measure_voltage_ac(Some(10.0))?;
    let idc = dmm.measure_current_dc(None)?;
    let ohms = dmm.measure_resistance(None)?;
    ```

=== "C#"

    ```csharp
    var vdc = dmm.MeasureVoltageDc();
    var vac = dmm.MeasureVoltageAc(10.0);
    var idc = dmm.MeasureCurrentDc();
    var ohms = dmm.MeasureResistance();
    ```

## Configure

=== "Rust"

    ```rust
    dmm.configure_voltage_dc(Some(10.0), Some(0.001))?;
    ```

=== "C#"

    ```csharp
    dmm.ConfigureVoltageDc(range: 10.0, resolution: 0.001);
    ```

## Mock example

=== "Rust"

    --8<-- "snippets/rust/mock-dmm.md"

=== "C#"

    --8<-- "snippets/csharp/mock-dmm.md"

## Async

=== "Rust"

    `AsyncDmm` mirrors the sync methods with `.await`. See [Rust async](../rust/async.md).

=== "C#"

    `AsyncDmm` exposes `MeasureVoltageDcAsync`, etc. See [C# async](../csharp/async.md).
