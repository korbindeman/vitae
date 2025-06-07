use crate::core::{
    element::{ElementId, ElementTree},
    style::{Direction, Length},
};

#[derive(Clone, Copy, Debug, Default)]
pub struct Layout {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Constraints {
    pub max_w: f32, // may be f32::INFINITY
    pub max_h: f32,
}

pub(crate) fn layout(
    tree: &mut ElementTree,
    id: ElementId,
    constraints: Constraints,
    cursor_x: f32,
    cursor_y: f32,
) -> (f32, f32) {
    // get style and direction first before any mutable borrows
    let style = { &tree.arena[id].style };
    let dir = style.direction;

    // extract margin and padding values
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
        Length::Auto => 0.,
        Length::Percent(percent) => percent / 100.0 * constraints.max_w,
    };

    let mut h = match style.height {
        Length::Px(py) => py,
        Length::Auto => 0.,
        Length::Percent(percent) => percent / 100.0 * constraints.max_h,
    };

    match style.aspect_ratio {
        // TODO: this might fail if auto length logic changes
        Some(ratio) => {
            if w == 0.0 {
                w = h * ratio;
            } else if h == 0.0 {
                h = w / ratio;
            }
        }
        None => {}
    };

    // visit children, stacking them Row- or Column-wise
    let mut max_cross: f32 = 0.0;
    let mut main_total: f32 = 0.0;

    // children start at the parent's position plus margin and padding
    let mut child_cursor_x = cursor_x + margin_left + padding_left;
    let mut child_cursor_y = cursor_y + margin_top + padding_top;

    // collect children first to avoid borrowing issues
    let children: Vec<ElementId> = tree.children(id).collect();
    for child in children {
        // child always gets *all* the remaining room on the cross axis, minus padding
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

    // if my own size was Auto, grow to fit children plus padding
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

    // add margin to the final size
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
