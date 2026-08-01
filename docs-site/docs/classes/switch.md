# Switch

Matrix-style close / open / query for channel pairs, plus open-all.

See [capability matrix](../capability-matrix.md#switch).

## Open

=== "Rust"

    ```rust
    let mut sw = catalog.open_switch(&address)?;
    ```

=== "C#"

    ```csharp
    var sw = catalog.OpenSwitch(address);
    ```

## Routes

=== "Rust"

    ```rust
    sw.close_route(101, 201)?;
    let closed = sw.is_closed(101, 201)?;
    sw.open_route(101, 201)?;
    sw.open_all()?;
    ```

=== "C#"

    ```csharp
    sw.CloseRoute(101, 201);
    var closed = sw.IsClosed(101, 201);
    sw.OpenRoute(101, 201);
    sw.OpenAll();
    ```

Path naming helpers and scan lists are tracked as todo / deferred.
