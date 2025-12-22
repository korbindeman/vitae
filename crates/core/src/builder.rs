use crate::element::NodeKind;
use crate::style::EdgeSizes;

use crate::color::Color;
use crate::element::ElementTree;
use crate::style::{Direction, Length, Style};

#[derive(Clone, Debug)]
enum ElementKind {
    Element,
    Text,
}

#[derive(Clone, Debug)]
pub struct ElementBuilder {
    node_type: ElementKind,
    style: Style,
    text: Option<String>,
    children: Vec<ElementBuilder>,
}

impl ElementBuilder {
    pub fn new() -> Self {
        Self {
            node_type: ElementKind::Element,
            style: Style::default(),
            text: None,
            children: Vec::new(),
        }
    }

    pub fn new_text(text: String) -> Self {
        Self {
            node_type: ElementKind::Text,
            style: Style::default(),
            text: Some(text),
            children: Vec::new(),
        }
    }

    /// Make the element render children in a row.
    pub fn row(mut self) -> Self {
        self.style.direction = Direction::Row;
        self
    }

    /// Make the element render children in a column.
    pub fn col(mut self) -> Self {
        self.style.direction = Direction::Column;
        self
    }

    /// Make the element render children in a direction.
    pub fn direction(mut self, dir: Direction) -> Self {
        self.style.direction = dir;
        self
    }

    /// The background color of the element.
    pub fn bg(mut self, color: Color) -> Self {
        self.style.bg_color = color;
        self
    }

    /// Set the width of the element.
    pub fn w(mut self, length: Length) -> Self {
        self.style.width = length;
        self
    }

    /// Set the height of the element.
    pub fn h(mut self, length: Length) -> Self {
        self.style.height = length;
        self
    }

    /// Set the width and height of the element simultaneously.
    pub fn size(mut self, size: Length) -> Self {
        self.style.width = size;
        self.style.height = size;
        self
    }

    /// Set the aspect ratio of the element. Only supply one dimension's length.
    pub fn aspect_ratio(mut self, ratio: f32) -> Self {
        self.style.aspect_ratio = Some(ratio);
        self
    }

    /// Set the aspect ratio of the element to a square (1:1).
    pub fn square(mut self) -> Self {
        self.style.aspect_ratio = Some(1.0);
        self
    }

    pub fn p(mut self, size: Length) -> Self {
        self.style.padding = EdgeSizes::splat(size);
        self
    }

    pub fn m(mut self, size: Length) -> Self {
        self.style.margin = EdgeSizes::splat(size);
        self
    }

    /// Add a child to the element.
    pub fn child(mut self, child: ElementBuilder) -> Self {
        self.children.push(child);
        self
    }

    /// Add children to the element.
    pub fn children<I>(mut self, new_children: I) -> Self
    where
        I: IntoIterator<Item = ElementBuilder>,
    {
        let iter = new_children.into_iter();

        if let (_, Some(len)) = iter.size_hint() {
            self.children.reserve(len);
        }

        self.children.extend(iter);
        self
    }

    pub fn build(self) -> ElementTree {
        let mut tree = ElementTree::new(self.style.clone());
        let mut stack = vec![(tree.root, self.children)];

        while let Some((parent_id, mut raw_children)) = stack.pop() {
            for child_builder in raw_children.drain(..).rev() {
                let node_kind = match child_builder.node_type {
                    ElementKind::Element => NodeKind::Element {
                        style: child_builder.style,
                    },
                    ElementKind::Text => NodeKind::Text {
                        content: child_builder.text.unwrap(),
                        style: child_builder.style,
                    },
                };

                let id = tree.add_child(parent_id, node_kind);
                if !child_builder.children.is_empty() {
                    stack.push((id, child_builder.children));
                }
            }
        }
        tree
    }
}

impl Default for ElementBuilder {
    fn default() -> Self {
        Self::new()
    }
}
