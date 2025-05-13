use super::draw::DrawCommand;

#[derive(Clone)]
pub struct Element {
    name: String,
    color: [f32; 4],
    children: Vec<Element>,
    width: u8,
    height: u8,
}

impl Element {
    pub fn new(name: &str, color: [f32; 4], children: Vec<Element>, width: u8, height: u8) -> Self {
        if width > 100 {
            panic!("Element width can't be larger than 100")
        }
        if height > 100 {
            panic!("Element width can't be larger than 100")
        }

        Element {
            name: name.to_string(),
            color,
            children,
            width,
            height,
        }
    }

    fn get_draw_command(&self) -> DrawCommand {
        let width_fraction = self.width as f32 / 50.;
        let height_fraction = self.height as f32 / 50.;

        DrawCommand::Rect {
            x: -1.0,
            y: 1.0 - height_fraction,
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
