# Vitae

A work-in-progress, GPU-accelerated desktop UI library for Rust, built on [Vello](https://github.com/linebender/vello) and [Parley](https://github.com/linebender/parley).

## Quick Start

Add vitae to your `Cargo.toml`:

```toml
[dependencies]
vitae = { git = "https://github.com/korbindeman/vitae", subdirectory = "crates/vitae" }
```

## Example

```rust
use vitae::prelude::*;

#[derive(Clone)]
struct Counter {
    count: i32,
}

fn view(model: &Counter) -> ElementBuilder {
    div()
        .size(FULL)
        .col()
        .child(text(format!("Count: {}", model.count)))
        .child(
            div()
                .bg(BLUE)
                .p(SM)
                .child(text("Click me"))
                .on_left_click(|m: &mut Counter| m.count += 1),
        )
}

fn main() {
    App::new(Counter { count: 0 }, view).run();
}
```

## Demo

Run the chess demo:

```bash
cargo run --bin vitae_chess
```

## Documentation

These are the most important ones, see [docs/](docs/) for more.

- [Getting Started](docs/getting-started.md)
- [Layout](docs/layout.md)
- [Styling](docs/styling.md)
- [State Management](docs/state-management.md)

## Status

- [x] Fluent builder API (basic)
- [x] Styling (basic)
- [x] Layout engine (basic)
- [x] Text rendering (basic)
- [x] State management (model + signals)
- [ ] Native platform features
- [ ] Performance optimizations
- [ ] Sound

## Acknowledgements

Inspired by [GPUI](https://github.com/zed-industries/zed/tree/main/crates/gpui), particularly its fluent builder API.

## Note on AI usage

Most of the code is AI generated (as is the norm lately). All design decisions are made by myself.
