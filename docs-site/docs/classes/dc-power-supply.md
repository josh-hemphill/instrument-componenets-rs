# DC power supply

Channel-oriented DC source control: set voltage / current limit, enable output, readback.

See [capability matrix](../capability-matrix.md#dcpowersupply).

## Open

=== "Rust"

    ```rust
    let mut psu = catalog.open_dc_power_supply(&address)?;
    ```

=== "C#"

    ```csharp
    var psu = catalog.OpenDcPowerSupply(address);
    ```

## Set and enable

=== "Rust"

    ```rust
    psu.set_voltage(1, 3.3)?;
    psu.set_current_limit(1, 0.5)?;
    psu.output_enable(1, true)?;
    ```

=== "C#"

    ```csharp
    psu.SetVoltage(1, 3.3);
    psu.SetCurrentLimit(1, 0.5);
    psu.OutputEnable(1, true);
    ```

## Readback

=== "Rust"

    ```rust
    let volts = psu.read_voltage(1)?;
    let amps = psu.read_current(1)?;
    ```

=== "C#"

    ```csharp
    var volts = psu.ReadVoltage(1);
    var amps = psu.ReadCurrent(1);
    ```

Channels are 1-based in the common SCPI dialects used here. Units are volts and amps (SI).
