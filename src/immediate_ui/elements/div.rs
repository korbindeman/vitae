use glam::Vec2;

use crate::immediate_ui::{
    color::ColorRGBA,
    element::{Direction, ElementHandle, Size},
};

pub fn div(color: ColorRGBA, size: Size, direction: Direction) -> ElementHandle {
    ElementHandle::new_root(Vec2::ZERO, color, size, direction)
}
