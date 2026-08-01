# Oscilloscope

Timebase, channel scale, run/stop, and ASCII voltage trace capture.

See [capability matrix](../capability-matrix.md#oscilloscope).

## Open

=== "Rust"

    ```rust
    let mut scope = catalog.open_oscilloscope(&address)?;
    ```

=== "C#"

    ```csharp
    var scope = catalog.OpenOscilloscope(address);
    ```

## Timebase and channel

=== "Rust"

    ```rust
    scope.set_timebase_scale(1e-3)?;           // s/div
    scope.set_channel_scale(1, 0.5)?;          // V/div
    scope.run()?;
    let scale = scope.read_timebase_scale()?;
    scope.stop()?;
    ```

=== "C#"

    ```csharp
    scope.SetTimebaseScale(1e-3);              // s/div
    scope.SetChannelScale(1, 0.5);             // V/div
    scope.Run();
    var scale = scope.ReadTimebaseScale();
    scope.Stop();
    ```

## Capture

=== "Rust"

    ```rust
    let trace = scope.capture_voltage_trace(1)?;
    // trace.samples: Vec<f64> in volts
    ```

=== "C#"

    ```csharp
    var trace = scope.CaptureVoltageTrace(1);
    // trace.Samples: IReadOnlyList<double> in volts
    ```

Edge trigger, single acquire, and binary waveforms are still on the matrix roadmap.
