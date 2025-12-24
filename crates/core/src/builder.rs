use std::any::Any;
use std::rc::Rc;

use crate::color::Color;
use crate::element::{ElementTree, NodeKind};
use crate::events::{Event, EventHandler, EventResult, MouseButton};
use crate::style::{Align, Direction, Distribute, EdgeSizes, Length, Position, Style};
use crate::svg_data::Svg;
use crate::texture::Texture;

#[derive(Clone, Debug)]
enum ElementKind {
    Element,
    Text,
    Texture,
    Svg,
}

#[derive(Clone)]
pub struct ElementBuilder {
    node_type: ElementKind,
    style: Style,
    text: Option<String>,
    texture: Option<Texture>,
    svg: Option<Svg>,
    children: Vec<ElementBuilder>,
    on_event: Option<EventHandler>,
}

// Manual Debug implementation since EventHandler doesn't implement Debug
impl std::fmt::Debug for ElementBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ElementBuilder")
            .field("node_type", &self.node_type)
            .field("style", &self.style)
            .field("text", &self.text)
            .field("texture", &self.texture)
            .field("svg", &self.svg)
            .field("children", &self.children)
            .field("on_event", &self.on_event.as_ref().map(|_| "EventHandler"))
            .finish()
    }
}

impl ElementBuilder {
    pub fn new() -> Self {
        Self {
            node_type: ElementKind::Element,
            style: Style::default(),
            text: None,
            texture: None,
            svg: None,
            children: Vec::new(),
            on_event: None,
        }
    }

    pub fn new_text(text: String) -> Self {
        Self {
            node_type: ElementKind::Text,
            style: Style::default(),
            text: Some(text),
            texture: None,
            svg: None,
            children: Vec::new(),
            on_event: None,
        }
    }

    pub fn new_texture(texture: Texture) -> Self {
        Self {
            node_type: ElementKind::Texture,
            style: Style::default(),
            text: None,
            texture: Some(texture),
            svg: None,
            children: Vec::new(),
            on_event: None,
        }
    }

    pub fn new_svg(svg: Svg) -> Self {
        Self {
            node_type: ElementKind::Svg,
            style: Style::default(),
            text: None,
            texture: None,
            svg: Some(svg),
            children: Vec::new(),
            on_event: None,
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

    /// Set cross-axis alignment for children (CSS: align-items).
    pub fn align(mut self, align: Align) -> Self {
        self.style.align = align;
        self
    }

    /// Set main-axis distribution of children (CSS: justify-content).
    pub fn distribute(mut self, distribute: Distribute) -> Self {
        self.style.distribute = distribute;
        self
    }

    /// Center children on both axes.
    pub fn center(mut self) -> Self {
        self.style.align = Align::Center;
        self.style.distribute = Distribute::Center;
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

    /// Set the position mode.
    pub fn position(mut self, position: Position) -> Self {
        self.style.position = position;
        self
    }

    /// Set position to absolute.
    pub fn absolute(mut self) -> Self {
        self.style.position = Position::Absolute;
        self
    }

    /// Set the top offset (for absolute positioning).
    pub fn top(mut self, value: Length) -> Self {
        self.style.top = Some(value);
        self
    }

    /// Set the right offset (for absolute positioning).
    pub fn right(mut self, value: Length) -> Self {
        self.style.right = Some(value);
        self
    }

    /// Set the bottom offset (for absolute positioning).
    pub fn bottom(mut self, value: Length) -> Self {
        self.style.bottom = Some(value);
        self
    }

    /// Set the left offset (for absolute positioning).
    pub fn left(mut self, value: Length) -> Self {
        self.style.left = Some(value);
        self
    }

    /// Set the font size for text elements.
    pub fn font_size(mut self, size: f32) -> Self {
        self.style.font_size = Some(size);
        self
    }

    /// Set the gap between children on both axes.
    pub fn gap(mut self, length: Length) -> Self {
        self.style.gap_x = length;
        self.style.gap_y = length;
        self
    }

    /// Set the horizontal gap between children.
    pub fn gap_x(mut self, length: Length) -> Self {
        self.style.gap_x = length;
        self
    }

    /// Set the vertical gap between children.
    pub fn gap_y(mut self, length: Length) -> Self {
        self.style.gap_y = length;
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

    /// Attach a generic event handler that receives all events.
    ///
    /// This is the foundation for all event handling. Typed helpers like
    /// `on_click` are convenience wrappers around this method.
    ///
    /// # Example
    /// ```
    /// div().on_event(|model: &mut MyModel, event: &Event| {
    ///     match event {
    ///         Event::Click => { model.count += 1; }
    ///     }
    ///     EventResult::Continue
    /// })
    /// ```
    pub fn on_event<M, F>(mut self, handler: F) -> Self
    where
        M: 'static,
        F: Fn(&mut M, &Event) -> EventResult + 'static,
    {
        self.on_event = Some(Rc::new(move |model: &mut dyn Any, event: &Event| {
            if let Some(m) = model.downcast_mut::<M>() {
                handler(m, event)
            } else {
                EventResult::Continue
            }
        }));
        self
    }

    /// Attach a left click event handler.
    ///
    /// # Example
    /// ```
    /// button("Click me").on_left_click(MyModel::increment)
    /// ```
    pub fn on_left_click<M, F>(self, handler: F) -> Self
    where
        M: 'static,
        F: Fn(&mut M) + 'static,
    {
        self.on_event(move |model: &mut M, event: &Event| {
            if matches!(
                event,
                Event::Click {
                    button: MouseButton::Left
                }
            ) {
                handler(model);
            }
            EventResult::Continue
        })
    }

    /// Attach a right click event handler.
    ///
    /// # Example
    /// ```
    /// button("Options").on_right_click(MyModel::show_context_menu)
    /// ```
    pub fn on_right_click<M, F>(self, handler: F) -> Self
    where
        M: 'static,
        F: Fn(&mut M) + 'static,
    {
        self.on_event(move |model: &mut M, event: &Event| {
            if matches!(
                event,
                Event::Click {
                    button: MouseButton::Right
                }
            ) {
                handler(model);
            }
            EventResult::Continue
        })
    }

    /// Get the event handler (used internally for event dispatch).
    pub fn get_event_handler(&self) -> Option<EventHandler> {
        self.on_event.clone()
    }

    pub fn build(self) -> ElementTree {
        let mut tree = ElementTree::new(self.style.clone(), self.on_event.clone());
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
                    ElementKind::Texture => NodeKind::Texture {
                        texture: child_builder.texture.unwrap(),
                        style: child_builder.style,
                    },
                    ElementKind::Svg => NodeKind::Svg {
                        svg: child_builder.svg.unwrap(),
                        style: child_builder.style,
                    },
                };

                let id = tree.add_child(parent_id, node_kind, child_builder.on_event.clone());
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
