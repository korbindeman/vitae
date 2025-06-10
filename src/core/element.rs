use generational_arena::{Arena, Index};

use super::layout::Layout;
use super::style::Style;

pub type NodeId = Index;

#[derive(Debug)]
pub enum NodeKind {
    Element { style: Style },
    Text { content: String },
}

#[derive(Debug)]
pub struct Node {
    // tree topology
    pub parent: Option<NodeId>,
    pub first_child: Option<NodeId>,
    pub next_sibling: Option<NodeId>,

    // data
    pub kind: NodeKind,
    pub layout: Layout, // mutated by layout pass
    pub dirty: bool,    // marks subtree needing re-layout
}

impl Node {
    fn new_element(style: Style, parent: Option<NodeId>) -> Self {
        Self {
            parent,
            first_child: None,
            next_sibling: None,
            kind: NodeKind::Element { style },
            layout: Layout::default(),
            dirty: true,
        }
    }

    fn new_text(content: String, parent: Option<NodeId>) -> Self {
        Self {
            parent,
            first_child: None,
            next_sibling: None,
            kind: NodeKind::Text { content },
            layout: Layout::default(),
            dirty: true,
        }
    }

    pub fn style(&self) -> Option<&Style> {
        match &self.kind {
            NodeKind::Element { style } => Some(style),
            _ => None,
        }
    }
}

pub struct ElementTree {
    pub arena: Arena<Node>,
    pub root: NodeId,
}

impl ElementTree {
    pub fn new(style: Style) -> Self {
        let mut arena = Arena::new();
        let root = arena.insert(Node::new_element(style, None));
        Self { arena, root }
    }

    pub fn add_child(&mut self, parent: NodeId, style: Style) -> NodeId {
        let child_id = self.arena.insert(Node::new_element(style, Some(parent)));

        // intrusive linked list: prepend
        if let Some(first) = self.arena[parent].first_child.replace(child_id) {
            self.arena[child_id].next_sibling = Some(first);
        }
        child_id
    }

    pub fn remove_subtree(&mut self, id: NodeId) {
        // depth-first delete children first
        while let Some(child) = self.arena[id].first_child {
            self.remove_subtree(child);
        }
        self.arena.remove(id);
    }

    pub fn children<'a>(&'a self, id: NodeId) -> impl Iterator<Item = NodeId> + 'a {
        std::iter::successors(self.arena[id].first_child, move |cur| {
            self.arena[*cur].next_sibling
        })
    }

    fn _mark_dirty(&mut self, id: NodeId) {
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

    pub fn get_node(&self, id: NodeId) -> &Node {
        &self.arena[id]
    }
}
