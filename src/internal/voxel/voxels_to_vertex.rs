use super::add_edge::append_edge;
use super::triangulation_table::{get_index_by_voxels, TABLE};
use super::{append_triangle::append_triangle, Voxel};
use crate::internal::chunks::Chunk;
use crate::internal::pos::{GlobalVoxelPos, VoxelPos};
use crate::plugins::game_world::resources::GameWorld;
use crate::plugins::static_mesh::components::Vertex;
use bevy::{math::Vec3, prelude::Color};

#[derive(Clone, Copy)]
struct VertexNode {
    index: usize,
    pos: Vec3,
}

const NODE_DN: VertexNode = VertexNode {
    index: 0,
    pos: Vec3 {
        x: 0.5,
        y: 0.0,
        z: 1.0,
    },
};
const NODE_DE: VertexNode = VertexNode {
    index: 1,
    pos: Vec3 {
        x: 1.0,
        y: 0.0,
        z: 0.5,
    },
};
const NODE_DS: VertexNode = VertexNode {
    index: 2,
    pos: Vec3 {
        x: 0.5,
        y: 0.0,
        z: 0.0,
    },
};
const NODE_DW: VertexNode = VertexNode {
    index: 3,
    pos: Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.5,
    },
};

const NODE_UN: VertexNode = VertexNode {
    index: 4,
    pos: Vec3 {
        x: 0.5,
        y: 1.0,
        z: 1.0,
    },
};
const NODE_UE: VertexNode = VertexNode {
    index: 5,
    pos: Vec3 {
        x: 1.0,
        y: 1.0,
        z: 0.5,
    },
};
const NODE_US: VertexNode = VertexNode {
    index: 6,
    pos: Vec3 {
        x: 0.5,
        y: 1.0,
        z: 0.0,
    },
};
const NODE_UW: VertexNode = VertexNode {
    index: 7,
    pos: Vec3 {
        x: 0.0,
        y: 1.0,
        z: 0.5,
    },
};

const NODE_NW: VertexNode = VertexNode {
    index: 8,
    pos: Vec3 {
        x: 0.0,
        y: 0.5,
        z: 1.0,
    },
};
const NODE_NE: VertexNode = VertexNode {
    index: 9,
    pos: Vec3 {
        x: 1.0,
        y: 0.5,
        z: 1.0,
    },
};
const NODE_SE: VertexNode = VertexNode {
    index: 10,
    pos: Vec3 {
        x: 1.0,
        y: 0.5,
        z: 0.0,
    },
};
const NODE_SW: VertexNode = VertexNode {
    index: 11,
    pos: Vec3 {
        x: 0.0,
        y: 0.5,
        z: 0.0,
    },
};

const NODES_POS_COUNT: usize = 12;
const BASE_NODES: [VertexNode; NODES_POS_COUNT] = [
    NODE_DN, NODE_DE, NODE_DS, NODE_DW, NODE_UN, NODE_UE, NODE_US, NODE_UW, NODE_NW, NODE_NE,
    NODE_SE, NODE_SW,
];

type Nodes = [Voxel; NODES_POS_COUNT];
type VoxelsBlock = [[[Voxel; 2]; 2]; 2];

fn get_voxel(chunk: &Chunk, pos: VoxelPos) -> Voxel {
    match chunk.get_voxel(GlobalVoxelPos::new(
        pos.x as i64,
        pos.y as i64,
        pos.z as i64,
    )) {
        Some(voxel) => voxel,
        _ => Voxel::EMPTY,
    }
}

fn get_voxels_for_vertex(chunk: &Chunk, base_pos: VoxelPos) -> VoxelsBlock {
    [
        [
            [
                get_voxel(chunk, base_pos + VoxelPos::new(0, 0, 0)),
                get_voxel(chunk, base_pos + VoxelPos::new(0, 0, 1)),
            ],
            [
                get_voxel(chunk, base_pos + VoxelPos::new(0, 1, 0)),
                get_voxel(chunk, base_pos + VoxelPos::new(0, 1, 1)),
            ],
        ],
        [
            [
                get_voxel(chunk, base_pos + VoxelPos::new(1, 0, 0)),
                get_voxel(chunk, base_pos + VoxelPos::new(1, 0, 1)),
            ],
            [
                get_voxel(chunk, base_pos + VoxelPos::new(1, 1, 0)),
                get_voxel(chunk, base_pos + VoxelPos::new(1, 1, 1)),
            ],
        ],
    ]
}

