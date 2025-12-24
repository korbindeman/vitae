use crate::element::{ElementTree, NodeId, NodeKind};
use crate::style::{Direction, Length, Position};

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

    // Check if this is a text node and measure it if needed
    let (text_w, text_h) = match &tree.arena[id].kind {
        NodeKind::Text { content, .. } => {
            let max_w = match style.width {
                Length::Auto => Some(constraints.max_w),
                Length::Px(px) => Some(px),
                Length::Percent(p) => Some(p / 100.0 * constraints.max_w),
            };
            measurer.measure(content, max_w)
        }
        _ => (0.0, 0.0),
    };

    let margin_left = style.margin.left.as_px();
    let margin_right = style.margin.right.as_px();
    let margin_top = style.margin.top.as_px();
    let margin_bottom = style.margin.bottom.as_px();

    let padding_left = style.padding.left.as_px();
    let padding_right = style.padding.right.as_px();
    let padding_top = style.padding.top.as_px();
    let padding_bottom = style.padding.bottom.as_px();

    let mut w = match style.width {
        Length::Px(px) => px,
        Length::Auto => text_w,
        Length::Percent(percent) => percent / 100.0 * constraints.max_w,
    };

    let mut h = match style.height {
        Length::Px(py) => py,
        Length::Auto => text_h,
        Length::Percent(percent) => percent / 100.0 * constraints.max_h,
    };

    match style.aspect_ratio {
        Some(ratio) => {
            if w == 0.0 {
                w = h * ratio;
            } else if h == 0.0 {
                h = w / ratio;
            }
        }
        None => {}
    };

    let mut max_cross: f32 = 0.0;
    let mut main_total: f32 = 0.0;

    let mut child_cursor_x = cursor_x + margin_left + padding_left;
    let mut child_cursor_y = cursor_y + margin_top + padding_top;

    let children: Vec<NodeId> = tree.children(id).collect();
    let mut absolute_children: Vec<NodeId> = Vec::new();

    // First pass: layout relative children in normal flow
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
            Direction::Row => {
                child_cursor_x += cw;
                main_total += cw;
                max_cross = max_cross.max(ch);
            }
            Direction::Column => {
                child_cursor_y += ch;
                main_total += ch;
                max_cross = max_cross.max(cw);
            }
        }
    }

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

    // Measure text if this is a text node
    let (text_w, text_h) = match &tree.arena[id].kind {
        NodeKind::Text { content, .. } => {
            let max_w = match style.width {
                Length::Auto => Some(parent_w),
                Length::Px(px) => Some(px),
                Length::Percent(p) => Some(p / 100.0 * parent_w),
            };
            measurer.measure(content, max_w)
        }
        _ => (0.0, 0.0),
    };

    // Calculate width
    let mut w = match style.width {
        Length::Px(px) => px,
        Length::Auto => text_w,
        Length::Percent(percent) => percent / 100.0 * parent_w,
    };

    // Calculate height
    let mut h = match style.height {
        Length::Px(px) => px,
        Length::Auto => text_h,
        Length::Percent(percent) => percent / 100.0 * parent_h,
    };

    // Handle aspect ratio
    if let Some(ratio) = style.aspect_ratio {
        if w == 0.0 {
            w = h * ratio;
        } else if h == 0.0 {
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
