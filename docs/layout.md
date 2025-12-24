# Layout Algorithm

Vitae uses a single-pass layout algorithm inspired by flexbox. Layout is computed recursively from the root element down through the tree.

## Overview

The layout system computes four values for each element:
- `x`, `y` - Position relative to the viewport
- `width`, `height` - Dimensions of the element

## Position Modes

### Relative (default)

Elements participate in normal flow. They are positioned by the parent based on the layout direction and accumulate along the main axis.

### Absolute

Elements are removed from normal flow and positioned relative to their parent's content box (after padding). They do not affect sibling layout.

### Portal

Elements are positioned relative to the viewport (0, 0) and rendered on top of all other content. Useful for overlays, modals, and fixed UI elements like toolbars.

## Layout Direction

Each element has a direction that determines how children are arranged:

- **Column** (default) - Children stack vertically, top to bottom
- **Row** - Children stack horizontally, left to right

## Alignment

Children can be aligned within their parent using two properties:

### Distribute (main axis)

Controls how children are distributed along the main axis (horizontal for row, vertical for column). Equivalent to CSS `justify-content`.

- `Start` (default) - Pack children at the start
- `Center` - Center children
- `End` - Pack children at the end
- `Between` - Equal space between children, no space at edges
- `Around` - Equal space around each child (half-size space at edges)
- `Evenly` - Equal space between children and at edges

### Align (cross axis)

Controls how children are aligned on the cross axis (vertical for row, horizontal for column). Equivalent to CSS `align-items`.

- `Start` (default) - Align to start
- `Center` - Center on cross axis
- `End` - Align to end

### Center shorthand

The `.center()` method sets both `align` and `distribute` to `Center`, centering children on both axes.

## Sizing

### Explicit Sizes

- `Length::Px(n)` - Fixed pixel size
- `Length::Percent(n)` - Percentage of parent's size (in that dimension)

### Auto Sizing

When width or height is `Length::Auto`:

1. **Text elements** - Size is measured using the text measurer (font-aware)
2. **Container elements** - Size shrinks to fit content:
   - In the main axis: sum of children sizes
   - In the cross axis: maximum child size

### Aspect Ratio

If `aspect_ratio` is set and one dimension is zero:
- If width is 0: `width = height * ratio`
- If height is 0: `height = width / ratio`

## Box Model

Each element has margin and padding:

```
┌─────────────────────────────────┐
│           margin                │
│   ┌─────────────────────────┐   │
│   │        padding          │   │
│   │   ┌─────────────────┐   │   │
│   │   │     content     │   │   │
│   │   └─────────────────┘   │   │
│   │                         │   │
│   └─────────────────────────┘   │
│                                 │
└─────────────────────────────────┘
```

- **Margin** - Space outside the element, affects position of subsequent siblings
- **Padding** - Space inside the element, affects position of children

The returned size from layout includes margins. The stored `Layout` dimensions do not include margins (just the border box).

## Layout Algorithm

### Phase 1: Normal Flow

For each element:

1. Clone the style and determine direction
2. Measure text content if this is a text node
3. Extract margin and padding values
4. Resolve explicit width/height from style
5. Apply aspect ratio if set
6. Iterate children:
   - Skip `Absolute` children (collect for phase 2)
   - Skip `Portal` children (collect for root-level processing)
   - Recursively layout `Relative` children
   - Accumulate main axis total and track cross axis maximum
7. If width/height is still auto, size to content
8. Store the computed layout
9. Layout absolute children (phase 2)

### Phase 2: Absolute Positioning

For each absolute child:

1. Resolve width/height (may stretch if both left+right or top+bottom are set)
2. Calculate x position:
   - If `left` is set: `parent_x + left`
   - Else if `right` is set: `parent_x + parent_width - width - right`
   - Else: `parent_x` (default to left edge)
3. Calculate y position:
   - If `top` is set: `parent_y + top`
   - Else if `bottom` is set: `parent_y + parent_height - height - bottom`
   - Else: `parent_y` (default to top edge)
4. Store layout and recursively layout children

### Phase 3: Portal Positioning

After the entire tree is laid out, portals are positioned:

1. Collect all portal elements encountered during traversal
2. Position each portal relative to the viewport (0, 0)
3. Use the same logic as absolute positioning, but with viewport dimensions

## Rendering Order

Elements are rendered in tree order (depth-first), with two exceptions:

1. **Absolute elements** - Rendered in tree order within their parent
2. **Portals** - Collected and rendered last, after the entire normal tree

This ensures portals always appear on top of regular content.

## Hit Testing

Hit testing follows the same order as rendering but in reverse for overlapping elements:

1. Check portals first (last rendered = frontmost)
2. Check normal tree, with children before parents (deeper = frontmost)

## Gap

The `gap`, `gap_x`, and `gap_y` properties add fixed spacing between children:

- `gap_x` - Horizontal gap (applies in Row direction)
- `gap_y` - Vertical gap (applies in Column direction)
- `gap` - Sets both `gap_x` and `gap_y`

Gap is additive with `distribute` spacing. For example, using `.gap(px(10))` with `.distribute(Distribute::Between)` will add 10px of fixed spacing plus the distributed free space between children.

## Limitations

Current limitations of the layout system:

- No flex-grow or flex-shrink
- No wrapping (wrap property exists but is not implemented)
- No min/max width/height constraints
