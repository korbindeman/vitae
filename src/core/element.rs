use generational_arena::{Arena, Index};

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
    pub arena: Arena<Element>,
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
