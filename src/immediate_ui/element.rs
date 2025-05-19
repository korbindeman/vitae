use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::{Rc, Weak},
};

use glam::Vec2;

use super::{builder::ElementBuilder, color::ColorRGBA, draw::DrawCommand, style::Style};

#[derive(Clone, Debug)]
pub enum Size {
    Percent(Vec2),
    // Pixel(Vec2), // TODO: maybe add this
}

impl Size {
    pub fn percent(width: f32, height: f32) -> Self {
        Self::Percent(glam::Vec2::new(width, height))
    }

    pub fn fraction(&self) -> Vec2 {
        match self {
            Size::Percent(p) => *p / 100.0,
        }
    }

    pub const FULL: Self = Self::Percent(glam::Vec2::splat(100.));
}

#[derive(Clone, Debug)]
pub enum Direction {
    Row,
    Column,
}

#[derive(Clone, Debug)]
pub struct Element {
    pub parent: Option<Weak<RefCell<Element>>>,
    pub children: RefCell<Vec<Rc<RefCell<Element>>>>,
    pub anchor: Vec2,
    pub size: Size,
    pub style: Style,
    pub direction: Direction,
}

impl Element {
    pub fn get_draw_command(&self, global_anchor: Vec2) -> DrawCommand {
        let (w_clip, h_clip) = self.get_clip();

        DrawCommand::Rect {
            x: -1.0 + global_anchor.x,
            y: 1.0 - h_clip - global_anchor.y,
            width: w_clip,
            height: h_clip,
            color: self.style.bg_color.to_struct(),
        }
    }

    pub fn get_clip(&self) -> (f32, f32) {
        match &self.size {
            Size::Percent(v) => (v.x / 50.0, v.y / 50.0),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ElementHandle(pub Rc<RefCell<Element>>);

impl Deref for ElementHandle {
    type Target = Rc<RefCell<Element>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for ElementHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<Rc<RefCell<Element>>> for ElementHandle {
    fn from(rc: Rc<RefCell<Element>>) -> Self {
        Self(rc)
    }
}

impl ElementHandle {
    pub fn new_root(anchor: Vec2, color: ColorRGBA, size: Size, direction: Direction) -> Self {
        Rc::new(RefCell::new(Element {
            parent: None,
            children: RefCell::new(Vec::new()),
            anchor,
            size,
            style: Style::from_bg_color(color),
            direction,
        }))
        .into()
    }

    pub fn child(&self, child: Self) -> Self {
        child.0.borrow_mut().parent = Some(Rc::downgrade(&self.0));
        self.0.borrow().children.borrow_mut().push(child.0.clone());
        child
    }
}

pub fn tree_to_draw_commands(root_builder: ElementBuilder, screen_px: Vec2) -> Vec<DrawCommand> {
    let root = root_builder.build();
    let mut out = Vec::new();

    /// depth-first layout & paint
    fn collect(
        node: &ElementHandle,
        origin: Vec2,    // absolute pos in px
        parent_px: Vec2, // parent w×h in px
        screen_px: Vec2, // screen dimensions in px
        out: &mut Vec<DrawCommand>,
    ) {
        let n = node.borrow();
        let frac = n.size.fraction();
        let my_size = parent_px * frac; // own w×h in px
        let my_origin = origin + n.anchor; // anchor is already in px

        // 1) paint myself ---------------------------------------------------
        out.push(DrawCommand::Rect {
            x: -1.0 + 2.0 * (my_origin.x / screen_px.x),
            y: 1.0 - 2.0 * ((my_origin.y + my_size.y) / screen_px.y),
            width: 2.0 * (my_size.x / screen_px.x),
            height: 2.0 * (my_size.y / screen_px.y),
            color: n.style.bg_color.to_struct(),
        });

        // 2) lay out children ----------------------------------------------
        let mut cursor = Vec2::ZERO;
        for child_rc in n.children.borrow().iter() {
            let child = ElementHandle(child_rc.clone());

            let child_origin = my_origin + cursor;
            collect(&child, child_origin, my_size, screen_px, out); // recurse

            // advance cursor by child's *actual* pixel size
            let c_frac = child_rc.borrow().size.fraction();
            let c_px = my_size * c_frac;
            cursor += match n.direction {
                Direction::Row => Vec2::new(c_px.x, 0.0),
                Direction::Column => Vec2::new(0.0, c_px.y),
            };
        }
    }

    collect(&root, Vec2::ZERO, screen_px, screen_px, &mut out);
    out
}
