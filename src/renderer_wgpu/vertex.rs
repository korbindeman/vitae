use crate::core::draw::DrawCommand;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub pos: [f32; 2],
    pub color: [f32; 4],
}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x4];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

pub fn build_mesh(commands: &[DrawCommand]) -> (Vec<Vertex>, Vec<u16>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    for cmd in commands {
        match *cmd {
            DrawCommand::Rect {
                x,
                y,
                width,
                height,
                color,
            } => {
                // base index for this quad
                let base = vertices.len() as u16;

                // push the four corners in pixel space
                vertices.push(Vertex { pos: [x, y], color });
                vertices.push(Vertex {
                    pos: [x + width, y],
                    color,
                });
                vertices.push(Vertex {
                    pos: [x + width, y + height],
                    color,
                });
                vertices.push(Vertex {
                    pos: [x, y + height],
                    color,
                });

                // two triangles: (0,1,2) and (2,3,0)
                indices.extend_from_slice(&[base, base + 1, base + 2, base + 2, base + 3, base]);
            }
        }
    }

    (vertices, indices)
}
