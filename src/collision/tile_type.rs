use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TileType {
    #[default]
    Empty,
    Dirt,
    Grass,
    YellowGrass,
    Shore,
    Water,
    Tree,
    Rock,
}

impl TileType {
    pub fn is_walkable(&self) -> bool {
        !matches!(self, Self::Water | Self::Tree | Self::Rock)
    }

    pub fn collision_adjustment(&self) -> f32 {
        match self {
            Self::Tree | Self::Rock => -0.2,
            _ => 0.,
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct TileMarker {
    pub tile_type: TileType,
}

impl TileMarker {
    pub fn new(tile_type: TileType) -> Self {
        Self { tile_type }
    }
}
