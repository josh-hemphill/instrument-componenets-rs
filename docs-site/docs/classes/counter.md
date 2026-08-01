# Counter

Frequency, period, and totalize helpers for frequency counters / timers.

See [capability matrix](../capability-matrix.md#counter).

## Open

=== "Rust"

    ```rust
    let mut counter = catalog.open_counter(&address)?;
    ```

=== "C#"

    ```csharp
    var counter = catalog.OpenCounter(address);
    ```

## Measure

=== "Rust"

    ```rust
    let hz = counter.measure_frequency()?;
    let period = counter.measure_period()?;   // seconds
    counter.reset_totalize()?;
    let count = counter.read_totalize()?;
    ```

=== "C#"

    ```csharp
    var hz = counter.MeasureFrequency();
    var period = counter.MeasurePeriod();     // seconds
    counter.ResetTotalize();
    var count = counter.ReadTotalize();
    ```

Gate time, channel select, and timeout configuration are still todo on the matrix.
