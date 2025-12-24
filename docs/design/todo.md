# Vitae Design To-Do

Open questions and decisions that need further discussion.

---

## Define Core vs Extended Boundary

**Problem:** We need a clear definition of what is "core vitae" vs what sits on top.

**Context:** Eventually targeting WASM/browser is of interest. In that world:
- No winit (browser handles the window)
- No native platform features (file dialogs, menus, etc.)
- Different rendering backend

**Core should be:**
- Platform-agnostic
- The element tree, layout, styling, builder API
- Event system (abstracted from input source)
- Signals / reactivity

**Not core:**
- winit integration
- Native platform features
- Specific rendering backend (vello)
- Routing, icons, components (already in extended.md)

**To decide:**
- Where exactly is the line?
- How do we structure crates to make this separation clean?
- What traits/abstractions are needed for platform backends?

---

## Name the "Batteries Included" Layer

**Problem:** Need a concrete name for the standard library / batteries-included layer.

**Examples from other ecosystems:**
- Svelte → SvelteKit
- Nuxt (Vue)
- Next (React)
- Rocket (Rust web) has `rocket` core and `rocket_contrib`

**Options to consider:**
- `vitae` (core) + `vitae_kit` (extended)
- `vitae` (core) + `vitae_studio` (extended) - creative software vibe
- `vitae` (core) + `vitae_full` (extended)
- `vitae_core` + `vitae` (flip it - `vitae` is the full experience)

**To decide:**
- What name?
- Does the extended layer re-export core, or are they separate imports?
- Feature flags vs separate crates?

---

## Context System + Model Access

**Problem:** How does context work, and how do views access models?

**Decided:**
- Context is framework-owned
- Context provides access to models, theme, routing, etc.
- View function is `fn(&Context) -> ElementBuilder`
- Child component functions take `ctx` as parameter - one argument unlocks everything
- Not hooks (`use_*`) - explicit is preferred over hidden thread-local state

**API direction:**
```rust
fn view(ctx: &Context) -> ElementBuilder {
    let user = ctx.model::<UserModel>();      // immutable access
    
    div()
        .child(header(ctx))
        .child(main_content(ctx))
}

fn header(ctx: &Context) -> ElementBuilder {
    let user = ctx.model::<UserModel>();
    div().child(text(&user.name))
}

// Mutable access (in handlers)
ctx.update::<CartModel>(|cart| {
    cart.add_item(item);
});
```

**Open questions:**
- How are models registered with context?
- Single model vs multiple models - how does this work with `App::new(model, view)`?
- Ownership: does context own the models, or just provide access?
- How do event handlers get mutable access? Same `ctx.update()` pattern?
- Relationship between signals (ephemeral) and models (persistent) in context

**Related:** This subsumes the Model Ownership question in core.md Topic 9.

---

## Unified Size System

**Decision:** Single `Size` enum with context-dependent values.

**How it works:**
- Theme defines mappings per context: `theme.icon_size(Size::Md, px(16.0))`
- Functions look up the relevant mapping: `icon().size(MD)` uses icon_size, `text().size(MD)` uses text_size
- Shorthand consts: `SM`, `MD`, `LG`, `XL` (like `FULL`, `HALF`)

**To decide:**
- Exact variants (Xs, Sm, Md, Lg, Xl, Xxl?)
- Default theme values
- How custom sizes work alongside the enum (e.g., `size(px(13.0))` vs `size(MD)`)

---

## Other Items

*(Add items here as they come up)*

---
