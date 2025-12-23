# Styling Methods

## Length Helpers

| Function | Description |
|----------|-------------|
| `px(value)` | Create a length in pixels |
| `pc(value)` | Create a length in percentage |

`Length::Auto` is the default for dimensions.

## ElementBuilder Methods

### Layout Direction

| Method | Description |
|--------|-------------|
| `.row()` | Render children in a row |
| `.col()` | Render children in a column (default) |
| `.direction(dir)` | Set direction with `Direction::Row` or `Direction::Column` |

### Sizing

| Method | Description |
|--------|-------------|
| `.w(length)` | Set width |
| `.h(length)` | Set height |
| `.size(length)` | Set both width and height |
| `.aspect_ratio(ratio)` | Set aspect ratio (only supply one dimension) |
| `.square()` | Set aspect ratio to 1:1 |

### Spacing

| Method | Description |
|--------|-------------|
| `.p(size)` | Set padding on all sides |
| `.m(size)` | Set margin on all sides |

### Appearance

| Method | Description |
|--------|-------------|
| `.bg(color)` | Set background color |
| `.font_size(size)` | Set font size for text elements |

### Children

| Method | Description |
|--------|-------------|
| `.child(element)` | Add a single child element |
| `.children(iter)` | Add multiple children from an iterator |

### Events

| Method | Description |
|--------|-------------|
| `.on_click(handler)` | Attach a click event handler |

## Style Properties (not yet exposed via builder)

These properties exist on `Style` but don't have builder methods yet:

- `text_color` - Text color (default: black)
- `wrap` - Enable wrapping (default: false)
- `reverse` - Reverse child order (default: false)
