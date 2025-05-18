use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::{Rc, Weak},
};

use glam::Vec2;

use super::{color::ColorRGBA, draw::DrawCommand, style::Style};

#[derive(Clone, Debug)]
pub enum Size {
    Percent(Vec2),
    // Pixel(Vec2), // TODO: maybe add this
}

impl Size {
    pub fn percent(width: f32, height: f32) -> Self {
        Self::Percent(glam::Vec2::new(width, height))
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
    parent: Option<Weak<RefCell<Element>>>,
    children: RefCell<Vec<Rc<RefCell<Element>>>>,
    anchor: Vec2,
    size: Size,
    style: Style,
    direction: Direction,
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
pub struct ElementHandle(Rc<RefCell<Element>>);

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

pub fn tree_to_draw_commands(root: &ElementHandle) -> Vec<DrawCommand> {
    /// depth-first walk that carries the parent’s absolute position.
    fn collect(node: &ElementHandle, parent_pos: Vec2, out: &mut Vec<DrawCommand>) {
        let node_ref = node.borrow();

        // absolute position of this element = parent absolute + local anchor
        let mut global_pos = parent_pos + node_ref.anchor;

        // generate the command using the global coordinate
        out.push(node_ref.get_draw_command(global_pos));

        // recurse into children
        for child_rc in node_ref.children.borrow().iter() {
            // wrap the raw Rc in our handle to reuse the same API
            let child = ElementHandle(child_rc.clone());
            collect(&child, global_pos, out);

            // add current element width to global_pos
            // TODO: replace with actual layout logic
            let current_anchor_offset = match node_ref.direction {
                Direction::Row => Vec2::X * child_rc.borrow().get_clip().0,
                Direction::Column => Vec2::Y * child_rc.borrow().get_clip().1,
            };
            global_pos += current_anchor_offset;
        }
    }

    let mut commands = Vec::new();
    collect(root, Vec2::ZERO, &mut commands);
    commands
}
