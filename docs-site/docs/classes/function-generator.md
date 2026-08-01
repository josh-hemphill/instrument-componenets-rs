# Function generator

Standard waveform, frequency, amplitude, offset, and output enable.

See [capability matrix](../capability-matrix.md#functiongenerator).

## Open

=== "Rust"

    ```rust
    let mut fgen = catalog.open_function_generator(&address)?;
    ```

=== "C#"

    ```csharp
    var fgen = catalog.OpenFunctionGenerator(address);
    ```

## Configure output

=== "Rust"

    ```rust
    fgen.set_waveform(Waveform::Sine)?;
    fgen.set_frequency(1_000.0)?;      // Hz
    fgen.set_amplitude(2.0)?;          // Vpp
    fgen.set_offset(0.0)?;             // V
    fgen.output_enable(true)?;
    let hz = fgen.read_frequency()?;
    ```

=== "C#"

    ```csharp
    fgen.SetWaveform(Waveform.Sine);
    fgen.SetFrequency(1000.0);   // Hz
    fgen.SetAmplitude(2.0);      // Vpp
    fgen.SetOffset(0.0);         // V
    fgen.OutputEnable(true);
    var hz = fgen.ReadFrequency();
    ```

Burst, duty cycle, and arbitrary waveforms are tracked in the capability matrix (todo / deferred).
