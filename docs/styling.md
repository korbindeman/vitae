# Styling Methods

## Length Helpers

| Function | Description |
|----------|-------------|
| `px(value)` | Create a length in pixels |
| `pc(value)` | Create a length in percentage |

`Length::Auto` is the default for dimensions.

## Color Helpers

| Method | Description |
|--------|-------------|
| `Color::rgb(r, g, b)` | Create color from RGB values (0-255) |
| `Color::from_hex("#rrggbb")` | Create color from hex string |
| `Color::new(r, g, b, a)` | Create color from RGBA floats (0.0-1.0) |

Predefined constants: `WHITE`, `BLACK`, `GRAY`, `RED`, `GREEN`, `BLUE`, `YELLOW`, `CYAN`, `MAGENTA`, `TRANSPARENT`

## ElementBuilder Methods

### Layout Direction

| Method | Description |
|--------|-------------|
| `.row()` | Render children in a row |
| `.col()` | Render children in a column (default) |
| `.direction(dir)` | Set direction with `Direction::Row` or `Direction::Column` |

### Alignment

| Method | Description |
|--------|-------------|
| `.align(align)` | Set cross-axis alignment (CSS: `align-items`) |
| `.distribute(dist)` | Set main-axis distribution (CSS: `justify-content`) |
| `.center()` | Center children on both axes |

**Align values** (`Align`):
- `Start` (default) - Align to start of cross axis
- `Center` - Center on cross axis
- `End` - Align to end of cross axis

**Distribute values** (`Distribute`):
- `Start` (default) - Pack children at start of main axis
- `Center` - Center children on main axis
- `End` - Pack children at end of main axis
- `Between` - Equal space between children, no space at edges
- `Around` - Equal space around each child (half-size space at edges)
- `Evenly` - Equal space between children and at edges

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
| `.gap(length)` | Set gap between children on both axes |
| `.gap_x(length)` | Set horizontal gap between children |
| `.gap_y(length)` | Set vertical gap between children |

### Positioning

| Method | Description |
|--------|-------------|
| `.position(pos)` | Set position mode (`Position::Relative`, `Position::Absolute`, or `Position::Portal`) |
| `.absolute()` | Shorthand for `.position(Position::Absolute)` |
| `.top(length)` | Set top offset (for absolute/portal positioning) |
| `.right(length)` | Set right offset (for absolute/portal positioning) |
| `.bottom(length)` | Set bottom offset (for absolute/portal positioning) |
| `.left(length)` | Set left offset (for absolute/portal positioning) |

**Position modes:**
- `Relative` (default) - Element participates in normal flow
- `Absolute` - Element is removed from flow, positioned relative to parent
- `Portal` - Element is positioned relative to viewport, rendered on top of everything

**Behavior:**
- Absolute elements don't affect sibling layout
- If both `left` and `right` are set with `Auto` width, the element stretches
- If both `top` and `bottom` are set with `Auto` height, the element stretches

### Element Constructors

| Function | Description |
|----------|-------------|
| `div()` | Create a div element |
| `text(content)` | Create a text element |
| `img(texture)` | Create an image element from a `Texture` |
| `svg(svg)` | Create an SVG element from an `Svg` |
| `portal()` | Create a portal element (positioned relative to viewport) |

### Appearance

| Method | Description |
|--------|-------------|
| `.bg(color)` | Set background color |
| `.color(color)` | Set text color |
| `.font_size(size)` | Set font size for text elements |
| `.opacity(value)` | Set opacity (0.0 = transparent, 1.0 = opaque) |

Opacity is inherited by children. A parent with `.opacity(0.5)` containing a child with `.opacity(0.5)` results in the child having an effective opacity of 0.25.

### Borders

| Method | Description |
|--------|-------------|
| `.border(width, color)` | Set border on all sides |
| `.border_t(width, color)` | Set top border |
| `.border_r(width, color)` | Set right border |
| `.border_b(width, color)` | Set bottom border |
| `.border_l(width, color)` | Set left border |

### Border Radius

| Method | Description |
|--------|-------------|
| `.radius(radius)` | Set border radius for all corners |
| `.radius_tl(radius)` | Set top-left corner radius |
| `.radius_tr(radius)` | Set top-right corner radius |
| `.radius_br(radius)` | Set bottom-right corner radius |
| `.radius_bl(radius)` | Set bottom-left corner radius |
| `.rounded()` | Make fully rounded (50% of smaller dimension) |

Border radius clips the background even without a border. Borders are drawn on top of the background.

### Children

| Method | Description |
|--------|-------------|
| `.child(element)` | Add a single child element |
| `.children(iter)` | Add multiple children from an iterator |

### Events

| Method | Description |
|--------|-------------|
| `.on_event(handler)` | Attach a generic event handler |
| `.on_left_click(handler)` | Attach a left click handler |
| `.on_right_click(handler)` | Attach a right click handler |

## Style Properties (not yet exposed via builder)

These properties exist on `Style` but don't have builder methods yet:

- `wrap` - Enable wrapping (default: false)
- `reverse` - Reverse child order (default: false)
