use generational_arena::{Arena, Index};

use crate::immediate_ui::layout::Constraints;
use crate::immediate_ui::style::{Direction, Length};

use super::layout::Layout;
use super::style::Style;

pub type ElementId = Index;

#[derive(Debug)]
pub struct Element {
    // tree topology
    pub parent: Option<ElementId>,
    pub first_child: Option<ElementId>,
    pub next_sibling: Option<ElementId>,

    // data
    pub style: Style,   // immutable except through API
    pub layout: Layout, // mutated by layout pass
    pub dirty: bool,    // marks subtree needing re-layout
}

impl Element {
    fn new(style: Style, parent: Option<ElementId>) -> Self {
        Self {
            parent,
            first_child: None,
            next_sibling: None,
            style,
            layout: Layout::default(),
            dirty: true,
        }
    }
}

pub struct ElementTree {
    arena: Arena<Element>,
    pub root: ElementId,
}

impl ElementTree {
    pub fn new(style: Style) -> Self {
        let mut arena = Arena::new();
        let root = arena.insert(Element::new(style, None));
        Self { arena, root }
    }

    pub fn add_child(&mut self, parent: ElementId, style: Style) -> ElementId {
        let child_id = self.arena.insert(Element::new(style, Some(parent)));

        // intrusive linked list: prepend
        if let Some(first) = self.arena[parent].first_child.replace(child_id) {
            self.arena[child_id].next_sibling = Some(first);
        }
        child_id
    }

    pub fn remove_subtree(&mut self, id: ElementId) {
        // depth-first delete children first
        while let Some(child) = self.arena[id].first_child {
            self.remove_subtree(child);
        }
        self.arena.remove(id);
    }

    pub fn children<'a>(&'a self, id: ElementId) -> impl Iterator<Item = ElementId> + 'a {
        std::iter::successors(self.arena[id].first_child, move |cur| {
            self.arena[*cur].next_sibling
        })
    }

    fn _mark_dirty(&mut self, id: ElementId) {
        let mut cur = Some(id);
        while let Some(node) = cur {
            if !self.arena[node].dirty {
                self.arena[node].dirty = true;
                cur = self.arena[node].parent; // bubble up
            } else {
                break;
            } // ancestor already dirty
        }
    }

    pub fn get_node(&self, id: ElementId) -> &Element {
        &self.arena[id]
    }
}

// TODO: this should be somewhere else
pub fn layout(
    tree: &mut ElementTree,
    id: ElementId,
    constraints: Constraints,
    cursor_x: f32,
    cursor_y: f32,
) -> (f32, f32) {
    // Get style and direction first before any mutable borrows
    let style = { &tree.arena[id].style };
    let dir = style.direction;

    // Extract margin and padding values
    let margin_left = style.margin.left.as_px();
    let margin_right = style.margin.right.as_px();
    let margin_top = style.margin.top.as_px();
    let margin_bottom = style.margin.bottom.as_px();

    let padding_left = style.padding.left.as_px();
    let padding_right = style.padding.right.as_px();
    let padding_top = style.padding.top.as_px();
    let padding_bottom = style.padding.bottom.as_px();

    // 1. Resolve my own size (Px takes value, Auto = shrink-to-fit)
    let (mut w, mut h) = match (style.width, style.height) {
        (Length::Px(px), Length::Px(py)) => (px, py),
        (Length::Px(px), Length::Auto) => (px, 0.0),
        (Length::Auto, Length::Px(py)) => (0.0, py),
        _ => (0.0, 0.0), // both Auto for now
    };

    // 2. Visit children, stacking them Row- or Column-wise
    let mut max_cross: f32 = 0.0;
    let mut main_total: f32 = 0.0;

    // Children start at the parent's position plus margin and padding
    let mut child_cursor_x = cursor_x + margin_left + padding_left;
    let mut child_cursor_y = cursor_y + margin_top + padding_top;

    // Collect children first to avoid borrowing issues
    let children: Vec<ElementId> = tree.children(id).collect();
    for child in children {
        // child always gets *all* the remaining room on the cross axis, minus padding
        let child_constraints = if dir == Direction::Row {
            Constraints {
                max_w: constraints.max_w - padding_left - padding_right,
                max_h: constraints.max_h - padding_top - padding_bottom,
            }
        } else {
            Constraints {
                max_w: constraints.max_w - padding_left - padding_right,
                max_h: constraints.max_h - padding_top - padding_bottom,
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

    // 3. If my own size was Auto, grow to fit children plus padding
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

    // Add margin to the final size
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
