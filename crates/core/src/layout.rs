use crate::element::{ElementTree, NodeId, NodeKind};
use crate::style::{Align, Direction, Distribute, Length, Position};

#[derive(Clone, Copy, Debug, Default)]
pub struct Layout {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Constraints {
    pub max_w: f32,
    pub max_h: f32,
}

/// Trait for measuring text content dimensions.
/// Implemented by the renderer to provide font-aware text measurement.
pub trait TextMeasurer {
    fn measure(&mut self, text: &str, max_width: Option<f32>) -> (f32, f32);
}

/// No-op text measurer that returns zero dimensions.
pub struct NoOpMeasurer;

impl TextMeasurer for NoOpMeasurer {
    fn measure(&mut self, _text: &str, _max_width: Option<f32>) -> (f32, f32) {
        (0.0, 0.0)
    }
}

/// Main entry point for layout. Lays out the tree and handles portals.
pub fn layout<M: TextMeasurer>(
    tree: &mut ElementTree,
    id: NodeId,
    constraints: Constraints,
    cursor_x: f32,
    cursor_y: f32,
    measurer: &mut M,
) -> (f32, f32) {
    let mut portals = Vec::new();
    let result = layout_inner(
        tree,
        id,
        constraints,
        cursor_x,
        cursor_y,
        measurer,
        &mut portals,
    );

    // Layout portals relative to viewport (using root constraints)
    for portal_id in portals {
        layout_portal(
            tree,
            portal_id,
            constraints.max_w,
            constraints.max_h,
            measurer,
        );
    }

    result
}

/// Internal layout function that collects portals.
fn layout_inner<M: TextMeasurer>(
    tree: &mut ElementTree,
    id: NodeId,
    constraints: Constraints,
    cursor_x: f32,
    cursor_y: f32,
    measurer: &mut M,
    portals: &mut Vec<NodeId>,
) -> (f32, f32) {
    let node = &tree.arena[id];
    let style = node.style().unwrap().clone();
    let dir = style.direction;

    // Get intrinsic size based on node type
    let (intrinsic_w, intrinsic_h, intrinsic_aspect) = match &tree.arena[id].kind {
        NodeKind::Text { content, .. } => {
            let max_w = match style.width {
                Length::Auto => Some(constraints.max_w),
                Length::Px(px) => Some(px),
                Length::Percent(p) => Some(p / 100.0 * constraints.max_w),
            };
            let (w, h) = measurer.measure(content, max_w);
            (w, h, None)
        }
        NodeKind::Texture { texture, .. } => {
            let w = texture.width() as f32;
            let h = texture.height() as f32;
            (w, h, Some(texture.aspect_ratio()))
        }
        NodeKind::Svg { svg, .. } => {
            let w = svg.width();
            let h = svg.height();
            (w, h, Some(svg.aspect_ratio()))
        }
        _ => (0.0, 0.0, None),
    };

    let margin_left = style.margin.left.as_px();
    let margin_right = style.margin.right.as_px();
    let margin_top = style.margin.top.as_px();
    let margin_bottom = style.margin.bottom.as_px();

    let padding_left = style.padding.left.as_px();
    let padding_right = style.padding.right.as_px();
    let padding_top = style.padding.top.as_px();
    let padding_bottom = style.padding.bottom.as_px();

    // Determine if dimensions are explicitly set
    let width_is_auto = matches!(style.width, Length::Auto);
    let height_is_auto = matches!(style.height, Length::Auto);

    let mut w = match style.width {
        Length::Px(px) => px,
        Length::Auto => intrinsic_w,
        Length::Percent(percent) => percent / 100.0 * constraints.max_w,
    };

    let mut h = match style.height {
        Length::Px(py) => py,
        Length::Auto => intrinsic_h,
        Length::Percent(percent) => percent / 100.0 * constraints.max_h,
    };

    // Handle aspect ratio - explicit style takes precedence, then intrinsic
    let effective_aspect = style.aspect_ratio.or(intrinsic_aspect);
    if let Some(ratio) = effective_aspect {
        // For textures: if only one dimension is set, preserve aspect ratio
        if width_is_auto && !height_is_auto && h > 0.0 {
            w = h * ratio;
        } else if height_is_auto && !width_is_auto && w > 0.0 {
            h = w / ratio;
        } else if w == 0.0 && h > 0.0 {
            w = h * ratio;
        } else if h == 0.0 && w > 0.0 {
            h = w / ratio;
        }
    }

    let content_x = cursor_x + margin_left + padding_left;
    let content_y = cursor_y + margin_top + padding_top;

    let children: Vec<NodeId> = tree.children(id).collect();
    let mut absolute_children: Vec<NodeId> = Vec::new();
    let mut flow_children: Vec<NodeId> = Vec::new();
    let mut child_sizes: Vec<(f32, f32)> = Vec::new();

    let mut child_cursor_x = content_x;
    let mut child_cursor_y = content_y;

    // First pass: layout children sequentially (at Start alignment positions)
    for child in &children {
        let child_style = tree.arena[*child].style().unwrap();
        match child_style.position {
            Position::Absolute => {
                absolute_children.push(*child);
                continue;
            }
            Position::Portal => {
                portals.push(*child);
                continue;
            }
            Position::Relative => {}
        }

        let child_constraints = Constraints {
            max_w: w - padding_left - padding_right,
            max_h: h - padding_top - padding_bottom,
        };

        let (cw, ch) = layout_inner(
            tree,
            *child,
            child_constraints,
            child_cursor_x,
            child_cursor_y,
            measurer,
            portals,
        );

        flow_children.push(*child);
        child_sizes.push((cw, ch));

        // Advance cursor for next child
        match dir {
            Direction::Row => child_cursor_x += cw,
            Direction::Column => child_cursor_y += ch,
        }
    }

    // Calculate totals
    let mut main_total: f32 = 0.0;
    let mut max_cross: f32 = 0.0;
    for &(cw, ch) in &child_sizes {
        match dir {
            Direction::Row => {
                main_total += cw;
                max_cross = max_cross.max(ch);
            }
            Direction::Column => {
                main_total += ch;
                max_cross = max_cross.max(cw);
            }
        }
    }

    // Determine container size
    match dir {
        Direction::Row => {
            if w == 0.0 {
                w = main_total + padding_left + padding_right;
            }
            if h == 0.0 {
                h = max_cross + padding_top + padding_bottom;
            }
        }
        Direction::Column => {
            if w == 0.0 {
                w = max_cross + padding_left + padding_right;
            }
            if h == 0.0 {
                h = main_total + padding_top + padding_bottom;
            }
        }
    }

    let content_w = w - padding_left - padding_right;
    let content_h = h - padding_top - padding_bottom;

    // Calculate alignment offsets and apply to children
    let main_size = match dir {
        Direction::Row => content_w,
        Direction::Column => content_h,
    };
    let free_space = (main_size - main_total).max(0.0);
    let child_count = flow_children.len();

    // Main-axis offset for all children
    let (main_offset, main_gap) = match style.distribute {
        Distribute::Start => (0.0, 0.0),
        Distribute::End => (free_space, 0.0),
        Distribute::Center => (free_space / 2.0, 0.0),
        Distribute::Between => {
            if child_count > 1 {
                (0.0, free_space / (child_count - 1) as f32)
            } else {
                (0.0, 0.0)
            }
        }
        Distribute::Around => {
            let gap = free_space / child_count as f32;
            (gap / 2.0, gap)
        }
        Distribute::Evenly => {
            let gap = free_space / (child_count + 1) as f32;
            (gap, gap)
        }
    };

    // Apply alignment offsets to each child
    let mut accumulated_gap = 0.0;
    for (i, &child_id) in flow_children.iter().enumerate() {
        let (cw, ch) = child_sizes[i];

        // Cross-axis alignment offset
        let cross_offset = match dir {
            Direction::Row => match style.align {
                Align::Start => 0.0,
                Align::End => content_h - ch,
                Align::Center => (content_h - ch) / 2.0,
            },
            Direction::Column => match style.align {
                Align::Start => 0.0,
                Align::End => content_w - cw,
                Align::Center => (content_w - cw) / 2.0,
            },
        };

        // Calculate delta from where child was placed to where it should be
        let (dx, dy) = match dir {
            Direction::Row => (main_offset + accumulated_gap, cross_offset),
            Direction::Column => (cross_offset, main_offset + accumulated_gap),
        };

        // Apply offset if non-zero
        if dx != 0.0 || dy != 0.0 {
            offset_subtree(tree, child_id, dx, dy);
        }

        accumulated_gap += main_gap;
    }

    let final_w = w + margin_left + margin_right;
    let final_h = h + margin_top + margin_bottom;

    // Store parent layout before positioning absolute children
    let parent_x = cursor_x + margin_left;
    let parent_y = cursor_y + margin_top;
    let parent_w = w;
    let parent_h = h;

    tree.arena[id].layout = Layout {
        x: parent_x,
        y: parent_y,
        width: parent_w,
        height: parent_h,
    };

    // Second pass: layout absolute children relative to parent
    for child in absolute_children {
        layout_absolute(
            tree,
            child,
            parent_x + padding_left,
            parent_y + padding_top,
            parent_w - padding_left - padding_right,
            parent_h - padding_top - padding_bottom,
            measurer,
            portals,
        );
    }

    (final_w, final_h)
}

/// Layout a portal element relative to the viewport.
fn layout_portal<M: TextMeasurer>(
    tree: &mut ElementTree,
    id: NodeId,
    viewport_w: f32,
    viewport_h: f32,
    measurer: &mut M,
) {
    // Portals are laid out exactly like absolute elements, but relative to viewport
    let mut nested_portals = Vec::new();
    layout_positioned(
        tree,
        id,
        0.0,
        0.0,
        viewport_w,
        viewport_h,
        measurer,
        &mut nested_portals,
    );

    // Layout any nested portals (they also use viewport coordinates)
    for nested_id in nested_portals {
        layout_portal(tree, nested_id, viewport_w, viewport_h, measurer);
    }
}

/// Layout an absolutely positioned element within its parent's content box.
fn layout_absolute<M: TextMeasurer>(
    tree: &mut ElementTree,
    id: NodeId,
    parent_x: f32,
    parent_y: f32,
    parent_w: f32,
    parent_h: f32,
    measurer: &mut M,
    portals: &mut Vec<NodeId>,
) {
    layout_positioned(
        tree, id, parent_x, parent_y, parent_w, parent_h, measurer, portals,
    );
}

/// Shared logic for positioning absolute and portal elements.
fn layout_positioned<M: TextMeasurer>(
    tree: &mut ElementTree,
    id: NodeId,
    parent_x: f32,
    parent_y: f32,
    parent_w: f32,
    parent_h: f32,
    measurer: &mut M,
    portals: &mut Vec<NodeId>,
) {
    let node = &tree.arena[id];
    let style = node.style().unwrap().clone();

    // Get intrinsic size based on node type
    let (intrinsic_w, intrinsic_h, intrinsic_aspect) = match &tree.arena[id].kind {
        NodeKind::Text { content, .. } => {
            let max_w = match style.width {
                Length::Auto => Some(parent_w),
                Length::Px(px) => Some(px),
                Length::Percent(p) => Some(p / 100.0 * parent_w),
            };
            let (w, h) = measurer.measure(content, max_w);
            (w, h, None)
        }
        NodeKind::Texture { texture, .. } => {
            let w = texture.width() as f32;
            let h = texture.height() as f32;
            (w, h, Some(texture.aspect_ratio()))
        }
        NodeKind::Svg { svg, .. } => {
            let w = svg.width();
            let h = svg.height();
            (w, h, Some(svg.aspect_ratio()))
        }
        _ => (0.0, 0.0, None),
    };

    // Determine if dimensions are explicitly set
    let width_is_auto = matches!(style.width, Length::Auto);
    let height_is_auto = matches!(style.height, Length::Auto);

    // Calculate width
    let mut w = match style.width {
        Length::Px(px) => px,
        Length::Auto => intrinsic_w,
        Length::Percent(percent) => percent / 100.0 * parent_w,
    };

    // Calculate height
    let mut h = match style.height {
        Length::Px(px) => px,
        Length::Auto => intrinsic_h,
        Length::Percent(percent) => percent / 100.0 * parent_h,
    };

    // Handle aspect ratio - explicit style takes precedence, then intrinsic
    let effective_aspect = style.aspect_ratio.or(intrinsic_aspect);
    if let Some(ratio) = effective_aspect {
        if width_is_auto && !height_is_auto && h > 0.0 {
            w = h * ratio;
        } else if height_is_auto && !width_is_auto && w > 0.0 {
            h = w / ratio;
        } else if w == 0.0 && h > 0.0 {
            w = h * ratio;
        } else if h == 0.0 && w > 0.0 {
            h = w / ratio;
        }
    }

    // If width is still auto and both left and right are set, stretch
    if w == 0.0 {
        if let (Some(left), Some(right)) = (&style.left, &style.right) {
            let left_px = resolve_length(left, parent_w);
            let right_px = resolve_length(right, parent_w);
            w = parent_w - left_px - right_px;
        }
    }

    // If height is still auto and both top and bottom are set, stretch
    if h == 0.0 {
        if let (Some(top), Some(bottom)) = (&style.top, &style.bottom) {
            let top_px = resolve_length(top, parent_h);
            let bottom_px = resolve_length(bottom, parent_h);
            h = parent_h - top_px - bottom_px;
        }
    }

    // Calculate x position
    let x = if let Some(left) = &style.left {
        parent_x + resolve_length(left, parent_w)
    } else if let Some(right) = &style.right {
        parent_x + parent_w - w - resolve_length(right, parent_w)
    } else {
        parent_x // Default to parent's left edge
    };

    // Calculate y position
    let y = if let Some(top) = &style.top {
        parent_y + resolve_length(top, parent_h)
    } else if let Some(bottom) = &style.bottom {
        parent_y + parent_h - h - resolve_length(bottom, parent_h)
    } else {
        parent_y // Default to parent's top edge
    };

    tree.arena[id].layout = Layout {
        x,
        y,
        width: w,
        height: h,
    };

    // Layout children of this absolute element
    let padding_left = style.padding.left.as_px();
    let padding_right = style.padding.right.as_px();
    let padding_top = style.padding.top.as_px();
    let padding_bottom = style.padding.bottom.as_px();

    let children: Vec<NodeId> = tree.children(id).collect();
    let mut absolute_children: Vec<NodeId> = Vec::new();
    let mut child_cursor_x = x + padding_left;
    let mut child_cursor_y = y + padding_top;
    let dir = style.direction;

    for child in &children {
        let child_style = tree.arena[*child].style().unwrap();
        match child_style.position {
            Position::Absolute => {
                absolute_children.push(*child);
                continue;
            }
            Position::Portal => {
                portals.push(*child);
                continue;
            }
            Position::Relative => {}
        }

        let child_constraints = Constraints {
            max_w: w - padding_left - padding_right,
            max_h: h - padding_top - padding_bottom,
        };

        let (cw, ch) = layout_inner(
            tree,
            *child,
            child_constraints,
            child_cursor_x,
            child_cursor_y,
            measurer,
            portals,
        );

        match dir {
            Direction::Row => child_cursor_x += cw,
            Direction::Column => child_cursor_y += ch,
        }
    }

    // Layout absolute children of this positioned element
    for child in absolute_children {
        layout_absolute(
            tree,
            child,
            x + padding_left,
            y + padding_top,
            w - padding_left - padding_right,
            h - padding_top - padding_bottom,
            measurer,
            portals,
        );
    }
}

/// Resolve a Length to pixels given a parent dimension.
fn resolve_length(length: &Length, parent_size: f32) -> f32 {
    match length {
        Length::Px(px) => *px,
        Length::Percent(p) => p / 100.0 * parent_size,
        Length::Auto => 0.0,
    }
}

/// Recursively offset a node and all its descendants.
fn offset_subtree(tree: &mut ElementTree, id: NodeId, dx: f32, dy: f32) {
    tree.arena[id].layout.x += dx;
    tree.arena[id].layout.y += dy;

    let children: Vec<NodeId> = tree.children(id).collect();
    for child in children {
        offset_subtree(tree, child, dx, dy);
    }
}
