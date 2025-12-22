# State Management in Vitae

Vitae provides a hybrid state management approach that combines the best aspects of reactive signals and model-based architecture. This gives you maximum flexibility while maintaining minimal boilerplate.

## Table of Contents

- [Quick Start](#quick-start)
- [Core Concepts](#core-concepts)
- [Models (Business Logic)](#models-business-logic)
- [Signals (UI State)](#signals-ui-state)
- [Event Handlers](#event-handlers)
- [When to Use What](#when-to-use-what)
- [Complete Examples](#complete-examples)
- [Best Practices](#best-practices)

## Quick Start

```rust
use vitae::prelude::*;

#[derive(Clone)]
struct Counter {
    count: i32,
}

impl Counter {
    fn increment(&mut self) {
        self.count += 1;
    }
}

fn view(model: &Counter) -> ElementBuilder {
    div()
        .col()
        .child(text(format!("Count: {}", model.count)))
        .child(
            div()
                .bg(Color::BLUE)
                .p(px(10.0))
                .child(text("Increment"))
                .on_click(Counter::increment)
        )
}

fn main() {
    App::new(Counter { count: 0 }, view).run();
}
```

## Core Concepts

Vitae uses a **Model-View-Update** architecture enhanced with **reactive signals**:

1. **Model** - Your application's core state (persistent, testable)
2. **View** - A pure function that renders UI from the model
3. **Signals** - Ephemeral UI state (hover, input fields, animations)
4. **Event Handlers** - Functions that update the model in response to user input

### Architecture Flow

```
User Click
    ↓
Event Handler
    ↓
Update Model
    ↓
Rebuild View (with Signals)
    ↓
Render with Vello
```

## Models (Business Logic)

Models represent your application's core state. They should be:
- `Clone` (for efficient updates)
- Contain business logic as methods
- Testable without UI

### Example: Todo App Model

```rust
#[derive(Clone)]
struct TodoApp {
    todos: Vec<Todo>,
    filter: Filter,
}

#[derive(Clone)]
struct Todo {
    id: usize,
    text: String,
    completed: bool,
}

#[derive(Clone, PartialEq)]
enum Filter {
    All,
    Active,
    Completed,
}

impl TodoApp {
    fn new() -> Self {
        Self {
            todos: vec![],
            filter: Filter::All,
        }
    }

    // Business logic as methods
    fn add_todo(&mut self, text: String) {
        if !text.is_empty() {
            self.todos.push(Todo {
                id: self.next_id(),
                text,
                completed: false,
            });
        }
    }

    fn toggle_todo(&mut self, id: usize) {
        if let Some(todo) = self.todos.iter_mut().find(|t| t.id == id) {
            todo.completed = !todo.completed;
        }
    }

    fn delete_todo(&mut self, id: usize) {
        self.todos.retain(|t| t.id != id);
    }

    fn set_filter(&mut self, filter: Filter) {
        self.filter = filter;
    }

    // Derived state (computed from model)
    fn filtered_todos(&self) -> Vec<&Todo> {
        match self.filter {
            Filter::All => self.todos.iter().collect(),
            Filter::Active => self.todos.iter().filter(|t| !t.completed).collect(),
            Filter::Completed => self.todos.iter().filter(|t| t.completed).collect(),
        }
    }

    fn next_id(&self) -> usize {
        self.todos.len()
    }
}
```

## Signals (UI State)

Signals are reactive values for ephemeral UI state. They automatically trigger re-renders when updated.

### Creating Signals

```rust
fn view(model: &MyModel) -> ElementBuilder {
    // Create signals with initial values
    let hover = use_signal(|| false);
    let input_text = use_signal(|| String::new());
    let selected_index = use_signal(|| None::<usize>);

    // Use signal values with .get()
    let is_hovered = hover.get();

    // Update signals with .set()
    hover.set(true);

    // ... build UI
}
```

### Signal Examples

#### Hover State

```rust
fn button_view(label: &str) -> ElementBuilder {
    let hover = use_signal(|| false);

    div()
        .bg(if hover.get() { Color::BLUE } else { Color::GRAY })
        .p(px(10.0))
        .child(text(label))
        .on_hover(|| hover.set(true))      // Not implemented yet
        .on_hover_end(|| hover.set(false))  // Not implemented yet
}
```

#### Input Field State

```rust
fn view(model: &TodoApp) -> ElementBuilder {
    let input = use_signal(|| String::new());

    div()
        .col()
        .child(
            // Input field shows signal value
            input_field(input.get())
                .on_change(|val| input.set(val))  // Not implemented yet
        )
        .child(
            div()
                .child(text("Add"))
                .on_click(move |app: &mut TodoApp| {
                    app.add_todo(input.get());
                    input.set(String::new());  // Clear input
                })
        )
}
```

#### Toggle State

```rust
fn view(model: &MyModel) -> ElementBuilder {
    let show_details = use_signal(|| false);

    div()
        .col()
        .child(
            div()
                .child(text(if show_details.get() { "Hide" } else { "Show" }))
                .on_click(|| show_details.set(!show_details.get()))
        )
        .child_if(show_details.get(), || {
            div().child(text("Detailed information here..."))
        })
}
```

### Signal Methods

```rust
// Get current value
let value = signal.get();

// Set new value (triggers re-render)
signal.set(new_value);

// Update based on current value
signal.update(|current| current + 1);
```

## Event Handlers

Event handlers update the model in response to user interactions.

### Attaching Handlers

```rust
// Using a model method
div().on_click(MyModel::increment)

// Using a closure
div().on_click(|model: &mut MyModel| model.count = 0)

// Using a closure with captured values
let id = 42;
div().on_click(move |model: &mut MyModel| model.delete_item(id))
```

### Available Events

Currently supported:
- `on_click` - Left mouse button click

Coming soon:
- `on_hover` - Mouse enters element
- `on_hover_end` - Mouse leaves element
- `on_change` - Text input change
- `on_submit` - Form submission
- Keyboard events

## When to Use What

### Use **Models** for:

✅ **Business Logic**
```rust
impl Game {
    fn make_move(&mut self, from: Pos, to: Pos) {
        // Game rules, validation
    }
}
```

✅ **Persistent Data**
```rust
#[derive(Clone)]
struct UserProfile {
    name: String,
    email: String,
    settings: Settings,
}
```

✅ **Testable State**
```rust
#[test]
fn test_add_todo() {
    let mut app = TodoApp::new();
    app.add_todo("Test".into());
    assert_eq!(app.todos.len(), 1);
}
```

✅ **Shared State**
```rust
// Multiple components read from the same model
fn sidebar(model: &App) -> ElementBuilder { /* ... */ }
fn main_panel(model: &App) -> ElementBuilder { /* ... */ }
```

### Use **Signals** for:

✅ **Hover/Focus States**
```rust
let hover = use_signal(|| false);
```

✅ **Input Field Values** (before submission)
```rust
let input = use_signal(|| String::new());
```

✅ **UI-Only State** (doesn't affect model)
```rust
let modal_open = use_signal(|| false);
let selected_tab = use_signal(|| 0);
```

✅ **Animation State**
```rust
let progress = use_signal(|| 0.0);
```

## Complete Examples

### Example 1: Counter with Hover

```rust
use vitae::prelude::*;

#[derive(Clone)]
struct Counter {
    count: i32,
    history: Vec<i32>,
}

impl Counter {
    fn new() -> Self {
        Self {
            count: 0,
            history: vec![],
        }
    }

    fn increment(&mut self) {
        self.count += 1;
        self.history.push(self.count);
    }

    fn decrement(&mut self) {
        self.count -= 1;
        self.history.push(self.count);
    }

    fn reset(&mut self) {
        self.count = 0;
        self.history.clear();
    }
}

fn view(model: &Counter) -> ElementBuilder {
    // Signal for UI state
    let show_history = use_signal(|| false);

    div()
        .col()
        .p(px(20.0))
        .child(
            text(format!("Count: {}", model.count))
                .size(px(32.0))
        )
        .child(
            div()
                .row()
                .child(
                    button("Increment")
                        .on_click(Counter::increment)
                )
                .child(
                    button("Decrement")
                        .on_click(Counter::decrement)
                )
                .child(
                    button("Reset")
                        .on_click(Counter::reset)
                )
        )
        .child(
            button(if show_history.get() { "Hide History" } else { "Show History" })
                .on_click(|| show_history.set(!show_history.get()))
        )
        .child_if(show_history.get(), || {
            div()
                .col()
                .children(
                    model.history.iter()
                        .map(|h| text(format!("  {}", h)))
                        .collect()
                )
        })
}

fn button(label: &str) -> ElementBuilder {
    div()
        .bg(Color::BLUE)
        .p(px(10.0))
        .m(px(5.0))
        .child(text(label))
}

fn main() {
    App::new(Counter::new(), view).run();
}
```

### Example 2: Todo List

```rust
use vitae::prelude::*;

#[derive(Clone)]
struct TodoApp {
    todos: Vec<Todo>,
    filter: Filter,
}

#[derive(Clone)]
struct Todo {
    id: usize,
    text: String,
    completed: bool,
}

#[derive(Clone, Copy, PartialEq)]
enum Filter {
    All,
    Active,
    Completed,
}

impl TodoApp {
    fn new() -> Self {
        Self {
            todos: vec![],
            filter: Filter::All,
        }
    }

    fn add_todo(&mut self, text: String) {
        if !text.is_empty() {
            self.todos.push(Todo {
                id: self.todos.len(),
                text,
                completed: false,
            });
        }
    }

    fn toggle_todo(&mut self, id: usize) {
        if let Some(todo) = self.todos.iter_mut().find(|t| t.id == id) {
            todo.completed = !todo.completed;
        }
    }

    fn delete_todo(&mut self, id: usize) {
        self.todos.retain(|t| t.id != id);
    }

    fn set_filter(&mut self, filter: Filter) {
        self.filter = filter;
    }

    fn filtered_todos(&self) -> Vec<&Todo> {
        match self.filter {
            Filter::All => self.todos.iter().collect(),
            Filter::Active => self.todos.iter().filter(|t| !t.completed).collect(),
            Filter::Completed => self.todos.iter().filter(|t| t.completed).collect(),
        }
    }
}

fn view(app: &TodoApp) -> ElementBuilder {
    // Signal for input field
    let input = use_signal(|| String::new());

    div()
        .col()
        .p(px(20.0))
        .child(text("Todo App").size(px(32.0)))
        .child(
            div()
                .row()
                .child(
                    // Input field (imaginary - would need text input widget)
                    text(format!("Input: {}", input.get()))
                )
                .child(
                    div()
                        .child(text("Add"))
                        .on_click(move |app: &mut TodoApp| {
                            app.add_todo(input.get());
                            input.set(String::new());
                        })
                )
        )
        .child(
            div()
                .row()
                .child(filter_button("All", app.filter == Filter::All)
                    .on_click(|app: &mut TodoApp| app.set_filter(Filter::All)))
                .child(filter_button("Active", app.filter == Filter::Active)
                    .on_click(|app: &mut TodoApp| app.set_filter(Filter::Active)))
                .child(filter_button("Completed", app.filter == Filter::Completed)
                    .on_click(|app: &mut TodoApp| app.set_filter(Filter::Completed)))
        )
        .child(
            div()
                .col()
                .children(
                    app.filtered_todos()
                        .iter()
                        .map(|todo| todo_item(todo))
                        .collect()
                )
        )
}

fn todo_item(todo: &Todo) -> ElementBuilder {
    let id = todo.id;
    div()
        .row()
        .child(text(if todo.completed { "[x]" } else { "[ ]" })
            .on_click(move |app: &mut TodoApp| app.toggle_todo(id)))
        .child(text(&todo.text))
        .child(text("×")
            .on_click(move |app: &mut TodoApp| app.delete_todo(id)))
}

fn filter_button(label: &str, active: bool) -> ElementBuilder {
    div()
        .bg(if active { Color::BLUE } else { Color::GRAY })
        .p(px(10.0))
        .m(px(5.0))
        .child(text(label))
}

fn main() {
    App::new(TodoApp::new(), view).run();
}
```

### Example 3: Chess Game (see `crates/chess/src/main.rs`)

The chess example demonstrates:
- **Model** for game state (selected square, turn, last move)
- **Signals** for hover state
- Click handlers for square selection
- Dynamic UI based on model state

## Best Practices

### 1. Keep Models Pure

```rust
// ✅ Good - pure business logic
impl Game {
    fn make_move(&mut self, from: Pos, to: Pos) -> Result<(), Error> {
        if self.is_valid_move(from, to) {
            self.apply_move(from, to);
            Ok(())
        } else {
            Err(Error::InvalidMove)
        }
    }
}

// ❌ Bad - mixing UI concerns
impl Game {
    fn make_move(&mut self, from: Pos, to: Pos, show_error_modal: &Signal<bool>) {
        // Don't pass signals to model methods
    }
}
```

### 2. Use Signals for Ephemeral State

```rust
// ✅ Good - hover state is UI-only
let hover = use_signal(|| false);

// ❌ Bad - hover state in model
#[derive(Clone)]
struct Model {
    hover_state: bool,  // Don't store UI state in model
}
```

### 3. Minimize Model Cloning

```rust
// ✅ Good - use Arc for large data
#[derive(Clone)]
struct Model {
    large_data: Arc<Vec<LargeItem>>,  // Cheap clone
    selected_id: usize,                // Copy
}

// ❌ Bad - expensive clones
#[derive(Clone)]
struct Model {
    large_data: Vec<LargeItem>,  // Cloned every render
}
```

### 4. Test Business Logic

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_todo() {
        let mut app = TodoApp::new();
        app.add_todo("Test task".into());

        assert_eq!(app.todos.len(), 1);
        assert_eq!(app.todos[0].text, "Test task");
        assert!(!app.todos[0].completed);
    }

    #[test]
    fn test_toggle_todo() {
        let mut app = TodoApp::new();
        app.add_todo("Test".into());
        app.toggle_todo(0);

        assert!(app.todos[0].completed);
    }
}
```

### 5. Compose View Functions

```rust
// ✅ Good - small, composable functions
fn view(model: &App) -> ElementBuilder {
    div()
        .col()
        .child(header(model))
        .child(sidebar(model))
        .child(main_content(model))
}

fn header(model: &App) -> ElementBuilder {
    div().child(text(&model.title))
}

fn sidebar(model: &App) -> ElementBuilder {
    div().children(/* ... */)
}
```

## Performance Considerations

### Current Behavior

- **Tree rebuilt every frame** - The entire UI tree is reconstructed on each render
- **Works with Vello** - Vello expects a fresh scene each frame, so this aligns perfectly
- **Signal updates trigger redraws** - Only when signals change

### Future Optimizations

Coming soon:
- **Dirty tracking** - Only rebuild subtrees that changed
- **Memoization** - Cache view functions by dependencies
- **Parallel rendering** - Build tree on separate thread

For now, the immediate-mode approach works well for most UIs. If you have performance concerns with large lists, consider:
- Using `Arc<_>` for large model data
- Implementing pagination or virtualization
- Profiling before optimizing

## FAQ

**Q: Why can't I use `&mut self` in event handlers?**

A: Event handlers receive `&mut Model`, not `&mut self`. Use closures:
```rust
// ✅ Correct
.on_click(|model: &mut MyModel| model.increment())

// ❌ Won't compile
.on_click(self.increment)  // self not available
```

**Q: Can signals be shared between components?**

A: Currently no - signals are scoped to the view function. For shared state, use the model.

**Q: How do I handle async operations?**

A: Async support is coming soon. For now, use channels or polling.

**Q: Can I use this with multi-threading?**

A: The architecture supports it! Models can be sent to render threads. Signals are thread-local.

## Next Steps

- Check out the [examples](../crates/chess/) for complete applications
- Read about [layout system](./layout.md) (coming soon)
- Learn about [Vello rendering](https://github.com/linebender/vello)
