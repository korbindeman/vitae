use crate::immediate_ui::style::EdgeSizes;

use super::{
    color::Color,
    element::ElementTree,
    style::{Direction, Length, Style},
};

// TODO: use typestate to disallow invalid combinations
#[derive(Clone, Debug)]
pub struct ElementBuilder {
    style: Style,
    children: Vec<ElementBuilder>,
}

impl ElementBuilder {
    pub fn new() -> Self {
        Self {
            style: Style::default(),
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
    pub fn bg(mut self, c: Color) -> Self {
        self.style.bg_color = c;
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

    // pub fn aspect_ratio(mut self, ratio: f32) -> Self {
    //     todo!()
    // }

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

    /// Add a children to the element.
    pub fn children<I>(mut self, new_children: I) -> Self
    where
        I: IntoIterator<Item = ElementBuilder>,
    {
        let iter = new_children.into_iter();

        // if the iterator can tell us its exact length, pre-reserve
        if let (_, Some(len)) = iter.size_hint() {
            self.children.reserve(len);
        }

        self.children.extend(iter);
        self
    }

    pub fn build(self) -> ElementTree {
        let mut tree = ElementTree::new(self.style.clone()); // root node
        let mut stack = vec![(tree.root, self.children)]; // DFS

        while let Some((parent_id, mut raw_children)) = stack.pop() {
            // iterate in reverse to preserve source order when we push_front
            for child_builder in raw_children.drain(..).rev() {
                let id = tree.add_child(parent_id, child_builder.style.clone());
                if !child_builder.children.is_empty() {
                    stack.push((id, child_builder.children));
                }
            }
        }
        tree
    }
}
