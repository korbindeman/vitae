use std::{cell::RefCell, rc::Rc};

use glam::Vec2;

use super::{
    color::ColorRGBA,
    element::{Direction, Element, ElementHandle, Size},
    style::Style,
};

/// Simple, non-typestate builder.
/// Call setters in any order; last one wins if you repeat.
#[derive(Clone, Debug)]
pub struct ElementBuilder {
    // author-time intent
    anchor: Vec2,
    style: Style,
    direction: Direction,
    width_pct: Option<f32>,
    height_pct: Option<f32>,
    aspect: Option<f32>,
    children: Vec<ElementBuilder>,
}

impl ElementBuilder {
    /// Entry point – defaults to Column, transparent background, full size.
    pub fn new() -> Self {
        Self {
            anchor: Vec2::ZERO,
            style: Style::default(),
            direction: Direction::Row,
            width_pct: None,
            height_pct: None,
            aspect: None,
            children: Vec::new(),
        }
    }

    /* ----- chainable setters (no exclusivity enforced) ------------------ */
    pub fn anchor(mut self, xy: Vec2) -> Self {
        self.anchor = xy;
        self
    }
    pub fn direction(mut self, dir: Direction) -> Self {
        self.direction = dir;
        self
    }

    pub fn background(mut self, c: ColorRGBA) -> Self {
        self.style.bg_color = c;
        self
    }

    pub fn width(mut self, pct: f32) -> Self {
        self.width_pct = Some(pct);
        self
    }
    pub fn height(mut self, pct: f32) -> Self {
        self.height_pct = Some(pct);
        self
    }
    pub fn aspect_ratio(mut self, r: f32) -> Self {
        self.aspect = Some(r);
        self
    }

    pub fn child(mut self, child: ElementBuilder) -> Self {
        self.children.push(child);
        self
    }

    pub fn children<I>(mut self, new_children: I) -> Self
    where
        I: Iterator<Item = ElementBuilder>,
    {
        self.children
            .append(&mut new_children.collect::<Vec<ElementBuilder>>());
        self
    }

    /* ----- finish ------------------------------------------------------- */
    pub fn build(self) -> ElementHandle {
        // Choose a Size – very simple rules:
        // 1) if both width & height set → Size::percent(w, h)
        // 2) else if aspect set → width = 100 %, height derived from ratio
        // 3) else if only one dim set → other = 100 %
        // 4) else full (100 %, 100 %)
        let size = match (self.width_pct, self.height_pct, self.aspect) {
            (Some(w), Some(h), _)      => Size::percent(w, h),
            (None,   None,   Some(r))  => Size::percent(100.0, 100.0 / r),
            (Some(w), None,   _)       => Size::percent(w, /*  auto  */ 100.0),
            (None,   Some(h), _)       => Size::percent(100.0, h),
            _ /* nothing set */        => Size::percent(100.0, 100.0),   // height auto
        };

        // Create the inner Element and wrap in ElementHandle
        let elem = Rc::new(RefCell::new(Element {
            parent: None,
            children: RefCell::new(Vec::new()),
            anchor: self.anchor,
            size,
            style: self.style,
            direction: self.direction,
        }));

        let handle = ElementHandle(elem.clone());

        // Build and add all children
        for child_builder in self.children {
            let child_handle = child_builder.build();
            // Set parent reference in child
            child_handle.0.borrow_mut().parent = Some(Rc::downgrade(&elem));
            // Add child to parent's children list
            elem.borrow_mut()
                .children
                .borrow_mut()
                .push(child_handle.0.clone());
        }

        handle
    }
}
