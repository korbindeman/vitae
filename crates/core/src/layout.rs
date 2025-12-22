use crate::element::{ElementTree, NodeId, NodeKind};
use crate::style::{Direction, Length};

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

pub fn layout<M: TextMeasurer>(
    tree: &mut ElementTree,
    id: NodeId,
    constraints: Constraints,
    cursor_x: f32,
    cursor_y: f32,
    measurer: &mut M,
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
    for child in children {
        let child_constraints = if dir == Direction::Row {
            Constraints {
                max_w: w - padding_left - padding_right,
                max_h: h - padding_top - padding_bottom,
            }
        } else {
            Constraints {
                max_w: w - padding_left - padding_right,
                max_h: h - padding_top - padding_bottom,
            }
        };

        let (cw, ch) = layout(
            tree,
            child,
            child_constraints,
            child_cursor_x,
            child_cursor_y,
            measurer,
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

    tree.arena[id].layout = Layout {
        x: cursor_x + margin_left,
        y: cursor_y + margin_top,
        width: w,
        height: h,
    };
    (final_w, final_h)
}
