use std::collections::HashMap;

use bevy::prelude::*;
use noise::Perlin;
use voxel::Voxel;

mod voxel;

pub struct MagmagridPlugin;

impl Plugin for MagmagridPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Magmagrid::default());
    }
}

#[derive(Resource)]
pub struct Magmagrid {
    pub config: MagmagridConfig,
    pub chunks_by_layer: HashMap<usize, HashMap<IVec3, VoxelChunk>>,
}

impl Magmagrid {
    pub fn new(config: MagmagridConfig) -> Self {
        Self {
            chunks_by_layer: HashMap::new(),
            config,
        }
    }
}

impl Default for Magmagrid {
    fn default() -> Self {
        Magmagrid::new(MagmagridConfig {
            scale: WorldScale {
                base_layer: 0,
                size: Vec3::ONE,
            },
            material_layer_index: 0,
            layers: vec![WorldLayer {
                name: "base".into(),
                resolution: 16,
                meshable: true,
                relative_to: None,
                inner_layer_index: None,
                meshing: Some(MeshingStrategy::Greedy),
                generator: Perlin::new(rand::random()),
            }],
        })
    }
}

pub struct MagmagridConfig {
    pub scale: WorldScale,
    pub layers: Vec<WorldLayer>,

    // terrain specific
    pub material_layer_index: usize,
}

#[derive(Debug, Clone)]
pub struct WorldLayer {
    pub name: String,
    pub resolution: u32,
    pub meshable: bool,
    pub relative_to: Option<usize>,
    pub inner_layer_index: Option<usize>,
    pub meshing: Option<MeshingStrategy>,
    pub generator: Perlin, // hardcoded perlin for now
}

#[derive(Debug, Clone)]
pub struct WorldScale {
    pub base_layer: usize,
    pub size: Vec3, // the size of a sub-voxel in meters of the base VoxelChunk
}

#[derive(Debug, Clone)]
pub struct VoxelChunk {
    pub size: UVec3,
    pub voxels: Vec<Voxel>,
    pub voxel_scale: Vec3,
    pub stitch_data: Option<StitchData>,
}

#[derive(Debug, Clone)]
pub struct StitchData {
    /// Map from cube-local IVec3 to final mesh vertex index
    pub edge_vertices: HashMap<IVec3, u32>,
    /// Map from cube-local IVec3 to shared normal vector
    pub edge_normals: HashMap<IVec3, Vec3>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MeshingStrategy {
    Greedy,
    DualContour, // placeholder for now
                 // MarchingCubes, etc.
}
