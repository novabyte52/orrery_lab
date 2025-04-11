// magmagrid/voxel.rs

use bevy::prelude::*;

/// Represents the type of material contained in a voxel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VoxelMaterial {
    Air,
    Dirt,
    Stone,
    Granite,
    Magma,
}

impl VoxelMaterial {
    pub fn default_density(self) -> f32 {
        match self {
            VoxelMaterial::Air => 1.0,
            VoxelMaterial::Dirt => -1.0,
            VoxelMaterial::Stone => -1.5,
            VoxelMaterial::Granite => -2.0,
            VoxelMaterial::Magma => -0.5,
        }
    }
}

impl Default for VoxelMaterial {
    fn default() -> Self {
        VoxelMaterial::Air
    }
}

/// A single voxel, containing data for simulation and rendering.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Voxel {
    pub material: VoxelMaterial,
    pub density: f32,
    // Additional simulation properties can be added here
}

impl Voxel {
    pub fn new(material: VoxelMaterial) -> Self {
        Self {
            material,
            density: 1.0,
        }
    }

    pub fn air() -> Self {
        Self::new(VoxelMaterial::Air)
    }

    pub fn is_air(&self) -> bool {
        self.material == VoxelMaterial::Air
    }
}

impl Default for Voxel {
    fn default() -> Self {
        Voxel::air()
    }
}
