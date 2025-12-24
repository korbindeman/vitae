# Getting Started with Vitae

Vitae is a GPU-accelerated desktop UI library for Rust. It uses a Model-View-Update architecture where you define your application state, a view function that renders it, and event handlers that update the state.

## Basic Setup

Add vitae to your `Cargo.toml`:

```toml
[dependencies]
vitae = { path = "../vitae" }
```

## Minimal Example

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

## Core Concepts

### 1. The Model

Your model is any `Clone + 'static` type that holds your application state:

```rust
#[derive(Clone)]
struct MyApp {
    name: String,
    items: Vec<Item>,
    selected: Option<usize>,
}
```

### 2. The View Function

The view function takes a reference to your model and returns an `ElementBuilder`:

```rust
fn view(model: &MyApp) -> ElementBuilder {
    div().child(text(&model.name))
}
```

### 3. Elements

Create elements with `div()` and `text()`:

```rust
div()                          // A container element
text("Hello")                  // Text content
text(format!("{}", value))     // Dynamic text
```

### 4. Layout

Use `.row()` and `.col()` to control child layout direction:

```rust
div().row().children([...])    // Horizontal layout
div().col().children([...])    // Vertical layout
```

### 5. Sizing

```rust
.w(px(100.0))       // Fixed width in pixels
.h(pc(50.0))        // Height as percentage
.size(FULL)         // Width and height to 100%
.square()           // 1:1 aspect ratio
.aspect_ratio(16.0 / 9.0)
```

Built-in size constants:
- `FULL` - 100%
- `HALF` - 50%

### 6. Spacing

```rust
.p(px(16.0))        // Padding
.m(px(8.0))         // Margin
```

Built-in spacing constants:
- `SM` - 8px
- `MD` - 16px
- `LG` - 32px

### 7. Colors

```rust
.bg(Color::rgb(255, 100, 50))   // Custom RGB
.bg(RED)                         // Preset color
```

Preset colors: `WHITE`, `BLACK`, `GRAY`, `RED`, `GREEN`, `BLUE`, `YELLOW`, `CYAN`, `MAGENTA`, `TRANSPARENT`

### 8. Text Styling

```rust
text("Large text").font_size(32.0)
```

### 9. Children

Add children with `.child()` or `.children()`:

```rust
div()
    .child(text("First"))
    .child(text("Second"))

// Or with an iterator:
div().children((0..5).map(|i| text(format!("Item {}", i))))
```

### 10. Event Handling

Handle clicks with `.on_left_click()` or `.on_right_click()`:

```rust
div()
    .child(text("Click me"))
    .on_left_click(|model: &mut MyApp| {
        model.count += 1;
    })
```

For other events, use `.on_event()`:

```rust
div().on_event(|model: &mut MyApp, event: &Event| {
    match event {
        Event::Click { button: MouseButton::Left } => { /* ... */ }
        _ => {}
    }
    EventResult::Continue
})
```

### 11. Signals (Local State)

For local reactive state that doesn't belong in the model, use signals:

```rust
fn view(model: &MyApp) -> ElementBuilder {
    let hover = use_signal(|| false);
    
    div()
        .bg(if hover.get() { BLUE } else { GRAY })
        .on_event(move |_: &mut MyApp, event: &Event| {
            match event {
                Event::MouseEnter => hover.set(true),
                Event::MouseLeave => hover.set(false),
                _ => {}
            }
            EventResult::Continue
        })
}
```

Signal methods:
- `signal.get()` - Get current value
- `signal.set(value)` - Set new value
- `signal.update(|v| v + 1)` - Update with function

## Complete Example

See the `chess` crate for a full working example that demonstrates layout, event handling, signals, and state management.