fn chose_voxel_for_node(a: Voxel, b: Voxel) -> Voxel {
    if a.is_empty() {
        return Voxel {
            color: b.color,
            value: (-a.value) / (b.value - a.value),
        };
    }
    if b.is_empty() {
        return Voxel {
            color: a.color,
            value: 1.0 - (-b.value) / (a.value - b.value),
        };
    }
    return Voxel {
        value: 0.,
        color: Color::BLACK,
    };
}

fn get_vertex_nodes(voxels: VoxelsBlock) -> Nodes {
    let mut result: Nodes = [Voxel::EMPTY; NODES_POS_COUNT];

    result[NODE_DS.index] = chose_voxel_for_node(voxels[0][0][0], voxels[1][0][0]);
    result[NODE_DE.index] = chose_voxel_for_node(voxels[1][0][0], voxels[1][0][1]);
    result[NODE_DN.index] = chose_voxel_for_node(voxels[0][0][1], voxels[1][0][1]);
    result[NODE_DW.index] = chose_voxel_for_node(voxels[0][0][0], voxels[0][0][1]);

    result[NODE_NE.index] = chose_voxel_for_node(voxels[1][0][1], voxels[1][1][1]);
    result[NODE_NW.index] = chose_voxel_for_node(voxels[0][0][1], voxels[0][1][1]);
    result[NODE_SE.index] = chose_voxel_for_node(voxels[1][0][0], voxels[1][1][0]);
    result[NODE_SW.index] = chose_voxel_for_node(voxels[0][0][0], voxels[0][1][0]);

    result[NODE_US.index] = chose_voxel_for_node(voxels[0][1][0], voxels[1][1][0]);
    result[NODE_UE.index] = chose_voxel_for_node(voxels[1][1][0], voxels[1][1][1]);
    result[NODE_UN.index] = chose_voxel_for_node(voxels[0][1][1], voxels[1][1][1]);
    result[NODE_UW.index] = chose_voxel_for_node(voxels[0][1][0], voxels[0][1][1]);

    return result;
}

fn shift_node_pos(pos: Vec3, value: f32) -> Vec3 {
    if pos.x == 0.5 {
        return Vec3::new(value, pos.y, pos.z);
    }
    if pos.y == 0.5 {
        return Vec3::new(pos.x, value, pos.z);
    }
    if pos.z == 0.5 {
        return Vec3::new(pos.x, pos.y, value);
    }

    panic!("failed to process pos {:?}", pos);
}

fn append_voxel_triangle(
    pos: VoxelPos,
    vertices: &mut Vec<Vertex>,
    nodes: Nodes,
    a: VertexNode,
    b: VertexNode,
    c: VertexNode,
    scale: f32,
    with_edges: bool,
) {
    let a_v = nodes[a.index];
    let b_v = nodes[b.index];
    let c_v = nodes[c.index];

    if a_v.is_empty() || a_v.is_empty() || c_v.is_empty() {
        return;
    }

    let pos_vec = Vec3::new(pos.x as f32, pos.y as f32, pos.z as f32);

    let a_pos = shift_node_pos(a.pos, a_v.value) + pos_vec;
    let b_pos = shift_node_pos(b.pos, b_v.value) + pos_vec;
    let c_pos = shift_node_pos(c.pos, c_v.value) + pos_vec;

    let color = a_v.color;
    let scale = Voxel::SCALE * scale;
    let normal = append_triangle(vertices, scale, color, a_pos, b_pos, c_pos);

    if with_edges {
        append_edge(vertices, color, scale, pos, normal, a_pos, b_pos, c_pos);
    }
}

pub fn append_vertex(pos: VoxelPos, chunk: &Chunk, vertices: &mut Vec<Vertex>, level: usize) {
    let scale = GameWorld::level_to_scale(level) as f32;
    let voxels = get_voxels_for_vertex(chunk, pos);
    let nodes = get_vertex_nodes(voxels);

    let triangle_points = TABLE[get_index_by_voxels(voxels)];

    let mut triangle_offset = 0;

    while triangle_points[triangle_offset] != -1 {
        let a = BASE_NODES[triangle_points[triangle_offset] as usize];
        let b = BASE_NODES[triangle_points[triangle_offset + 1] as usize];
        let c = BASE_NODES[triangle_points[triangle_offset + 2] as usize];

        append_voxel_triangle(
            pos,
            vertices,
            nodes,
            a,
            b,
            c,
            scale,
            level != GameWorld::MAX_DETAIL_LEVEL,
        );

        triangle_offset += 3;
    }
}
