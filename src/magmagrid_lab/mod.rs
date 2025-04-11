use std::collections::HashMap;

use bevy::prelude::*;
use noise::Perlin;
use voxel::Voxel;

mod voxel;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum MagmagridInitSet {
    LoadTerrain,
    DrawTerrain,
}

pub struct MagmagridPlugin;

impl Plugin for MagmagridPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Startup,
            (MagmagridInitSet::LoadTerrain, MagmagridInitSet::DrawTerrain),
        );

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
        let terrain_layer = TerrainLayer {
            meshable: true,
            meshing: Some(MeshingStrategy::Greedy),
            generator: Perlin::new(rand::random()),
        };

        Magmagrid::new(MagmagridConfig {
            scale: WorldScale {
                base_layer: 0,
                size: Vec3::ONE,
            },
            layers: vec![WorldLayer {
                name: "base".into(),
                resolution: 16,
                relative_to: None,
                inner_layer_index: None,
                layer: terrain_layer,
            }],
        })
    }
}

pub struct MagmagridConfig {
    pub scale: WorldScale,
    pub layers: Vec<WorldLayer>,
    // terrain specific
}

impl MagmagridConfig {
    pub fn outermost_layer_index(&self) -> usize {
        /// Because layers can be defined in any order we need to follow the chain
        fn chain_depth(mut index: usize, layers: &[WorldLayer]) -> usize {
            let mut depth = 0;
            while let Some(inner) = layers.get(index).and_then(|l| l.inner_layer_index) {
                index = inner;
                depth += 1;
            }
            depth
        }

        self.layers
            .iter()
            .enumerate()
            .max_by_key(|(i, _)| chain_depth(*i, &self.layers))
            .map(|(i, _)| i)
            .expect("No layers defined in WorldConfig")
    }
}

/// defines a layer in the world and its basic spatial information.
///
/// layer is where the actual behavior and data of the layer will live.
///
/// defaults to a terrain layer.
#[derive(Debug, Clone)]
pub struct WorldLayer<LayerType = TerrainLayer> {
    pub name: String,
    pub resolution: u32,
    pub relative_to: Option<usize>,
    pub inner_layer_index: Option<usize>,
    pub layer: LayerType,
}

#[derive(Debug, Clone)]
pub struct TerrainLayer {
    pub generator: Perlin, // hardcoded perlin for now
    pub meshable: bool,
    pub meshing: Option<MeshingStrategy>,
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
