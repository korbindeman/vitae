use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::{Rc, Weak},
};

use glam::Vec2;

use super::draw::DrawCommand;

#[derive(Clone, Debug)]
pub enum Size {
    Percentage(Vec2),
    // Pixel(Vec2), // TODO: maybe add this
}

#[derive(Clone, Debug)]
pub struct Element {
    parent: Option<Weak<RefCell<Element>>>,
    children: RefCell<Vec<Rc<RefCell<Element>>>>,
    anchor: Vec2,
    color: [f32; 4],
    size: Size,
}

impl Element {
    pub fn get_draw_command(&self, global_anchor: Vec2) -> DrawCommand {
        let (w_clip, h_clip) = match &self.size {
            Size::Percentage(v) => (v.x / 50.0, v.y / 50.0),
        };

        DrawCommand::Rect {
            x: -1.0 + global_anchor.x,
            y: 1.0 - h_clip - global_anchor.y,
            width: w_clip,
            height: h_clip,
            color: self.color,
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
    pub fn new_root(anchor: Vec2, color: [f32; 4], size: Size) -> Self {
        Rc::new(RefCell::new(Element {
            parent: None,
            children: RefCell::new(Vec::new()),
            anchor,
            color,
            size,
        }))
        .into()
    }

    pub fn make_child(&self, anchor: Vec2, color: [f32; 4], size: Size) -> Self {
        let child_rc = Rc::new(RefCell::new(Element {
            parent: Some(Rc::downgrade(&self.0)),
            children: RefCell::new(Vec::new()),
            anchor,
            color,
            size,
        }));

        self.0
            .borrow_mut()
            .children
            .borrow_mut()
            .push(child_rc.clone());

        ElementHandle(child_rc)
    }

    pub fn add_child(&self, child: Self) {
        child.0.borrow_mut().parent = Some(Rc::downgrade(&self.0));
        self.0.borrow().children.borrow_mut().push(child.0.clone());
    }
}

pub fn tree_to_draw_commands(root: &ElementHandle) -> Vec<DrawCommand> {
    /// depth-first walk that carries the parent’s absolute position.
    fn collect(node: &ElementHandle, parent_pos: Vec2, out: &mut Vec<DrawCommand>) {
        let node_ref = node.borrow();

        // absolute position of this element = parent absolute + local anchor
        let global_pos = parent_pos + node_ref.anchor;

        // generate the command using the global coordinate
        out.push(node_ref.get_draw_command(global_pos));

        // recurse into children
        for child_rc in node_ref.children.borrow().iter() {
            // wrap the raw Rc in our handle to reuse the same API
            let child = ElementHandle(child_rc.clone());
            collect(&child, global_pos, out);
        }
    }

    let mut commands = Vec::new();
    collect(root, Vec2::ZERO, &mut commands);
    commands
}
