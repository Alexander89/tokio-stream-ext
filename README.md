# Tokio StreamExt [WIP]

Stream extension with new operators, this will grow over the time.

Feel free to contribute!

## Todo

- [] Better readme
- [] RustDoc
- [] Further operators

## Operators

### Debounce

Debounce a stream until it is sattled over a given duration.

```rust
ReceiverStream::new(self.input)
    .debounce(Duration::from_millis(80)),
```

### distinct until changed

Filters events, similar to the last value.

The initial value is always emitted.

```rust
ReceiverStream::new(self.input)
    .distinct_until_changed(),
```

### Switch Map

Reactive composing of streams.

```rust
switch_map(ReceiverStream::new(keyboard), move |value| {
    if value == 'k' {
        Some(ReceiverStream::new(gamepad))
    } else {
        Some(ReceiverStream::new(joystick))
    }
});
```

### Combine Latest

Collects a value from all streams and switch to a live mode. Every new combination will be emitted from now on.

```rust
switch_map(ReceiverStream::new(keyboard), move |value| {
    if value == 'k' {
        Some(ReceiverStream::new(gamepad))
    } else {
        Some(ReceiverStream::new(joystick))
    }
});
```
