use glam::Vec2;

use super::draw::DrawCommand;

#[derive(Clone)]
pub enum Size {
    Pixel(Vec2),
    Percentage(Vec2),
}

#[derive(Clone)]
pub struct Element {
    anchor: Vec2,
    color: [f32; 4],
    children: Vec<Element>,
    size: Size,
}

impl Element {
    pub fn new(color: [f32; 4], children: Vec<Element>, size: Size) -> Self {
        Element {
            anchor: Vec2::splat(0.),
            color,
            children,
            size,
        }
    }

    fn get_draw_command(&self) -> DrawCommand {
        let (width_fraction, height_fraction) = match &self.size {
            Size::Percentage(size) => (size.x / 50., size.y / 50.),
            Size::Pixel(size) => (size.x / 50., size.y / 50.),
        };

        DrawCommand::Rect {
            x: -1.0 + self.anchor.x,
            y: 1.0 - height_fraction - self.anchor.y,
            width: width_fraction,
            height: height_fraction,
            color: self.color,
        }
    }
}

pub fn tree_to_draw_commands(root: &Element) -> Vec<DrawCommand> {
    let mut draw_commands = vec![root.get_draw_command()];

    for child in root.children.iter() {
        let child_commands = tree_to_draw_commands(child);
        draw_commands.extend(child_commands);
    }

    draw_commands
}
