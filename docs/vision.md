# Vitae Vision Document

This document captures the architectural vision and design decisions for vitae - a Rust GUI library for building creative software.

## Table of Contents

- [Project Vision](#project-vision)
- [Topic 1: Event Handling](#topic-1-event-handling)
- [Topic 2: Keybinds](#topic-2-keybinds)
- [Topic 3: State Management](#topic-3-state-management)
- [Topic 4: Threading](#topic-4-threading)
- [Topic 5: Native Features](#topic-5-native-features)
- [Topic 6: Typestate Builder API](#topic-6-typestate-builder-api)
- [Topic 7: Rendering Arbitrary Pixels](#topic-7-rendering-arbitrary-pixels)
- [Topic 8: Theme Configuration](#topic-8-theme-configuration)
- [Topic 9: Model Ownership (Open)](#topic-9-model-ownership-open)

---

## Project Vision

**Goal:** A "killer" Rust GUI library - minimal core, great developer experience, intuitive API, strong native support.

**Guiding Principles:**
- **Minimal and explicit** over magic
- **Intuitive defaults** - predictable behavior
- **Fresh perspective** - not bound by web baggage
- **API quality and documentation** are first-class concerns
- **Minimal architecture, batteries-included experience**

**Anti-goals:**
- Verbosity
- Complexity hidden behind abstraction
- Web-style patching/legacy patterns

**Primary use case:** Creative software (DAW, image editors, 3D tools) - performance matters, custom rendering matters, precise control matters.

**Target audience:** Building a general-purpose GUI library, but primarily driven by the need to build a suite of creative applications.

**Inspiration:**
- GPUI: Great builder API (but too verbose)
- Tailwind 4: Flexible theme configuration
- Fresh perspective on state management and rendering

---

## Topic 1: Event Handling

### Decisions

**Propagation Model:**
- Two-phase propagation: Capture phase (root → target) then Bubble phase (target → root)
- Single clear result type: `EventResult { Continue, Stop }`
- No DOM confusion (`stopPropagation` vs `preventDefault` vs both)

**Core Layer:**
- Blank canvas - no default behaviors
- Just event propagation mechanics

**Standard Library:**
- Pre-built elements (`Link`, `TextInput`, `Button`) with built-in behaviors
- These are just normal event handlers - nothing magic
- Users can build from primitives (`div`, `text`) for custom behavior

**Handler API:**
- Typed helpers for common cases: `.on_click()`, `.on_key_down()`, etc.
- Full control when needed: `.on_event(|model, event| ...)`
- Typed helpers are convenience wrappers around `on_event`

**Event Types (all should be supported):**
- Mouse: click, double-click, hover, drag, scroll
- Keyboard: key press, key release, text input
- Focus: focus, blur (note: full accessibility out of scope for now)
- Custom events

### Open Questions

- How do handlers specify capture vs bubble phase?
- Event struct design (what data does each event carry?)
- How does standard library's built-in behavior interact with user handlers?

### Rationale

Event handling is the foundation that other features depend on. The two-phase model provides maximum flexibility (parents can intercept, children can notify, propagation can be stopped) while keeping the API simpler than the DOM through a single `EventResult` concept.

---

## Topic 2: Keybinds

### Decisions

**Scope:**
- Both global (app-wide) and context-based (active when certain elements are focused)

**Architecture:**
- Centralized registry (keybinds defined in one place, not scattered)
- Actions as indirection layer (handlers respond to actions, not physical keys)
- Conflict detection at registration time (runtime, at app startup)
- User remapping supported by design

**Conceptual API:**
```rust
// Somewhere central - defines all actions
enum Action {
    Save,
    Undo,
    ZoomIn,
}

// Keybind config - maps keys to actions (user-configurable)
let bindings = Keybinds::new()
    .bind(Key::S.cmd(), Action::Save)
    .bind(Key::Z.cmd(), Action::Undo)
    .context("piano_roll", |ctx| {
        ctx.bind(Key::A, Action::SelectAll)
    });

// In UI - respond to actions, not keys
div().on_action(Action::Save, |model| { ... })
```

### Open Questions

- Exact syntax for key definitions
- How contexts are defined/activated
- Config file format for user remapping

### Rationale

Keybinds build on the event system but have unique concerns. Centralized definition prevents conflicts and makes keybinds discoverable. The action indirection layer enables user remapping without changing handler code - critical for creative software where users expect customizable shortcuts.

---

## Topic 3: State Management

### Decisions

**Keep Current Approach:**
- Signal-based reactive system for ephemeral UI state
- Model-View-Update architecture for business logic
- Hybrid approach combining both

**Model (Business Logic):**
- Persistent, testable
- Contains methods for business logic
- Can be serialized for persistence
- Can be shared across threads

**Signals (UI State):**
- Ephemeral, thread-local
- View-scoped
- For hover, focus, input fields, animations
- Not serialized

**For Threading:**
- Signals stay thread-local
- Models can be sent across threads
- Clear separation of concerns

### Open Questions

- Derived/computed state (if needed)
- Exact threading model for model access
- See "Model Ownership" below for open architectural questions

### Rationale

The current model-view-update with signals pattern works well for the chess example and aligns with creative software needs where business logic (model) should be separate from UI concerns (signals).

---

## Topic 4: Threading

### Decisions

**Philosophy:**
- Main thread is "holy" - event loop, rendering, input happen here, must never block
- Follow platform opinions (like GPUI)

**Vitae Provides:**
- Foreground executor (main thread, for UI updates)
- Background executor (off main thread, for heavy work)
- Platform-native dispatching for system-level efficiency

**Vitae Does NOT Manage:**
- Audio/real-time threads
- Lock-free primitives (apps use their own: rtrb, basedrop, etc.)

**Boundary Pattern:**
- Vitae helps with UI integration from external threads
- Conceptual API: `subscribe_channel(rx, |model, msg| ...)` to bridge external threads to model updates
- Details TBD during implementation

**Thread Responsibilities:**

| Thread | Responsibility | Owner |
|--------|----------------|-------|
| Main/UI | Event loop, rendering, input | Vitae |
| Background | Heavy work (file I/O, processing) | Vitae provides executor |
| Audio RT | Real-time audio processing | App's responsibility |
| Compute | Intensive calculations | App's responsibility |

### Open Questions

- Exact API for external thread boundary
- Platform-specific executor implementation
- How to make this flexible for various real-time/compute needs

### Rationale

Creative software (especially DAWs) requires sophisticated threading. By providing executors for common cases (UI + background) but staying out of specialized threading (audio RT), vitae enables high-performance apps while remaining flexible.

---

## Topic 5: Native Features

### Decisions

**Philosophy:**
- Comprehensive unified API for all platforms
- Platform-specific escape hatches when needed

**Architecture:**
- Companion crates (vitae_native, etc.)
- Re-exported by default through main `vitae` crate
- Opt-out via `minimal` feature flag: `vitae = { features = ["minimal"] }`

**Priority Features:**
1. App menus (high priority)
2. File picker (high priority)
3. Window management (mostly from winit)
4. Clipboard (medium priority)

**Full Wishlist (add as needed):**
- Context menus
- System dialogs/alerts
- Drag and drop
- Notifications
- System tray

**What Winit Provides:**
- Window creation, resizing, fullscreen, minimize
- Multi-window
- Basic input events

**What We Need to Add:**
- Native menus
- File dialogs (e.g., `rfd`)
- Clipboard (e.g., `arboard`)
- System tray

### Rationale

A "killer" GUI library must have comprehensive native features. The companion crate architecture keeps the core minimal while providing a batteries-included default experience. Users wanting minimal can opt out, but most users get "it just works" behavior.

---

## Topic 6: Typestate Builder API

### Decision

**Skip for now.** Current builder API is sufficient.

### Reasoning

Typestate conflicts with conditional/dynamic UI patterns, which are common in real apps:

```rust
// This doesn't work with typestate:
let el = div();
let el = if wide {
    el.w(px(200.0))  // -> DivBuilder<HasWidth>
} else {
    el               // -> DivBuilder<NoSize>  ← Different types!
};
```

The trade-off of compile-time safety vs. runtime flexibility doesn't favor typestate for UI building.

### Revisit If

Specific pain points emerge where compile-time enforcement would help and the flexibility trade-off is acceptable.

---

## Topic 7: Rendering Arbitrary Pixels

### Decisions

**Two Primitives:**

1. **`canvas` element** - Custom vector drawing
   ```rust
   canvas()
       .w(FULL).h(px(200.0))
       .draw(|scene, bounds| {
           // Direct vello scene access
           // Draw waveform as paths, EQ curves, etc.
       })
   ```

2. **`texture` element** - Rasterized/GPU content
   ```rust
   texture()
       .w(FULL).h(px(200.0))
       .source(my_texture_handle)
   ```

**Texture Sources:**
- CPU-rendered pixels (spectrum from FFT data)
- Shared GPU texture (external renderer)
- Shader output (future)

**Immediate Needs (DAW):**
- Waveform display (vector or texture)
- Spectrum analyzer (texture, updated every frame)
- Parametric EQ visualization (vector curves + texture spectrum)

### Deferred Questions

- Custom shader integration (future)
- GPU context ownership (figure out when needed)
- Compositing/layering details
- Synchronization for external frame sources

### Rationale

Creative software needs custom rendering beyond standard UI widgets. The canvas/texture split provides flexibility while fitting cleanly into the layout system. Vello already handles vector graphics well, so the focus is on rasterized content and GPU integration.

---

## Topic 8: Theme Configuration

### Decisions

**Configuration Approach:**
- Runtime configuration (not compile-time)
- Dynamic - can be changed by runtime values
- Inspired by Tailwind 4's flexible token system

**Theme Structure:**
- Namespaced tokens: `color.*`, `spacing.*`, `text.*`, `font.*`, `radius.*`
- Semantic naming with flexibility

**Conceptual API:**
```rust
// Define
let theme = Theme::default()
    .color("primary", Color::rgb(63, 60, 187))
    .color("surface", Color::rgb(255, 255, 255))
    .spacing_scale(4.0)  // base unit
    .text("md", 16.0);

// Use
div()
    .bg(theme.color.primary)
    .p(theme.spacing.md)
```

**User Customization:**
- End users should be able to theme apps in theory
- Whether they can depends on if the developer enables it

### Open Questions

- Access pattern (explicit passing vs context system)
- Exact API design
- Depends on broader ownership/context design (see Model Ownership)

### Rationale

Creative software often needs theming. Tailwind 4's approach of namespaced tokens with runtime flexibility maps well to Rust. The semantic naming enables both structure and flexibility.

---

## Topic 9: Model Ownership (Open)

### Current State

**Ownership:**
```rust
App<M>
  └─ VitaeApp<M>
       └─ model: M  // VitaeApp owns the model
```

**Flow:**
- `App::new(initial_model, view)` - user passes ownership
- View: `fn(&M)` - immutable borrow
- Handlers: `fn(&mut M)` - mutable borrow
- Works well for simple apps (chess example)

### Problems Identified

1. **Organization:** One giant struct for complex apps (DAW with hundreds of fields)
2. **Reusability:** Components can't have encapsulated state easily
3. **Multi-window:** Unclear how multiple windows share model
4. **Prop drilling:** Must pass model through component layers

### Approaches Explored

**1. Single model + pass everywhere** (current)
- Simple but doesn't scale

**2. Context system** - `use_context::<T>()`
- Avoids prop drilling
- Question: single context vs multiple pieces of state?

**3. Message/Action pattern**
- Centralized action handling
- Concern: inflexible, hard to extend for reusable components

**4. ECS-inspired**
- World storage, entity IDs, no hierarchy
- Access via queries, no prop drilling
- Could combine with model methods for encapsulation
- Natural for DAW architecture (tracks, clips, plugins as entities)

### Open Questions

- Do we need multiple pieces of state, or organize one better?
- How do framework concerns (theme, i18n) fit with user state?
- ECS as core pattern or just for complex apps?
- How does ownership work with context system?
- What's the right balance between explicit and convenient?

### Decision

**Defer.** Need more real-world experience building with vitae before committing to an architecture. The current single-model approach works for now. Revisit when building the DAW reveals concrete pain points.

### Notes

The model ownership question ties into theme access, multi-window support, and component reusability. A comprehensive solution should address all these concerns together rather than piecemeal.

---

## Implementation Order

Based on dependencies and priorities:

1. **Event handling** - Foundation for everything else
2. **Keybinds** - Builds directly on event handling
3. **State Management** - Refine based on event patterns
4. **Threading** - Affects state management design
5. **Theme configuration** - Self-contained, influences rendering
6. **Rendering arbitrary pixels** - Builds on theming, needs clear threading model
7. **Native features** - Higher-level, depends on event/state foundations
8. **Typestate builder API** - Skipped for now

**Model ownership** should be revisited after gaining experience with topics 1-7.

---

## Next Steps

1. Implement event handling foundation
2. Build out keybind system
3. Add canvas/texture primitives for custom rendering
4. Develop companion crates for native features
5. Revisit state management patterns based on real-world usage
6. Document learnings and update this vision document
