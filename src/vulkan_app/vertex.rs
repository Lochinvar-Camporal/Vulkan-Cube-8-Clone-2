use ash::vk;
use std::mem::offset_of;

#[derive(Clone, Debug, Copy)]
#[repr(C)]
pub struct Vertex {
    pub pos: [f32; 3],
    pub color: [f32; 3],
}

impl Vertex {
    pub fn get_binding_description() -> vk::VertexInputBindingDescription {
        vk::VertexInputBindingDescription::builder()
            .binding(0)
            .stride(std::mem::size_of::<Self>() as u32)
            .input_rate(vk::VertexInputRate::VERTEX)
            .build()
    }

    pub fn get_attribute_descriptions() -> [vk::VertexInputAttributeDescription; 2] {
        [
            vk::VertexInputAttributeDescription::builder()
                .binding(0)
                .location(0)
                .format(vk::Format::R32G32B32_SFLOAT)
                .offset(offset_of!(Self, pos) as u32)
                .build(),
            vk::VertexInputAttributeDescription::builder()
                .binding(0)
                .location(1)
                .format(vk::Format::R32G32B32_SFLOAT)
                .offset(offset_of!(Self, color) as u32)
                .build(),
        ]
    }
}

pub const VERTICES: [Vertex; 8] = [
    Vertex {
        pos: [-0.5, -0.5, 0.5],
        color: [0.298, 0.686, 0.314],
    },
    Vertex {
        pos: [0.5, -0.5, 0.5],
        color: [0.298, 0.686, 0.314],
    },
    Vertex {
        pos: [0.5, 0.5, 0.5],
        color: [0.298, 0.686, 0.314],
    },
    Vertex {
        pos: [-0.5, 0.5, 0.5],
        color: [0.298, 0.686, 0.314],
    },
    Vertex {
        pos: [-0.5, -0.5, -0.5],
        color: [0.298, 0.686, 0.314],
    },
    Vertex {
        pos: [0.5, -0.5, -0.5],
        color: [0.298, 0.686, 0.314],
    },
    Vertex {
        pos: [0.5, 0.5, -0.5],
        color: [0.298, 0.686, 0.314],
    },
    Vertex {
        pos: [-0.5, 0.5, -0.5],
        color: [0.298, 0.686, 0.314],
    },
];

pub const INDICES: [u16; 36] = [
    0, 1, 2, 2, 3, 0, // front
    4, 6, 5, 4, 7, 6, // back
    0, 7, 4, 0, 3, 7, // left
    1, 5, 6, 6, 2, 1, // right
    3, 2, 6, 6, 7, 3, // top
    0, 5, 1, 5, 0, 4, // bottom
];

pub fn generate_wireframe_vertices(divisions: u32) -> Vec<Vertex> {
    let color = [0.298, 0.686, 0.314];
    let mut vertices = Vec::new();
    let step = 1.0 / divisions as f32;
    for i in 0..=divisions {
        let a = -0.5 + i as f32 * step;
        for j in 0..=divisions {
            let b = -0.5 + j as f32 * step;
            // lines parallel to x
            vertices.push(Vertex { pos: [-0.5, a, b], color });
            vertices.push(Vertex { pos: [0.5, a, b], color });
            // lines parallel to y
            vertices.push(Vertex { pos: [b, -0.5, a], color });
            vertices.push(Vertex { pos: [b, 0.5, a], color });
            // lines parallel to z
            vertices.push(Vertex { pos: [a, b, -0.5], color });
            vertices.push(Vertex { pos: [a, b, 0.5], color });
        }
    }
    vertices
}
