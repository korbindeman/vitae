# Vitae Extended Features

This document captures design decisions for features beyond Vitae's minimal core. These are "batteries-included" features that build on top of the core primitives.

See [core.md](./core.md) for core architectural decisions.

## Table of Contents

- [Topic 1: Routing](#topic-1-routing)
- [Topic 2: Icons](#topic-2-icons)
- [Topic 3: Unstyled Components](#topic-3-unstyled-components)

---

## Topic 1: Routing

### Decisions

**Philosophy:**
- Not URL-based routing (this is not the web)
- Plain Rust enum matching for view selection
- Framework provides history and native integration, not the matching itself

**Core pattern (user code):**
```rust
#[derive(Route)]
enum View {
    #[route(title = "Home")]
    Home,
    
    #[route(title = "Settings")]
    Settings,
    
    Editor { id: ProjectId },  // dynamic title via trait method
}

fn view(model: &Model, ctx: &Context) -> ElementBuilder {
    match ctx.route::<View>() {
        View::Home => home_view(model, ctx),
        View::Settings => settings_view(model, ctx),
        View::Editor { id } => editor_view(model, ctx, id),
    }
}
```

**Navigation API (via context):**
```rust
ctx.navigate(View::Editor { id });  // push to history
ctx.back();                          // go back
ctx.forward();                       // go forward
ctx.replace(View::Settings);         // replace without adding to history
```

**Dynamic titles via optional trait method:**
```rust
impl View {
    fn title(&self, model: &Model) -> Option<String> {
        match self {
            View::Editor { id } => {
                let project = model.projects.get(id)?;
                Some(format!("Editing: {}", project.name))
            }
            _ => None,  // fall back to #[route(title = "...")] default
        }
    }
}
```

**Native integration:**
- Window title updates automatically based on current route
- Menu items can trigger navigation
- Keyboard shortcuts for back/forward (Cmd+[ / Cmd+])

**History:**
- Lives in context (framework-managed), not in user model
- Configurable max depth (default ~50)

### Architectural Implications

- **Prerequisite:** Context system (see [core.md Topic 9](./core.md#topic-9-model-ownership-open))
- **Precedent:** Framework concerns (routing, theme, i18n) live in context; user state lives in model
- **View signature:** Changes from `fn(&Model)` to `fn(&Model, &Context)` (or context accessed via other mechanism)

### Rationale

The enum-based approach leverages Rust's type system for exhaustive matching and refactoring safety. History and native integration are the real value-add - the matching is just standard Rust. Keeping history in context separates framework concerns from user state.

---

## Topic 2: Icons

### Decisions

**Philosophy:**
- Icon sets are first-class citizens
- Framework provides the primitive and trait, icon sets are pluggable
- Core stays agnostic; standard library may include some sets
- Multiple icon sets can be used simultaneously

**Core primitive:**
```rust
icon(lucide::Save)
    .size(Size::Md)         // standardized sizing (see theming)
    .color(Color::Current)  // inherits from text color by default
```

**Icon set trait:**
```rust
pub trait IconSet {
    type Icon;
    fn get(icon: Self::Icon) -> IconData;
}
```

**Creating icon sets:**

*As a library:*
```rust
// vitae_icons_lucide crate
pub struct Save;
pub struct ChevronRight;
// ... generated or hand-written

impl IconSet for Lucide { ... }
```

*For an app (macro from directory):*
```rust
vitae::icon_set!("my_icons", "assets/icons/");
// Generates: mod my_icons { pub struct Save; pub struct Edit; ... }
```

**Typed access with autocomplete:**
```rust
use vitae_icons_lucide as lucide;
use my_icons;

icon(lucide::Save)           // UI icon from library
icon(my_icons::CustomThing)  // domain-specific icon
```

**Default behavior:**
- Color inherits from text color (like CSS `currentColor`)
- Size defaults to `Size::Md`

### Standardized Sizing

Icons use the unified `Size` enum:
```rust
pub enum Size {
    Xs,
    Sm,
    Md,  // default
    Lg,
    Xl,
    // potentially more
}

icon(lucide::Save).size(Size::Lg)
```

The actual pixel values are context-dependent and theme-defined:
- `Size::Md` for icons might be 16px
- `Size::Md` for text might be 16px
- `Size::Md` for spacing might be 16px
- But these can vary by theme

### Deferred to Theming

- Exact `Size` enum variants and naming
- How `Size` maps to pixel values per context
- Whether a single enum or context-specific enums (leaning toward single)
- Theme customization of size scales

See [core.md Topic 8](./core.md#topic-8-theme-configuration) for theming decisions.

### Rationale

Typed icon access provides autocomplete and compile-time safety. The trait-based approach allows ecosystem growth (community icon set crates) while keeping core minimal. Unified sizing creates consistency across the framework.

---

## Topic 3: Unstyled Components

### Decisions

**Philosophy:**
- Headless components: provide behavior + state, user provides rendering
- Functional primitives, not classes
- Lives in separate `vitae_components` crate
- Form tooling included to make validation/submission easier

**Pattern:** `component(value, on_change).render(|state| ...)`

**Input example:**
```rust
input(model.username.clone(), |val| model.username = val)
    .placeholder("Enter username")
    .render(|state| {
        div()
            .border(px(1.0))
            .border_color(if state.focused { Color::Blue } else { Color::Gray })
            .child(
                text(&state.value)
                    .color(if state.is_placeholder { Color::Gray } else { Color::Black })
            )
    })

// State available:
// state.value         - current text
// state.focused       - has focus
// state.is_placeholder - showing placeholder (value empty)
// state.selection     - cursor position / selection range
// state.disabled      - input disabled
// state.error         - validation error (if using .validate())
```

**Checkbox example:**
```rust
checkbox(model.accepted, |val| model.accepted = val)
    .render(|state| {
        div()
            .w(px(20.0)).h(px(20.0))
            .bg(if state.checked { Color::Blue } else { Color::White })
            .border(px(1.0))
            .child(if state.checked { icon(lucide::Check) } else { div() })
    })
```

### Component List

**Core set:**
- `input()` - text entry, selection, cursor, placeholder
- `checkbox()` - boolean toggle
- `radio()` / `radio_group()` - single selection from options
- `button()` - click handling, disabled state, loading state
- `select()` - pick from list (dropdown)
- `slider()` - numeric range input
- `toggle()` - on/off switch

**Overlay components:**
- `dialog()` - modal overlay with focus trapping
- `tooltip()` - hover info

**Structural components:**
- `tabs()` - tabbed interface
- `accordion()` - collapsible sections

### Form Tooling

**Validation on individual inputs:**
```rust
input(model.email.clone(), |v| model.email = v)
    .validate(|v| {
        if v.contains('@') { Ok(()) } 
        else { Err("Invalid email") }
    })
    .render(|state| {
        col()
            .child(/* input rendering */)
            .child(if let Some(err) = &state.error {
                text(err).color(Color::Red)
            } else { div() })
    })
```

**Form grouping:**
```rust
form()
    .on_submit(|model| { /* all valid, do something */ })
    .child(input(...).validate(...))
    .child(input(...).validate(...))
    .child(button().submit())  // disabled until form valid
```

### Open Questions

- Exact validation API (sync vs async, when to validate - on blur, on change, on submit)
- How overlay components interact with portals/z-index
- Accessibility attributes (aria) - automatic or user-provided?

### Rationale

Headless components give maximum styling flexibility while handling the hard parts (focus management, keyboard navigation, state). The `component(value, on_change)` pattern mirrors React's controlled components and fits naturally with Vitae's model-view-update architecture. Form tooling reduces boilerplate for common validation patterns.

---
