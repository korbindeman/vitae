use crate::core::element::{ElementId, ElementTree};

pub enum DrawCommand {
    Rect {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        color: [f32; 4],
    },
    // … later: Glyph { atlas_uv: […], x,y,w,h, color }
}

pub fn push_draw_commands(
    tree: &ElementTree,
    id: ElementId,
    cmds: &mut Vec<DrawCommand>,
    viewport_w: f32,
    viewport_h: f32,
) {
    let node = tree.get_node(id);
    let layout = node.layout;

    // convert to NDC
    let ndc_x = -1.0 + 2.0 * (layout.x / viewport_w);
    let ndc_y = 1.0 - 2.0 * ((layout.y + layout.height) / viewport_h);
    let ndc_width = 2.0 * (layout.width / viewport_w);
    let ndc_height = 2.0 * (layout.height / viewport_h);

    // emit a command (use padding / border if you add them later)
    cmds.push(DrawCommand::Rect {
        x: ndc_x,
        y: ndc_y,
        width: ndc_width,
        height: ndc_height,
        color: node.style.bg_color.to_array(),
    });

    // recurse over children
    let mut child = node.first_child;
    while let Some(id) = child {
        push_draw_commands(tree, id, cmds, viewport_w, viewport_h);
        child = tree.get_node(id).next_sibling;
    }
}
