use generational_arena::{Arena, Index};

use crate::events::EventHandler;
use crate::layout::Layout;
use crate::style::Style;

pub type NodeId = Index;

#[derive(Clone)]
pub enum NodeKind {
    Element { style: Style },
    Text { content: String, style: Style },
}

// Manual Debug implementation to handle EventHandler
impl std::fmt::Debug for NodeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeKind::Element { style } => f.debug_struct("Element").field("style", style).finish(),
            NodeKind::Text { content, style } => f
                .debug_struct("Text")
                .field("content", content)
                .field("style", style)
                .finish(),
        }
    }
}

pub struct Node {
    // tree topology
    pub parent: Option<NodeId>,
    pub first_child: Option<NodeId>,
    pub next_sibling: Option<NodeId>,

    // data
    pub kind: NodeKind,
    pub layout: Layout,
    pub dirty: bool,

    // event handler
    pub on_event: Option<EventHandler>,
}

// Manual Debug implementation
impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("parent", &self.parent)
            .field("first_child", &self.first_child)
            .field("next_sibling", &self.next_sibling)
            .field("kind", &self.kind)
            .field("layout", &self.layout)
            .field("dirty", &self.dirty)
            .field("on_event", &self.on_event.as_ref().map(|_| "EventHandler"))
            .finish()
    }
}

impl Node {
    fn new_element(style: Style, parent: Option<NodeId>, on_event: Option<EventHandler>) -> Self {
        Self {
            parent,
            first_child: None,
            next_sibling: None,
            kind: NodeKind::Element { style },
            layout: Layout::default(),
            dirty: true,
            on_event,
        }
    }

    fn new_text(
        content: String,
        style: Style,
        parent: Option<NodeId>,
        on_event: Option<EventHandler>,
    ) -> Self {
        Self {
            parent,
            first_child: None,
            next_sibling: None,
            kind: NodeKind::Text { content, style },
            layout: Layout::default(),
            dirty: true,
            on_event,
        }
    }

    pub fn style(&self) -> Option<&Style> {
        match &self.kind {
            NodeKind::Element { style } => Some(style),
            NodeKind::Text { content: _, style } => Some(style),
        }
    }
}

pub struct ElementTree {
    pub arena: Arena<Node>,
    pub root: NodeId,
}

impl ElementTree {
    pub fn new(style: Style, on_click: Option<EventHandler>) -> Self {
        let mut arena = Arena::new();
        let root = arena.insert(Node::new_element(style, None, on_click));
        Self { arena, root }
    }

    pub fn add_child(
        &mut self,
        parent: NodeId,
        node_type: NodeKind,
        on_click: Option<EventHandler>,
    ) -> NodeId {
        let child_id = match node_type {
            NodeKind::Element { style } => {
                self.arena
                    .insert(Node::new_element(style, Some(parent), on_click))
            }
            NodeKind::Text { content, style } => {
                self.arena
                    .insert(Node::new_text(content, style, Some(parent), on_click))
            }
        };

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
                cur = self.arena[node].parent;
            } else {
                break;
            }
        }
    }

    pub fn get_node(&self, id: NodeId) -> &Node {
        &self.arena[id]
    }
}
