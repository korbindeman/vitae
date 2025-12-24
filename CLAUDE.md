# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands

```bash
cargo build                    # Build all crates
cargo run --bin vitae_chess    # Run the chess demo
cargo check                    # Type check without building
```

## Architecture

Vitae is a GPU-accelerated desktop UI library for Rust built on Vello (rendering) and Parley (text layout). It uses a Model-View-Update architecture with reactive signals.

### Crate Structure

- **vitae** (`crates/vitae`) - Main entry point. Exports `App`, `Signal`, and re-exports from core/render. Users import `vitae::prelude::*`.
- **vitae_core** (`crates/core`) - Core UI primitives: element tree, builder API, layout engine, styling, events.
- **vitae_render** (`crates/render`) - Vello-based renderer, texture/SVG loading.
- **vitae_chess** (`crates/chess`) - Demo application showing layout, events, and state management.
- **lumen** (`crates/lumen`) - Another demo application.

### Key Components

**Element System** (`crates/core/src/`):
- `builder.rs` - Fluent `ElementBuilder` API (`.row()`, `.col()`, `.bg()`, `.child()`, etc.)
- `element.rs` - `ElementTree`, `Node`, `NodeId`, `NodeKind` for the UI tree
- `elements/` - Element constructors: `div()`, `text()`, `img()`, `svg()`, `portal()`
- `layout.rs` - Single-pass flexbox-inspired layout algorithm
- `style.rs` - `Style` struct, `Length` (px/percent/auto), positioning, alignment
- `events.rs` - `Event` enum, `EventHandler`, mouse/keyboard handling

**Application Layer** (`crates/vitae/src/`):
- `window.rs` - winit integration, event loop, render scheduling
- `signal.rs` - Reactive signals for ephemeral UI state (`use_signal()`)
- `prelude.rs` - Common exports and constants (FULL, HALF, SM, MD, LG, colors)

### State Management Pattern

- **Model**: Application state passed to `App::new(model, view)`. Must be `Clone + 'static`.
- **View function**: `fn view(model: &Model) -> ElementBuilder` - pure function rebuilding UI each frame.
- **Signals**: Local reactive state via `use_signal(|| initial)` for hover, input fields, toggles.
- **Event handlers**: `.on_left_click(|m: &mut Model| ...)` mutates the model.

### Layout System

- Direction: `.row()` (horizontal) or `.col()` (vertical, default)
- Sizing: `px()`, `pc()`, `Length::Auto`
- Alignment: `.align()` (cross-axis), `.distribute()` (main-axis), `.center()` (both)
- Position modes: `Relative` (flow), `Absolute` (parent-relative), `Portal` (viewport-relative)

## Code Style

- Keep the builder API fluent and chainable
- Element constructors (`div()`, `text()`, etc.) return `ElementBuilder`
- Use `impl Into<Length>` for size parameters to accept both `px()` and `pc()` values
